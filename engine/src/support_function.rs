//! Inner products and support values over a vertex list.

/// Euclidean inner product, accumulated in ascending index order.
///
/// # Panics
/// If the lengths disagree (in debug builds).
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    debug_assert_eq!(a.len(), b.len(), "inner product: dimension mismatch");
    let mut sum = 0.0;
    for i in 0..a.len() {
        sum += a[i] * b[i];
    }
    sum
}

/// Support value of a point set in one direction: `max_v <direction, v>`.
///
/// Returns negative infinity for an empty vertex list; callers treat that as a failure
/// before it can reach the output.
pub fn support_of_vertices(vertices: &[Vec<f64>], direction: &[f64]) -> f64 {
    let mut best = f64::NEG_INFINITY;
    for vertex in vertices {
        let value = dot(vertex, direction);
        if value > best {
            best = value;
        }
    }
    best
}

/// Support vector: the support value in each of `directions`.
pub fn support_vector(vertices: &[Vec<f64>], directions: &[Vec<f64>]) -> Vec<f64> {
    directions
        .iter()
        .map(|direction| support_of_vertices(vertices, direction))
        .collect()
}
