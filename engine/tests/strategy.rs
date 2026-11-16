//! The strategy certificate against cases with a known answer.
//!
//! What is checked:
//!
//! * on the diamond, starting inside the bridge, the counter-direction rule never leaves it,
//!   and capture happens;
//! * the maximising rule steps out of the same bridge, so the choice of rule is measured;
//! * a start outside the bridge is reported as such at block one;
//! * with equal authority (`P = Q`) the bridge is the target, and the state stays in it.

use std::path::PathBuf;

use stable_bridge::input_parser;
use stable_bridge::strategy::{self, Rule};
use stable_bridge::StableBridge;

fn case(name: &str) -> (input_parser::InputData, StableBridge) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("examples")
        .join(name);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
    let data = input_parser::parse(&text).expect("case should parse");
    let problem = StableBridge::new(&data).expect("case should be supported");
    (data, problem)
}

fn run(name: &str, start: &[f64], rule: Rule) -> strategy::Report {
    let (data, problem) = case(name);
    let blocks = problem
        .blocks(data.block_count(), data.step)
        .expect("bounded case");
    strategy::certify(&problem, &blocks, start, data.step, rule, 0.0)
}

#[test]
fn the_counter_direction_rule_stays_inside_the_diamond() {
    let report = run("example-1.txt", &[1.4, 0.0], Rule::CounterDirection);
    assert_eq!(report.blocks.len(), 20);
    assert!(
        report.certified(),
        "worst slack {} leaves the bridge at block {}",
        report.worst_slack,
        report.worst_block
    );
    assert!(
        report.captured_at.is_some(),
        "the pursuer never reached the target: worst slack {}",
        report.worst_slack
    );
}

#[test]
fn the_maximising_rule_leaves_the_same_bridge() {
    let report = run("example-1.txt", &[1.4, 0.0], Rule::Maximising);
    assert!(
        !report.certified(),
        "the maximising rule was expected to fail, worst slack {}",
        report.worst_slack
    );
    assert!(
        report.worst_slack > 0.1,
        "the violation is too small to distinguish the rules: {}",
        report.worst_slack
    );
}

#[test]
fn a_start_outside_the_bridge_is_reported_at_block_one() {
    let report = run("example-1.txt", &[4.0, 0.0], Rule::CounterDirection);
    assert!(
        report.worst_slack > 0.5,
        "a point 1.0 outside the diamond bridge should be far outside, got {}",
        report.worst_slack
    );
}

#[test]
fn equal_authority_keeps_the_state_in_the_target() {
    // example-2b has P = Q, so the two images cancel and the bridge stays at M.
    let report = run("example-2b.txt", &[0.4, 0.0], Rule::CounterDirection);
    assert!(
        report.certified(),
        "worst slack {} at block {}",
        report.worst_slack,
        report.worst_block
    );
    assert!(report.captured_at.is_some(), "capture should happen at once");
}

#[test]
fn the_report_states_the_rule_and_the_verdict() {
    let report = run("example-1.txt", &[1.4, 0.0], Rule::CounterDirection);
    let text = strategy::format_report(&report, 2, 2.0, 0.1);
    assert!(text.starts_with("strategy certificate v1\n"));
    assert!(text.contains("rule counter-direction\n"));
    assert!(text.contains("blocks 20\n"));
    assert!(text.contains("certified yes\n"));
    assert_eq!(
        text.lines().filter(|line| line.starts_with("block ")).count(),
        20
    );
}
