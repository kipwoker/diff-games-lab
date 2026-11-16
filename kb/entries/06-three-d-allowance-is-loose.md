# 06-three-d-allowance-is-loose — the measured 3D excess is 27–69% of what the allowance charges

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | octahedron `{|x|+|y|+|z| <= 1}`, cube `max|x| <= 1.5` |
| resolution | `11 x 10` and `16 x 15`; 4024 sweep directions, every facet normal included |

**Claim.** With supports exact at the table and the reconstruction enumerated as a polyhedron, the
worst excess over a sweep is `27.3%` of the engine's scalar allowance on the octahedron at
`11 x 10` and `68.6%` on the cube at `16 x 15`. The allowance is 1.5–3.7× the measured error, so it
is loose but not absurd — and the cube is where it is nearly tight, because a cube's facet normal
sits `18°` from the nearest sample and `1.5 tan 18° = 0.487339` against a measured `0.487380`.

| body | table | `m` | `ρ` max | worst excess | gap | bound there | of bound | of scalar |
|---|---|---|---|---|---|---|---|---|
| octahedron | `11 x 10` | 110 | `19.3787°` | `0.111638` | `9.1587°` | `0.310247` | `36.0%` | `27.3%` |
| octahedron | `16 x 15` | 240 | `13.2754°` | `0.149188` | `11.5604°` | `0.173469` | `86.0%` | `54.6%` |
| cube `1.5` | `11 x 10` | 110 | `19.3787°` | `0.487380` | `18.0000°` | `0.625896` | `77.9%` | `43.7%` |
| cube `1.5` | `16 x 15` | 240 | `13.2754°` | `0.478247` | `13.3967°` | `0.452099` | `105.8%` | `68.6%` |

"bound there" is `h(ψ) tan ρ + R(1/cos ρ − 1)` at the probe with its cell's `ρ`; "of scalar" is the
worst excess over the single `max_allowance` a certificate subtracts. The 2D control reproduces the
known result: `97.3%` of the bound on the square at 110 directions, where the earlier part measured
`0.014284` against `0.0142839`, and `72.9%` on the diamond, exact at its vertex normal and
`0.045448` out at its facet normal.

Predicted before the run: at most `0.5` of the bound on the octahedron, `0.6–1.0` on the cube, a
looseness of 1.5–3×. Measured: both per-probe figures match (`0.36`, `0.78`), and the scalar
comparison does not for the cube — `43.7%`, not `60-100%`, because the scalar is set by another cell.

**The gate does not move.** P1's gate is `11.5%` of the support. The cube's worst excess at 240 is
`0.478247` on a support of `1.5`, i.e. `31.9%`, so the *measured* error is `2.8×` the gate. A
looseness of 1.5–3.7× cannot close a gap that size: making the allowance exact would still leave P1
uncertified, and the table is the blocker rather than the bound.

**Corrected.** With the exact radius: octahedron `11 x 10` `19.8733°` / `0.339042` / `32.9%` / `25.1%`,
`16 x 15` `13.3967°` / `0.175798` / `84.9%` / `54.3%`, cube `11 x 10` `19.8733°` / `0.702781` / `69.4%` /
`42.4%`, cube `16 x 15` `13.3967°` / `0.457298` / `104.6%` / `67.9%` (`ρ` / bound / of bound / scalar):
the excess is `25.1–67.9%` of the scalar, `1.5–4.0×` the error, and the cube's `104.6%` still
**falsifies** the bound. The 2D control is bit-identical.

**Limits.** Exact supports at the table, so nothing here covers the dual update's inward loss. Two
analytic bodies and one sweep of 4024 directions; `3d`'s shell is neither body, and no bridge block
was involved. The local bound uses `h(ψ)` where the engine's per-direction allowance uses `h` at the
sample and is therefore larger; the scalar column is unaffected by that choice.

**Evidence.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as cb;
cb.part_k(); cb.part_k_2d()"` — part K reconstructs by 3-subset enumeration and sweeps every facet
normal, part L is the 2D control.

**Reuse.** Cite before quoting a 3D allowance as an error estimate: it is 1.5–3.7× the measured
excess, and the cube shows the first term is attained exactly, so it cannot be tightened much
without losing that case.

**Links.** `06-direction-bound.md` (the form), `06-two-neighbour-bound.md` (the 2D failure it does
not repair), `06-three-d-radius-is-a-lower-bound.md` (why the local column is not a bound),
`02-p1-step-refinement.md` (the gate).
