# 06-three-d-violation-is-alignment-not-density — three shapes are clean, and the exact one samples the facet normals

| field | value |
|---|---|
| type | measurement, prediction falsified |
| track | 6 |
| date | 2026-11-13 |
| case | cube `1.5`, seven `az x el` tables from `110` to `240` directions, `2525` probes |
| resolution | `covering_bound.py` part N: one-term allowance at the probe's cell, reconstruction excess |

**Claim.** The one-term violation is not a density effect. At `2525` probes the cube exceeds the
allowance at `14 x 13` (`1.016`) and `16 x 15` (`1.046`), and not at `11 x 10` (`0.694`), `15 x 14`
(`0.765`), `18 x 12` (`0.246`) or `20 x 12` (`0.223`). `13 x 12` is exact to `6e-14`: its grid
samples the cube's facet normals, so the reconstruction *is* the cube. The failure tracks the grid's
alignment, not its count.

| table | n | `ρ` | violations | worst `excess/local` | max `excess/R` |
|---|---|---|---|---|---|
| `11 x 10` | 110 | 19.8733° | 0 | 0.694 | 14.0880% |
| `13 x 12` | 156 | 16.6227° | 0 | 0.000 | 0.0000% |
| `14 x 13` | 182 | 15.4502° | **1** | 1.016 | 15.8733% |
| `15 x 14` | 210 | 14.2808° | 0 | 0.765 | 9.5296% |
| `16 x 15` | 240 | 13.3967° | **1** | 1.046 | 13.3723% |
| `18 x 12` | 216 | 15.8291° | 0 | 0.246 | 3.9097% |
| `20 x 12` | 240 | 15.6672° | 0 | 0.223 | 3.5265% |

**Prediction.** Written before the run: the violation survives every shape up to `240`, the worst
ratio falls from `1.046` towards `1.01`, and the excess share falls with `tan ρ`. Falsified on all
three: three shapes are clean, the worst ratio moves both ways (`0.000`, `0.694`, `1.016`, `1.046`),
and the excess share is `14.09%` at `11 x 10` but `3.53%` at `20 x 12` while `16 x 15` sits at
`13.37%` — the count is not the axis.

**Why `13 x 12` is exact.** Elevations `j·360/12` include `0°` and `90°`; azimuths `-90°…90°` at
`15°` include `0` and `±90`. So `±e_1`, `±e_2`, `±e_3` are all sampled, and the six facet caps alone
pin the cube. The max excess of `6.3e-14` over `2525` probes is the reconstruction reproducing the
body, which is also a check on the machinery. The `12`-elevation shapes are clean for the same
reason; the `13`/`14`/`15` elevation grids miss the poles, and `16 x 15` and `14 x 13` miss enough
of the alignment to leave one probe at `1.05` and `1.02`.

**Consequence.** The practical 3D fix is a table that contains the body's facet normals, or a check
against them — not more directions. Density alone leaves `1.046` where it matters, which is why the
`16 x 15` band halves while the label stays `estimate`.

**Limits.** One body (the cube), `2500` random plus `25` fixed probes, shapes up to `240`
directions. The exact row is a degenerate win: it needs the body's own normals, which an
approximated bridge does not offer.

**Evidence.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as c;
c.part_n()"` after the release build.

**Reuse.** Quote the violation as shape-dependent — "`0` at five shapes, `1.02–1.05` at `14 x 13` and
`16 x 15`" — and never as "3D is too coarse at `240`".

**Links.** `06-three-d-radius-is-exact.md` (the falsifying probe at `16 x 15`),
`06-three-d-table-shape-is-a-parameter.md` (the band that halves),
`06-three-d-max-bound-leaves-the-flank-unproved.md` (the triple's coverage),
`06-what-the-falsifications-say.md` (the scaling-law pattern this belongs to).
