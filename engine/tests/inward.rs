//! The inward allowance: the dual update's own shortfall, per block and direction.
//!
//! What is checked:
//!
//! * the traced update returns the same support as the plain one, and a loss that never gains;
//! * the traced blocks equal the printed blocks bit for bit;
//! * `2c`'s verdict is unchanged by the shortfall, because it is zero at the direction where the
//!   run leaves the set — the term cannot excuse that failure;
//! * `3d` records a shortfall and the report says the pass is weak.

use std::path::PathBuf;

use stable_bridge::directions;
use stable_bridge::error;
use stable_bridge::input_parser;
use stable_bridge::pontryagin_difference;
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

fn traced(name: &str) -> (input_parser::InputData, StableBridge, Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let (data, problem) = case(name);
    let (supports, inward) = problem
        .blocks_traced(data.block_count(), data.step)
        .expect("bounded case");
    (data, problem, supports, inward)
}

#[test]
fn the_plain_update_matches_the_traced_one() {
    let table = directions::build(2).expect("2D table");
    let square: Vec<f64> = table
        .iter()
        .map(|psi| psi[0].abs() + psi[1].abs())
        .collect();
    let cq: Vec<f64> = square.iter().map(|value| 0.1 * value).collect();
    let (support, loss) = pontryagin_difference::dual_update_with_loss(&cq, &table, &square, 2);
    assert_eq!(support, pontryagin_difference::dual_update(&cq, &table, &square, 2));
    assert!(loss.iter().all(|value| *value >= 0.0 && value.is_finite()));
}

#[test]
fn the_traced_blocks_are_the_printed_blocks() {
    let (data, problem) = case("example-1.txt");
    let printed = problem
        .blocks(data.block_count(), data.step)
        .expect("bounded case");
    let (supports, inward) = problem
        .blocks_traced(data.block_count(), data.step)
        .expect("bounded case");
    assert_eq!(supports, printed);
    assert_eq!(inward.len(), printed.len());
    assert!(inward.iter().all(|row| row.len() == problem.directions.len()));
}

#[test]
fn two_c_is_not_rescued_by_the_inward_allowance() {
    let (data, problem, supports, inward) = traced("example-2c.txt");
    let allowance = supports
        .iter()
        .map(|support| error::max_allowance(&problem.directions, support))
        .fold(0.0_f64, f64::max);
    let report = strategy::certify_traced_with(
        &problem,
        &supports,
        Some(&inward),
        &[0.0, 0.09],
        data.step,
        Rule::CounterDirection,
        allowance,
        false,
        data.n == 2,
    );
    assert!(
        (report.adjusted_slack - report.worst_slack).abs() < 1e-9,
        "the shortfall moved the slack: {} vs {}",
        report.adjusted_slack,
        report.worst_slack
    );
    assert!(report.inward < 1e-9, "inward at the failing direction {}", report.inward);
    assert!(
        report.inward_max > 0.03,
        "the shortfall elsewhere should still be recorded: {}",
        report.inward_max
    );
    assert!(
        !report.certified(),
        "2c was expected to stay uncertified, adjusted slack {}",
        report.adjusted_slack
    );
}

#[test]
fn three_d_records_a_shortfall_and_says_the_pass_is_weak() {
    let (data, problem, supports, inward) = traced("example-3d.txt");
    let allowance = supports
        .iter()
        .map(|support| error::max_allowance(&problem.directions, support))
        .fold(0.0_f64, f64::max);
    let report = strategy::certify_traced_with(
        &problem,
        &supports,
        Some(&inward),
        &[1.4, 1.4, 1.4],
        data.step,
        Rule::CounterDirection,
        allowance,
        false,
        data.n == 2,
    );
    assert!(report.inward > 0.04, "inward {}", report.inward);
    assert!(
        report.adjusted_slack < report.worst_slack,
        "the shortfall should lower the slack: {} vs {}",
        report.adjusted_slack,
        report.worst_slack
    );
    assert!(
        report.inward_of_support > 2.0,
        "the pass should be labelled weak, got {}%",
        report.inward_of_support
    );
    let text = strategy::format_report(&report, 3, data.t, data.step);
    assert!(text.contains("inward max"));
    assert!(text.contains("weak"));
    assert!(text.contains("estimate"));
}
