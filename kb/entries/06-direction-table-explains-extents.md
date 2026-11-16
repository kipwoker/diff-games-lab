# 06-direction-table-explains-extents — the 110 directions alone produce the tracked anomalies

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2b` block 8, `example-3d` block 4 |
| resolution | 110 directions, `Step 0.05` and `0.25` |

**Claim.** Reconstructing a set from its exact supports on the engine's 110 directions reproduces
the tracked 2D bulge and the 3D `z` factor to four decimals with no tolerance applied, so both
anomalies are a pure function of the direction table, computable before any solver run.

| case | axis | tracked | reconstructed, no slack | reconstructed, slack `0.01` |
|---|---|---|---|---|
| `example-3d` | x, y | 1.5000 | 1.5000 | 1.5100 |
| `example-3d` | z | 1.9874 | 1.9874 | 1.9979 |
| `example-2b` | x | 0.5000 | 0.5000 | 0.5100 |
| `example-2b` | y | 0.5143 | 0.514284 | 0.524288 |

The excess is large: `32%` of the half-width in `z` for the cube, `2.9%` for the square. A local
curvature bound of the form `R δ²/8` predicts `0.0006` and `0.0002` respectively, three orders of
magnitude low, so the error is governed by the covering radius of the table and the normal cones
at the vertices, not by the second derivative of the support function.

**Evidence.** `python tools/direction_table_check.py` (from the repository root, needs numpy);
tracked values from `examples/out/example-3d.out.txt` and `example-2b.out.txt`.

**Limits.** `A = 0` cases only, where the exact support of the propagated shell is known. Adding
`0.01` to every half-space is a crude model of the acceptance slack, not the engine's exact rule;
it is enough to show the slack is not what inflates these shells. Says nothing about the
`example-1` deficit, which runs the other way.

**Reuse.** Cite for any claim that the `1.325` factor or the `0.514` bulge is the grid's; rules out
the slack and the dynamics as causes, and rules out a curvature-only error model.

**Links.** `proposals/06-method-evolution.md`, `proposals/01-linear-bridges.md`.
