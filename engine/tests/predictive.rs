use stable_bridge::input_parser;
use stable_bridge::strategy::{self, Evader, Rule};
use stable_bridge::StableBridge;

fn case(name: &str) -> (StableBridge, Vec<Vec<f64>>, f64) {
    let text = std::fs::read_to_string(format!("../examples/{name}.txt")).expect("case file");
    let data = input_parser::parse(&text).expect("parse");
    let problem = StableBridge::new(&data).expect("setup");
    let supports = problem.blocks(data.block_count(), data.step).expect("blocks");
    (problem, supports, data.step)
}

/// The predictive rule's whole claim is that it answers the evader, so it must not do worse
/// than the rule it replaces on the evader that rule already handles.
#[test]
fn predictive_is_no_worse_than_counter_direction() {
    for (name, start) in [
        ("example-1", [1.4, 0.0]),
        ("example-2a", [0.05, 0.05]),
        ("example-2b", [0.05, 0.05]),
        ("example-2c", [0.0, 0.09]),
    ] {
        let (problem, supports, step) = case(name);
        let counter = strategy::run(
            &problem,
            &supports,
            &start,
            step,
            Rule::CounterDirection,
            &Evader::Extremal,
        );
        let predictive = strategy::run(
            &problem,
            &supports,
            &start,
            step,
            Rule::Predictive,
            &Evader::Extremal,
        );
        assert!(
            predictive.worst_slack <= counter.worst_slack + 1e-9,
            "{name}: predictive {:.6} worse than counter {:.6}",
            predictive.worst_slack,
            counter.worst_slack
        );
    }
}

/// The rule has to return a real control, not a blend of vertices.
#[test]
fn predictive_returns_vertices_of_the_pursuer_set() {
    let (problem, supports, step) = case("example-3d");
    let start = [0.0, 0.0, 1.0];
    let run = strategy::run(
        &problem,
        &supports,
        &start,
        step,
        Rule::Predictive,
        &Evader::Extremal,
    );
    let vertices = problem.p.vertices();
    for block in &run.blocks {
        assert!(
            vertices.iter().any(|vertex| vertex == &block.u),
            "block {} control {:?} is not a vertex",
            block.index,
            block.u
        );
    }
}

/// The search path has to work under the new rule, and still reproduce from its seed.
#[test]
fn the_search_runs_under_the_predictive_rule() {
    let (problem, supports, step) = case("example-2a");
    let start = [0.05, 0.05];
    let first = strategy::search(
        &problem,
        &supports,
        &start,
        step,
        Rule::Predictive,
        40,
        5,
    );
    let second = strategy::search(
        &problem,
        &supports,
        &start,
        step,
        Rule::Predictive,
        40,
        5,
    );
    assert_eq!(first.evader, second.evader);
    assert!((first.worst_slack - second.worst_slack).abs() < 1e-12);
    assert_eq!(first.evader.len(), supports.len());
}
