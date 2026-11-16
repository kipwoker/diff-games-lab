//! Command-line entry point.
//!
//! Reads a problem, computes the terminal support vector, then for every printed block
//! advances the recursion and writes the vertex set.
//!
//! Defaults are `problem.txt` and `out.txt`; both can be overridden positionally.
//! `--certify x,y` runs the closed loop from that state instead and reports the slack of the
//! bridge at every block, which is track 4's certificate.

use std::env;
use std::fs;
use std::process::ExitCode;

use stable_bridge::convex_polygon;
use stable_bridge::error;
use stable_bridge::input_parser;
use stable_bridge::nonlinear;
use stable_bridge::output_writer::{self, Header};
use stable_bridge::strategy::{self, Rule};
use stable_bridge::{StableBridge, DIRECTION_COUNT};

fn print_usage() {
    eprintln!("usage: stable_bridge [input] [output] [options]");
    eprintln!();
    eprintln!("  input    problem.txt unless given");
    eprintln!("  output   out.txt unless given");
    eprintln!();
    eprintln!("  --certify <x1,x2,...>  run the closed loop from this state and report slacks");
    eprintln!("  --rule <counter|max|predictive>  pursuer's vertex rule (default counter)");
    eprintln!("  --allowance <a>        override the computed allowance (reported as manual)");
    eprintln!("  --play <x1,x2,...>     write the closed-loop trajectory from this state");
    eprintln!("  --search <trials>      adversarial evader search from the --certify state");
    eprintln!("  --seed <n>             search seed (default 1)");
    eprintln!("  --bracket              per-block two-sided band of the printed set");
    eprintln!("  --trace <path>         per-block, per-direction shortfall of the recursion");
    eprintln!("  --directions <n>      2D table size (default 110; 3D is fixed at 110)");
    eprintln!("  --table <az>x<el>     3D table shape (default 11x10); 2D uses --directions");
}

/// One parsed command line.
struct Options {
    input: String,
    output: Option<String>,
    certify: Option<Vec<f64>>,
    play: Option<Vec<f64>>,
    search: Option<u64>,
    seed: u64,
    bracket: bool,
    trace: Option<String>,
    directions: usize,
    table: Option<(usize, usize)>,
    rule: Rule,
    allowance: Option<f64>,
}

/// `max_psi (h_outer - h_inner)` for two 2D polygons, exactly.
///
/// Swapping the maxima turns the support gap into the largest distance from a vertex of
/// `outer` to the polygon `inner`, which is zero for a vertex already inside. `inner`'s vertices
/// are in enumeration order, so they are ordered around their centroid first; a convex polygon's
/// vertices are in convex position, so that order is its boundary.
fn one_sided(outer: &[Vec<f64>], inner: &[Vec<f64>]) -> f64 {
    if inner.len() < 3 || outer.is_empty() {
        return f64::INFINITY;
    }
    let count = inner.len() as f64;
    let cx = inner.iter().map(|vertex| vertex[0]).sum::<f64>() / count;
    let cy = inner.iter().map(|vertex| vertex[1]).sum::<f64>() / count;
    let mut ring: Vec<(f64, usize)> = inner
        .iter()
        .enumerate()
        .map(|(index, vertex)| ((vertex[1] - cy).atan2(vertex[0] - cx), index))
        .collect();
    ring.sort_by(|left, right| left.0.total_cmp(&right.0));

    let mut worst = 0.0_f64;
    for point in outer {
        let mut inside = true;
        let mut closest = f64::INFINITY;
        for position in 0..ring.len() {
            let start = &inner[ring[position].1];
            let end = &inner[ring[(position + 1) % ring.len()].1];
            let ex = end[0] - start[0];
            let ey = end[1] - start[1];
            let length = (ex * ex + ey * ey).sqrt();
            if length <= 0.0 {
                continue;
            }
            let signed = (ey * (point[0] - start[0]) - ex * (point[1] - start[1])) / length;
            if signed > 0.0 {
                inside = false;
            }
            let projection = (((point[0] - start[0]) * ex + (point[1] - start[1]) * ey)
                / (ex * ex + ey * ey))
                .clamp(0.0, 1.0);
            let dx = point[0] - (start[0] + projection * ex);
            let dy = point[1] - (start[1] + projection * ey);
            closest = closest.min((dx * dx + dy * dy).sqrt());
        }
        if !inside {
            worst = worst.max(closest);
        }
    }
    worst
}

fn parse_state(text: &str) -> Option<Vec<f64>> {
    let values: Vec<f64> = text
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|piece| !piece.is_empty())
        .map(|piece| piece.parse::<f64>())
        .collect::<Result<_, _>>()
        .ok()?;
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn main() -> ExitCode {
    let mut positional: Vec<String> = Vec::new();
    let mut certify: Option<Vec<f64>> = None;
    let mut rule = Rule::CounterDirection;
    let mut allowance: Option<f64> = None;
    let mut play: Option<Vec<f64>> = None;
    let mut search: Option<u64> = None;
    let mut seed = 1_u64;
    let mut bracket = false;
    let mut trace: Option<String> = None;
    let mut directions = DIRECTION_COUNT;
    let mut table: Option<(usize, usize)> = None;

    let mut arguments = env::args().skip(1);
    while let Some(arg) = arguments.next() {
        match arg.as_str() {
            "--help" | "-h" => {
                print_usage();
                return ExitCode::SUCCESS;
            }
            "--certify" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --certify needs a state, for example `--certify 1.4,0`");
                    return ExitCode::FAILURE;
                };
                match parse_state(&text) {
                    Some(state) => certify = Some(state),
                    None => {
                        eprintln!("error: --certify could not read a state from {text:?}");
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--rule" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --rule needs `counter`, `max` or `predictive`");
                    return ExitCode::FAILURE;
                };
                match Rule::parse(&text) {
                    Some(parsed) => rule = parsed,
                    None => {
                        eprintln!(
                            "error: unknown rule {text:?}: expected `counter`, `max` or `predictive`"
                        );
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--allowance" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --allowance needs a number");
                    return ExitCode::FAILURE;
                };
                match text.parse::<f64>() {
                    Ok(value) if value >= 0.0 => allowance = Some(value),
                    _ => {
                        eprintln!("error: --allowance needs a non-negative number, found {text:?}");
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--play" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --play needs a state, for example `--play 1.4,0`");
                    return ExitCode::FAILURE;
                };
                match parse_state(&text) {
                    Some(state) => play = Some(state),
                    None => {
                        eprintln!("error: --play could not read a state from {text:?}");
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--search" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --search needs a trial count");
                    return ExitCode::FAILURE;
                };
                match text.parse::<u64>() {
                    Ok(value) if value > 0 => search = Some(value),
                    _ => {
                        eprintln!("error: --search needs a positive trial count, found {text:?}");
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--bracket" => bracket = true,
            "--trace" => {
                let Some(path) = arguments.next() else {
                    eprintln!("error: --trace needs a path, for example `--trace shortfall.txt`");
                    return ExitCode::FAILURE;
                };
                trace = Some(path);
            }
            "--directions" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --directions needs a count");
                    return ExitCode::FAILURE;
                };
                match text.parse::<usize>() {
                    Ok(value) if value >= 3 => directions = value,
                    _ => {
                        eprintln!(
                            "error: --directions needs a count of at least 3, found {text:?}"
                        );
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--table" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --table needs a shape, for example `--table 16x15`");
                    return ExitCode::FAILURE;
                };
                let parsed = text
                    .split_once('x')
                    .and_then(|(azimuths, elevations)| {
                        Some((
                            azimuths.parse::<usize>().ok()?,
                            elevations.parse::<usize>().ok()?,
                        ))
                    });
                match parsed {
                    Some((azimuths, elevations)) if azimuths >= 2 && elevations >= 2 => {
                        table = Some((azimuths, elevations));
                    }
                    _ => {
                        eprintln!(
                            "error: --table needs `azimuths x elevations` of at least 2 each, found {text:?}"
                        );
                        return ExitCode::FAILURE;
                    }
                }
            }
            "--seed" => {
                let Some(text) = arguments.next() else {
                    eprintln!("error: --seed needs a number");
                    return ExitCode::FAILURE;
                };
                match text.parse::<u64>() {
                    Ok(value) => seed = value,
                    Err(_) => {
                        eprintln!("error: --seed needs a number, found {text:?}");
                        return ExitCode::FAILURE;
                    }
                }
            }
            other if other.starts_with('-') && other.len() > 1 => {
                eprintln!("error: unknown option {other}");
                print_usage();
                return ExitCode::FAILURE;
            }
            other => positional.push(other.to_string()),
        }
    }

    let options = Options {
        input: positional
            .first()
            .cloned()
            .unwrap_or_else(|| String::from("problem.txt")),
        output: positional.get(1).cloned(),
        certify,
        play,
        search,
        seed,
        bracket,
        trace,
        directions,
        table,
        rule,
        allowance,
    };

    let text = match fs::read_to_string(&options.input) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("error: cannot read {}: {error}", options.input);
            return ExitCode::FAILURE;
        }
    };

    // A non-linear description is read and then refused: the interval map is a support-function
    // construction and `ẋ = f(x, u, v)` has none. Reporting the case and the missing method beats
    // reporting `nonlinear` as the first unknown line.
    if nonlinear::contains_block(&text) {
        match nonlinear::parse(&text) {
            Ok(problem) => {
                eprintln!("error: {}", problem.refusal());
                if let Err(missing) = problem.unicycle() {
                    eprintln!("note: {missing}");
                }
            }
            Err(error) => eprintln!("error: {error}"),
        }
        return ExitCode::FAILURE;
    }

    let data = match input_parser::parse(&text) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let built = match options.table {
        Some((azimuths, elevations)) => StableBridge::new_3d(&data, azimuths, elevations),
        None => StableBridge::new_with(&data, options.directions),
    };
    let problem = match built {
        Ok(problem) => problem,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    let block_count = data.block_count();
    // The shortfall is collected on the same pass as the supports, so the certificate's inward
    // allowance is the recursion's own measurement rather than a second run.
    let (support_vectors, inward) = match problem.blocks_traced(block_count, data.step) {
        Ok(blocks) => blocks,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::FAILURE;
        }
    };

    if let Some(start) = &options.play {
        if start.len() != problem.dimension() {
            eprintln!(
                "error: --play takes {} values for this problem, found {}",
                problem.dimension(),
                start.len()
            );
            return ExitCode::FAILURE;
        }
        let run = strategy::run(
            &problem,
            &support_vectors,
            start,
            data.step,
            options.rule,
            &strategy::Evader::Extremal,
        );
        let text = strategy::format_trajectory(
            &run,
            data.n,
            data.t,
            data.step,
            &strategy::Evader::Extremal,
        );
        match &options.output {
            Some(path) => match fs::write(path, &text) {
                Ok(()) => println!("wrote trajectory to {path}"),
                Err(error) => {
                    eprintln!("error: cannot write {path}: {error}");
                    return ExitCode::FAILURE;
                }
            },
            None => print!("{text}"),
        }
        println!(
            "rule {}, worst slack {:.6} at block {}, captured {}",
            options.rule.name(),
            run.worst_slack,
            run.worst_block,
            match run.captured_at {
                Some(time) => format!("at {time}"),
                None => String::from("no"),
            }
        );
        return ExitCode::SUCCESS;
    }

    if let Some(trials) = options.search {
        let Some(start) = options.certify.as_ref() else {
            eprintln!("error: --search needs a start state, given with --certify <x1,x2,...>");
            return ExitCode::FAILURE;
        };
        if start.len() != problem.dimension() {
            eprintln!(
                "error: --certify takes {} values for this problem, found {}",
                problem.dimension(),
                start.len()
            );
            return ExitCode::FAILURE;
        }
        let allowance = support_vectors
            .iter()
            .map(|support| error::max_allowance(&problem.directions, support))
            .fold(0.0_f64, f64::max);
        let best = strategy::search(
            &problem,
            &support_vectors,
            start,
            data.step,
            options.rule,
            trials,
            options.seed,
        );
        let extremal = strategy::run(
            &problem,
            &support_vectors,
            start,
            data.step,
            options.rule,
            &strategy::Evader::Extremal,
        );
        println!(
            "search {trials} trials (seed {}), worst slack {:.6} at block {}, captured {}",
            options.seed,
            best.worst_slack,
            best.worst_block,
            match best.captured_at {
                Some(time) => format!("at {time}"),
                None => String::from("no"),
            }
        );
        let escape = best.worst_slack > allowance && best.captured_at.is_none();
        println!(
            "extremal baseline {:.6}; escape {} (allowance {:.6})",
            extremal.worst_slack,
            if escape { "found" } else { "none" },
            allowance
        );
        println!(
            "evader sequence {}",
            best.evader
                .iter()
                .map(usize::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        );
        if let Some(path) = &options.output {
            let text = strategy::format_trajectory(
                &best,
                data.n,
                data.t,
                data.step,
                &strategy::Evader::Sequence(best.evader.clone()),
            );
            if let Err(error) = fs::write(path, &text) {
                eprintln!("error: cannot write {path}: {error}");
                return ExitCode::FAILURE;
            }
            println!("wrote trajectory to {path}");
        }
        return ExitCode::SUCCESS;
    }

    if let Some(start) = &options.certify {
        if start.len() != problem.dimension() {
            eprintln!(
                "error: --certify takes {} values for this problem, found {}",
                problem.dimension(),
                start.len()
            );
            return ExitCode::FAILURE;
        }
        // The allowance is the printed boundary's slide between samples, computed from the same
        // supports the check uses. `--allowance` remains only as a labelled override.
        let (allowance, manual) = match options.allowance {
            Some(value) => (value, true),
            None => (
                support_vectors
                    .iter()
                    .map(|support| error::max_allowance(&problem.directions, support))
                    .fold(0.0_f64, f64::max),
                false,
            ),
        };
        let report = strategy::certify_traced_with(
            &problem,
            &support_vectors,
            Some(&inward),
            start,
            data.step,
            options.rule,
            allowance,
            manual,
            data.n == 2,
        );
        let text = strategy::format_report(&report, data.n, data.t, data.step);
        match &options.output {
            Some(path) => match fs::write(path, &text) {
                Ok(()) => println!("wrote certificate to {path}"),
                Err(error) => {
                    eprintln!("error: cannot write {path}: {error}");
                    return ExitCode::FAILURE;
                }
            },
            None => print!("{text}"),
        }
        println!(
            "allowance {:.6} ({}), rule {}, worst slack {:.6} at block {}, captured {}",
            report.allowance,
            if report.allowance_manual {
                "manual"
            } else if report.allowance_validated {
                "two-neighbour, validated"
            } else {
                "two-neighbour, estimate"
            },
            report.rule.name(),
            report.worst_slack,
            report.worst_block,
            match report.captured_at {
                Some(time) => format!("at {time}"),
                None => String::from("no"),
            }
        );
        println!(
            "inward {:.6} ({:.2}% of the support at that direction), adjusted slack {:.6}",
            report.inward, report.inward_of_support, report.adjusted_slack
        );
        return ExitCode::SUCCESS;
    }

    if let Some(path) = &options.trace {
        // The shortfall is what stage 5 of the dual update gave up on the way to each block, so
        // it is a property of the recursion rather than of an evader. Dumping it separates the
        // bracket's two terms at the block they were printed for, which is what decides whether
        // a band that grows as the set collapses is its allowance or its shortfall.
        let mut text = String::from("inward trace v1\n");
        text.push_str(&format!("dimension {}\n", data.n));
        text.push_str(&format!("blocks {}\n", support_vectors.len()));
        text.push_str(&format!("directions {}\n\n", problem.directions.len()));
        for (position, (support, shortfall)) in support_vectors.iter().zip(&inward).enumerate() {
            let charges = error::allowance(&problem.directions, support);
            for (direction, (bound, short)) in support.iter().zip(shortfall).enumerate() {
                text.push_str(&format!(
                    "block {} direction {} inward {:.6} support {:.6} allowance {:.6}\n",
                    position + 1,
                    direction,
                    short,
                    bound,
                    charges[direction]
                ));
            }
        }
        match fs::write(path, &text) {
            Ok(()) => println!("wrote trace to {path}"),
            Err(error) => {
                eprintln!("error: cannot write {path}: {error}");
                return ExitCode::FAILURE;
            }
        }
        return ExitCode::SUCCESS;
    }

    if options.bracket {
        // Each sampled half-space supports the printed set from both sides: `c_i + e_i` outward,
        // and `c_i - e_i` inward, the support of the intersection of the half-spaces shrunk by
        // their own allowance. The inner side is inner for the *printed* set; whether it is inner
        // for the true bridge is a separate measurement, and in 2D the shifted set stayed inside
        // the true body at every probe of `tools/trace_deficit.py` part G. The band is therefore
        // two-sided where that test has been run, and one-sided in 3D where the allowance is an
        // estimate.
        let mut text = String::from("bridge bracket v1\n");
        text.push_str(&format!("dimension {}\n", data.n));
        text.push_str(&format!("blocks {}\n", support_vectors.len()));
        text.push_str(&format!(
            "allowance source {}\n",
            if data.n == 2 {
                "two-neighbour, validated"
            } else {
                "two-neighbour, estimate"
            }
        ));
        // The covering radius is what the one-term allowance is a function of, and in 3D it is
        // the cell's circumradius rather than half the nearest-neighbour gap.
        let covering = error::covering_radius(&problem.directions)
            .into_iter()
            .fold(0.0_f64, f64::max);
        text.push_str(&format!(
            "covering radius max {:.4} deg\n",
            covering.to_degrees()
        ));
        text.push_str(
            "inner side shifted by the outward allowance and the block's inward shortfall\n\n",
        );
        let mut widest = 0.0_f64;
        let mut widest_block = 0;
        for (position, support) in support_vectors.iter().enumerate() {
            let values = error::allowance(&problem.directions, support);
            let shortfall = &inward[position];
            // The total gap at a direction is the outward allowance twice, once for each side of
            // the printed support, plus the inward shortfall, which lowers the inner side only.
            let mut direction = 0usize;
            let mut best = f64::NEG_INFINITY;
            for (index, value) in values.iter().enumerate() {
                let width = 2.0 * value + shortfall[index];
                if width.is_finite() && width > best {
                    best = width;
                    direction = index;
                }
            }
            let allowance = values[direction];
            let short = shortfall[direction];
            let inner = support[direction] - allowance - short;
            let outer = support[direction] + allowance;
            let width = 2.0 * allowance + short;
            if width.is_finite() && width > widest {
                widest = width;
                widest_block = position + 1;
            }
            if width.is_finite() {
                text.push_str(&format!(
                    "block {} direction {} inner {:.6} outer {:.6} width {:.6} of diameter {:.4}%\n",
                    position + 1,
                    direction,
                    inner,
                    outer,
                    width,
                    if outer > 0.0 {
                        100.0 * width / (2.0 * outer)
                    } else {
                        0.0
                    }
                ));
            } else {
                text.push_str(&format!(
                    "block {} direction {} inner {:.6} outer infinite width infinite of diameter 100%\n",
                    position + 1,
                    direction,
                    inner
                ));
            }
            if data.n == 2 {
                // The printed width is defined at the 110 samples, but the band is the gap
                // between two polytopes: their support functions differ at every angle, and both
                // are reconstructible from their caps. Probing is a quarter of a degree.
                const PROBES: usize = 1440;
                let caps_outer: Vec<f64> = support
                    .iter()
                    .zip(&values)
                    .map(|(bound, charge)| bound + charge)
                    .collect();
                let caps_inner: Vec<f64> = support
                    .iter()
                    .zip(&values)
                    .zip(shortfall)
                    .map(|((bound, charge), loss)| bound - charge - loss)
                    .collect();
                // The document's `0.01` vertex slack is larger than a dense table's band, so the
                // two polytopes are rebuilt at the solve tolerance; otherwise the measured band
                // can fall below the width at the samples.
                let tight = 1e-9;
                let outer_polytope =
                    convex_polygon::reconstruct_with(&problem.directions, &caps_outer, tight);
                let inner_polytope =
                    convex_polygon::reconstruct_with(&problem.directions, &caps_inner, tight);
                let reconstructible = !outer_polytope.is_empty() && !inner_polytope.is_empty();
                let mut probed = f64::NEG_INFINITY;
                let mut probed_angle = 0.0_f64;
                if reconstructible {
                    for step in 0..PROBES {
                        let angle = std::f64::consts::TAU * step as f64 / PROBES as f64;
                        let probe = [angle.cos(), angle.sin()];
                        let mut reach_outer = f64::NEG_INFINITY;
                        let mut reach_inner = f64::NEG_INFINITY;
                        for vertex in &outer_polytope {
                            reach_outer =
                                reach_outer.max(vertex[0] * probe[0] + vertex[1] * probe[1]);
                        }
                        for vertex in &inner_polytope {
                            reach_inner =
                                reach_inner.max(vertex[0] * probe[0] + vertex[1] * probe[1]);
                        }
                        let gap = reach_outer - reach_inner;
                        if gap > probed {
                            probed = gap;
                            probed_angle = angle.to_degrees();
                        }
                    }
                }
                let shown = if reconstructible && probed.is_finite() {
                    format!("{probed:.6}")
                } else {
                    String::from("infinite")
                };
                text.push_str(&format!(
                    "block {} probed width {} at angle {:.4} deg over {} probes\n",
                    position + 1, shown, probed_angle, PROBES
                ));
                // The worst gap over *every* angle is `max_psi (h_outer - h_inner)`, which is
                // the largest distance from a vertex of the outer polytope to the inner one —
                // not the diameter of the pairwise differences, which is the sets' size rather
                // than the band and came out `25-34x` too large.
                let exact = one_sided(&outer_polytope, &inner_polytope);
                text.push_str(&format!(
                    "block {} exact width {}\n",
                    position + 1,
                    if reconstructible {
                        format!("{exact:.6}")
                    } else {
                        String::from("infinite")
                    }
                ));
            }
        }
        text.push_str(&format!("widest {:.6} at block {}\n", widest, widest_block));
        match &options.output {
            Some(path) => match fs::write(path, &text) {
                Ok(()) => println!("wrote bracket to {path}"),
                Err(error) => {
                    eprintln!("error: cannot write {path}: {error}");
                    return ExitCode::FAILURE;
                }
            },
            None => print!("{text}"),
        }
        println!("widest {:.6} at block {} ({})", widest, widest_block, if data.n == 2 { "validated" } else { "estimate" });
        return ExitCode::SUCCESS;
    }

    let output_path = options
        .output
        .clone()
        .unwrap_or_else(|| String::from("out.txt"));

    let mut rendered: Vec<Vec<Vec<f64>>> = Vec::with_capacity(support_vectors.len());
    for support in &support_vectors {
        rendered.push(convex_polygon::reconstruct(
            &problem.directions,
            support,
        ));
    }

    let output = output_writer::format_document(
        &Header {
            dimension: data.n,
            terminal_time: data.t,
            step: data.step,
            blocks: rendered.len(),
            directions: problem.directions.len(),
        },
        &rendered,
    );

    if let Err(error) = fs::write(&output_path, output) {
        eprintln!("error: cannot write {output_path}: {error}");
        return ExitCode::FAILURE;
    }

    let vertices: usize = rendered.iter().map(Vec::len).sum();
    println!(
        "wrote {} block(s), {vertices} vertices, to {output_path} ({} support values each)",
        rendered.len(),
        problem.directions.len(),
    );
    ExitCode::SUCCESS
}
