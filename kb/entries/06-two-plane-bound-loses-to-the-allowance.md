# 06-two-plane-bound-loses-to-the-allowance — on `2c` the pair bound is `1.50×`, the one-term `1.05×`

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2c`, all eight blocks, the exact polygon at each block end |
| resolution | 110 directions; `c_i = h_W(ψ_i)` exact; samples, cell midpoints, facet normals, 4024 sweep |

**Claim.** The stored supports are exact at the sampled directions, so the outer error is interpolation
alone. Measured against the two candidates, the one-term allowance is the tighter: at the cell that
sets the band it is `1.051×` the true excess, and the two-plane bound is `1.501×`. Replacing the
allowance with the pair bound would widen `2c`'s band by half, not narrow it.

| block | `R` | worst excess | excess / `R` | one-term / excess | two-plane / excess |
|---|---|---|---|---|---|
| 1 | `0.635460` | `0.012820` | `2.017%` | `1.051` | `1.501` |
| 2 | `0.562143` | `0.011284` | `2.007%` | `1.061` | `1.501` |
| 3 | `0.487084` | `0.009677` | `1.987%` | `1.082` | `1.501` |
| 4 | `0.410282` | `0.007999` | `1.950%` | `1.120` | `1.501` |
| 5 | `0.332188` | `0.006249` | `1.881%` | `1.193` | `1.501` |
| 6 | `0.253031` | `0.004428` | `1.750%` | `1.342` | `1.501` |
| 7 | `0.174283` | `0.002535` | `1.455%` | `1.746` | `1.501` |
| 8 | `0.101980` | `0.000571` | `0.560%` | `5.099` | `1.501` |

No probe violates the two-plane bound, and block 1's `a1 = 0.013476` is the engine's own
`max_allowance` for `2c`, so the two instruments agree.

**Prediction.** Written before the run: the pair bound within `1.3×` of the excess, the allowance at
`1.5–4×`. Dominance held; both magnitudes failed the same way — pair `1.501×`, allowance `1.051×`.

**Scale.** `excess / R` stays within `2.017% – 1.881%` over blocks 1–5 and only decays at 7–8, where
the set has nearly collapsed; since the allowance is also proportional to `R`, the band is a property
of the table, which is what lets a rung quote one number. The true band at block 1 is `2.017%` of the
diameter — the rung's `2%` — against `3.717%` printed: `2.121%` per-side charge, `1.596%` inward.

**Why the pair is loose.** `P^` is the intersection of 110 half-spaces, not two: the pair's vertex
lies far outside `W`, so its extent buys nothing over `h_W` beyond what the other 108 constraints cut
away. The `1.501` is identical at every block because it is a ratio of two scale-free corners.

**Consequence.** Tightening the outer term cannot move the band: that cell is within `5%` of the truth
and the rest is inward, so the 2D budget is `e_in` plus a `5%` margin, not a bound-tightness problem.

**Limits.** One case, one geometry family (nilpotent `A`, symmetric square controls), and the widest
excess is not at block 8 where the support failure is. The pair bound is measured in its straddling
form only: for any other pair `pair_extent` returns that pair's vertex projection, which is not a
support. The engine's `cell_bound` did minimise over such pairs and that understates a bound in 2D
as well as 3D — `−1.414214` at `45°` on a unit square — which is now repaired
(`06-cell-bound-was-not-a-bound.md`).

**Evidence.** `python tools/exact_polygon.py` part D; the reconstruction is clipped exactly rather
than sampled, so `e` is the defect of interpolation and nothing else.

**Reuse.** Cite before replacing the allowance with a per-cell pair bound: in 2D it costs `50%` more.

**Links.** `06-two-neighbour-bound.md` (the pair form in 2D), `06-direction-bound.md` (the one-term
form), `04-two-c-slack-is-drift.md` (`2c`'s support failure is the rule), `06-bracket-folds-inward.md`
(the band this explains).
