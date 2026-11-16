//! Index-combination enumeration.
//!
//! Every `C(m, n)` index set is handed to a visitor in lexicographic order, which is why the
//! printed vertex order is an enumeration rather than a traversal around the boundary.

/// Calls `visit` once with every `k`-subset of `0..n`, in lexicographic order.
///
/// Does nothing when `k == 0` or `k > n`.
pub fn for_each_combination(n: usize, k: usize, mut visit: impl FnMut(&[usize])) {
    if k == 0 || k > n {
        return;
    }
    let mut index: Vec<usize> = (0..k).collect();
    loop {
        visit(&index);

        // Advance to the next combination: find the rightmost index that can still grow.
        let mut i = k;
        loop {
            if i == 0 {
                return;
            }
            i -= 1;
            if index[i] != i + n - k {
                break;
            }
        }
        index[i] += 1;
        for j in i + 1..k {
            index[j] = index[j - 1] + 1;
        }
    }
}

/// Number of `k`-subsets of `n` items.
pub fn combination_count(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    let k = k.min(n - k);
    let mut result: usize = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}
