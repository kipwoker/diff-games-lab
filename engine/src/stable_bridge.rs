//! Matrix series and the alternating-sum driver.

use crate::directions;
use crate::input_parser::InputData;
use crate::matrix::Matrix;
use crate::minkowski_sum;
use crate::polytope::{Polytope, SupportError};
use crate::pontryagin_difference;
use crate::{DIRECTION_COUNT, SERIES_TOLERANCE, SUBSTEPS_PER_STEP};

/// Hard stop for the matrix series, so a pathological input cannot loop forever.
///
/// With the `1e-5` tolerance the series needs at most a handful of terms for the step sizes
/// this program uses; the limit is a safety net, not part of the algorithm.
const MAX_SERIES_TERMS: usize = 64;

/// `exp(A · h)`, by a truncated power series.
///
/// `A^k · h^k / k!` per term, accumulating `h^k` by repeated multiplication, `k!` as an
/// integer-valued `f64`, and `A^k` by matrix products. The series stops once the largest
/// entry of the increment is at or below [`SERIES_TOLERANCE`].
pub fn expm(a: &Matrix, h: f64) -> Matrix {
    let n = a.rows();
    let mut sum = Matrix::identity(n);
    let mut power = Matrix::identity(n);
    let mut h_power = 1.0;
    let mut factorial = 1.0;

    for k in 1..=MAX_SERIES_TERMS {
        power = power.mul(a);
        h_power *= h;
        factorial *= k as f64;
        let increment = power.scale(h_power / factorial);
        let largest = increment.max_abs();
        sum = sum.add(&increment);
        if largest <= SERIES_TOLERANCE {
            break;
        }
    }

    sum
}

/// `∫₀^Δ exp(A · s) ds`, by the same series.
///
/// The `k`-th term is `A^k · Δ^(k+1) / (k+1)!`, and the same tolerance applies.
pub fn integrate_exp(a: &Matrix, delta: f64) -> Matrix {
    let n = a.rows();
    let mut sum = Matrix::zeros(n, n);
    let mut power = Matrix::identity(n);
    let mut delta_power = delta;
    let mut factorial = 1.0;

    for k in 0..MAX_SERIES_TERMS {
        let increment = power.scale(delta_power / factorial);
        let largest = increment.max_abs();
        sum = sum.add(&increment);
        if largest <= SERIES_TOLERANCE {
            break;
        }
        power = power.mul(a);
        delta_power *= delta;
        factorial *= (k + 2) as f64;
    }

    sum
}

/// A problem layout this program cannot handle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SetupError {
    /// The direction table is defined only for state dimensions 2 and 3.
    UnsupportedDimension(usize),
    /// A direction count the dimension cannot use: 2D takes any count from three up, 3D only
    /// the shipped one.
    UnsupportedDirectionCount { dimension: usize, count: usize },
    /// A matrix or polytope has the wrong shape for its role.
    DimensionMismatch {
        what: &'static str,
        expected: usize,
        found: usize,
    },
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedDimension(n) => write!(
                f,
                "unsupported state dimension {n}: directions are defined for 2 and 3 only"
            ),
            Self::UnsupportedDirectionCount { dimension, count } => {
                write!(f, "no {count}-direction table for dimension {dimension}")
            }
            Self::DimensionMismatch {
                what,
                expected,
                found,
            } => write!(f, "{what}: expected {expected}, found {found}"),
        }
    }
}

impl std::error::Error for SetupError {}

/// The differential game and everything needed to advance its bridge.
#[derive(Clone, Debug)]
pub struct StableBridge {
    pub a: Matrix,
    pub b: Matrix,
    pub c: Matrix,
    pub pi: Matrix,
    /// Terminal set `M`.
    pub m_set: Polytope,
    /// Control constraint `P`.
    pub p: Polytope,
    /// Disturbance constraint `Q`.
    pub q: Polytope,
    pub directions: Vec<Vec<f64>>,
}

impl StableBridge {
    /// Validates the input and builds the shipped direction table.
    pub fn new(input: &InputData) -> Result<Self, SetupError> {
        Self::new_with(input, DIRECTION_COUNT)
    }

    /// Validates the input and builds a table of `count` directions.
    ///
    /// `110` is the shipped size. 2D accepts any count from three up, so a density sweep
    /// reuses every other layer unchanged; 3D is fixed at the shipped `11 x 10` shape.
    pub fn new_with(input: &InputData, count: usize) -> Result<Self, SetupError> {
        let n = input.n;
        if n != 2 && n != 3 {
            return Err(SetupError::UnsupportedDimension(n));
        }
        let built =
            directions::build_with(n, count).ok_or(SetupError::UnsupportedDirectionCount {
                dimension: n,
                count,
            })?;
        Self::new_with_table(input, built)
    }

    /// Validates the input and builds a 3D table of `azimuths x elevations` directions.
    ///
    /// `build_3d(11, 10)` is the shipped table, so this is the constructor a run uses when the
    /// 3D shape is a parameter rather than a constant.
    pub fn new_3d(
        input: &InputData,
        azimuths: usize,
        elevations: usize,
    ) -> Result<Self, SetupError> {
        if input.n != 3 {
            return Err(SetupError::UnsupportedDirectionCount {
                dimension: input.n,
                count: azimuths * elevations,
            });
        }
        let built = directions::build_3d(azimuths, elevations).ok_or(
            SetupError::UnsupportedDirectionCount {
                dimension: 3,
                count: azimuths * elevations,
            },
        )?;
        Self::new_with_table(input, built)
    }

    /// The validation and construction shared by every table.
    fn new_with_table(input: &InputData, built: Vec<Vec<f64>>) -> Result<Self, SetupError> {
        fn check(what: &'static str, expected: usize, found: usize) -> Result<(), SetupError> {
            if expected == found {
                Ok(())
            } else {
                Err(SetupError::DimensionMismatch {
                    what,
                    expected,
                    found,
                })
            }
        }

        let n = input.n;

        check("A row count", n, input.a.rows())?;
        check("A column count", n, input.a.cols())?;
        check("B row count", n, input.b.rows())?;
        check("C row count", n, input.c.rows())?;
        check("pi column count", n, input.pi.cols())?;
        check("M dimension", n, input.m_rows[0].len() - 1)?;
        check("P dimension", input.b.cols(), input.p[0].len() - 1)?;
        check("Q dimension", input.c.cols(), input.q[0].len() - 1)?;

        Ok(Self {
            a: input.a.clone(),
            b: input.b.clone(),
            c: input.c.clone(),
            pi: input.pi.clone(),
            m_set: Polytope::new(input.m_rows.clone()),
            p: Polytope::new(input.p.clone()),
            q: Polytope::new(input.q.clone()),
            directions: built,
        })
    }

    /// State dimension.
    pub fn dimension(&self) -> usize {
        self.a.rows()
    }

    /// Support vector of the terminal set, the starting point of the recursion.
    pub fn terminal_support(&self) -> Result<Vec<f64>, SupportError> {
        self.m_set.support(&self.directions, None)
    }

    /// One piece of an interval: the sum, the difference of the images, and the dual update.
    ///
    /// `G = π · Ie · E`, with `G·B` negated as a whole (the single structural sign flip),
    /// then `c ← dual_update(c_Q, c + c_P)`.
    /// One piece of an interval, also returning how far stage 5 fell short of the dual sum.
    fn substep_traced(
        &self,
        c: &[f64],
        e: &Matrix,
        ie: &Matrix,
    ) -> Result<(Vec<f64>, Vec<f64>), SupportError> {
        let g = self.pi.mul(ie).mul(e);

        let mut gb = g.mul(&self.b);
        gb.negate();
        let gc = g.mul(&self.c);

        let cp = self.p.support(&self.directions, Some(&gb))?;
        let cq = self.q.support(&self.directions, Some(&gc))?;

        let sum = minkowski_sum::support_sum(&cp, c);
        Ok(pontryagin_difference::dual_update_with_loss(
            &cq,
            &self.directions,
            &sum,
            self.dimension(),
        ))
    }

    /// One block: the alternating sum over the interval `[(k−1)·Step, k·Step]`.
    ///
    /// The interval is split into 110 equal pieces, and piece `i` carries the flow factor
    /// `e^{(k·Step − (i+1)h)A}` of its distance to the terminal instant, with
    /// `Φ_h = ∫₀^h e^{uA}du`. The pieces telescope to the interval map
    /// `A_k = e^{k·Step·A} ∫₀^Step e^{−sA}ds` as `h → 0`; for `A = 0` every factor is the
    /// identity, so only the mesh matters.
    pub fn advance(&self, c_prev: &[f64], step: f64, block: usize) -> Result<Vec<f64>, SupportError> {
        Ok(self.advance_traced(c_prev, step, block)?.0)
    }

    /// [`advance`], also returning the block's accumulated shortfall per direction.
    ///
    /// Each substep's loss is added, because the printed support is what the next substep
    /// consumes: a support short by `δ` feeds a sum short by `δ`, so the shortfalls of one
    /// block add up rather than cancel. The result is the certificate's `e_in` at the block.
    pub fn advance_traced(
        &self,
        c_prev: &[f64],
        step: f64,
        block: usize,
    ) -> Result<(Vec<f64>, Vec<f64>), SupportError> {
        let h = step / SUBSTEPS_PER_STEP as f64;
        let phi = integrate_exp(&self.a, h);
        let mut c = c_prev.to_vec();
        let mut inward = vec![0.0; c.len()];

        for index in 0..SUBSTEPS_PER_STEP {
            let remaining = block as f64 * step - (index + 1) as f64 * h;
            let e = expm(&self.a, remaining);
            let (next, loss) = self.substep_traced(&c, &e, &phi)?;
            for (total, value) in inward.iter_mut().zip(&loss) {
                *total += value;
            }
            c = next;
        }

        Ok((c, inward))
    }

    /// The support vectors of the printed blocks.
    ///
    /// Block `k` (one-based) is the `k`-th interval counted back from the terminal instant.
    pub fn blocks(&self, count: usize, step: f64) -> Result<Vec<Vec<f64>>, SupportError> {
        self.blocks_with(count, step, |_, _, _| {})
    }

    /// The same recursion, reporting every finished block to `progress`.
    ///
    /// `progress(block, total, support)` runs as soon as block `block` (one-based) has been
    /// advanced, which is what lets a server show that a long run is still alive instead of
    /// silent. `blocks` is this function with an empty reporter, so both paths compute the
    /// same numbers.
    pub fn blocks_with<F>(
        &self,
        count: usize,
        step: f64,
        mut progress: F,
    ) -> Result<Vec<Vec<f64>>, SupportError>
    where
        F: FnMut(usize, usize, &[f64]),
    {
        let mut c = self.terminal_support()?;
        let mut blocks = Vec::with_capacity(count);
        for k in 1..=count {
            c = self.advance(&c, step, k)?;
            progress(k, count, &c);
            blocks.push(c.clone());
        }
        Ok(blocks)
    }

    /// [`blocks`], with each block's accumulated shortfall per direction.
    ///
    /// The shortfall is the recursion's own measurement, not a typed figure: it is what stage 5
    /// of the dual update gave up on the way to this block. `advance` and `advance_traced` share
    /// [`substep_traced`](Self::substep_traced), so the supports are the same numbers.
    pub fn blocks_traced(
        &self,
        count: usize,
        step: f64,
    ) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), SupportError> {
        let mut c = self.terminal_support()?;
        let mut supports = Vec::with_capacity(count);
        let mut inward = Vec::with_capacity(count);
        for k in 1..=count {
            let (next, loss) = self.advance_traced(&c, step, k)?;
            c = next;
            supports.push(c.clone());
            inward.push(loss);
        }
        Ok((supports, inward))
    }
}
