# 02-p1-refinement — the P1 interior distance is still moving with the `d` grid

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1b.txt`, `w_e = 0.5` |
| resolution | `nd` 121, 241, 481; `36 x 18` angles; `Step 0.1`, `T = 4` |

**Claim.** The bridge's furthest reachable distance at `psi_e = 0` rises with the `d` resolution —
`1.233` at `nd 121`, `1.250` at `241`, `1.275` at `481` — so the first grid reference overstated the
margin over the `1.3 m` closure limit by `0.042`, and the interior survives at every resolution but
with a margin of `0.025` at the finest.

| `nd` | `d` spacing | `psi_e = 0°` | `45°` | `90°` | `135°` |
|---|---|---|---|---|---|
| `121` | `0.0167` | `1.233` | `1.317` | `1.700` | `2.000` |
| `241` | `0.0083` | `1.250` | `1.325` | `1.708` | `2.000` |
| `481` | `0.0042` | `1.275` | `1.337` | `1.708` | `2.000` |

The `135°/180°/225°/270°` columns sit exactly at `2.000` in all three runs: that is `dmax`, not a
boundary, so those four rows say the slice escapes the box and nothing more. The informative column
is `psi_e = 0` — the evader fleeing along the line of sight, where closure alone allows
`(1.3 − 0.1)/0.3 = 4 s` — and there the answer has not converged: each doubling of `nd` adds about
`0.02 m`. The honest statement is "between `1.24` and `1.28`, still rising", not `1.233`.

**Evidence.** `python tools/p1_relative.py --nd 481 --npp 36 --npe 18 --we 0.5`; the `nd 121` row is
the original run recorded in `02-p1-relative-interior.md`.

**Angle axes.** Refining the angle grids at fixed `d` leaves `psi_e = 0` unchanged (`1.250` at
`nd 241`, `1.275` at `481`), while `90°` moves `1.708 → 1.792` — so only the `0°` column was
clean (`02-p1-is-not-yet-certifiable.md`).

**Limits.** Only the `d` axis was refined; the angle grids stayed at `36 x 18`, and the value there
can still move with `npp` and `npe`. The value is a grid maximum over sampled controls, not a
supremum, and no Richardson extrapolation is attempted because the two finest points do not yet
follow a clean order. The `dmax`-capped columns are not evidence of a boundary.

**Reuse.** Cite instead of `1.233` when quoting P1's interior, and as the reason a P1 claim needs a
resolution study before it is called a number.

**Links.** `02-p1-relative-interior.md` (the coarse value), `02-p1-case-definition.md`,
`proposals/02-nonlinear-games.md` (E2).
