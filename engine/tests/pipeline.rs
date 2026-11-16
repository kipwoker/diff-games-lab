//! End-to-end checks: parser, direction tables, matrix series, a full run, and the
//! degenerate zero-dynamics case where the bridge must stay at the terminal set.
//!
//! What is checked:
//!
//! * the sample problem parses completely and describes the expected shapes;
//! * the direction tables follow the documented formulas and are unit length;
//! * the terminal support vector equals the analytic support function of the sample's `M`;
//! * the matrix series is exact on a nilpotent `A`;
//! * a full run produces the specified text format;
//! * reconstructed vertices respect the support values they came from;
//! * with zero dynamics and `B = C = π = I`, the bridge stays the terminal set.

use std::path::PathBuf;

use stable_bridge::matrix::Matrix;
use stable_bridge::stable_bridge::{expm, integrate_exp};
use stable_bridge::{convex_polygon, directions, input_parser, output_writer, StableBridge};

fn sample_text() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("input_descriptive.txt");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn norm(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// The four rows of a square `|x_i| <= half` in two dimensions.
fn square_rows(half: f64) -> Vec<Vec<f64>> {
    vec![
        vec![1.0, 0.0, half],
        vec![-1.0, 0.0, half],
        vec![0.0, -1.0, half],
        vec![0.0, 1.0, half],
    ]
}

/// A problem with zero dynamics, identity maps, and `P = Q` equal to a wide square.
fn zero_dynamics_input() -> input_parser::InputData {
    input_parser::InputData {
        n: 2,
        m: 4,
        m_rows: square_rows(0.5),
        dim_a: 2,
        a: Matrix::zeros(2, 2),
        cols_b: 2,
        b: Matrix::identity(2),
        cols_c: 2,
        c: Matrix::identity(2),
        rows_pi: 2,
        pi: Matrix::identity(2),
        p: square_rows(2.0),
        q: square_rows(2.0),
        t: 1.0,
        step: 0.25,
    }
}

/// The six rows of a cube `|x_i| <= half` in three dimensions.
fn cube_rows(half: f64) -> Vec<Vec<f64>> {
    vec![
        vec![1.0, 0.0, 0.0, half],
        vec![-1.0, 0.0, 0.0, half],
        vec![0.0, 1.0, 0.0, half],
        vec![0.0, -1.0, 0.0, half],
        vec![0.0, 0.0, 1.0, half],
        vec![0.0, 0.0, -1.0, half],
    ]
}

/// The degenerate layout of [`zero_dynamics_input`], in three dimensions.
fn zero_dynamics_input_3d() -> input_parser::InputData {
    input_parser::InputData {
        n: 3,
        m: 6,
        m_rows: cube_rows(0.5),
        dim_a: 3,
        a: Matrix::zeros(3, 3),
        cols_b: 3,
        b: Matrix::identity(3),
        cols_c: 3,
        c: Matrix::identity(3),
        rows_pi: 3,
        pi: Matrix::identity(3),
        p: cube_rows(2.0),
        q: cube_rows(2.0),
        t: 1.0,
        step: 0.5,
    }
}

fn is_fixed_9(text: &str) -> bool {
    let body = text.strip_prefix('-').unwrap_or(text);
    let Some((integer, fraction)) = body.split_once('.') else {
        return false;
    };
    !integer.is_empty()
        && integer.bytes().all(|b| b.is_ascii_digit())
        && fraction.len() == 9
        && fraction.bytes().all(|b| b.is_ascii_digit())
}

#[test]
fn sample_input_parses_completely() {
    let data = input_parser::parse(&sample_text()).expect("sample should parse");
    assert_eq!((data.n, data.m), (2, 4));
    assert_eq!((data.dim_a, data.cols_b, data.cols_c), (2, 2, 2));
    assert_eq!((data.rows_pi, data.p.len(), data.q.len()), (2, 4, 4));
    assert_eq!((data.t, data.step), (2.0, 0.2));
    assert_eq!(data.block_count(), 10);
}

#[test]
fn two_dimensional_directions_are_the_sampled_circle() {
    let table = directions::build(2).expect("2D table");
    assert_eq!(table.len(), 110);
    for (i, psi) in table.iter().enumerate() {
        let angle = i as f64 * std::f64::consts::TAU / 110.0;
        assert!((psi[0] - angle.cos()).abs() < 1e-15, "direction {i}: x");
        assert!((psi[1] - angle.sin()).abs() < 1e-15, "direction {i}: y");
        assert!((norm(psi) - 1.0).abs() < 1e-12, "direction {i} is not unit");
    }
}

#[test]
fn three_dimensional_directions_cover_the_latitude_grid() {
    let table = directions::build(3).expect("3D table");
    assert_eq!(table.len(), 110);
    for i in 0..11 {
        let azimuth = i as f64 * std::f64::consts::PI / 10.0 - std::f64::consts::FRAC_PI_2;
        for j in 0..10 {
            let elevation = j as f64 * std::f64::consts::TAU / 10.0;
            let psi = &table[i * 10 + j];
            assert!((psi[0] - azimuth.cos() * elevation.cos()).abs() < 1e-15);
            assert!((psi[1] - elevation.cos() * azimuth.sin()).abs() < 1e-15);
            assert!((psi[2] - elevation.sin()).abs() < 1e-15);
            assert!((norm(psi) - 1.0).abs() < 1e-12, "direction {i}/{j} is not unit");
        }
    }
    assert!(directions::build(4).is_none(), "only 2 and 3 are defined");
}

/// The 2D table takes any count from three up, `110` reproduces the shipped table bit for
/// bit, and 3D refuses anything but its own shape.
#[test]
fn a_two_d_table_can_be_any_count() {
    assert_eq!(
        directions::build_with(2, 110).expect("110"),
        directions::build(2).expect("shipped")
    );
    for count in [3, 158, 205, 254] {
        let table = directions::build_with(2, count).expect("2D table");
        assert_eq!(table.len(), count);
        for (i, psi) in table.iter().enumerate() {
            let angle = i as f64 * std::f64::consts::TAU / count as f64;
            assert!((psi[0] - angle.cos()).abs() < 1e-15, "count {count}: direction {i} x");
            assert!((psi[1] - angle.sin()).abs() < 1e-15, "count {count}: direction {i} y");
            assert!((norm(psi) - 1.0).abs() < 1e-12, "count {count}: direction {i} not unit");
        }
        let gap = (table[0][0] * table[1][0] + table[0][1] * table[1][1])
            .clamp(-1.0, 1.0)
            .acos();
        assert!(
            (gap - std::f64::consts::TAU / count as f64).abs() < 1e-12,
            "count {count}: gap {gap}"
        );
    }
    assert!(
        directions::build_with(2, 2).is_none(),
        "a 2D table needs three directions"
    );
    assert!(
        directions::build_with(3, 158).is_none(),
        "3D is fixed at the shipped shape"
    );
    assert!(directions::build_with(1, 110).is_none(), "one dimension has no table");
}

/// A 2D problem runs on another table, and the recursion follows the table's own length.
#[test]
fn a_two_d_problem_runs_on_another_table() {
    let data = input_parser::parse(&sample_text()).expect("sample should parse");
    let problem = StableBridge::new_with(&data, 158).expect("158-direction 2D problem");
    assert_eq!(problem.directions.len(), 158);
    let blocks = problem.blocks(1, data.step).expect("bounded problem");
    assert_eq!(blocks[0].len(), 158);
    assert!(blocks[0].iter().all(|value| value.is_finite()));
    assert!(
        StableBridge::new_with(&data, 2).is_err(),
        "two directions is not a table"
    );
    let three_d = zero_dynamics_input_3d();
    assert!(
        StableBridge::new_with(&three_d, 158).is_err(),
        "3D has no 158-direction table"
    );
    assert_eq!(
        StableBridge::new(&three_d).expect("shipped 3D").directions.len(),
        110
    );
}

#[test]
fn terminal_support_of_the_square_is_its_support_function() {
    let data = input_parser::parse(&sample_text()).expect("sample should parse");
    let problem = StableBridge::new(&data).expect("sample should be supported");
    let support = problem.terminal_support().expect("bounded terminal set");
    for (i, psi) in problem.directions.iter().enumerate() {
        let expected = 0.5 * (psi[0].abs() + psi[1].abs());
        assert!(
            (support[i] - expected).abs() < 1e-12,
            "direction {i}: got {}, expected {expected}",
            support[i]
        );
    }
}

#[test]
fn nilpotent_exponential_terminates_exactly() {
    let a = Matrix::from_rows(&[vec![0.0, 0.0], vec![1.0, 0.0]]);
    let h = 0.125;

    let e = expm(&a, h);
    assert!((e.get(0, 0) - 1.0).abs() < 1e-15);
    assert!((e.get(0, 1)).abs() < 1e-15);
    assert!((e.get(1, 0) - h).abs() < 1e-15);
    assert!((e.get(1, 1) - 1.0).abs() < 1e-15);

    let integral = integrate_exp(&a, h);
    assert!((integral.get(0, 0) - h).abs() < 1e-15);
    assert!((integral.get(1, 0) - h * h / 2.0).abs() < 1e-15);
}

#[test]
fn a_full_run_produces_the_specified_format() {
    let data = input_parser::parse(&sample_text()).expect("sample should parse");
    let problem = StableBridge::new(&data).expect("sample should be supported");
    let blocks = problem
        .blocks(data.block_count(), data.step)
        .expect("bounded problem");
    assert_eq!(blocks.len(), 10, "floor(T / Step) blocks");

    let mut text = String::new();
    for support in &blocks {
        let vertices = convex_polygon::reconstruct(&problem.directions, support);
        assert!(!vertices.is_empty(), "every block must contain vertices");
        text.push_str(&output_writer::format_block(&vertices));
    }

    let mut blocks_seen = 0;
    for line in text.lines() {
        if line == "separator" {
            blocks_seen += 1;
            continue;
        }
        let fields: Vec<&str> = line.split(' ').collect();
        assert_eq!(fields.len(), data.n, "line should hold {n} numbers: {line}", n = data.n);
        for field in fields {
            assert!(is_fixed_9(field), "not a %.9lf value: {field:?} in {line:?}");
        }
    }
    assert_eq!(blocks_seen, 10, "one separator per block");
}

#[test]
fn reconstructed_vertices_respect_the_computed_support() {
    let data = input_parser::parse(&sample_text()).expect("sample should parse");
    let problem = StableBridge::new(&data).expect("sample should be supported");
    let blocks = problem.blocks(1, data.step).expect("bounded problem");
    let vertices = convex_polygon::reconstruct(&problem.directions, &blocks[0]);

    let mut worst = f64::NEG_INFINITY;
    for vertex in &vertices {
        let violation = convex_polygon::max_violation(&problem.directions, &blocks[0], vertex);
        if violation > worst {
            worst = violation;
        }
    }
    // Vertices are accepted within the 0.01 slack, never more.
    assert!(
        worst <= stable_bridge::VERTEX_EPSILON + 1e-9,
        "worst half-space violation {worst} exceeds the acceptance slack"
    );
}

#[test]
fn zero_dynamics_keeps_the_terminal_set() {
    let data = zero_dynamics_input();
    let problem = StableBridge::new(&data).expect("degenerate problem should be supported");
    let blocks = problem
        .blocks(data.block_count(), data.step)
        .expect("bounded problem");

    assert_eq!(blocks.len(), 4, "floor(T / Step) blocks");

    for (index, support) in blocks.iter().enumerate() {
        // With A = 0 nothing moves, so the recursion has to stay at the support function of
        // the terminal square. It stays there to the algorithm's own accuracy: applied to the
        // exact square support, the dual update returns values between `c` and `c - 1.2771e-2`
        // (masked directions keep the exact value, the rest go through the hull formula). The
        // residual is identical in every block, so it does not accumulate.
        for (i, psi) in problem.directions.iter().enumerate() {
            let exact = 0.5 * (psi[0].abs() + psi[1].abs());
            assert!(
                (support[i] - exact).abs() <= 0.014,
                "block {index}, direction {i}: support {} vs exact {exact}",
                support[i]
            );
        }

        let vertices = convex_polygon::reconstruct(&problem.directions, support);
        assert!(!vertices.is_empty(), "block {index} is empty");

        // The 0.01 acceptance slack lets a vertex sit that far outside each half-space, and
        // the 110 directions miss the square's facet normals, so the reconstructed polygon is
        // slightly taller than M: the measured worst extent is 0.5143.
        for vertex in &vertices {
            assert!(
                vertex[0].abs() <= 0.5 + 0.02 && vertex[1].abs() <= 0.5 + 0.02,
                "block {index}: vertex {vertex:?} left the terminal square"
            );
        }

        // It still covers M: every corner is close to a reconstructed vertex.
        for corner in [[0.5, 0.5], [0.5, -0.5], [-0.5, 0.5], [-0.5, -0.5]] {
            let nearest = vertices
                .iter()
                .map(|v| (v[0] - corner[0]).abs().max((v[1] - corner[1]).abs()))
                .fold(f64::INFINITY, f64::min);
            assert!(
                nearest <= 0.02,
                "block {index}: corner {corner:?} is {nearest:.4} away from the nearest vertex"
            );
        }
    }
}

/// Exact support function of the terminal cube `|x_i| <= 0.5`.
fn cube_support(psi: &[f64]) -> f64 {
    0.5 * (psi[0].abs() + psi[1].abs() + psi[2].abs())
}

#[test]
fn three_dimensional_run_reaches_a_fixed_point() {
    let data = zero_dynamics_input_3d();
    let problem = StableBridge::new(&data).expect("3D problem should be supported");
    assert_eq!(problem.dimension(), 3);
    assert_eq!(problem.directions.len(), 110);

    // Vertex enumeration is exact on a cube, so the starting vector must match the analytic
    // support function before the recursion touches it.
    let terminal = problem.terminal_support().expect("bounded terminal set");
    for (i, psi) in problem.directions.iter().enumerate() {
        let expected = cube_support(psi);
        assert!(
            (terminal[i] - expected).abs() < 1e-12,
            "terminal direction {i}: got {}, expected {expected}",
            terminal[i]
        );
    }

    let blocks = problem
        .blocks(data.block_count(), data.step)
        .expect("bounded problem");
    assert_eq!(blocks.len(), 2, "floor(T / Step) blocks");
    assert!(
        blocks[0].iter().all(|value| value.is_finite()),
        "support vector must stay finite"
    );

    // With A = 0 and P = Q nothing moves, so block two must equal block one. The measured
    // difference is 2e-16, i.e. the recursion is already sitting on its fixed point.
    let drift = blocks[0]
        .iter()
        .zip(&blocks[1])
        .map(|(a, b)| (a - b).abs())
        .fold(0.0, f64::max);
    assert!(
        drift < 1e-12,
        "blocks differ by {drift}: the degenerate case is not stationary"
    );

    // The 3D accuracy is much worse than the 2D one. The 110 directions sit on 10 elevation
    // rings, so the hull formula in stage 5 of the dual update underestimates by up to
    // 9.4979e-2 (1.2771e-2 in 2D). That figure belongs to the grid, not to the time step, so
    // 0.1 is the honest bound.
    let worst = blocks[0]
        .iter()
        .zip(&problem.directions)
        .map(|(value, psi)| (value - cube_support(psi)).abs())
        .fold(0.0, f64::max);
    assert!(
        worst <= 0.1,
        "3D support drifts {worst} away from the terminal cube"
    );
}

#[test]
fn three_dimensional_output_has_three_fields() {
    let data = zero_dynamics_input_3d();
    let problem = StableBridge::new(&data).expect("3D problem should be supported");
    let blocks = problem.blocks(1, data.step).expect("bounded problem");

    let vertices = convex_polygon::reconstruct(&problem.directions, &blocks[0]);
    assert!(!vertices.is_empty(), "3D reconstruction produced no vertices");

    let mut worst = f64::NEG_INFINITY;
    for vertex in &vertices {
        assert_eq!(vertex.len(), 3, "a 3D vertex has three components");
        worst = worst.max(convex_polygon::max_violation(
            &problem.directions,
            &blocks[0],
            vertex,
        ));
    }
    assert!(
        worst <= stable_bridge::VERTEX_EPSILON + 1e-9,
        "worst half-space violation {worst} exceeds the acceptance slack"
    );

    // The intersection is wider than M in the directions the grid does not sample. `x` and
    // `y` have their facet normals in the table and stay at exactly 0.5, but the z axis is
    // not in it — the 10 elevation levels step by 36°, so the closest is 18° from the pole —
    // so nothing bounds z directly and the printed polytope reaches 0.6676 there. It happens
    // even though the support values sit below the cube's, so it is asserted rather than left
    // to be rediscovered as a bug.
    for vertex in &vertices {
        assert!(
            vertex[0].abs() <= 0.52 && vertex[1].abs() <= 0.52,
            "x/y extent left the cube: {vertex:?}"
        );
        assert!(vertex[2].abs() <= 0.68, "z extent beyond the grid gap: {vertex:?}");
    }

    let text = output_writer::format_block(&vertices);
    let mut lines = 0;
    for line in text.lines() {
        if line == "separator" {
            continue;
        }
        let fields: Vec<&str> = line.split(' ').collect();
        assert_eq!(fields.len(), 3, "3D line should hold three numbers: {line}");
        for field in fields {
            assert!(is_fixed_9(field), "not a %.9lf value: {field:?} in {line:?}");
        }
        lines += 1;
    }
    assert_eq!(lines, vertices.len(), "one line per vertex");
    assert_eq!(text.lines().filter(|line| *line == "separator").count(), 1);
}

/// The 3D sample is the degenerate layout written out in the input grammar, so the parser
/// path and the in-code problem must agree to the last bit.
#[test]
fn three_dimensional_file_parses_and_matches_the_built_problem() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join("three_dimensional.txt");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));

    let data = input_parser::parse(&text).expect("3D sample should parse");
    assert_eq!((data.n, data.m, data.dim_a), (3, 6, 3));
    assert_eq!((data.cols_b, data.cols_c, data.rows_pi), (3, 3, 3));
    assert_eq!((data.rows_pi, data.p.len(), data.q.len()), (3, 6, 6));
    assert_eq!((data.t, data.step), (1.0, 0.5));
    assert_eq!(data.block_count(), 2);

    let from_text = StableBridge::new(&data).expect("supported");
    let from_code = StableBridge::new(&zero_dynamics_input_3d()).expect("supported");
    let parsed = from_text.blocks(data.block_count(), data.step).expect("bounded");
    let built = from_code.blocks(2, 0.5).expect("bounded");
    assert_eq!(parsed.len(), built.len());
    for (block, expected) in parsed.iter().zip(&built) {
        for (value, target) in block.iter().zip(expected) {
            assert!(
                (value - target).abs() < 1e-12,
                "file and in-code problem disagree: {value} vs {target}"
            );
        }
    }
}
