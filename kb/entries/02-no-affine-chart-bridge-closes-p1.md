# 02-no-affine-chart-bridge-closes-p1 — the step is the wall, not the chart count

| field | value |
|---|---|
| type | measurement, decision; two predictions falsified |
| track | 2 |
| date | 2026-11-14 |
| case | `examples/p1/example-p1b.txt`, `w_e = 0.5`, `T = 4`, `Step 0.1` (40 blocks) |
| resolution | `p1_relative.py --flow-error`: exact flow against the grid's Euler step, and the affine fit of the block map over one chart |

**Claim.** P1's interior cannot be certified by an affine-chart support-function bridge at its own
step. The per-block budget is `0.150 / 40 = 3.75e-3`; the grid's Euler error is `8.4×` that budget in
the smoothest band and `257×` near capture, and that error belongs to the *step*, so no charting
removes it. The charting is worse: the residual falls only as `half^0.44`, so meeting the budget needs a
half width of `5.7e-6` and about `4.9e16` charts.

| band `d` | Euler error | × budget |
|---|---|---|
| 0.15–0.30 | `9.634e-1` | 257 |
| 0.30–0.60 | `2.836e-1` | 76 |
| 0.60–1.00 | `7.604e-2` | 20 |
| 1.00–1.41 | `3.138e-2` | 8.4 |

| half width | affine residual | × budget | charts |
|---|---|---|---|
| `0.02` | `1.306e-1` | 34.8 | `1.20e6` |
| `0.05` | `1.947e-1` | 51.9 | `7.54e4` |
| `0.10` | `3.507e-1` | 93.5 | `1.02e4` |
| `0.20` | `8.215e-1` | 219 | 1280 |
| `0.40` | `1.522` | 406 | 192 |

**Prediction.** Written before the run: the residual is quadratic in the half width, so `~0.005` meets
the budget under `10^5` charts, and the Euler error stays under budget for `d >= 0.6`. Falsified twice.
The local exponent is `0.44`, not `2`; the required half is `5.7e-6` with `4.9e16` charts; and the Euler
error is over budget in every band, `20×` at `0.60-1.00` and `8.4×` at `1.00-1.41`.

**Why the residual is not quadratic.** The block map's nonlinearity is `1/d` and `cos psi`, not a
quadratic form, and the residual saturates near `1.5` — the block's own displacement — so a `2.5×`
refinement buys only `1.5×`, which is why the extrapolation lands far below the tested charts.

**What it decides.** Three routes, with their prices. (i) A smaller step: the error quarters per halving while the budget halves, so the
ratio improves `~2×`; `8.4×` needs about three halvings, `0.0125 s` and `320` blocks, before charting
is reconsidered. (ii) Reformulate P1 as a *discrete-time* game at `0.05-0.1 s`:
then the step is the model and there is no discretisation error to carry, at the cost of certifying the
sampled game rather than the continuous one. (iii) Propagate a nonlinear set (level set, Taylor model,
validated integrator), which is outside the affine-bridge proposal. The recommendation is (ii) plus the
track-6 band, with P1 quoted as measured and not certified.

**Limits.** One step (`0.1 s`), controls at vertices, chart metric `scale_theta = 1`; the residual also
scales with the step, so `5.7e-6` and `4.9e16` are this step's numbers. The bands near capture are the
model's `1/d` singular line and are quoted separately.

**Evidence.** `python tools/p1_relative.py --flow-error` prints the Euler bands, the chart table, the local exponent and the extrapolation.

**Reuse.** Before proposing any linearised or affine-chart bridge, quote the step's Euler ratio first;
it is the part of the error that charting cannot touch.

**Links.** `02-p1-bridge-is-not-affordable.md` (the old `200×` remainder, which is `34.8×` at the
corrected margin), `02-p1-step-refinement.md` (where `0.150` comes from), `02-p1-relative-interior.md`,
`proposals/02-nonlinear-games.md`.
