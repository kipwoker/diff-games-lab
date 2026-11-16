# 06-cell-bound-was-not-a-bound — `support_at` returned `−1.414214` where the support is `1.414214`

| field | value |
|---|---|
| type | defect, measured |
| track | 6 |
| date | 2026-11-13 |
| case | a unit square on four directions, probed at `45°`; the shipped 3D table at a generic probe |
| resolution | `cargo test --release --test error`, before and after the repair |

**Claim.** `cell_bound` minimised `pair_extent` over every pair among a cell's nearest samples.
For a pair whose two planes do not straddle the probe, the two-plane set is unbounded along it
and `pair_extent` returns that pair's vertex projection, not an extent — so the minimum was not
a bound at all. On a unit square at `45°` it returned **`−1.414214`**, the negative of the true
support `1.414214`, for a set that contains the origin; on a generic 3D probe it returned a
finite number where no pair bounds anything. The repair counts only pairs with the probe in
their positive cone and returns `INFINITY` when none straddles.

| probe | before | after | truth |
|---|---|---|---|
| unit square, `45°` | `−1.414214` | `1.414214` | `√2` |
| 3D table, generic unit vector | finite | `INFINITY` | no pair bound exists |
| 2D table at a sample | exact | exact | the stored support |

**Why the tests missed it.** The only probe in the suite was a sampled direction, where every
pair containing that sample straddles it and the minimum is exact. Between samples the defect
is total, and negative answers are its signature: two non-straddling pairs project to opposite
vertices.

**Limits.** `cell_bound` and `support_at` are called only from `engine/tests/error.rs`, so no
certificate, bracket or committed output used them; the five outputs are byte-identical after
the repair. The cone test is a residual tolerance of `1e-9` on a unit sphere, so a probe within
about `10⁻⁹` of a great circle through two samples is counted as straddling.

**Evidence.** `cargo test --release --manifest-path engine/Cargo.toml --test error`; the failing
run is in `engine/target/error_before.log`.

**Reuse.** Do not cite `support_at` as an instrument for the allowance: it was never a bound
between samples, and in 3D it has no pair bound to give. The certificate's figure is
`allowance`, which is a separate function.

**Links.** `06-two-plane-bound-loses-to-the-allowance.md` (which asked whether the minimum over
such pairs can understate a bound), `06-three-d-bound-fails-at-a-cell-vertex.md` (the triple
that covers the 3D probes), `06-two-neighbour-bound.md` (the straddling condition in the formula).
