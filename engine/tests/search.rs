use stable_bridge::input_parser;
use stable_bridge::strategy::{self, Evader, Rule};
use stable_bridge::StableBridge;

fn case(name: &str) -> (StableBridge, Vec<Vec<f64>>, f64, usize, f64) {
    let text = std::fs::read_to_string(format!("../examples/{name}.txt")).expect("case file");
    let data = input_parser::parse(&text).expect("parse");
    let problem = StableBridge::new(&data).expect("setup");
    let supports = problem.blocks(data.block_count(), data.step).expect("blocks");
    (problem, supports, data.step, data.n, data.t)
}

#[test]
fn the_extremal_run_matches_the_certificate() {
    let (problem, supports, step, _, _) = case("example-2a");
    let start = [0.05, 0.05];
    let report = strategy::certify(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        0.0,
    );
    let run = strategy::run(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        &Evader::Extremal,
    );
    assert!((report.worst_slack - run.worst_slack).abs() < 1e-12);
    assert_eq!(report.blocks.len(), run.blocks.len());
    assert_eq!(run.captured_at, report.captured_at);
}

/// A sequence equal to the extremal choices must reproduce the extremal run exactly.
#[test]
fn the_extremal_sequence_is_the_extremal_run() {
    let (problem, supports, step, _, _) = case("example-2a");
    let start = [0.05, 0.05];
    let extremal = strategy::run(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        &Evader::Extremal,
    );
    let replay = strategy::run(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        &Evader::Sequence(extremal.evader.clone()),
    );
    assert!((replay.worst_slack - extremal.worst_slack).abs() < 1e-12);
    assert_eq!(replay.captured_at, extremal.captured_at);
}

#[test]
fn the_search_never_returns_worse_than_the_extremal_run() {
    let (problem, supports, step, _, _) = case("example-2c");
    let start = [0.0, 0.09];
    let extremal = strategy::run(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        &Evader::Extremal,
    );
    let best = strategy::search(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        50,
        7,
    );
    assert!(best.worst_slack >= extremal.worst_slack - 1e-12);
    assert_eq!(best.evader.len(), supports.len());
}

/// The seed decides the search, so a run can be reproduced.
#[test]
fn the_search_is_reproducible_from_its_seed() {
    let (problem, supports, step, _, _) = case("example-2b");
    let start = [0.05, 0.05];
    let first = strategy::search(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        30,
        3,
    );
    let second = strategy::search(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        30,
        3,
    );
    assert_eq!(first.evader, second.evader);
    assert!((first.worst_slack - second.worst_slack).abs() < 1e-12);
}

#[test]
fn the_trajectory_carries_both_controls() {
    let (problem, supports, step, dimension, terminal) = case("example-2a");
    let start = [0.05, 0.05];
    let run = strategy::run(
        &problem,
        &supports,
        &start,
        step,
        Rule::CounterDirection,
        &Evader::Extremal,
    );
    let text = strategy::format_trajectory(
        &run,
        dimension,
        terminal,
        step,
        &Evader::Extremal,
    );
    assert!(text.starts_with("strategy trajectory v1\n"));
    assert!(text.contains("evader extremal\n"));
    let block_lines = text
        .lines()
        .filter(|line| line.starts_with("block "))
        .count();
    assert_eq!(block_lines, supports.len());
    assert!(text
        .lines()
        .filter(|line| line.starts_with("block "))
        .all(|line| line.contains(" u ") && line.contains(" v ")));
}
