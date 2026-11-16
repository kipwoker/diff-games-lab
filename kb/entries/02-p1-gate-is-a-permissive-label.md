# 02-p1-gate-is-a-permissive-label — `11.5%` is the loosest honest denominator

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-16 |
| case | `examples/p1/example-p1b.txt`, `w_e = 0.5`, `T = 4`, `Step 0.0125` |
| resolution | grid `121 x 36 x 18`, 320 blocks; table fractions from the cell-radius convention |

**Claim.** P1's gate `0.150 / 1.3 = 11.5%` divides the margin by the closure limit, not by the
support the allowance is charged on. Measured denominators give `13.04%` at the interior,
`11.54%` at that limit, `<= 7.50%` at the widest interior and `4.77%` on the angle axis. The
`11.5%` is neither the loosest nor the strictest reading, and no measured table clears even the
loosest: the nearest is `2.1×` over.

| denominator | value | allowed fraction of it |
|---|---|---|
| interior at `psi_e = 0` | `1.150` | `13.04%` |
| closure limit | `1.300` | `11.54%` |
| widest interior, at the `dmax` edge | `>= 2.000` | `<= 7.50%` |
| angular extent, `scale_theta = 1.0` | `3.1416` | `4.77%` |

| table | allowance, cell radius | absolute at `1.150` | ratio to `0.150` |
|---|---|---|---|
| `11 x 10` | `44.65%` | `0.513` | `3.4×` |
| `16 x 15` | `27.14%` | `0.312` | `2.1×` |
| `13 x 12`, aligned | `35.78%` | `0.412` | `2.7×` |

**Prediction.** Written before the run: the loosest reading is `margin / d*` at `psi_e = 0`
(`13%`), the closure limit's `11.5%` sits inside that bracket, the angular extent is far stricter
(`4.8%`), and no measured table clears even `13%`. All four held. One surprise: the widest
interior returns `2.000`, the `--dmax` edge, so it is a clip and is quoted as `>=`.

**What it decides.** The `12.4%` provenance bug was an absolute number used as a fraction; `11.5%`
is dimensionless and correct, but it is a ratio to the closure limit and reading it as "the table
must be within `11.5%` of support" is generous by `1.13×`. The strict end is the one worth quoting:
with `scale_theta = 1.0 m/rad` an angular cell charges `pi tan rho`, so a table sized against the
`d` axis can still be `2.7×` over on the angle axis (`13.04 / 4.77`). The verdict is unchanged at
every denominator, which is why P1's table requirement is a measurement and not a convention.

**Limits.** `1.150` is resolved to the `d` grid's `0.0167` spacing and is still rising at the
finest `d` axis measured; the widest interior is domain-clipped at `dmax = 2.0`; the table
fractions are measured on a cube, not on P1's own body; the angular denominator uses the box's
full `pi` and is therefore an upper bound on the set's angular reach, hence a lower bound on the
fraction. Only `w_e = 0.5` and `step = 0.0125` were run.

**Evidence.** `python tools/p1_relative.py --gate --step 0.0125 --nd 121 --npp 36 --npe 18`
prints the four denominators, the three tables and the ratios; the table fractions are
`covering_bound.py`'s cell-radius column.

**Reuse.** Quote `13.04%` for the `d` axis and `4.77%` for the angle axis, each with its
denominator, and never quote `11.5%` as a fraction of support.

**Links.** `02-p1-step-refinement.md` (the `1.150`), `06-allowance-fractions-need-one-convention.md`
(where `44.65%` comes from), `02-p1-is-not-yet-certifiable.md` (the table this rebuilds),
`06-alignment-is-sound-not-tight.md` (the `35.78%`).
