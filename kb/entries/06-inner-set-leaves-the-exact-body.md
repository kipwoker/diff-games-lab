# 06-inner-set-leaves-the-exact-body — every inner vertex is outside the exact `2c` polygon

| field | value |
|---|---|
| type | measurement, prediction falsified |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2c`, eight blocks, inner polytope from `c − a − s` |
| resolution | exact clipping of the shifted caps, exact block polygons, vertex test |

**Claim.** The shift `a + s` satisfies `c − a − s ≤ h_W` at every one of the `880` sampled
directions, yet the inner *set* is not inside the exact body: at all eight blocks every one of its
vertices is outside, by `0.0109–0.0256` — `18%` of the inner radius at block 8. The sampled
inequality puts the inner inside the **reconstruction** of `W` from its own supports, and that
reconstruction is larger than `W` at its corners, which is the chamfering of
`01-deficit-is-chamfering.md`.

| block | inner vertices | outside | worst outside | inner `R` |
|---|---|---|---|---|
| 1 | 20 | 20 | 0.023467 | 0.599887 |
| 2 | 22 | 22 | 0.020995 | 0.537463 |
| 3 | 24 | 24 | 0.019570 | 0.465923 |
| 4 | 34 | 34 | 0.025646 | 0.376236 |
| 5 | 36 | 36 | 0.022598 | 0.309675 |
| 6 | 44 | 44 | 0.022778 | 0.228744 |
| 7 | 50 | 50 | 0.022059 | 0.155261 |
| 8 | 20 | 20 | 0.010926 | 0.059996 |

**Prediction.** Written before the run: every inner vertex inside, worst outside distance zero,
because between samples the inner support falls along chords while the exact body's does not.
Falsified at all eight blocks. The chords are not the binding direction — the inner's **corners**
are, and at a corner the inner is the reconstruction's corner, not the body's.

**Consequence.** The bracket's inner side is inner for the printed reconstruction, not for the true
bridge. A certificate that treats it as inside the true bridge is short by the chamfering gap,
which is the same order as the bands: `0.0109–0.0256` against `0.0412` at block 8. So the
certificate's inner side is sound in the support sense at the samples and unsound as a set.

**Limits.** One case. The inner polytope is clipped exactly from the caps, so this is the shift's
property rather than the engine's reconstruction tolerance. The distance is Euclidean to the
polygon, not a support gap.

**Evidence.** `python tools/inner_between_samples.py` after the release build.

**Reuse.** Do not read `0` negative slacks as set inclusion: it is a support test at the samples,
and the set leaves the body at its corners.

**Correction.** The outside column is the euclidean distance to the nearest edge, not a violation:
`outside_distance`'s inside test was inverted (the exact centroid read `0.448750`). Measured with
`support_violation`, every vertex of these eight blocks is inside; the real between-sample
violation is `4.8e-4` at `205` (`06-chamfering-gap-is-not-the-table.md`).

**Links.** `06-band-direction-is-a-short-direction.md` (the support test this qualifies),
`01-deficit-is-chamfering.md` (the gap), `06-two-sided-bracket.md` (the 24 probes),
`06-band-maximum-and-the-reconstruction-slack.md` (the reconstruction's own slack).
