# 06-density-scaling-on-two-c — both terms track `tan ρ`, and `2c` needs `205`, not `158`

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | `example-2c`, blocks 1 and 8, `m` from 110 to 254 |
| resolution | exact polygon; supports exact at the samples; block 1 is the one that sets the band |

**Claim.** At the block that sets `2c`'s band, the allowance's share of the diameter falls
`0.742 → 0.723` times `tan ρ` and the true excess's is `0.706 tan ρ` flat, so density buys both terms
proportionally. The printed band adds the inward shortfall (`1.5964%` at 110, assumed to track `tan ρ`
too), so `2c` would print `2.567%` at `158` and `1.967%` at `205`: its rung needs `m ≈ 205`, not the
`158` the `f = 1` cases imply.

| `m` | `ρ` | `tan ρ`, % | allowance, % | true excess, % | `A/T` | allowance/`tan ρ` | excess/`tan ρ` |
|---|---|---|---|---|---|---|---|
| 110 | 1.6364° | 2.8568 | 2.1206 | 2.0174 | 1.051 | 0.742 | 0.706 |
| 135 | 1.3333° | 2.3275 | 1.7085 | 1.6481 | 1.037 | 0.734 | 0.708 |
| 158 | 1.1392° | 1.9886 | 1.4556 | 1.4043 | 1.037 | 0.732 | 0.706 |
| 205 | 0.8780° | 1.5326 | 1.1105 | 1.0852 | 1.023 | 0.725 | 0.708 |
| 254 | 0.7087° | 1.2369 | 0.8942 | 0.8734 | 1.024 | 0.723 | 0.706 |

Both figure columns are `value / R`, which is already a share of the diameter because the diameter is
`2R` and each band is two-sided. `2c`'s printed `3.7170%` at 110 is `2.1206%` allowance plus `1.5964%`
inward, i.e. `f = 1.30`; at `158` that projects to `2.567%` and at `205` to `1.967%`, so the `158` is
the `f = 1` case and `2c` needs the widest-`f` row of the rung entry.

**Prediction.** Written before the run: `A/T` stays `1.051` at every `m`, and `158` gives
`A = 1.475%`, `T = 1.403%`. `T` landed at `1.4043%` and `A` at `1.4556%`, but `A/T` fell to `1.024` —
the over-charge shrinks as the cell does, which is the one thing the run corrected.

**Alignment is not density.** At block 8 the columns alternate: allowance/`tan ρ` reads `1.000` at
`110`, `0.231` at `135`, `0.994` at `158`, `0.219` at `205`, because the worst probe there sits near a
sample at some counts and mid-cell at others. A scale law read off the small block would be an artifact
of where the probe fell; block 1 is stable and is the only block whose excess sets the band.

**Corrected by measurement.** The engine now runs other tables: `2c` prints `2.6059%` at `158` and
`2.3920%` at `205`, so the `2.567` held and the `1.967` did not, and the inward share does not track
`tan ρ` (`06-density-sweep-measured.md`). The inward share is still not this tool's number.

**Evidence.** `python tools/exact_polygon.py` part E, on the same exact blocks as parts B and C.

**Reuse.** Quote `2c`'s rung as `205`, and its band as `(0.73 + 0.56) tan ρ`.

**Links.** `06-rung-tolerance-is-a-table-size.md` (the `158`), `06-two-plane-bound-loses-to-the-allowance.md`
(block 1's decomposition), `06-bracket-folds-inward.md` (`2c`'s `3.7170%`), `04-two-c-slack-is-drift.md`.
