//! Dense `f64` matrix arithmetic.
//!
//! A flat row-major buffer: every product, sum and scaling below is a loop over the same
//! buffer, and the operation order is fixed so results do not depend on the allocator.

/// A dense row-major matrix of `f64`.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    /// A zero-filled `rows × cols` matrix.
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// The `n × n` identity matrix.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.set(i, i, 1.0);
        }
        m
    }

    /// Builds a matrix from rows of equal length.
    ///
    /// # Panics
    /// If `rows` is empty or the rows have different lengths. Both are treated as
    /// programming errors; the input file is validated by [`crate::stable_bridge`] before
    /// anything reaches this point.
    pub fn from_rows(rows: &[Vec<f64>]) -> Self {
        assert!(!rows.is_empty(), "matrix needs at least one row");
        let cols = rows[0].len();
        assert!(
            rows.iter().all(|r| r.len() == cols),
            "matrix rows have different lengths"
        );
        let mut data = Vec::with_capacity(rows.len() * cols);
        for row in rows {
            data.extend_from_slice(row);
        }
        Self {
            rows: rows.len(),
            cols,
            data,
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }

    /// Row `row` as a slice.
    pub fn row(&self, row: usize) -> &[f64] {
        &self.data[row * self.cols..(row + 1) * self.cols]
    }

    /// Dense matrix product, accumulated in ascending index order.
    ///
    /// # Panics
    /// If the inner dimensions disagree.
    pub fn mul(&self, other: &Self) -> Self {
        assert_eq!(
            self.cols, other.rows,
            "matrix product: inner dimensions disagree"
        );
        let mut out = Self::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let a = self.get(i, k);
                for j in 0..other.cols {
                    let v = out.get(i, j) + a * other.get(k, j);
                    out.set(i, j, v);
                }
            }
        }
        out
    }

    /// Matrix-vector product, applied to every direction of the grid.
    ///
    /// # Panics
    /// If `v` has the wrong length.
    pub fn mul_vec(&self, v: &[f64]) -> Vec<f64> {
        assert_eq!(self.cols, v.len(), "matrix-vector: dimension mismatch");
        (0..self.rows)
            .map(|i| {
                let mut sum = 0.0;
                for j in 0..self.cols {
                    sum += self.get(i, j) * v[j];
                }
                sum
            })
            .collect()
    }

    /// Elementwise scaling.
    pub fn scale(&self, factor: f64) -> Self {
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|x| x * factor).collect(),
        }
    }

    /// Elementwise sum.
    ///
    /// # Panics
    /// If the shapes disagree.
    pub fn add(&self, other: &Self) -> Self {
        assert_eq!(
            (self.rows, self.cols),
            (other.rows, other.cols),
            "matrix sum: shape mismatch"
        );
        Self {
            rows: self.rows,
            cols: self.cols,
            data: self
                .data
                .iter()
                .zip(&other.data)
                .map(|(a, b)| a + b)
                .collect(),
        }
    }

    /// Negates every entry in place. Applying it to the whole `G·B` matrix is where the
    /// game's alternating sign lives.
    pub fn negate(&mut self) {
        for x in &mut self.data {
            *x = -*x;
        }
    }

    /// Largest absolute entry, used for the convergence test of the matrix series.
    pub fn max_abs(&self) -> f64 {
        self.data.iter().fold(0.0_f64, |m, x| m.max(x.abs()))
    }
}
