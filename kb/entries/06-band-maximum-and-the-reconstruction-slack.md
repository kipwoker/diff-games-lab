# 06-band-maximum-and-the-reconstruction-slack — the band is `1.001–1.033×` the sampled width, not `1.13×`

| field | value |
|---|---|
| type | measurement, one falsified identity |
| track | 6 |
| date | 2026-11-13 |
| case | four 2D cases at `110`; `2c` at `110`, `158`, `205`, `254` |
| resolution | tight reconstruction (`1e-9`), 1440-angle probe, exact angular maximum |

**Claim.** The band's maximum over angles is `max_ψ (h_outer − h_inner)`, the largest distance from
an outer vertex to the inner polygon — not the diameter of the pairwise vertex differences, which
is the sets' separation and came out `25–34×` too large (`example-1`: `6.000299` against
`0.173854`). Measured with the polytopes rebuilt at the solve tolerance, the maximum is
`1.0008–1.0333×` the sampled width, and it equals the 1440-angle probe to six decimals.

| case | block | at the samples | probed | exact | exact/sampled |
|---|---|---|---|---|---|
| `example-1` | 20 | 0.173854 | 0.174291 | 0.174291 | 1.0025 |
| `2a` | 8 | 0.079583 | 0.082231 | 0.082231 | 1.0333 |
| `2b` | 1 | 0.050714 | 0.050770 | 0.050770 | 1.0011 |
| `2c` | 1 | 0.047131 | 0.047168 | 0.047168 | 1.0008 |

**The slack.** The first run used the document's `0.01` vertex acceptance, read `1.0254–1.1250×`,
and at `254` the "exact" band fell *below* the sampled width (`1.4000%` against `1.6553%`), which
no true maximum can: the `0.01` tolerance is larger than a dense table's band, so the
reconstructed inner polytope sat outside the caps that admitted it. `convex_polygon::reconstruct_with`
takes the tolerance and the band uses `1e-9`; the document's own output is unchanged.

**The rung at the exact instrument.** `2c` block 1 reads `3.7259`, `2.6078`, `2.3920`, `1.6561%`
at `110/158/205/254`. `205` is still over `2%` and `254` under, so the rung stays `254`, and the
first-block figures are the sampled ones to `0.1%`.

**Prediction.** Written before the run: the exact maximum within `0.5%` of the probe. Held, at
`1.0000`. What did not hold was the identity about *what* the maximum is, and its `25–34×` error
is recorded above.

**Limits.** The maximum is a support gap, not a Hausdorff distance. `one_sided` orders the inner
vertices around their centroid, which is the boundary order because a convex polygon's vertices
are in convex position. Only `2c`'s first-block rung was rebuilt at the four counts.

**Evidence.** `python tools/band_between_samples.py` and `python tools/density_sweep.py` after the
release build.

**Reuse.** Quote the between-samples excess as `0.1–3.3%`, not `13%`; quote the exact band as
`max_ψ`, never as a pairwise diameter.

**Links.** `06-band-between-the-samples.md` (the corrected `13%`), `06-bracket-folds-inward.md`
(the width convention), `06-density-sweep-measured.md` (the first-block table),
`06-band-direction-is-a-short-direction.md` (what the band is not).
