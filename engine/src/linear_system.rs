//! Dense linear solve by Gaussian elimination with partial pivoting.
//!
//! A system is rejected when a pivot falls below [`PIVOT_EPSILON`] rather than exactly at
//! zero. The epsilon sits well below the `0.01` slack used elsewhere, so it only rejects
//! genuinely singular systems.

/// Pivot magnitude below which a system is considered singular.
///
/// The systems here are built from small integer or unit coefficients, so the exact value
/// does not influence any result on well-formed input.
pub const PIVOT_EPSILON: f64 = 1e-12;

/// Solves `a · x = b`, returning `None` for a singular system.
///
/// `a` is a square system given as rows; `b` is the right-hand side.
///
/// # Panics
/// If `a` is not square or `b` has the wrong length.
pub fn solve(a: &[Vec<f64>], b: &[f64], eps: f64) -> Option<Vec<f64>> {
    let n = b.len();
    assert_eq!(a.len(), n, "linear solve: row count mismatch");
    assert!(
        a.iter().all(|row| row.len() == n),
        "linear solve: system is not square"
    );

    let mut m: Vec<Vec<f64>> = a.to_vec();
    let mut rhs = b.to_vec();

    for col in 0..n {
        // Partial pivoting: the largest available pivot in this column.
        let mut pivot = col;
        for row in col + 1..n {
            if m[row][col].abs() > m[pivot][col].abs() {
                pivot = row;
            }
        }
        if m[pivot][col].abs() < eps {
            return None;
        }
        if pivot != col {
            m.swap(pivot, col);
            rhs.swap(pivot, col);
        }

        let diagonal = m[col][col];
        for row in col + 1..n {
            let factor = m[row][col] / diagonal;
            if factor == 0.0 {
                continue;
            }
            for c in col..n {
                m[row][c] -= factor * m[col][c];
            }
            rhs[row] -= factor * rhs[col];
        }
    }

    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = rhs[i];
        for j in i + 1..n {
            sum -= m[i][j] * x[j];
        }
        x[i] = sum / m[i][i];
    }
    Some(x)
}
