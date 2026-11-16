# 06-chamfering-gap-is-not-the-table — `gap / tan ρ` runs `0.38–1.55` and a cap shrink empties block 8

| field | value |
|---|---|
| type | measurement, prediction falsified; an earlier entry corrected |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2c`, blocks 1 and 8, tables `110 / 158 / 205 / 254` |
| resolution | `inner_between_samples.py` part 2, `support_violation` against the exact block polygons |

**Claim.** The inner-side gap is not a table chamfer: `gap / tan ρ` is `0.82 / 0.75 / 1.35 / 0.94`
at block 1 and `0.38 / 0.41 / 0.99 / 1.55` at block 8, and it rises at `254` rather than falling,
so it does not scale with the spacing. Shrinking every cap by the measured gap leaves the thin
block 8 empty (`R = 0.0600`) instead of repairing it, and the `110` fit `k tan ρ` changes nothing
at `254` because nothing there is out of the body.

| m | block 1 `gap/tan` | block 8 `gap/tan` | block 8 `R` | shrink by gap |
|---|---|---|---|---|
| 110 | 0.8215 | 0.3824 | 0.059996 | empty |
| 158 | 0.7461 | 0.4061 | 0.065620 | stays |
| 205 | 1.3483 | 0.9888 | 0.072923 | empty |
| 254 | 0.9445 | 1.5454 | 0.075097 | empty |

**Prediction.** Written before the run: `gap / tan ρ` constant within `15%`, and the `110` fit
covers `254` with zero residual violation. Falsified on the constancy (spread `4×`) and vacuous on
the repair: `254` has no violation to cover.

**The earlier claim was the instrument.** `outside_distance` returned the euclidean distance to the
nearest edge for *interior* points too — the exact block-1 centroid read `0.448750`, and the sign
test was inverted. So `06-inner-set-leaves-the-exact-body`'s `0.0109–0.0256` is a distance, not a
violation. `support_violation`, the largest facet-normal excess over outward normals, is zero at
every vertex at `110`, `158` and `254`; at `205` it is `0.000478` at block 1 and `0.000779` at
block 8, at vertices between samples while every sampled cap is under the exact support by at least
`0.000255`.

**Consequence.** The inner side is sound as a set, not just at the samples, for three of the four
counts; the one failure is `205`'s, at `0.08%` of block 1's radius, and it is between samples.
Nothing here is a `tan ρ` term, so a cap shrink cannot be the fix; the `205` violation needs the
exact body, not the table.

**Limits.** Blocks 1 and 8, one case, up to `254` directions. `support_violation` bounds the
euclidean distance from below and is sound at zero, which is the direction the claim needs. The
exact polygons are `exact_polygon.two_c()`.

**Evidence.** `python tools/inner_between_samples.py` after the release build.

**Reuse.** Quote the inner side as inside at `110/158/254` and out by `4.8e-4` at `205`; do not
quote `0.0109–0.0256` as a violation.

**Links.** `06-inner-set-leaves-the-exact-body.md` (the corrected entry),
`06-band-direction-is-a-short-direction.md` (the sampled support test),
`06-rung-is-254-at-the-exact-band.md` (the count the repair would need),
`01-deficit-is-chamfering.md` (the chamfering the gap was supposed to be).
