//! Support-vector arithmetic.
//!
//! The Minkowski sum of two convex sets has a support function equal to the sum of theirs,
//! so in the dual representation used throughout this program a Minkowski sum is just a
//! pointwise addition of the two 110-element support vectors.

/// Pointwise sum of two support vectors.
///
/// # Panics
/// If the two vectors have different lengths.
pub fn support_sum(a: &[f64], b: &[f64]) -> Vec<f64> {
    assert_eq!(a.len(), b.len(), "support sum: length mismatch");
    a.iter().zip(b).map(|(x, y)| x + y).collect()
}

/// Pointwise difference of two support vectors.
///
/// This is only the algebraic difference of the two vectors; it is *not* the Pontryagin
/// difference of the sets, which is what [`crate::pontryagin_difference::dual_update`]
/// computes.
pub fn support_difference(a: &[f64], b: &[f64]) -> Vec<f64> {
    assert_eq!(a.len(), b.len(), "support difference: length mismatch");
    a.iter().zip(b).map(|(x, y)| x - y).collect()
}
