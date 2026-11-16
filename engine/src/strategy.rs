//! Positional strategies from a computed bridge, the blockwise certificate, and the
//! closed loop that both the certificate and the adversarial search run through.
//!
//! The bridge `W^1 … W^N` is carried in the coordinates the recursion uses,
//! `z(t) = e^{(T-t)A} x(t)`: block `j` is a set of `z` values, and `z` moves by
//! `e^{(j-1)·Step·A} · Φ · (B u + C v)` over one block with `j` blocks left, `Φ` the
//! integral of the flow over `Step`. Nothing here re-derives a support value; the module
//! consumes the vectors `StableBridge::blocks` produced.
//!
//! Two rules are implemented, and which one certifies is a measurement:
//!
//! * [`Rule::CounterDirection`] — the pursuer takes the vertex of `P` that *minimises*
//!   `⟨ψ, Φ B u⟩` along the active direction. This is the literal "maximise the support"
//!   rule of the dual with the engine's structural sign flip on `G·B` applied: in the
//!   recurrence `c(W^k) = c(W^{k-1}) + c(P_k) - c(Q_k)` the pursuer's image enters through
//!   `-(G·B)`, so the vertex that grows the bridge is the one the unflipped map would
//!   call the minimum.
//! * [`Rule::Maximising`] — the same rule without the flip, i.e. the pursuer takes the
//!   vertex that maximises its own support value. It is here to be falsified: on a case
//!   with a known answer it steps straight out of the bridge.
//! * [`Rule::Predictive`] — the evader's vertex is observed at the block's start, so the
//!   pursuer scores each of its own vertices by the *worst* excess it leaves over every
//!   direction at the end of the block and takes the vertex that minimises it. This is the
//!   rule that answers a deviation: the counter-direction vertex maximises the support in
//!   the direction that was active before the evader moved, which is why a sequence that
//!   deviates early escapes a case the extremal evader cannot (`04-adversarial-search.md`).
//!
//! The evader maximises `⟨ψ, Φ C v⟩` in both, which is the adversarial vertex of `Q`.
//! A fixed sequence of evader vertices can replace that rule — that is what the search
//! does — but the pursuer never sees the sequence before choosing, so every sequence in
//! the search is a nonanticipative strategy and a run it wins is a counterexample rather
//! than an oracle.

use crate::matrix::Matrix;
use crate::stable_bridge::{expm, integrate_exp, StableBridge};
use crate::support_function::dot;

/// Which vertex of `P` the pursuer takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// Against the active direction, matching the sign flip on `G·B`.
    CounterDirection,
    /// Along the active direction, matching the support values before the flip.
    Maximising,
    /// Minimising the worst excess over every direction at the end of the block.
    Predictive,
}

impl Rule {
    /// Short name used in the report.
    pub fn name(self) -> &'static str {
        match self {
            Self::CounterDirection => "counter-direction",
            Self::Maximising => "maximising",
            Self::Predictive => "predictive",
        }
    }

    /// Parses `counter`, `counter-direction` or `max`/`maximising`.
    pub fn parse(text: &str) -> Option<Self> {
        match text.to_ascii_lowercase().as_str() {
            "counter" | "counter-direction" | "min" => Some(Self::CounterDirection),
            "max" | "maximising" | "maximizing" => Some(Self::Maximising),
            "predictive" | "lookahead" | "worst" => Some(Self::Predictive),
            _ => None,
        }
    }
}

/// How the evader chooses its vertex.
#[derive(Clone, Debug)]
pub enum Evader {
    /// The vertex of `Q` that maximises the active direction's support.
    Extremal,
    /// A fixed vertex per one-based block, taken as `sequence[k - 1 % len]`.
    Sequence(Vec<usize>),
}

/// One block of a closed-loop run.
#[derive(Clone, Debug)]
pub struct BlockReport {
    /// One-based block index, counted forward from the start.
    pub index: usize,
    /// Time of day: `index · Step`.
    pub time: f64,
    /// `max_i (⟨ψ_i, z⟩ − c_i)` before the block was applied; non-positive means inside.
    pub slack: f64,
    /// Index of the direction that attained the slack.
    pub direction: usize,
    /// Pursuer control applied over the block.
    pub u: Vec<f64>,
    /// Evader control applied over the block.
    pub v: Vec<f64>,
    /// State after the block, in `x` coordinates.
    pub state: Vec<f64>,
}

/// A closed loop: the blocks, the worst slack and where it happened.
#[derive(Clone, Debug)]
pub struct Run {
    pub blocks: Vec<BlockReport>,
    /// Largest raw slack over all blocks.
    pub worst_slack: f64,
    pub worst_block: usize,
    pub worst_direction: usize,
    /// First time the state satisfied `M`, if it did.
    pub captured_at: Option<f64>,
    /// Evader vertex index used per block.
    pub evader: Vec<usize>,
}

/// The whole certificate.
#[derive(Clone, Debug)]
pub struct Report {
    pub rule: Rule,
    /// Outward allowance subtracted from the slack before a run counts as certified.
    pub allowance: f64,
    /// Whether the allowance was typed on the command line rather than computed.
    pub allowance_manual: bool,
    /// Whether the computed allowance is a validated bound (2D) or an estimate (3D).
    pub allowance_validated: bool,
    pub worst_slack: f64,
    pub worst_block: usize,
    pub worst_direction: usize,
    pub captured_at: Option<f64>,
    pub blocks: Vec<BlockReport>,
    /// Inward allowance at the block and direction that decide the verdict.
    ///
    /// The stored supports may sit below the exact recurrence (`01-deficit-is-chamfering.md`),
    /// so a state just outside the printed set can still be inside the true one. This is that
    /// shortfall, accumulated by the recursion itself, not typed.
    pub inward: f64,
    /// `inward` as a percentage of the stored support at the same direction.
    pub inward_of_support: f64,
    /// Largest shortfall anywhere in the run, over every block and direction.
    pub inward_max: f64,
    /// Slack net of the inward allowance, `max_k (slack_k − inward_k[i_k])`, which is what the
    /// outward allowance is compared against.
    pub adjusted_slack: f64,
}

impl Report {
    /// Whether the slack net of the inward allowance is within the outward one.
    pub fn certified(&self) -> bool {
        self.adjusted_slack <= self.allowance
    }
}

/// `mᵀ v`, the direction a linear map pushes back into the dual space.
fn transpose_apply(matrix: &Matrix, v: &[f64]) -> Vec<f64> {
    (0..matrix.cols())
        .map(|j| {
            let mut sum = 0.0;
            for i in 0..matrix.rows() {
                sum += matrix.get(i, j) * v[i];
            }
            sum
        })
        .collect()
}

/// The active direction: the one in which `z` sits closest to the boundary.
fn active(directions: &[Vec<f64>], support: &[f64], z: &[f64]) -> (f64, usize) {
    let mut best = f64::NEG_INFINITY;
    let mut index = 0;
    for (i, psi) in directions.iter().enumerate() {
        let slack = dot(psi, z) - support[i];
        if slack > best {
            best = slack;
            index = i;
        }
    }
    (best, index)
}

/// Slack under which two vertex scores count as the same face.
const TIE: f64 = 1e-9;

/// The vertex of `vertices` with the smallest (`minimise`) or largest score.
///
/// When the weight vanishes in some coordinate the extremal point is a whole face, and every
/// vertex of it scores the same. Picking the first one is a mistake that costs the certificate:
/// on `example-2b` it selected a pursuer control whose projection on the active direction was
/// optimal but which was not the evader's opposite, and the state crawled out of the target.
/// `towards`, when given, breaks the tie in favour of the vertex most aligned with it, which is
/// how the pursuer takes the control opposite the evader's.
fn extremal(
    vertices: &[Vec<f64>],
    weights: &[f64],
    minimise: bool,
    towards: Option<&[f64]>,
) -> Vec<f64> {
    let mut best = if minimise {
        f64::INFINITY
    } else {
        f64::NEG_INFINITY
    };
    let mut face: Vec<&Vec<f64>> = Vec::new();
    for vertex in vertices {
        let score = dot(weights, vertex);
        if (minimise && score < best - TIE) || (!minimise && score > best + TIE) {
            best = score;
            face.clear();
            face.push(vertex);
        } else if (score - best).abs() <= TIE {
            face.push(vertex);
        }
    }

    let mut choice = face
        .first()
        .map(|vertex| (*vertex).clone())
        .unwrap_or_else(|| vec![0.0; weights.len()]);
    if let Some(target) = towards {
        let mut best_alignment = dot(target, &choice);
        for vertex in face {
            let alignment = dot(target, vertex);
            if alignment > best_alignment + TIE {
                best_alignment = alignment;
                choice = vertex.clone();
            }
        }
    }
    choice
}

/// Runs the closed loop once.
///
/// `supports` is the block sequence `W^1 … W^N` as `StableBridge::blocks` returns it;
/// `start` is the initial state in `x` coordinates.
///
/// # Panics
/// If `start` has the wrong dimension or `step` is not positive.
pub fn run(
    problem: &StableBridge,
    supports: &[Vec<f64>],
    start: &[f64],
    step: f64,
    rule: Rule,
    evader: &Evader,
) -> Run {
    let n = problem.dimension();
    assert_eq!(start.len(), n, "certificate: start dimension mismatch");
    assert!(step > 0.0, "certificate: step must be positive");

    let total = supports.len();
    let phi = integrate_exp(&problem.a, step);
    let p_vertices = problem.p.vertices();
    let q_vertices = problem.q.vertices();

    // The recursion's coordinates: z(T) = x(T), z(0) = e^{TA} x(0).
    let mut z = expm(&problem.a, total as f64 * step).mul_vec(start);
    let mut blocks = Vec::with_capacity(total);
    let mut evader_used = Vec::with_capacity(total);
    let mut worst_slack = f64::NEG_INFINITY;
    let mut worst_block = 0;
    let mut worst_direction = 0;
    let mut captured_at = None;

    for k in 1..=total {
        let remaining = total - k + 1;
        let support = &supports[remaining - 1];
        let (slack, direction) = active(&problem.directions, support, &z);

        if slack > worst_slack {
            worst_slack = slack;
            worst_block = k;
            worst_direction = direction;
        }

        // The map from this block's controls to this block's displacement of z.
        let flow = expm(&problem.a, (remaining as f64 - 1.0) * step);
        let map = flow.mul(&phi);
        let psi = &problem.directions[direction];
        let pushed = transpose_apply(&map, psi);

        let v_weights = transpose_apply(&problem.c, &pushed);
        let chosen = match evader {
            Evader::Extremal => None,
            Evader::Sequence(sequence) if !sequence.is_empty() => Some(sequence[(k - 1) % sequence.len()]),
            Evader::Sequence(_) => None,
        };
        let (v, v_index) = match chosen {
            Some(index) if index < q_vertices.len() => (q_vertices[index].clone(), index),
            _ => {
                let vertex = extremal(&q_vertices, &v_weights, false, None);
                let index = q_vertices
                    .iter()
                    .position(|candidate| candidate == &vertex)
                    .unwrap_or(0);
                (vertex, index)
            }
        };
        let u_weights = transpose_apply(&problem.b, &pushed);
        // The pursuer opposes the evader inside the optimal face: `-v` is the direction that
        // cancels the evader's displacement, and for `P = Q` it is exactly the control there.
        let opposition: Vec<f64> = v.iter().map(|value| -value).collect();
        let u = if rule == Rule::Predictive {
            // Score every pursuer vertex by the worst excess it leaves at the end of the block.
            // `v` is known, so the displacement is `M·(B u + C v)` and the test is the
            // certificate's own: the largest `⟨ψ_i, z'⟩ − c_i` once the block is applied.
            let cv = problem.c.mul_vec(&v);
            let mut best: Option<(f64, Vec<f64>)> = None;
            for vertex in &p_vertices {
                let mut displacement = problem.b.mul_vec(vertex);
                for (value, other) in displacement.iter_mut().zip(&cv) {
                    *value += other;
                }
                let delta = map.mul_vec(&displacement);
                let mut score = f64::NEG_INFINITY;
                for (index, psi) in problem.directions.iter().enumerate() {
                    let mut projected = 0.0;
                    for axis in 0..n {
                        projected += psi[axis] * (z[axis] + delta[axis]);
                    }
                    score = score.max(projected - support[index]);
                }
                let better = match &best {
                    None => true,
                    Some((best_score, best_vertex)) => {
                        score < best_score - TIE
                            || ((score - best_score).abs() <= TIE
                                && dot(&opposition, vertex) > dot(&opposition, best_vertex))
                    }
                };
                if better {
                    best = Some((score, vertex.clone()));
                }
            }
            best.map(|(_, vertex)| vertex).unwrap_or_else(|| vec![0.0; n])
        } else {
            extremal(
                &p_vertices,
                &u_weights,
                rule == Rule::CounterDirection,
                if rule == Rule::CounterDirection {
                    Some(&opposition)
                } else {
                    None
                },
            )
        };

        let mut displacement = problem.b.mul_vec(&u);
        let cv = problem.c.mul_vec(&v);
        for (value, other) in displacement.iter_mut().zip(&cv) {
            *value += other;
        }
        let step_vector = map.mul_vec(&displacement);
        for (value, delta) in z.iter_mut().zip(&step_vector) {
            *value += delta;
        }

        if captured_at.is_none() && problem.m_set.satisfies(&z, 1e-9) {
            captured_at = Some(k as f64 * step);
        }

        evader_used.push(v_index);
        blocks.push(BlockReport {
            index: k,
            time: k as f64 * step,
            slack,
            direction,
            u,
            v,
            state: z.clone(),
        });
    }

    Run {
        blocks,
        worst_slack,
        worst_block,
        worst_direction,
        captured_at,
        evader: evader_used,
    }
}

/// Runs the closed loop with the extremal evader and reports the slacks.
pub fn certify(
    problem: &StableBridge,
    supports: &[Vec<f64>],
    start: &[f64],
    step: f64,
    rule: Rule,
    allowance: f64,
) -> Report {
    certify_with(problem, supports, start, step, rule, allowance, false, true)
}

/// [`certify`], recording whether the allowance was typed rather than computed.
pub fn certify_with(
    problem: &StableBridge,
    supports: &[Vec<f64>],
    start: &[f64],
    step: f64,
    rule: Rule,
    allowance: f64,
    manual: bool,
    validated: bool,
) -> Report {
    certify_traced_with(
        problem, supports, None, start, step, rule, allowance, manual, validated,
    )
}

/// [`certify_with`], also using the recursion's own shortfall at every block.
///
/// `inward` is `StableBridge::blocks_traced`'s second value: one row per block, one entry per
/// direction. The test is evaluated per block at the direction that block was closest to,
/// because the shortfall is local: a support that is exact at one direction and short at
/// another only excuses the second. `None` leaves the old one-sided test untouched.
pub fn certify_traced_with(
    problem: &StableBridge,
    supports: &[Vec<f64>],
    inward: Option<&[Vec<f64>]>,
    start: &[f64],
    step: f64,
    rule: Rule,
    allowance: f64,
    manual: bool,
    validated: bool,
) -> Report {
    let run = run(problem, supports, start, step, rule, &Evader::Extremal);
    let total = supports.len();
    let (adjusted_slack, inward_at, inward_of_support, inward_max) = match inward {
        None => (run.worst_slack, 0.0, 0.0, 0.0),
        Some(table) => {
            let largest = table.iter().flatten().fold(0.0_f64, |a, b| a.max(*b));
            let mut adjusted = f64::NEG_INFINITY;
            let mut at = 0.0;
            let mut fraction = 0.0;
            for block in &run.blocks {
                let index = total.saturating_sub(block.index);
                let short = table
                    .get(index)
                    .and_then(|row| row.get(block.direction))
                    .copied()
                    .unwrap_or(0.0);
                let value = block.slack - short;
                if value > adjusted {
                    adjusted = value;
                    at = short;
                    let support = supports
                        .get(index)
                        .and_then(|row| row.get(block.direction))
                        .copied()
                        .unwrap_or(0.0);
                    fraction = if support.abs() > 0.0 {
                        100.0 * short / support.abs()
                    } else {
                        0.0
                    };
                }
            }
            if adjusted.is_finite() {
                (adjusted, at, fraction, largest)
            } else {
                (run.worst_slack, 0.0, 0.0, largest)
            }
        }
    };
    Report {
        rule,
        allowance,
        allowance_manual: manual,
        allowance_validated: validated || manual,
        worst_slack: run.worst_slack,
        worst_block: run.worst_block,
        worst_direction: run.worst_direction,
        captured_at: run.captured_at,
        blocks: run.blocks,
        inward: inward_at,
        inward_of_support,
        inward_max,
        adjusted_slack,
    }
}

/// Renders a report: a fixed header, one line per block, and the summary.
pub fn format_report(report: &Report, dimension: usize, terminal_time: f64, step: f64) -> String {
    use crate::output_writer::format_real;

    let mut text = String::from("strategy certificate v1\n");
    text.push_str(&format!("rule {}\n", report.rule.name()));
    text.push_str(&format!("allowance {}\n", format_real(report.allowance)));
    text.push_str(&format!(
        "allowance source {}\n",
        if report.allowance_manual {
            "manual"
        } else if report.allowance_validated {
            "two-neighbour, validated"
        } else {
            "two-neighbour, estimate"
        }
    ));
    text.push_str(&format!("inward {:.6}\n", report.inward));
    text.push_str(&format!(
        "inward of support {:.2}%\n",
        report.inward_of_support
    ));
    text.push_str(&format!("inward max {:.6}\n", report.inward_max));
    text.push_str(
        "inward source stage-5 minorisation of the dual update, accumulated over the block\n",
    );
    text.push_str(&format!("dimension {dimension}\n"));
    text.push_str(&format!("T {}\n", format_real(terminal_time)));
    text.push_str(&format!("Step {}\n", format_real(step)));
    text.push_str(&format!("blocks {}\n", report.blocks.len()));
    match report.captured_at {
        Some(time) => text.push_str(&format!("captured {}\n", format_real(time))),
        None => text.push_str("captured none\n"),
    }
    text.push_str(&format!(
        "worst slack {:.6} at block {} direction {}\n",
        report.worst_slack, report.worst_block, report.worst_direction
    ));
    text.push_str(&format!("adjusted slack {:.6}\n", report.adjusted_slack));
    // The verdict carries every caveat that applies, rather than the first one: a 3D pass bought
    // by a large shortfall is an estimate *and* weak, and dropping either would overstate it.
    let mut verdict = String::from(if report.certified() {
        "certified yes"
    } else {
        "certified no"
    });
    if report.certified() {
        if !report.allowance_validated {
            verdict.push_str(" (estimate, not a bound)");
        }
        if report.inward_of_support > 2.0 {
            verdict.push_str(" (inward above 2% of the support: weak)");
        }
    }
    verdict.push_str("\n\n");
    text.push_str(&verdict);

    for block in &report.blocks {
        let state: Vec<String> = block
            .state
            .iter()
            .map(|value| format!("{value:.6}"))
            .collect();
        text.push_str(&format!(
            "block {} time {} slack {:.6} direction {} state {}\n",
            block.index,
            format_real(block.time),
            block.slack,
            block.direction,
            state.join(" ")
        ));
    }
    text
}

/// Renders a closed loop as a trajectory: both controls, the slack and the state per block.
pub fn format_trajectory(
    run: &Run,
    dimension: usize,
    terminal_time: f64,
    step: f64,
    evader: &Evader,
) -> String {
    use crate::output_writer::format_real;

    let mut text = String::from("strategy trajectory v1\n");
    text.push_str(&format!("dimension {dimension}\n"));
    text.push_str(&format!("T {}\n", format_real(terminal_time)));
    text.push_str(&format!("Step {}\n", format_real(step)));
    text.push_str(&format!("blocks {}\n", run.blocks.len()));
    match evader {
        Evader::Extremal => text.push_str("evader extremal\n"),
        Evader::Sequence(sequence) => {
            let indices: Vec<String> = run.evader.iter().map(usize::to_string).collect();
            text.push_str(&format!(
                "evader sequence {} ({})\n",
                sequence.len(),
                indices.join(" ")
            ));
        }
    }
    match run.captured_at {
        Some(time) => text.push_str(&format!("captured {}\n", format_real(time))),
        None => text.push_str("captured none\n"),
    }
    text.push_str(&format!(
        "worst slack {:.6} at block {} direction {}\n\n",
        run.worst_slack, run.worst_block, run.worst_direction
    ));

    for block in &run.blocks {
        let real = |values: &[f64]| {
            values
                .iter()
                .map(|value| format!("{value:.6}"))
                .collect::<Vec<String>>()
                .join(" ")
        };
        text.push_str(&format!(
            "block {} time {} slack {:.6} direction {} u {} v {} state {}\n",
            block.index,
            format_real(block.time),
            block.slack,
            block.direction,
            real(&block.u),
            real(&block.v),
            real(&block.state)
        ));
    }
    text
}

/// Small deterministic generator, so a search is reproducible from its seed.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.0
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
}

/// Searches for an evader sequence that beats the extremal run.
///
/// `trials` random sequences are drawn, the best is kept, and each block is then scanned
/// over every vertex of `Q` until no single change improves the worst slack. The worst slack
/// is the search's objective because it is also the certificate's test: a sequence that
/// raises it past the allowance is the counterexample, and one that cannot is evidence that
/// the extremal choice is the adversary.
pub fn search(
    problem: &StableBridge,
    supports: &[Vec<f64>],
    start: &[f64],
    step: f64,
    rule: Rule,
    trials: u64,
    seed: u64,
) -> Run {
    let vertices = problem.q.vertices().len();
    let blocks = supports.len();
    let mut generator = Lcg(seed | 1);

    let mut best = run(problem, supports, start, step, rule, &Evader::Extremal);
    if vertices == 0 || blocks == 0 {
        return best;
    }

    for _ in 0..trials {
        let sequence: Vec<usize> = (0..blocks).map(|_| generator.below(vertices)).collect();
        let candidate = run(
            problem,
            supports,
            start,
            step,
            rule,
            &Evader::Sequence(sequence),
        );
        if candidate.worst_slack > best.worst_slack {
            best = candidate;
        }
    }

    // Coordinate descent: one block at a time, every vertex, keep an improvement.
    for _ in 0..2 {
        let mut improved = false;
        for block in 0..blocks {
            for vertex in 0..vertices {
                let mut sequence = best.evader.clone();
                if sequence.len() != blocks {
                    sequence.resize(blocks, 0);
                }
                sequence[block] = vertex;
                let candidate = run(
                    problem,
                    supports,
                    start,
                    step,
                    rule,
                    &Evader::Sequence(sequence),
                );
                if candidate.worst_slack > best.worst_slack + 1e-12 {
                    best = candidate;
                    improved = true;
                }
            }
        }
        if !improved {
            break;
        }
    }

    best
}
