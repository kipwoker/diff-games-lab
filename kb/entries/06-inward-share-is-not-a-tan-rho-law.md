# 06-inward-share-is-not-a-tan-rho-law — it runs `0.56` to `0.84` times `tan ρ`, and `f` moves with it

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2c`, first block, `m = 110, 158, 205, 254` |
| resolution | the engine's band minus the exact polygon's `A`, both as shares of `R`; `0.4%` agreement at `110` |

**Claim.** The inward share of `2c`'s band is not proportional to `tan ρ`. It reads `1.6024%`,
`1.1503%`, `1.2815%`, `0.7611%` at `110 / 158 / 205 / 254` against a `tan ρ` that falls
monotonically at every step — `0.561`, `0.578`, `0.836`, `0.615` times it. The band's plateau
between `158` and `205` is therefore the inward term jumping, not the allowance stalling:
`f = band / tan ρ` reads `1.303`, `1.310`, `1.561`, `1.338`, where the `(0.73 + 0.56)`
decomposition would hold it near `1.30`.

| `m` | `tan ρ %` | band `%` | `A %` | inward `%` | inward / `tan ρ` | `f` |
|---|---|---|---|---|---|---|
| `110` | `2.8568` | `3.7230` | `2.1206` | `1.6024` | `0.561` | `1.303` |
| `158` | `1.9886` | `2.6059` | `1.4556` | `1.1503` | `0.578` | `1.310` |
| `205` | `1.5326` | `2.3920` | `1.1105` | `1.2815` | `0.836` | `1.561` |
| `254` | `1.2369` | `1.6553` | `0.8942` | `0.7611` | `0.615` | `1.338` |

**Prediction.** Written before the run: the inward share tracks `tan ρ`, so `f` stays near `1.30` at
every table. Held at `158` (`1.310`) and failed at `205`, where the inward share is `49%` above its
`110` ratio and `f` `20%` above `1.303` — the same table the sweep expected to land under `2%`.

**Why size is not the variable.** `e_in` is the dual update's own minorisation loss, stage 5's hull
shortfall at the active facet, so it depends on how the grid lines up with that facet rather than on
the cell's angular width. Part E sees the same alignment effect on the small block, where the
allowance ratio alternates `1.000` and `0.231` between counts. This is the first measurement of the
inward term against density; the cause is a hypothesis, not a measurement.

**Consequence.** `f` is not a constant of the case: `1.30–1.34` at `110 / 158 / 254` and `1.56` at
`205`, so quoting `2c` as `1.30 tan ρ` would understate that table by `20%`. Quote the measured band,
and quote the table with it.

**Limits.** One case, first block, four counts. The pairing assumes the engine's printed `R` equals
the exact polygon's `R`; they agree to `0.4%` at `110` (`1.6024` against the recorded `1.5964`) and
that difference is not separated from the inward term. The last block's inward share is not
decomposed — only the total band is known there, and it is `17–20%` at every count.

**Evidence.** `python tools/density_sweep.py` for the band; `python tools/exact_polygon.py` part E
for `A`.

**Reuse.** Do not quote `e_in` as `c tan ρ`. Measure it at the table in use, and expect it to be
non-monotone in the count.

**Links.** `06-density-scaling-on-two-c.md` (the `tan ρ` assumption this falsifies),
`06-density-sweep-measured.md` (the plateau), `06-two-c-slack-is-drift.md` (`2c`'s inward term),
`06-three-d-bound-fails-at-a-cell-vertex.md` (alignment as a cause).
