//! HTTP front end for the solver.
//!
//! A page posts a problem; the server answers from a cache directory or runs the algorithm and
//! reports how many blocks are finished while it runs. Standard library only: a listener, a
//! small request parser, a directory of cached documents and a mutex-guarded job table.
//!
//! ```text
//! bridge_server [port] [cache-dir]     # defaults: 8787, cache
//! ```

use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use stable_bridge::convex_polygon;
use stable_bridge::input_parser;
use stable_bridge::output_writer::{self, Header};
use stable_bridge::StableBridge;

/// Port used when none is given.
const DEFAULT_PORT: u16 = 8787;
/// Largest request head accepted, in bytes.
const MAX_HEADER: usize = 64 * 1024;
/// Largest problem accepted, in bytes.
const MAX_BODY: usize = 8 * 1024 * 1024;

/// Where a job is in its life.
#[derive(Clone, Debug)]
enum State {
    Running,
    Done,
    Failed(String),
}

impl State {
    fn name(&self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Done => "done",
            Self::Failed(_) => "error",
        }
    }
}

/// One request for one case.
#[derive(Clone, Debug)]
struct Job {
    state: State,
    /// Blocks finished so far.
    done: usize,
    /// Blocks the run will produce; `0` until the input has been parsed.
    total: usize,
    /// `"hit"` when it was answered from disk, `"miss"` when it is computed.
    cache: &'static str,
    started: Instant,
    elapsed_ms: Option<u64>,
    output: Option<String>,
}

type Jobs = Arc<Mutex<HashMap<String, Job>>>;

fn main() {
    let mut args = std::env::args().skip(1);
    let port = args
        .next()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);
    let cache_dir = PathBuf::from(args.next().unwrap_or_else(|| String::from("cache")));

    if let Err(error) = fs::create_dir_all(&cache_dir) {
        eprintln!(
            "error: cannot create cache directory {}: {error}",
            cache_dir.display()
        );
        std::process::exit(1);
    }

    let listener = match TcpListener::bind(("127.0.0.1", port)) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("error: cannot listen on 127.0.0.1:{port}: {error}");
            std::process::exit(1);
        }
    };

    eprintln!(
        "bridge_server listening on http://127.0.0.1:{port} (cache {})",
        cache_dir.display()
    );

    let jobs: Jobs = Arc::new(Mutex::new(HashMap::new()));
    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let jobs = Arc::clone(&jobs);
        let cache_dir = cache_dir.clone();
        thread::spawn(move || {
            let _ = handle(&mut stream, &jobs, &cache_dir, port);
        });
    }
}

/// Routes one HTTP request. Every response carries permissive CORS headers, because the page
/// is normally served by another process (a static server on port 8000).
fn handle(
    stream: &mut TcpStream,
    jobs: &Jobs,
    cache_dir: &PathBuf,
    port: u16,
) -> std::io::Result<()> {
    let Some((method, target, body)) = read_request(stream)? else {
        return Ok(());
    };
    let (path, query) = target.split_once('?').unwrap_or((target.as_str(), ""));

    if method == "OPTIONS" {
        send(stream, "204 No Content", "text/plain; charset=utf-8", "");
        return Ok(());
    }

    match (method.as_str(), path) {
        ("GET", "/api/health") => {
            let body = format!(
                "{{\"ok\":true,\"service\":\"bridge_server\",\"port\":{port},\"cache_dir\":{}}}",
                json_string(&cache_dir.display().to_string())
            );
            send(stream, "200 OK", "application/json", &body);
        }
        ("GET", "/api/progress") => {
            let id = param(query, "id");
            match jobs.lock().unwrap().get(&id) {
                Some(job) => send(stream, "200 OK", "application/json", &job_json(job)),
                None => send(
                    stream,
                    "404 Not Found",
                    "application/json",
                    "{\"error\":\"unknown job\"}",
                ),
            }
        }
        ("GET", "/api/result") => {
            let id = param(query, "id");
            let store = jobs.lock().unwrap();
            match store.get(&id) {
                Some(job) => match (&job.state, &job.output) {
                    (State::Done, Some(output)) => {
                        send(stream, "200 OK", "text/plain; charset=utf-8", output)
                    }
                    (State::Failed(message), _) => send(
                        stream,
                        "500 Internal Server Error",
                        "text/plain; charset=utf-8",
                        message,
                    ),
                    _ => send(
                        stream,
                        "409 Conflict",
                        "application/json",
                        "{\"state\":\"running\"}",
                    ),
                },
                None => send(
                    stream,
                    "404 Not Found",
                    "application/json",
                    "{\"error\":\"unknown job\"}",
                ),
            }
        }
        ("POST", "/api/run") => {
            if body.trim().is_empty() {
                send(
                    stream,
                    "400 Bad Request",
                    "application/json",
                    "{\"error\":\"empty problem\"}",
                );
                return Ok(());
            }
            start_run(body, query, jobs, cache_dir, stream);
        }
        _ => send(
            stream,
            "404 Not Found",
            "application/json",
            "{\"error\":\"no such route\"}",
        ),
    }
    Ok(())
}

/// Answers `POST /api/run`: hit the cache, reuse a live job, or start a computation.
///
/// The case key is the hash of the request body, so two requests for the same text are the
/// same case. `?invalidate=1` deletes that case's cached document before the run and is
/// therefore the only way to force a recomputation.
fn start_run(body: String, query: &str, jobs: &Jobs, cache_dir: &PathBuf, stream: &mut TcpStream) {
    let raw = param(query, "invalidate");
    let invalidate = !raw.is_empty() && raw != "0";
    let key = hash_key(&body);
    let cache_path = cache_dir.join(format!("{key}.out.txt"));

    {
        let mut store = jobs.lock().unwrap();

        if let Some(job) = store.get(&key) {
            if matches!(job.state, State::Running) {
                let answer = format!("{{\"id\":\"{key}\",\"cache\":\"{}\"}}", job.cache);
                send(stream, "200 OK", "application/json", &answer);
                return;
            }
        }

        if !invalidate {
            if let Ok(output) = fs::read_to_string(&cache_path) {
                let total = blocks_in_document(&output);
                store.insert(
                    key.clone(),
                    Job {
                        state: State::Done,
                        done: total,
                        total,
                        cache: "hit",
                        started: Instant::now(),
                        elapsed_ms: Some(0),
                        output: Some(output),
                    },
                );
                let answer = format!("{{\"id\":\"{key}\",\"cache\":\"hit\"}}");
                send(stream, "200 OK", "application/json", &answer);
                return;
            }
        }

        store.insert(
            key.clone(),
            Job {
                state: State::Running,
                done: 0,
                total: 0,
                cache: "miss",
                started: Instant::now(),
                elapsed_ms: None,
                output: None,
            },
        );
    }

    // Removing the old answer before the run means a crash cannot leave a stale document that
    // the next request would serve as a cache hit.
    let _ = fs::remove_file(&cache_path);

    let answer = format!("{{\"id\":\"{key}\",\"cache\":\"miss\"}}");
    send(stream, "200 OK", "application/json", &answer);

    let jobs = Arc::clone(jobs);
    thread::spawn(move || compute(body, key, cache_path, jobs));
}

/// Runs one case and records the outcome, the elapsed time and the cached document.
fn compute(input: String, key: String, cache_path: PathBuf, jobs: Jobs) {
    let started = Instant::now();
    let outcome = analyse(&input, &key, &jobs);
    let elapsed = started.elapsed().as_millis() as u64;

    let mut store = jobs.lock().unwrap();
    let Some(job) = store.get_mut(&key) else { return };
    job.elapsed_ms = Some(elapsed);
    match outcome {
        Ok(output) => {
            let _ = fs::write(&cache_path, &output);
            job.state = State::Done;
            job.done = job.total;
            job.output = Some(output);
        }
        Err(message) => job.state = State::Failed(message),
    }
}

/// The computation itself: parse, run the blocks, format the document.
fn analyse(input: &str, key: &str, jobs: &Jobs) -> Result<String, String> {
    let data = input_parser::parse(input).map_err(|error| error.to_string())?;
    let problem = StableBridge::new(&data).map_err(|error| error.to_string())?;
    let total = data.block_count();
    if total == 0 {
        return Err(String::from(
            "T / Step gives no blocks: check the `time` and `step` lines",
        ));
    }

    if let Some(job) = jobs.lock().unwrap().get_mut(key) {
        job.total = total;
    }

    let mut rendered: Vec<Vec<Vec<f64>>> = Vec::with_capacity(total);
    problem
        .blocks_with(total, data.step, |_block, _count, support| {
            rendered.push(convex_polygon::reconstruct(&problem.directions, support));
            if let Some(job) = jobs.lock().unwrap().get_mut(key) {
                job.done = rendered.len();
            }
        })
        .map_err(|error| error.to_string())?;

    Ok(output_writer::format_document(
        &Header {
            dimension: data.n,
            terminal_time: data.t,
            step: data.step,
            blocks: rendered.len(),
            directions: problem.directions.len(),
        },
        &rendered,
    ))
}

/// A case key: FNV-1a over the request body, hex encoded.
///
/// Deterministic across processes, which a `DefaultHasher` is not.
fn hash_key(text: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Blocks a document declares, read from its `blocks` line.
fn blocks_in_document(text: &str) -> usize {
    text.lines()
        .find_map(|line| line.strip_prefix("blocks "))
        .and_then(|value| value.trim().parse().ok())
        .unwrap_or(0)
}

/// One query parameter, without decoding: ids are hex and the only flag is `1`.
fn param(query: &str, name: &str) -> String {
    query
        .split('&')
        .filter_map(|pair| pair.split_once('='))
        .find(|(key, _)| *key == name)
        .map(|(_, value)| value.to_string())
        .unwrap_or_default()
}

fn job_json(job: &Job) -> String {
    let elapsed = job
        .elapsed_ms
        .unwrap_or_else(|| job.started.elapsed().as_millis() as u64);
    let message = match &job.state {
        State::Failed(message) => json_string(message),
        _ => String::from("null"),
    };
    format!(
        "{{\"state\":\"{}\",\"blocks\":{},\"total\":{},\"elapsed_ms\":{},\"cache\":\"{}\",\"message\":{}}}",
        job.state.name(),
        job.done,
        job.total,
        elapsed,
        job.cache,
        message
    )
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for character in value.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            other if (other as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", other as u32));
            }
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

fn send(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
    let head = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\n\
         Access-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
    let _ = stream.flush();
}

/// Reads one request: method, target and body. Returns `None` for anything unreadable.
fn read_request(stream: &mut TcpStream) -> std::io::Result<Option<(String, String, String)>> {
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;

    let mut data: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 8192];
    let header_end = loop {
        if let Some(position) = data.windows(4).position(|window| window == b"\r\n\r\n") {
            break position + 4;
        }
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            return Ok(None);
        }
        data.extend_from_slice(&chunk[..read]);
        if data.len() > MAX_HEADER + 4 {
            return Ok(None);
        }
    };

    let head = String::from_utf8_lossy(&data[..header_end]).to_string();
    let mut lines = head.split("\r\n");
    let mut parts = lines.next().unwrap_or_default().split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or("/").to_string();

    let mut content_length = 0usize;
    for line in lines {
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().unwrap_or(0);
            }
        }
    }
    if content_length > MAX_BODY {
        return Ok(None);
    }

    let mut body = data[header_end..].to_vec();
    while body.len() < content_length {
        let read = stream.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        body.extend_from_slice(&chunk[..read]);
    }

    Ok(Some((
        method,
        target,
        String::from_utf8_lossy(&body).to_string(),
    )))
}
