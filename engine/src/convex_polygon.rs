//! Set reconstruction from a support function.
//!
//! The 110 values `c_i` define 110 half-spaces `<psi_i, x> <= c_i`. Their intersection is a
//! polytope: every `C(110, n)` choice of `n` half-spaces is solved as a linear system, and a
//! solution is kept when it satisfies all 110 half-spaces within the `0.01` slack.

use crate::combinations;
use crate::linear_system;
use crate::polytope;
use crate::support_function::dot;
use crate::VERTEX_EPSILON;

/// Reconstructs the vertex set defined by `directions` and their `support` values.
///
/// The result is in enumeration order and may contain near-duplicates created by the
/// acceptance slack.
pub fn reconstruct(directions: &[Vec<f64>], support: &[f64]) -> Vec<Vec<f64>> {
    reconstruct_with(directions, support, VERTEX_EPSILON)
}

/// [`reconstruct`] with an explicit acceptance slack.
///
/// The default `0.01` is the document's vertex tolerance, and at a dense table it is larger than
/// the band the polytopes are being used to measure: a vertex of the inner polytope can be up to
/// `0.01` outside the caps that admitted it, so the measured band falls below the width at the
/// samples, which no true maximum can (`06-exact-band-and-the-rung.md`). A band measurement
/// needs the solve tolerance instead, about `1e-9`.
pub fn reconstruct_with(
    directions: &[Vec<f64>],
    support: &[f64],
    epsilon: f64,
) -> Vec<Vec<f64>> {
    assert_eq!(
        directions.len(),
        support.len(),
        "reconstruction: direction and support counts differ"
    );
    if directions.is_empty() {
        return Vec::new();
    }
    let n = directions[0].len();
    let mut vertices: Vec<Vec<f64>> = Vec::new();

    combinations::for_each_combination(directions.len(), n, |indices| {
        let coefficients: Vec<Vec<f64>> = indices.iter().map(|&i| directions[i].clone()).collect();
        let rhs: Vec<f64> = indices.iter().map(|&i| support[i]).collect();
        let Some(point) = linear_system::solve(&coefficients, &rhs, linear_system::PIVOT_EPSILON)
        else {
            return;
        };
        if satisfies_all(directions, support, &point, epsilon)
            && !polytope::contains_near(&vertices, &point, epsilon)
        {
            vertices.push(point);
        }
    });

    vertices
}

/// Whether `point` satisfies every half-space within `eps`.
pub fn satisfies_all(
    directions: &[Vec<f64>],
    support: &[f64],
    point: &[f64],
    eps: f64,
) -> bool {
    directions
        .iter()
        .zip(support)
        .all(|(direction, bound)| dot(direction, point) <= bound + eps)
}

/// Largest violation `max_i (<psi_i, point> - c_i)`; non-positive means inside.
///
/// Used by the tests to check the reconstruction against the support values it came from
/// without depending on the acceptance slack.
pub fn max_violation(directions: &[Vec<f64>], support: &[f64], point: &[f64]) -> f64 {
    let mut worst = f64::NEG_INFINITY;
    for (direction, bound) in directions.iter().zip(support) {
        let value = dot(direction, point) - bound;
        if value > worst {
            worst = value;
        }
    }
    worst
}
