//! The sampled directions: 110 by default, any count in 2D.
//!
//! A direction table exists for state dimensions 2 and 3 only; anything else is `None`.

use std::f64::consts::{FRAC_PI_2, PI};

use crate::DIRECTION_COUNT;

/// Applies the angle metric `diag(1, …, 1, 1 / scale_theta)` and renormalises.
///
/// A heading is an angle, so a support function over `(x, y, θ)` needs a length before the
/// table's inner products mean anything; `scale_theta` is the turning radius `v_p / ω̄`, which
/// is the length one radian of heading is worth. The last component is divided and the result
/// normalised, so a direction that is pure position or pure angle comes back unchanged.
///
/// # Panics
/// If `scale_theta` is not positive, or a direction has fewer than two components.
pub fn apply_metric(table: &[Vec<f64>], scale_theta: f64) -> Vec<Vec<f64>> {
    assert!(scale_theta > 0.0, "metric: scale_theta must be positive");
    assert!(
        table.iter().all(|direction| direction.len() >= 2),
        "metric: every direction needs two components"
    );
    table
        .iter()
        .map(|direction| {
            let mut scaled = direction.clone();
            if let Some(last) = scaled.last_mut() {
                *last /= scale_theta;
            }
            let norm = scaled.iter().map(|value| value * value).sum::<f64>().sqrt();
            if norm > 0.0 {
                for value in &mut scaled {
                    *value /= norm;
                }
            }
            scaled
        })
        .collect()
}

/// Builds the direction table for a state dimension of 2 or 3 at [`DIRECTION_COUNT`].
pub fn build(dimension: usize) -> Option<Vec<Vec<f64>>> {
    build_with(dimension, DIRECTION_COUNT)
}

/// Builds a direction table of `count` directions.
///
/// `count` is the 2D table's size, and `110` reproduces [`build`] to the last bit. 3D is fixed
/// at the shipped shape below: a different size needs azimuth and elevation counts, which this
/// single-count signature cannot express, so only [`DIRECTION_COUNT`] is accepted there.
///
/// # Table
/// * 2D: `ψ_i = (cos θ_i, sin θ_i)` with `θ_i = i · 2π / count`, `i = 0 … count − 1`.
/// * 3D: 11 azimuths × 10 elevations,
///   `ψ = (cos γ_i · cos θ_j, cos θ_j · sin γ_i, sin θ_j)` with
///   `γ_i = i · π / 10 − π/2` (`i = 0 … 10`) and `θ_j = j · 2π / 10` (`j = 0 … 9`).
///
/// The first two components are scaled by `cos θ_j` and the third is `sin θ_j`, so the 10
/// elevation levels of `θ` fix the `z` coordinate and the 11 azimuths of `γ` sweep the
/// equator.
///
/// The arithmetic order (`i · 2π` then `/ count`, `i · π` then `/ 10` then `− π/2`) is fixed so
/// the table is reproducible to the last bit.
pub fn build_with(dimension: usize, count: usize) -> Option<Vec<Vec<f64>>> {
    match dimension {
        2 if count >= 3 => {
            let mut out = Vec::with_capacity(count);
            for i in 0..count {
                let angle = i as f64 * (2.0 * PI) / count as f64;
                out.push(vec![angle.cos(), angle.sin()]);
            }
            Some(out)
        }
        3 if count == DIRECTION_COUNT => build_3d(11, 10),
        _ => None,
    }
}

/// Builds a 3D table of `azimuths x elevations` directions.
///
/// `build_3d(11, 10)` reproduces [`build`] bit for bit: the arithmetic order is the shipped one
/// with the two counts written generically, so the constants `10` in the angles are the counts
/// minus one and the count itself. Any shape of two or more each is a table; the product is the
/// only count it implies, and [`build_with`] still accepts the shipped product alone.
pub fn build_3d(azimuths: usize, elevations: usize) -> Option<Vec<Vec<f64>>> {
    if azimuths < 2 || elevations < 2 {
        return None;
    }
    let mut out = Vec::with_capacity(azimuths * elevations);
    for i in 0..azimuths {
        let azimuth = i as f64 * PI / (azimuths - 1) as f64 - FRAC_PI_2;
        let (cos_azimuth, sin_azimuth) = (azimuth.cos(), azimuth.sin());
        for j in 0..elevations {
            let elevation = j as f64 * (2.0 * PI) / elevations as f64;
            let (cos_elevation, sin_elevation) = (elevation.cos(), elevation.sin());
            out.push(vec![
                cos_azimuth * cos_elevation,
                cos_elevation * sin_azimuth,
                sin_elevation,
            ]);
        }
    }
    Some(out)
}
