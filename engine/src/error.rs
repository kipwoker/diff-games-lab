//! The two-neighbour bound on a sampled reconstruction, and the certificate's allowance.
//!
//! A table of directions `ψ_i` and supports `c_i` reconstructs the set
//! `P = {x : ⟨ψ_i, x⟩ ≤ c_i}`. Its support at a direction the table misses is not in the
//! engine's algebra, but it is in two of the constraints: dropping all but two planes only
//! enlarges the set, so the extent of the two-plane set along `ψ` bounds `h_P(ψ)`, and the
//! smallest bound over the cell's neighbour pairs is the tightest this gives. In 2D that is
//! the closed form measured in `kb/entries/06-two-neighbour-bound.md`,
//! `(c_a·sin ρ_b + c_b·sin ρ_a)/sin(ρ_a + ρ_b)`; the two-plane solve below is the same
//! number in any dimension, and in 3D the same solve over the nearest three samples is
//! exact on the cell's edges and loose only where all three planes bind.
//!
//! Two consequences are used here. The reconstruction's support at the cell's midpoints
//! gives an upper bound on the set's circumradius, by sublinearity of the support function,
//! and that radius turns the one-term form `h·tan ρ + R(1/cos ρ − 1)` into an allowance
//! computed from the table instead of typed. `ρ` is the covering radius of the direction's
//! own cell, which is local: a table with uneven spacing is allowed a smaller allowance in
//! its dense parts, which the global covering radius cannot express.
//!
//! The covering radius is the largest angle from the direction to any point of its spherical
//! Voronoi cell. In 2D that is half the gap to the nearest neighbour, because the cell is an
//! arc whose ends are the two midpoints. In 3D it is not: a cell is a spherical polygon whose
//! vertices are the circumcenters of sample triples, and the farthest point of a long thin cell
//! is one of those vertices, well beyond half the nearest-neighbour gap. The first version of
//! this module used the half gap in 3D too, which understated the radius and therefore the
//! allowance (`06-three-d-covering-radius.md`).
//!
//! In 2D the allowance bounds how far the printed set can exceed the true one. In 3D it does
//! not: with the exact covering radius a cube's cell vertex puts the measured excess at
//! `104.6%` of the allowance, so a 3D figure is an estimate and not a certificate
//! (`06-three-d-allowance-is-loose.md`, `06-three-d-bound-fails-at-a-cell-vertex.md`). Neither
//! dimension's form bounds the opposite error — stored supports sitting short of the true ones —
//! which is track 1's inward deficit and needs `e_in`.

use std::f64::consts::FRAC_PI_2;

use crate::linear_system;
use crate::support_function::dot;

/// Inner products this close are a tie: `1e-12` on a unit sphere is `8·10⁻⁵` degrees.
const TIE_TOLERANCE: f64 = 1e-12;

/// Extent of `{x : ⟨a, x⟩ ≤ c_a, ⟨b, x⟩ ≤ c_b}` along `ψ`.
///
/// `a` and `b` are unit directions. The two binding points are solved in the plane they
/// span and projected on `ψ`. Parallel planes leave the set unbounded across their
/// direction, so the answer is [`f64::INFINITY`] rather than a large number.
pub fn pair_extent(a: &[f64], c_a: f64, b: &[f64], c_b: f64, psi: &[f64]) -> f64 {
    let ab = dot(a, b);
    let det = 1.0 - ab * ab;
    if det <= 1e-14 {
        return f64::INFINITY;
    }
    let alpha = (c_a - ab * c_b) / det;
    let beta = (c_b - ab * c_a) / det;
    (0..psi.len())
        .map(|k| psi[k] * (alpha * a[k] + beta * b[k]))
        .sum()
}

/// Angle between two unit vectors, clamped against rounding.
pub fn angle(a: &[f64], b: &[f64]) -> f64 {
    dot(a, b).clamp(-1.0, 1.0).acos()
}

/// The `count` nearest directions to `index`, nearest first.
fn nearest(directions: &[Vec<f64>], index: usize, count: usize) -> Vec<usize> {
    let mut scored: Vec<(f64, usize)> = (0..directions.len())
        .filter(|&other| other != index)
        .map(|other| (dot(&directions[index], &directions[other]), other))
        .collect();
    scored.sort_by(|left, right| right.0.total_cmp(&left.0));
    scored
        .into_iter()
        .take(count)
        .map(|(_, other)| other)
        .collect()
}

/// Whether `ψ` lies in the positive cone of `a` and `b`.
///
/// This is the condition under which the two planes bound the set along `ψ` at all. Outside the
/// cone the two-plane set runs off along `ψ`, so `pair_extent` returns a vertex projection
/// rather than an extent — a number that a minimum will happily prefer while it bounds nothing.
fn in_positive_cone(a: &[f64], b: &[f64], psi: &[f64]) -> bool {
    let ab = dot(a, b);
    let det = 1.0 - ab * ab;
    if det <= 1e-14 {
        return false;
    }
    let alpha = (dot(a, psi) - ab * dot(b, psi)) / det;
    let beta = (dot(b, psi) - ab * dot(a, psi)) / det;
    if alpha < -1e-12 || beta < -1e-12 {
        return false;
    }
    (0..psi.len())
        .map(|k| {
            let difference = psi[k] - alpha * a[k] - beta * b[k];
            difference * difference
        })
        .sum::<f64>()
        .sqrt()
        <= 1e-9
}

/// Upper bound on `h_P(ψ)` from the pairs among `members`: the smallest of them.
///
/// Only a pair whose two planes straddle `ψ` has a bounded two-plane set along it; a pair that
/// does not straddle contributes a vertex projection, which is not an extent and can understate
/// the support (`06-cell-bound-was-not-a-bound.md`). When no pair straddles, the answer is
/// [`f64::INFINITY`]: two planes give no bound there, which is the generic 3D probe.
pub fn cell_bound(directions: &[Vec<f64>], supports: &[f64], index: usize, psi: &[f64]) -> f64 {
    let dimension = directions[index].len();
    let neighbours = if dimension == 2 { 3 } else { 4 };
    let mut members = nearest(directions, index, neighbours);
    members.push(index);
    let mut best = f64::INFINITY;
    for (position, &a) in members.iter().enumerate() {
        for &b in &members[position + 1..] {
            if !in_positive_cone(&directions[a], &directions[b], psi) {
                continue;
            }
            let value = pair_extent(
                &directions[a],
                supports[a],
                &directions[b],
                supports[b],
                psi,
            );
            if value < best {
                best = value;
            }
        }
    }
    best
}

/// The reconstruction's support at an arbitrary unit direction `ψ`.
///
/// [`f64::INFINITY`] when no pair in the nearest cell straddles `ψ`, which is the generic 3D
/// probe: two planes give no bound there, and the triple is the alternative for that case.
pub fn support_at(directions: &[Vec<f64>], supports: &[f64], psi: &[f64]) -> f64 {
    let mut nearest_index = 0;
    let mut nearest_dot = f64::NEG_INFINITY;
    for (index, direction) in directions.iter().enumerate() {
        let value = dot(direction, psi);
        if value > nearest_dot {
            nearest_dot = value;
            nearest_index = index;
        }
    }
    cell_bound(directions, supports, nearest_index, psi)
}

/// The `count` directions nearest to `psi`, nearest first.
fn nearest_to(directions: &[Vec<f64>], psi: &[f64], count: usize) -> Vec<usize> {
    let mut scored: Vec<(f64, usize)> = directions
        .iter()
        .enumerate()
        .map(|(index, direction)| (dot(direction, psi), index))
        .collect();
    scored.sort_by(|left, right| right.0.total_cmp(&left.0));
    scored
        .into_iter()
        .take(count)
        .map(|(_, index)| index)
        .collect()
}

/// Tightest three-plane bound on `h_P(ψ)`, or [`f64::INFINITY`] when no triple straddles it.
///
/// Each triple of half-spaces bounds the reconstruction from outside, so each straddling triple
/// gives a bound and the smallest is the tightest. This is the 3D figure that is a bound where the
/// one-term form is not: the cube at `16 x 15` exceeds the one-term at one probe in `2500`, and the
/// triple is exact there (`06-three-d-max-bound-leaves-the-flank-unproved.md`).
///
/// The triples are searched among the nearest six samples; a straddling triple outside that set is
/// missed, which makes the `11–16%` of directions with no triple an upper bound on the unproved
/// flank rather than an exact count.
pub fn triple_bound(directions: &[Vec<f64>], supports: &[f64], psi: &[f64]) -> f64 {
    if directions.len() < 3 || directions[0].len() != 3 {
        return f64::INFINITY;
    }
    let members = nearest_to(directions, psi, 6);
    let mut best = f64::INFINITY;
    for first in 0..members.len() {
        for second in (first + 1)..members.len() {
            for third in (second + 1)..members.len() {
                let a = members[first];
                let b = members[second];
                let c = members[third];
                let matrix = vec![
                    directions[a].clone(),
                    directions[b].clone(),
                    directions[c].clone(),
                ];
                // `ψ` must be in the positive cone of the three normals. The solve's rows are its
                // equations, so the coefficients come from the *transpose*: `Nᵀ λ = ψ`.
                let transposed = vec![
                    vec![directions[a][0], directions[b][0], directions[c][0]],
                    vec![directions[a][1], directions[b][1], directions[c][1]],
                    vec![directions[a][2], directions[b][2], directions[c][2]],
                ];
                let Some(weights) =
                    linear_system::solve(&transposed, psi, linear_system::PIVOT_EPSILON)
                else {
                    continue;
                };
                if weights.iter().any(|weight| *weight < -1e-12) {
                    continue;
                }
                let rhs = vec![supports[a], supports[b], supports[c]];
                let Some(vertex) =
                    linear_system::solve(&matrix, &rhs, linear_system::PIVOT_EPSILON)
                else {
                    continue;
                };
                best = best.min(dot(psi, &vertex));
            }
        }
    }
    best
}

/// Upper bound on the circumradius from the sampled supports alone.
///
/// For `ψ` between two samples, sublinearity gives
/// `h_P(ψ) ≤ (h(ψ_a) + h(ψ_b)) / (2 cos(ρ/2))`; the largest of those over adjacent pairs
/// bounds the largest support, which is the circumradius of the printed set, and the true
/// set sits inside it.
pub fn circumradius_bound(directions: &[Vec<f64>], supports: &[f64]) -> f64 {
    let mut radius = supports.iter().copied().fold(0.0_f64, f64::max);
    for index in 0..directions.len() {
        for other in nearest(directions, index, 4) {
            let gap = angle(&directions[index], &directions[other]);
            if gap < FRAC_PI_2 {
                let bound = (supports[index] + supports[other]) / (2.0 * (gap / 2.0).cos());
                radius = radius.max(bound);
            }
        }
    }
    radius
}

/// Unit vector perpendicular to the plane through `a`, `b`, `c`, on their side of the origin.
///
/// This is the spherical circumcenter of the triple: the point equidistant from all three.
/// `None` when the three are collinear with the origin, where no circumcenter exists.
fn spherical_circumcenter(a: &[f64], b: &[f64], c: &[f64]) -> Option<Vec<f64>> {
    let u: Vec<f64> = (0..3).map(|k| b[k] - a[k]).collect();
    let v: Vec<f64> = (0..3).map(|k| c[k] - a[k]).collect();
    let cross = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let norm = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    if norm < 1e-12 {
        return None;
    }
    let mut center: Vec<f64> = cross.iter().map(|value| value / norm).collect();
    let side: f64 = (0..3)
        .map(|k| center[k] * (a[k] + b[k] + c[k]))
        .sum();
    if side < 0.0 {
        for value in &mut center {
            *value = -*value;
        }
    }
    Some(center)
}

/// Largest angle from each direction to a point of its spherical Voronoi cell.
///
/// In 2D the cell is an arc and the answer is half the gap to the nearest neighbour. In 3D the
/// cell's vertices are the circumcenters of sample triples, so the answer is the largest angle
/// from the direction to a circumcenter whose nearest samples include it. Every such centre is
/// counted, ties included: a shared vertex bounds all the cells it is incident to. This is the
/// `ρ` the one-term form assumes; half the nearest-neighbour gap is its value in 2D only.
///
/// Exact up to floating point. A triple whose plane passes through the origin has no unique
/// circumcenter, so both poles of its great circle are credited; that is what makes the figure
/// exact rather than a lower bound (`06-degenerate-vertices-need-the-antipode.md`).
///
/// # Panics
/// If the table is empty or its directions are not two- or three-dimensional.
pub fn covering_radius(directions: &[Vec<f64>]) -> Vec<f64> {
    let (unique, back) = unique_directions(directions);
    let radius = covering_radius_distinct(&unique);
    back.iter().map(|slot| radius[*slot]).collect()
}

/// First occurrence of each distinct direction, and the slot every entry occupies.
///
/// The shipped 3D table stacks the `±90°` azimuths and repeats ten of its directions. A
/// repeated site has no separating bisector, so its Voronoi cell is the cell of the site it
/// repeats; computing on the distinct directions and mapping back is the only reading that
/// gives it a radius instead of zero.
fn unique_directions(directions: &[Vec<f64>]) -> (Vec<Vec<f64>>, Vec<usize>) {
    let mut unique: Vec<Vec<f64>> = Vec::new();
    let mut back = Vec::with_capacity(directions.len());
    for direction in directions {
        let mut slot = unique.len();
        for (index, kept) in unique.iter().enumerate() {
            if (1.0 - dot(kept, direction)).abs() <= 1e-9 {
                slot = index;
                break;
            }
        }
        if slot == unique.len() {
            unique.push(direction.clone());
        }
        back.push(slot);
    }
    (unique, back)
}

/// [`covering_radius`] for a table in which no direction is repeated.
fn covering_radius_distinct(directions: &[Vec<f64>]) -> Vec<f64> {
    assert!(!directions.is_empty(), "covering radius: empty table");
    let count = directions.len();
    let dimension = directions[0].len();
    assert!(
        dimension == 2 || dimension == 3,
        "covering radius: dimension {dimension}"
    );

    let mut radius = vec![0.0_f64; count];
    for index in 0..count {
        let mut nearest = f64::NEG_INFINITY;
        for other in 0..count {
            if other != index {
                nearest = nearest.max(dot(&directions[index], &directions[other]));
            }
        }
        radius[index] = 0.5 * nearest.clamp(-1.0, 1.0).acos();
    }

    if dimension != 3 {
        return radius;
    }

    for a in 0..count {
        for b in (a + 1)..count {
            for c in (b + 1)..count {
                let Some(center) = spherical_circumcenter(&directions[a], &directions[b], &directions[c])
                else {
                    continue;
                };
                // A centre is a vertex of the cell of every sample at the maximum distance from
                // it: the three that generated it, and any co-circular sample. The first version
                // treated that tie as degenerate and skipped it, which discarded *every* vertex —
                // the generating samples are equidistant by construction — leaving only
                // non-Delaunay centres, each a point of some cell, so the result was always a
                // strict lower bound (`06-three-d-radius-is-a-lower-bound.md`).
                // Both sides of the great-circle plane, not only the side the sign convention
                // keeps. A vertex defined only by a degenerate triple (its plane through the
                // origin) is the antipode of the centre that convention lands on, and one-sided
                // credit misses it: four equatorial directions plus one pole return half their
                // radius (`06-degenerate-vertices-need-the-antipode.md`). An antipode is a genuine
                // sphere point credited only to its nearest samples, so it can raise the radius to
                // the truth and cannot overshoot it.
                for side in [1.0_f64, -1.0] {
                    let centre: Vec<f64> = center.iter().map(|value| side * value).collect();
                    let mut best = f64::NEG_INFINITY;
                    for index in 0..count {
                        best = best.max(dot(&directions[index], &centre));
                    }
                    for index in 0..count {
                        if best - dot(&directions[index], &centre) <= TIE_TOLERANCE {
                            radius[index] = radius[index].max(angle(&directions[index], &centre));
                        }
                    }
                }
            }
        }
    }

    radius
}

/// Per direction, the one-term allowance with a table-derived `h`, `ρ` and `R`.
///
/// In 2D this is an upper bound on the excess. In 3D it is an estimate, exceeded at a cube's
/// cell vertex; see the module note and `06-three-d-bound-fails-at-a-cell-vertex.md`.
///
/// `ρ` is the covering radius of the direction's own cell. A direction whose cell is more than
/// `60°` across cannot be certified by this construction and gets [`f64::INFINITY`], which no
/// run can pass: a smaller figure would be a number without a bound behind it.
pub fn allowance(directions: &[Vec<f64>], supports: &[f64]) -> Vec<f64> {
    assert_eq!(
        directions.len(),
        supports.len(),
        "allowance: one support per direction"
    );
    let radius = circumradius_bound(directions, supports);
    let covering = covering_radius(directions);
    (0..directions.len())
        .map(|index| {
            let rho = covering[index];
            if rho >= 30.0_f64.to_radians() {
                return f64::INFINITY;
            }
            // The one-term form is what the band uses. The triple is a bound only where it
            // straddles and is loose everywhere except at a cell vertex, so their maximum widened
            // the 3D band by more than 2x; the triple is reported by `triple_coverage` instead of
            // charged (`06-triple-is-a-bound-but-not-a-tight-one.md`).
            supports[index] * rho.tan() + radius * (1.0 / rho.cos() - 1.0)
        })
        .collect()
}

/// The largest allowance over the table, which is what one scalar test can subtract.
///
/// The scalar is conservative: a per-direction test could pass where this fails. It is
/// reported as one number because a certificate that subtracts a different figure per
/// direction is a different document, and the report has to stay comparable.
pub fn max_allowance(directions: &[Vec<f64>], supports: &[f64]) -> f64 {
    allowance(directions, supports)
        .into_iter()
        .fold(0.0_f64, f64::max)
}
