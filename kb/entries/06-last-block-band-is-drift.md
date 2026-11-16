# 06-last-block-band-is-drift — `2c`'s last block is `93%` shortfall, so no table size moves it

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-1`, `2a`, `2b`, `2c` at `m = 110` |
| resolution | `--trace`, split at the direction `--bracket` used for each block |

**Claim.** The band `width = 2·allowance + shortfall` is two terms, and at `2c`'s last block the
second dominates: `0.038166` of `0.041216` (`92.6%`), against `0.011416` of `0.047131`
(`24.2%`) at its first block. The table's share of that band is `0.003050`, `7.4%`, so density
cannot move the figure — `2c`'s last-block share fell only `20.4142 → 17.3445%` from `110` to
`254` (`06-density-sweep-measured.md`). The other three cases are the opposite: at their last
block the shortfall is `0` or `0.005099`, at most `6.4%` of the band.

| case | block | inward | support | inward % | `2·allowance` | width | `2·allowance/width` |
|---|---|---|---|---|---|---|---|
| `example-1` | 1 | 0.028829 | 1.0707 | 2.692 | 0.062074 | 0.090903 | 68.3% |
| `example-1` | 20 | 0.000000 | 3.0000 | 0.000 | 0.173854 | 0.173854 | 100.0% |
| `2a` | 1 | 0.013449 | 0.7476 | 1.799 | 0.043343 | 0.056792 | 76.3% |
| `2a` | 8 | 0.005099 | 1.2852 | 0.397 | 0.074484 | 0.079583 | 93.6% |
| `2b` | 1 | 0.011313 | 0.6796 | 1.665 | 0.039401 | 0.050714 | 77.7% |
| `2b` | 8 | 0.000000 | 0.6978 | 0.000 | 0.040437 | 0.040437 | 100.0% |
| `2c` | 1 | 0.011416 | 0.6161 | 1.853 | 0.035715 | 0.047131 | 75.8% |
| `2c` | 8 | 0.038166 | 0.0521 | 73.300 | 0.003050 | 0.041216 | 7.4% |

**Prediction.** Written before the run: the last block's absolute shortfall is within `2×` of
block 1's, so its `17–20%` share is the support collapsing under a shortfall that did not. That
held on `example-1`, `2a` and `2b` (ratio `0.00`, `0.38`, `0.00`), where the sets at the last
block are the largest of the block, and **failed on `2c` at `3.34×`**: there the shortfall grows
as the support collapses, so the share is both effects at once.

**The denominator.** `2c`'s last-block band is `0.041216`, *narrower* than its first block's
`0.047131`, while the block's printed circumradius is `0.1009` against `0.634`. Its share at the
band's own direction is `38.45%`, so the `20.41%` of the sweep and this `38.45%` are the same
band over two different radii. The band got smaller; its composition flipped.

**Limits.** One table size and four 2D cases. The dump is the recursion's own minorisation
shortfall, not the exact support deficit of `04-two-c-slack-is-drift.md`, and the split is taken
at each block's widest direction, so a shortfall living elsewhere is not credited to the band.

**Evidence.** `cargo build --release --manifest-path engine/Cargo.toml`, then
`python tools/inward_profile.py`.

**Reuse.** Quote `2c`'s last-block band as drift: `92.6%` of it is shortfall, so it is not a
table cost and `--directions` is not its lever.

**Links.** `06-density-sweep-measured.md` (the shares), `04-two-c-slack-is-drift.md` (the exact
deficit), `06-bracket-folds-inward.md` (the width convention), `06-inward-share-is-not-a-tan-rho-law.md`
(the `205` jump, same mechanism).
