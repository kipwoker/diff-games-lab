# 06-inner-side-violation-is-a-chamfer-shortfall — the `4.8e-4` is a poke, not the defect

| field | value |
|---|---|
| type | measurement, prediction half-falsified |
| track | 6 |
| date | 2026-11-14 |
| case | `example-2c` block 1, tables `110` / `158` / `205` / `254` |
| resolution | `inner_between_samples.py` `explain_205()`: worst inner vertex against each exact facet |

**Claim.** The `4.8e-4` is real, and it is not the size of the defect. At `205` the inner polytope has
one vertex outside block 1's exact polygon, by `0.000478`, at a normal mid-way between samples
(`offset 0.5`). The exact chamfer facet there has support `0.448750`, the nearest sampled cap is
`0.456891`, and the table's allowance is `0.007077` — so the shifted cap lands `0.001064` *above* that
facet rather than below it, and the vertex spends `45%` of the shortfall.

| m | vertices | out | excess | offset | facet `h` | cap | allowance | shifted − `h` |
|---|---|---|---|---|---|---|---|---|
| `110` | 20 | 0 | −0.000623 | 0.5 | 0.450000 | 0.462702 | 0.013474 | −0.000772 |
| `158` | 26 | 0 | −0.000303 | 0.5 | 0.450000 | 0.458883 | 0.009250 | −0.000367 |
| `205` | 30 | **1** | **+0.000478** | 0.5 | 0.448750 | 0.456891 | 0.007077 | **+0.001064** |
| `254` | 42 | 0 | −0.000117 | 0.5 | 0.450000 | 0.455547 | 0.005683 | −0.000136 |

`shifted − h` is `cap − allowance − h`: negative means the shifted half-space cuts inside the true
facet, which is what makes the inner set safe; positive means it does not. The four counts straddle
zero and `205` is the one above it.

**Prediction.** Written before the run: the outside vertex sits at a between-sample normal, with
`excess` equal to `bulge`, the amount the body outgrows the shifted caps. Half-falsified — the normal
is mid-cell at all four counts and the chamfer reading is right, but `excess` (`4.78e-4`) is `2.2×` the
`bulge` (`2.14e-4`) and `45%` of the real gap: the vertex is not the extreme point along that normal,
so the poke is a fraction of the shortfall rather than its measure.

**What it means.** The inner claim is not structural at these tables. It holds where
`allowance > cap − h_facet` at every cell, and that inequality depends on the case's own chamfer as
much as on the table. `2c`'s exact body has facets from `0.448750` to `0.450000` because the
recursion's `110` directions sit *in* its boundary, so an allowance sized by its spacing can fall under
a chamfer it does not sample: `254` clears it by `1.4e-4`, `205` misses by `1.06e-3`. No probe, clip or
inside-test bug is involved — the shift is simply too small in that one direction.

**Consequence for the bracket.** `2.8160%` stands as the measured band and the inner side is inside at
`110`, `158` and `254`. At `205` it is outside by `4.8e-4`, a small fraction of a block radius, so the
band needs a caveat rather than a repair. The test to add before calling a table two-sided is
`min over cells (allowance − (cap − h_facet)) >= 0`, which the exact body can answer per case.

**Limits.** One case, one block, four counts. The facet was found by scanning the exact polygon's
facets, so a facet the scan orders differently is not represented. `offset 0.5` at all four counts says
the worst facet sits at a fixed geometric angle that each table happens to straddle evenly.

**Evidence.** `python -u -c "import sys; sys.path.insert(0,'tools'); import inner_between_samples as t;
t.explain_205()"`.

**Reuse.** Quote the `4.8e-4` as the poke and `1.06e-3` as the shortfall; never say "the inner set
leaves by `4.8e-4`" without the second number.

**Links.** `06-chamfering-gap-is-not-the-table.md`, `06-inner-set-leaves-the-exact-body.md` (the
retracted reading), `06-two-sided-bracket.md`, `06-inner-side-holds-on-the-other-cases.md`.
