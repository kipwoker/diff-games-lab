# 06-bracket-and-cost — the 2D band is 2.816% of the diameter on every case, and `3d` is 29.2%

**Superseded in scope** by `06-bracket-folds-inward.md`: the widths below are the outward half
only, and are neither a band nor the number to quote.

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | the five tracked examples |
| resolution | `--bracket` at 110 directions, shipped tables, the allowance as computed |

**Claim.** The two-sided band's width as a fraction of the diameter is `2.8160%` on all four 2D
cases and `29.2171%` on `3d`, because both the allowance and the diameter are proportional to the
set's own support, so the fraction collapses to a property of the table. Producing a document costs
`109` to `556 ms`, and the band itself costs under `1%` of that: certainty is not a trade-off at
this direction count.

| case | covering radius | widest width | of diameter | block | inward max | wall time |
|---|---|---|---|---|---|---|
| `example-1` | `1.6364°` | `0.173854` | `2.8160%` | 20 | `0.028829` | `233 ms` |
| `example-2a` | `1.6364°` | `0.074966` | `2.8160%` | 8 | `0.014820` | `109 ms` |
| `example-2b` | `1.6364°` | `0.040437` | `2.8160%` | 1 | `0.012771` | `115 ms` |
| `example-2c` | `1.6364°` | `0.036372` | `2.8160%` | 1 | `0.038166` | `110 ms` |
| `example-3d` | `19.3787°` | `1.891973` | `29.2171%` | 4 | `0.142468` | `556 ms` |

The repeated `2.8160%` is the finding, not a coincidence: the per-direction allowance
`h tan ρ + R(1/cos ρ − 1)` scales with the support `h`, the printed diameter is `2(h + e)`, and
`R/h` is the same for these four shapes, so the ratio is set by the table. A new 2D case therefore
inherits the number rather than measuring it — which is what makes it usable as a tolerance:
`2.816%` against the `2%` a rung's bracket is allowed, and `29.2%` against it in 3D.

Cost, from the same runs on the development host: `3d` takes `552 ms` to write its document and
`556 ms` with the band, so the two-sided report costs `0.7%`. `example-1` is the slowest 2D case at
`233 ms` because it has 20 blocks of 110 substeps. At these numbers E5's "certainty that costs more
than the answer" has no content: the answer is the expensive part.

**Limits.** The direction count is compiled in (`DIRECTION_COUNT = 110`), so this is not E5's grid
over counts; the counts in `06-adaptive-refinement.md` come from the analytic tool and their cost is
not the engine's. Memory is not reported: this host offered no portable peak-RSS reading in the
session, and a number invented around a missing instrument is worse than an empty column. The band's
inner side is still `c_i − e_out`; since `06-two-sidedness.md` the certificate knows `e_in` per
direction, but `--bracket` does not subtract it, so the inner side carries the outward figure only.

**Reuse.** Cite for any bracket width or cost claim about the five cases, and as the reason a 2D
band can be quoted as a table property rather than measured per case.

**Corrected.** The `example-3d` row is stale: with the exact 3D radius it reads `19.8733°`,
`1.999277`, `30.5916%`. `06-three-d-radius-is-exact.md`.

**Links.** `06-two-sided-bracket.md` (the construction), `06-three-d-covering-radius.md` (why 3D is
`29%`), `06-two-sidedness.md` (the inward term not yet folded in),
`proposals/06-method-evolution.md` (E5).
