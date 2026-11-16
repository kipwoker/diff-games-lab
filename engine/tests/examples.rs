//! The example problems, checked against their analytic answers.
//!
//! Two families: a diamond target with no drift (where the interval map is the identity), and
//! a square target with `A = [[0,0],[1,0]]` at three control-size ratios. The cases where
//! `P = Q` cancel in the recurrence, and the remaining two show the bridge growing or
//! shrinking with the first player's advantage.

use stable_bridge::matrix::Matrix;
use stable_bridge::{convex_polygon, input_parser, StableBridge};

/// The four rows of the diamond `|x1| + |x2| <= radius`.
fn diamond_rows(radius: f64) -> Vec<Vec<f64>> {
    vec![
        vec![1.0, 1.0, radius],
        vec![-1.0, 1.0, radius],
        vec![1.0, -1.0, radius],
        vec![-1.0, -1.0, radius],
    ]
}

/// The four rows of the square `|x_i| <= half`.
fn square_rows(half: f64) -> Vec<Vec<f64>> {
    vec![
        vec![1.0, 0.0, half],
        vec![-1.0, 0.0, half],
        vec![0.0, 1.0, half],
        vec![0.0, -1.0, half],
    ]
}

fn input(
    m_rows: Vec<Vec<f64>>,
    a: Matrix,
    p: Vec<Vec<f64>>,
    q: Vec<Vec<f64>>,
    t: f64,
    step: f64,
) -> input_parser::InputData {
    let n = a.rows();
    input_parser::InputData {
        n,
        m: m_rows.len(),
        m_rows,
        dim_a: n,
        a,
        cols_b: n,
        b: Matrix::identity(n),
        cols_c: n,
        c: Matrix::identity(n),
        rows_pi: n,
        pi: Matrix::identity(n),
        p,
        q,
        t,
        step,
    }
}

/// `ẋ = u − v`, diamond `M` of radius 1, `P` of radius 2, `Q` of radius 1, `T = 2`.
fn example_1() -> input_parser::InputData {
    input(
        diamond_rows(1.0),
        Matrix::zeros(2, 2),
        diamond_rows(2.0),
        diamond_rows(1.0),
        2.0,
        0.1,
    )
}

/// `A = [[0,0],[1,0]]`, square `M` of half-width 0.5, `P` radius `alpha`, `Q` radius `beta`.
fn example_2(alpha: f64, beta: f64) -> input_parser::InputData {
    let mut a = Matrix::zeros(2, 2);
    a.set(1, 0, 1.0);
    input(
        square_rows(0.5),
        a,
        square_rows(alpha),
        square_rows(beta),
        0.4,
        0.05,
    )
}

/// The six rows of the cube `|x_i| <= half`.
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

/// `A = 0`, cube `M` of half-width 0.5, `P` radius 2, `Q` radius 1, `T = 1`.
fn example_3d() -> input_parser::InputData {
    input(
        cube_rows(0.5),
        Matrix::zeros(3, 3),
        cube_rows(2.0),
        cube_rows(1.0),
        1.0,
        0.25,
    )
}

/// Support in `+x1` per block, which for these symmetric sets is the extreme `|x1|`.
fn x_supports(data: &input_parser::InputData, count: usize) -> Vec<f64> {
    let problem = StableBridge::new(data).expect("setup");
    problem
        .blocks(count, data.step)
        .expect("blocks")
        .iter()
        .map(|c| c[0])
        .collect()
}

#[test]
fn example_1_grows_to_the_diamond_of_radius_three() {
    let data = example_1();
    let xs = x_supports(&data, data.block_count());

    let last = *xs.last().expect("blocks");
    assert!((last - 3.0).abs() < 2e-3, "last block x-support {last}");
    for pair in xs.windows(2) {
        assert!(pair[1] >= pair[0] - 1e-6, "growth reversed: {pair:?}");
    }
}

#[test]
fn example_2_with_equal_bounds_stays_at_the_terminal_set() {
    // `P = Q` cancels the two images, so the set is the target for every block.
    let data = example_2(2.0, 2.0);
    for x in x_supports(&data, data.block_count()) {
        assert!((x - 0.5).abs() < 1e-6, "x-support {x}");
    }
}

#[test]
fn example_2_grows_and_shrinks_with_the_constraint_ratio() {
    let growing = example_2(2.0, 1.0);
    let xs = x_supports(&growing, growing.block_count());
    let last = *xs.last().expect("blocks");
    assert!(last > 0.5, "alpha 2 beta 1 did not grow: {last}");
    assert!(last < 1.2, "alpha 2 beta 1 ran away: {last}");

    let shrinking = example_2(1.0, 2.0);
    let xs = x_supports(&shrinking, shrinking.block_count());
    let last = *xs.last().expect("blocks");
    assert!(last > 0.0, "alpha 1 beta 2 collapsed to {last}");
    assert!(last < 0.5, "alpha 1 beta 2 did not shrink: {last}");
}

#[test]
fn refining_the_step_leaves_the_limit_alone() {
    // Each block already refines its interval 110 ways, so halving the step at fixed `T`
    // must not move the final set: that is the mesh consistency an alternating sum needs.
    let coarse = example_2(2.0, 1.0);
    let mut fine = example_2(2.0, 1.0);
    fine.step = 0.0125;

    let last = |data: &input_parser::InputData| {
        *x_supports(data, data.block_count())
            .last()
            .expect("blocks")
    };
    let (coarse_last, fine_last) = (last(&coarse), last(&fine));
    assert!(
        (coarse_last - fine_last).abs() < 1e-3,
        "step 0.05 gives {coarse_last}, step 0.0125 gives {fine_last}"
    );
}

#[test]
fn the_three_dimensional_case_grows_a_quarter_per_block() {
    let data = example_3d();
    let problem = StableBridge::new(&data).expect("setup");
    let blocks = problem
        .blocks(data.block_count(), data.step)
        .expect("blocks");
    assert_eq!(blocks.len(), 4, "floor(T / Step) blocks");

    // `x` is a sampled direction, so its support is the exact half-width of the cube the
    // bridge grows into: 0.75, 1.00, 1.25, 1.50.
    for (index, support) in blocks.iter().enumerate() {
        let expected = 0.5 + (index + 1) as f64 * data.step;
        assert!(
            (support[0] - expected).abs() < 1e-6,
            "block {}: x support {}, expected {expected}",
            index + 1,
            support[0]
        );
    }

    // No sample lies near the `z` pole, so the printed shell is taller than the cube by the
    // grid's own factor. It is the same factor in every block, because all support values
    // scale together.
    let last = blocks.last().expect("blocks");
    let half = 0.5 + blocks.len() as f64 * data.step;
    let vertices = convex_polygon::reconstruct(&problem.directions, last);
    let z = vertices.iter().map(|v| v[2].abs()).fold(0.0, f64::max);
    let ratio = z / half;
    assert!(
        (ratio - 1.325).abs() < 1e-2,
        "z extent {z} for half-width {half} is a factor {ratio} taller"
    );
}
