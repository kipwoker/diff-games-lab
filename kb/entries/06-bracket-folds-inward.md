# 06-bracket-folds-inward — the band widens to `3.72%` once the shortfall is on the inner side

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | the five tracked examples |
| resolution | `--bracket` at 110 directions, inner side `c_i − e_out_i − e_in_i` |

**Claim.** Folding the recursion's shortfall into the band's inner side breaks the uniform `2.8160%`:
the worst 2D width is now `3.7170%` on `2c`, `example-1` is unmoved at `2.8160%` because its widest
direction carries no shortfall, and `3d` reaches `29.9505%`.

| case | inner | outer | width | of diameter | before |
|---|---|---|---|---|---|
| `example-1` | `2.913073` | `3.086927` | `0.173854` | `2.8160%` | `2.8160%` |
| `example-2a` | `1.242829` | `1.322411` | `0.079583` | `3.0090%` | `2.8160%` |
| `example-2b` | `0.648621` | `0.699334` | `0.050714` | `3.6259%` | `2.8160%` |
| `example-2c` | `0.586860` | `0.633991` | `0.047131` | `3.7170%` | `2.8160%` |
| `example-3d` | `1.298320` | `3.237782` | `1.939462` | `29.9505%` | `29.2171%` |

Each width grows by the shortfall at the direction the band is widest in, not by the case's
`inward max`: `2c` gains `0.010759` while its largest shortfall anywhere is `0.038166`, because the
widest direction and the largest shortfall are different directions. That is the mechanism: the
outward allowance is a property of the table, the shortfall is a property of the alignment between
the table and the body (`01-deficit-is-chamfering.md`), so a two-sided band is a per-case quantity
after all. The previous entry's convenient reading — `2.8160%` on four cases, therefore the table's
number — was true of the outward half only.

Predicted before the run: the 2D widths would land in `3.0-3.3%`. Measured: `3.009%`, `3.626%`,
`3.717%` — the top of the range was too low, and the spread across cases is larger than predicted.

The consequence is a tolerance question rather than a measurement one. Every 2D case now exceeds
the `2%` a rung's bracket is allowed, by `1.4×` to `1.9×`, and 3D by `15×`. Either the tolerance is
restated for 3D-as-well-as-2D bands, or the band is narrowed — and the only lever on the inward half
is the table's alignment, which is track 1's subject rather than track 6's.

**Corrected.** `example-3d`'s row is `inner 1.268413`, `outer 3.267690`, `width 1.999277`,
`30.5916%` at block 4 direction 36 once the 3D radius is exact; the four 2D rows are unchanged, so
"3D by `15×`" becomes `15.3×`. `06-three-d-radius-is-exact.md`.

**Limits.** The inner shift is now `e_out + e_in`, additive by choice: both are uncertainties of the
stored support, one outward and one inward, and a band that omitted either would be narrower than it
can defend. `e_in` is a measurement, not a bound (`06-two-sidedness.md`), and the widest direction is
chosen by total width, so it can differ from the direction that was widest under the outward
allowance alone. Nothing here says the shifted set is inside the true body in 3D, where that question
was never measured.

**Reuse.** Cite for any width quoted after this session: the one-sided `2.8160%` belongs to the
outward half. Cite the spread, not a single number, as the 2D band.

**Links.** `06-bracket-and-cost.md` (the outward-only table this replaces), `06-two-sidedness.md`
(the term), `01-deficit-is-chamfering.md` (why the inward half is alignment-dependent),
`proposals/06-method-evolution.md` (E4).
