# 06-bracket-is-one-sided — the inner side is a ball, and it is 30% of the diameter

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | the five tracked examples |
| resolution | 110 directions; allowance from `06-two-neighbour-3d.md`; widest block reported |

**Claim.** A support vector supports an outer bound — `c_i + e_i` with the calculated allowance — but
only a crude inner one, the ball of radius `min_i c_i`, so the bracket's widest width is `30.3–42.4%`
of the diameter and no tracked case has a bracket narrow enough to certify anything.

| case | widest block | inner | outer | width | of diameter |
|---|---|---|---|---|---|
| `example-1` | `20` | `2.151` | `3.087` | `1.871` | `30.3%` |
| `example-2a` | `8` | `0.923` | `1.331` | `0.816` | `30.7%` |
| `example-2b` | `1` | `0.500` | `0.718` | `0.436` | `30.4%` |
| `example-2c` | `1` | `0.449` | `0.646` | `0.394` | `30.5%` |
| `example-3d` | `4` | `1.500` | `2.603` | `2.206` | `42.4%` |

The width is the ball-to-polytope gap, not the allowance: in 2D the allowance is `0.018–0.087`, while
the gap to the inscribed ball is `30%` of the diameter. So the outer side is the useful half, and the
inner side needs a construction the support vector does not carry — an ellipsoid or zonotope from
track 3, or a vertex set proven to lie inside. Without it the five cases have a one-sided bracket,
and `certified` remains a statement about one direction set.

**Evidence.** `./target/release/stable_bridge ../examples/example-2a.txt --bracket` prints one line
per block; the table quotes the block with the widest width.

**Superseded in scope.** The ball is the wrong inner side: shifting each sampled half-space by
its allowance gives a band `10.8×` narrower that is measured inside the true body
(`06-two-sided-bracket.md`). The widths below stand as the ball's numbers.

**Limits.** The ball is inner for the *printed* set; whether it is inner for the true bridge depends
on which way the stored supports drifted, which is track 1's measured deficit and not a bound. The
ball is centred at the origin because the bridge is; a translated or ellipsoidal inner set would be
larger, and that is exactly the untried construction.

**Reuse.** Cite before quoting a bracket width, and as the reason track 3's constructions are a
prerequisite for two-sided claims.

**Links.** `06-two-neighbour-3d.md`, `01-dual-update-is-the-deficit.md`,
`proposals/06-method-evolution.md` (E4).
