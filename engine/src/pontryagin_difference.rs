//! The geometric difference performed on support vectors.
//!
//! The routine is a five-stage pipeline:
//!
//! ```text
//! 1.  diff[j] = sum[j] - cq[j]
//! 2.  v[k]    = (n / 110) * sum_j diff[j] * psi[j][k]
//! 3.  g[j]    = diff[j] - <psi_j, v>
//! 4.  c2[i]   = min { g[j] / <psi_i, psi_j> : <psi_i, psi_j> > 0 },  mask[argmin] = true
//! 5.  out[i]  = diff[i]                                          if mask[i]
//!             = <psi_i, v> + max_j ( <c2[j] * psi_j, psi_i> )    otherwise
//! ```
//!
//! Stages 1 and 2 are the recurrence `c(W^k) = c(W^{k-1}) + c(P) - c(Q)` and the
//! `(n/m) * sum c_i psi_i` reconstruction of the body's centre. Stages 3 to 5 are one round
//! trip through a primal reconstruction: `c2` holds the radii of the half-space intersection
//! `{x : <psi_j, x> <= g[j]}`, and stage 5 evaluates the support of `conv{v + c2[j] psi_j}`.
//! Directions that are active facets of that intersection keep their exact dual value
//! instead, because the hull formula is not exact on them.

use crate::support_function::dot;

/// Performs the update, discarding the loss. See [`dual_update_with_loss`].
pub fn dual_update(cq: &[f64], directions: &[Vec<f64>], sum: &[f64], n: usize) -> Vec<f64> {
    dual_update_with_loss(cq, directions, sum, n).0
}

/// Performs the update, also reporting how much support stage 5 gave up per direction.
///
/// Stage 5 is the only stage that can lose: an active facet keeps `diff[i]` exactly, while the
/// rest get the support of `conv{v + c2_j ψ_j}`, a minorant of the half-space intersection. The
/// loss `max(0, diff[i] − out[i])` is therefore the amount by which a stored support may sit
/// below the exact recurrence **at a sampled direction** — track 1's inward deficit computed
/// rather than typed (`01-deficit-is-chamfering.md` measures the same shortfall against an
/// analytic body). Accumulated over a block it gives the certificate's `e_in`.
///
/// * `cq` — support vector of the disturbance image under `G·C`.
/// * `directions` — the `DIRECTION_COUNT` sampled directions.
/// * `sum` — `c_prev + cP`, where `cP` is the control image support vector.
/// * `n` — state dimension, used for the `n / 110` reconstruction scale.
///
/// Returns `(support, loss)`.
///
/// # Panics
/// If the three vectors disagree in length, or `n` does not match the direction width.
pub fn dual_update_with_loss(
    cq: &[f64],
    directions: &[Vec<f64>],
    sum: &[f64],
    n: usize,
) -> (Vec<f64>, Vec<f64>) {
    let count = directions.len();
    assert_eq!(cq.len(), count, "dual update: cq length mismatch");
    assert_eq!(sum.len(), count, "dual update: sum length mismatch");
    if count == 0 {
        return (Vec::new(), Vec::new());
    }
    assert_eq!(directions[0].len(), n, "dual update: dimension mismatch");

    // 1. diff = sum - cq
    let diff: Vec<f64> = (0..count).map(|j| sum[j] - cq[j]).collect();

    // 2. v = (n / count) * sum_j diff[j] * psi[j], scale applied after accumulation so the
    //    summation order is fixed.
    let scale = n as f64 / count as f64;
    let mut v = vec![0.0; n];
    for j in 0..count {
        for k in 0..n {
            v[k] += diff[j] * directions[j][k];
        }
    }
    for value in &mut v {
        *value *= scale;
    }

    // 3. g[j] = diff[j] - <psi_j, v>
    let g: Vec<f64> = (0..count)
        .map(|j| diff[j] - dot(&directions[j], &v))
        .collect();

    // 4. Minimum ratio over strictly positive inner products, recording the active facet.
    let mut c2 = vec![0.0; count];
    let mut mask = vec![false; count];
    for i in 0..count {
        let mut best = f64::INFINITY;
        let mut argmin = 0usize;
        let mut found = false;
        for j in 0..count {
            let inner = dot(&directions[i], &directions[j]);
            if inner > 0.0 {
                let ratio = g[j] / inner;
                if !found || ratio < best {
                    best = ratio;
                    argmin = j;
                    found = true;
                }
            }
        }
        if found {
            c2[i] = best;
            mask[argmin] = true;
        }
    }

    // 5. Active directions keep the dual value, the rest get the reconstructed hull support.
    let mut out = vec![0.0; count];
    for i in 0..count {
        if mask[i] {
            out[i] = diff[i];
            continue;
        }
        let mut best = f64::NEG_INFINITY;
        for j in 0..count {
            let mut value = 0.0;
            for k in 0..n {
                value += (c2[j] * directions[j][k]) * directions[i][k];
            }
            if value > best {
                best = value;
            }
        }
        out[i] = dot(&directions[i], &v) + best;
    }

    // 6. The loss of the round trip, clamped against rounding: the minorant never gains.
    let loss: Vec<f64> = (0..count).map(|i| (diff[i] - out[i]).max(0.0)).collect();

    (out, loss)
}
