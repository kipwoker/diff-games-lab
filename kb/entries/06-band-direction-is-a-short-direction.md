# 06-band-direction-is-a-short-direction — the inner side is sound, and block 8's band sits inside the truth

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2c`, all eight blocks, the exact polygons of `04-two-c-slack-is-drift.md` |
| resolution | `--trace` support, charge and allowance; exact support at all 110 directions |

**Claim.** At the direction that sets each block's band, the printed support is *below* the exact
one by `0.0008–0.0114`, so the inner side needs no inward charge there and the charge is pure
margin. At block 8 the outer side `c + a` is below the exact support too, by `0.003756`, so the
widest band lies wholly inside the exact body. The inner side is inside at **all 880** sampled
directions (8 blocks × 110): the 2D allowance, not the charge, is what makes it sound.

| block | dir | printed − exact | `a` | `s` | slack at band | tightest slack | at | outer − exact |
|---|---|---|---|---|---|---|---|---|
| 1 | 66 | −0.011417 | 0.017858 | 0.011416 | 0.040691 | 0.013076 | 0 | +0.006441 |
| 2 | 71 | −0.011397 | 0.015842 | 0.002228 | 0.029467 | 0.011512 | 0 | +0.004445 |
| 3 | 98 | −0.000908 | 0.013993 | 0.006115 | 0.021016 | 0.009846 | 28 | +0.013085 |
| 4 | 43 | −0.007524 | 0.011566 | 0.020164 | 0.039254 | 0.007849 | 28 | +0.004042 |
| 5 | 51 | −0.001081 | 0.007809 | 0.020007 | 0.028897 | 0.005720 | 28 | +0.006728 |
| 6 | 49 | −0.000833 | 0.006170 | 0.025796 | 0.032799 | 0.003444 | 28 | +0.005337 |
| 7 | 49 | −0.001483 | 0.003856 | 0.031261 | 0.036600 | 0.001028 | 0 | +0.002373 |
| 8 | 48 | −0.005281 | 0.001525 | 0.038166 | 0.044972 | 0.000609 | 0 | **−0.003756** |

**Prediction.** Written before the run: the slack is positive at every direction — held, `0` of
`880` negative, tightest `0.000609` at block 8 direction 0 — and the band directions are short
directions — held. The earlier half of the prediction, that a needed charge would be at least `2×`
the need, was vacuous: no block needs a charge at its band direction.

**Consequence.** `2c`'s `20.4%` band is not an uncertainty about the true set: at its widest
direction the band is inside the exact body, the truth being `0.0038` beyond its outer side. The
band measures the printed set's own slide, `2a + s`, of which `92.6%` is the charge. For the
certificate only the inner side matters, and it is sound at every sample, tightest at the
directions the allowance nearly consumes (`0.0006–0.0131`).

**Corrected by measurement.** The slack above is a *support* inequality at the 110 samples; it
makes the inner set a subset of the reconstruction of `W`, not of `W`. Tested at its vertices the
inner polytope leaves the exact body at every block, by `0.0109–0.0256`
(`06-inner-set-leaves-the-exact-body.md`).

**Limits.** One case. The check is at the 110 sampled directions; between them the inner polytope
can still leave the exact body, which is the separate inclusion question
(`06-band-between-the-samples.md`, `06-two-sided-bracket.md`). The charge and allowance are the
band-direction and tight-direction values, not a statement about other directions.

**Evidence.** `python tools/charge_vs_deficit.py` after the release build.

**Reuse.** Cite the 2D inner side as sound at the samples, carried by the allowance; do not cite
`2c`'s last-block band as an uncertainty about the true set.

**Links.** `04-two-c-slack-is-drift.md` (the exact support), `06-last-block-band-is-drift.md` (the
`92.6%` charge), `06-bracket-folds-inward.md` (the `2a + s` width), `06-band-between-the-samples.md`
(the between-sample gap), `06-two-sided-bracket.md` (the 24 inclusion probes).
