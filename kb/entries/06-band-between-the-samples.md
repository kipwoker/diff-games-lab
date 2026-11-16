# 06-band-between-the-samples — the printed band is up to `13%` narrow between the samples

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-1`, `2a`, `2b`, `2c` at `m = 110` |
| resolution | `--bracket`'s 1440-angle probe of `h_outer − h_inner`, against the widest printed width |

**Claim.** A block's printed width is defined at the 110 sampled directions, but the band is the
gap between the outer and inner polytopes, which has a support at every angle. Probing 1440
angles finds the widest gap above the widest sample on three of four cases:

| case | block | at the samples | probed | ratio |
|---|---|---|---|---|
| `example-1` | 20 | 0.173854 | 0.173854 | 1.0000 |
| `2a` | 8 | 0.079583 | 0.086727 | 1.0898 |
| `2b` | 1 | 0.050714 | 0.054151 | 1.0678 |
| `2c` | 1 | 0.047131 | 0.048329 | 1.0254 |
| `2c` | 8 | 0.041216 | 0.046370 | 1.1250 |

In the entries' units the bands at `110` are `3.87%` for `2b`, `3.81%` for `2c`'s first block and
`3.28%` for `2a`'s last, against the `3.6259`, `3.7170` and `3.0090%` quoted.

**Prediction.** Written before the run: the probed widest within `1%` of the sampled one.
Falsified on three cases, worst `+12.5%` at `2c`'s collapsed block. `example-1` held, with its
widest gap at a sampled angle (ratio `1.0000` to four decimals).

**Why.** The gap is maximised where the two support functions separate most, and that need not be
a sampled angle. The samples are where the caps are exact, not where the gap is largest.

**Consequence.** A band quoted from `--bracket` is a lower bound on the set's band, by up to
`13%`. The density sweep's trend is unaffected — the same bias applies at each count — but any
absolute band should be the probed one, and `2%` should be compared against `3.28–3.87%`, not
`3.01–3.72%`.

**Corrected by measurement.** The ratios above were taken on polytopes reconstructed with the
document's `0.01` vertex slack, which is larger than a dense table's band. Rebuilt at `1e-9` the
excess is `1.0008–1.0333×`, so the bands are `2.82–3.72%` — essentially the sampled figures — and
the `13%` is the instrument, not the band (`06-band-maximum-and-the-reconstruction-slack.md`).

**Limits.** 2D only, 1440 angles at `0.25°`; the probe is a support gap at a direction, not a
Hausdorff distance.

**Evidence.** `python tools/band_between_samples.py` after the release build.

**Reuse.** Quote a band as "at the samples", or cite the probed figure; do not present the sampled
width as the set's band.

**Superseded.** At a `1e-9` tolerance the excess is `1.0008–1.0333×`, so the `13%` was the
`0.01` slack and not the band (`06-band-maximum-and-the-reconstruction-slack.md`).

**Links.** `06-bracket-folds-inward.md` (the width convention), `06-bracket-and-cost.md` (the
bands this corrects), `06-density-sweep-measured.md` (the trend it leaves alone),
`06-two-sided-bracket.md` (the 24 probes that tested inclusion, not width).
