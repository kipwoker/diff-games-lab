# 02-p1-is-not-yet-certifiable — P1's interior survives the angle axes, and the certificate is blocked elsewhere

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1b.txt`, `w_e = 0.5`, `T = 4`, `r = 0.1` |
| resolution | `nd` 241 and 481, angles `72 x 36` against `36 x 18` |

**Claim.** Refining the angle axes leaves the `psi_e = 0` interior unchanged — `1.250` at `nd 241` and
`1.275` at `481`, the same as `36 x 18` — so the `d`-axis trend in `02-p1-refinement.md` is not an
angle artefact. The certificate is still unreachable, for two quantified reasons: there is no bridge
to certify (the engine refuses the nonlinear case by name), and the 3D allowance at the shipped table
is an order larger than the margin the interior actually has.

| `nd` | angles | `psi_e = 0` | `45°` | `90°` |
|---|---|---|---|---|
| `241` | `36 x 18` | `1.250` | `1.325` | `1.708` |
| `241` | `72 x 36` | `1.250` | `1.308` | `1.783` |
| `481` | `36 x 18` | `1.275` | `1.337` | `1.708` |
| `481` | `72 x 36` | `1.275` | `1.337` | `1.792` |

The `0°` column is converged in angle and still rising in `d`, so the honest figure is a range
ending above `1.275`; the `45°` and `90°` columns are *not* converged (`90°` moves `1.708 → 1.792`
under angle refinement), so only the `0°` column was ever clean — a correction to the earlier reading.

The arithmetic of the certificate, `1.3 - 1.150 = 0.150` as the margin, with one convention per
column — the cell covering radius (`06-allowance-fractions-need-one-convention.md`):

| table | allowance, fraction of support | for a `1.3` set | against a `0.150` margin |
|---|---|---|---|
| shipped `11 x 10` | `44.65%` cube, `44.49%` octahedron | `0.58` | `3.9×` too large |
| adaptive `110`, `240` | `118.13-118.35%` / `119.48-120.53%` | `1.54` | `10×` too large |
| shipped, half-gap `ρ` | `17.90%` | `0.233` | `1.6×` too large |

The blocker is not the grid: P1 has no support-function bridge at all (the recursion is a linear-map
construction and `ẋ = f(x, u, v)` has none), and the 3D table is an azimuth-by-elevation shape
rather than an allowance-greedy one. A bridge first, then a table under the margin.

**Corrected.** The margin is `0.150`, not `0.025`: refining `Step` converges `d*` to `1.150`
(`02-p1-step-refinement.md`). The old `12.4%` was `example-3d`'s absolute allowance `0.322693` over
`2.603` — another case's number, not a fraction — and the adaptive rows used the half-gap radius;
both are replaced above (`06-allowance-fractions-need-one-convention.md`).

**Evidence.** `python tools/p1_relative.py --nd 481 --npp 72 --npe 36 --we 0.5` (103 s) and the same
at `--nd 241`; the allowance fractions are `06-allowance-fractions-need-one-convention.md`, applied
to a `1.3` support by multiplying.

**Limits.** The allowance fractions are measured on an octahedron and a cube, not on a P1 set, so the
`1.3` column is a scale argument and not a measurement of P1's own allowance. `d*` is a grid maximum
at `d` spacing `0.0042` and is still rising; the `45°/90°` columns remain unresolved in angle. No
certificate is attempted because there is nothing to certify.

**Reuse.** Cite before calling P1 certifiable, and as the reason the 3D table change is a
prerequisite for track 2 rather than a track 6 nicety.

**Links.** `02-p1-refinement.md` (the `d`-axis study this completes), `02-p1-relative-interior.md`
(the first reference), `02-nonlinear-grammar.md` (why the engine refuses the case),
`06-three-d-adaptive-allowance.md` (the table numbers), `proposals/02-nonlinear-games.md`.
