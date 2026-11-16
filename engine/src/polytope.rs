//! Polytopes given by systems of linear inequalities.
//!
//! Three operations: a sampled boundedness guard, vertex enumeration by `C(m, n)` with
//! Gaussian elimination and de-duplication, and the support vector over the resulting
//! vertices.

use crate::combinations;
use crate::linear_system;
use crate::matrix::Matrix;
use crate::support_function;
use crate::VERTEX_EPSILON;

/// A convex polytope `{x : <a_j, x> <= b_j}`.
///
/// Each row is `(a_0 … a_{n-1}, b)` — coefficients first, right-hand side last, exactly
/// as the input file stores `M`, `P` and `Q`.
#[derive(Clone, Debug)]
pub struct Polytope {
    rows: Vec<Vec<f64>>,
}

/// Why a support vector could not be produced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupportError {
    /// The sampled boundedness guard failed: some direction has no positive support.
    UnboundedOnSample,
    /// Vertex enumeration produced no admissible point at all.
    NoVertices,
}

impl std::fmt::Display for SupportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnboundedOnSample => {
                write!(f, "polytope is unbounded in a sampled direction")
            }
            Self::NoVertices => write!(f, "polytope has no admissible vertex"),
        }
    }
}

impl std::error::Error for SupportError {}

impl Polytope {
    /// Wraps a set of inequality rows.
    ///
    /// # Panics
    /// If `rows` is empty: a set with no inequalities has no dimension.
    pub fn new(rows: Vec<Vec<f64>>) -> Self {
        assert!(!rows.is_empty(), "polytope needs at least one row");
        Self { rows }
    }

    pub fn rows(&self) -> &[Vec<f64>] {
        &self.rows
    }

    /// Dimension of the ambient space.
    pub fn dimension(&self) -> usize {
        self.rows[0].len() - 1
    }

    /// All vertices, in enumeration order.
    ///
    /// Every `C(m, n)` choice of `n` rows is solved; a solution is kept when it satisfies
    /// all rows within the `0.01` slack and is not within `0.01` of a point already stored.
    pub fn vertices(&self) -> Vec<Vec<f64>> {
        let n = self.dimension();
        let m = self.rows.len();
        let mut vertices: Vec<Vec<f64>> = Vec::new();

        combinations::for_each_combination(m, n, |indices| {
            let mut coefficients: Vec<Vec<f64>> = Vec::with_capacity(n);
            let mut rhs: Vec<f64> = Vec::with_capacity(n);
            for &index in indices {
                coefficients.push(self.rows[index][..n].to_vec());
                rhs.push(self.rows[index][n]);
            }
            let Some(point) = linear_system::solve(&coefficients, &rhs, linear_system::PIVOT_EPSILON)
            else {
                return;
            };
            if self.satisfies(&point, VERTEX_EPSILON) && !contains_near(&vertices, &point, VERTEX_EPSILON)
            {
                vertices.push(point);
            }
        });

        vertices
    }

    /// Whether `point` satisfies every inequality within `eps`.
    pub fn satisfies(&self, point: &[f64], eps: f64) -> bool {
        let n = point.len();
        self.rows
            .iter()
            .all(|row| support_function::dot(&row[..n], point) <= row[n] + eps)
    }

    /// The boundedness guard: `min_i max_j <a_j, psi_i> > 0`.
    ///
    /// This samples the boundedness condition instead of checking it exactly, so it can
    /// reject a bounded set.
    pub fn is_bounded_on_sample(&self, directions: &[Vec<f64>]) -> bool {
        let mut worst = f64::INFINITY;
        for direction in directions {
            assert_eq!(
                self.dimension(),
                direction.len(),
                "boundedness guard: direction dimension mismatch"
            );
            let mut best = f64::NEG_INFINITY;
            for row in &self.rows {
                let value = support_function::dot(&row[..direction.len()], direction);
                if value > best {
                    best = value;
                }
            }
            if best < worst {
                worst = best;
            }
        }
        worst > 0.0
    }

    /// Support vector in `directions`, optionally after a linear map of the directions.
    ///
    /// `map` pushes the directions into the space of the polytope before the support values
    /// are taken; `None` leaves them untouched.
    pub fn support(
        &self,
        directions: &[Vec<f64>],
        map: Option<&Matrix>,
    ) -> Result<Vec<f64>, SupportError> {
        let mapped: Vec<Vec<f64>> = match map {
            Some(matrix) => directions
                .iter()
                .map(|direction| matrix.mul_vec(direction))
                .collect(),
            None => directions.to_vec(),
        };
        if !self.is_bounded_on_sample(&mapped) {
            return Err(SupportError::UnboundedOnSample);
        }
        let vertices = self.vertices();
        if vertices.is_empty() {
            return Err(SupportError::NoVertices);
        }
        Ok(support_function::support_vector(&vertices, &mapped))
    }
}

/// Whether `points` already holds a point within `eps` of `point`, coordinatewise.
pub fn contains_near(points: &[Vec<f64>], point: &[f64], eps: f64) -> bool {
    points.iter().any(|other| {
        other.len() == point.len()
            && other
                .iter()
                .zip(point)
                .all(|(a, b)| (a - b).abs() < eps)
    })
}
