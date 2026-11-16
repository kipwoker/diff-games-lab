//! The `nonlinear` block: what it parses, what it validates, and what the engine refuses.
//!
//! The case is `examples/p1/example-p1b.txt`, the redefined P1 game. Nothing here computes a
//! bridge; the point is that the file is read as the case it claims to be, that its defining
//! inequality (`turn_rate_evader < turn_rate_pursuer`) is enforced, and that the driver's
//! message names the missing method rather than a typo.

use std::path::PathBuf;

use stable_bridge::directions;
use stable_bridge::input_parser::ParseError;
use stable_bridge::nonlinear;

fn case_text() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("examples")
        .join("p1")
        .join("example-p1b.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

#[test]
fn the_redefined_case_parses_into_its_fields_and_grid() {
    let problem = nonlinear::parse(&case_text()).expect("the case should parse");
    assert_eq!(problem.kind, "unicycle-pursuit");
    assert_eq!(problem.fields.len(), 8);

    let case = problem.unicycle().expect("the eight required fields");
    assert!((case.turn_rate_evader - 0.5).abs() < 1e-12);
    assert!((case.turn_rate_pursuer - 1.0).abs() < 1e-12);
    assert!((case.capture_radius - 0.1).abs() < 1e-12);
    assert!((case.horizon - 4.0).abs() < 1e-12);

    assert_eq!(problem.grid.len(), 3);
    assert_eq!(
        problem
            .grid
            .iter()
            .map(|axis| (axis.name.as_str(), axis.count))
            .collect::<Vec<_>>(),
        vec![("d", 121), ("psi_p", 36), ("psi_e", 18)]
    );
}

#[test]
fn the_driver_is_told_what_is_missing() {
    let problem = nonlinear::parse(&case_text()).expect("the case should parse");
    let text = problem.refusal();
    assert!(text.contains("not built"), "unhelpful refusal: {text}");
    assert!(text.contains("p1_relative.py"), "unhelpful refusal: {text}");
    assert!(text.contains("grid d 121, psi_p 36, psi_e 18"), "{text}");
}

#[test]
fn a_free_evader_is_rejected_by_the_field_check() {
    let text = "\
nonlinear unicycle-pursuit
  speed_pursuer 1.0
  speed_evader 0.7
  turn_rate_pursuer 1.0
  turn_rate_evader 1.0
  scale_theta 1.0
  capture_radius 0.1
  horizon 4.0
  step 0.1
";
    let problem = nonlinear::parse(text).expect("it parses");
    let message = problem.unicycle().expect_err("a free evader must be refused");
    assert!(message.contains("turn_rate_evader"), "{message}");
}

#[test]
fn a_duplicate_field_is_reported_with_its_line() {
    let text = "nonlinear unicycle-pursuit\n  horizon 4\n  horizon 5\n";
    match nonlinear::parse(text) {
        Err(ParseError::Descriptive { line, message }) => {
            assert_eq!(line, 3);
            assert!(message.contains("horizon"), "unhelpful message: {message}");
        }
        other => panic!("expected a line-tagged error, got {other:?}"),
    }
}

#[test]
fn a_missing_field_is_named() {
    let problem = nonlinear::parse("nonlinear unicycle-pursuit\n  horizon 4\n").expect("parses");
    let message = problem.unicycle().expect_err("fields are missing");
    assert!(message.contains("speed_pursuer"), "{message}");
}

#[test]
fn the_metric_is_the_identity_at_scale_one_and_shortens_the_angle_axis() {
    // Unit directions, because apply_metric renormalises: at scale one a unit table is fixed.
    let half = 0.5_f64.sqrt();
    let table = vec![vec![1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0], vec![half, 0.0, half]];
    assert_eq!(directions::apply_metric(&table, 1.0), table);

    let scaled = directions::apply_metric(&table, 2.0);
    let norm = (0.5_f64 + 0.125).sqrt();
    assert!((scaled[0][0] - 1.0).abs() < 1e-12, "a pure position axis is unchanged");
    assert!((scaled[1][2] - 1.0).abs() < 1e-12, "a pure angle axis is unchanged");
    assert!((scaled[2][0] - half / norm).abs() < 1e-12);
    assert!((scaled[2][2] - 0.5 * half / norm).abs() < 1e-12, "the angle component halves before normalising");
}
