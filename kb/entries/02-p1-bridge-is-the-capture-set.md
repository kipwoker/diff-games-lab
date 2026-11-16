# 02-p1-bridge-is-the-capture-set — P1 has no interior, and its relative state is 3D

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1.txt`, `T = 4` |
| resolution | value iteration in `(d, ψ)`, `401 × 144`, 5 turn rates, 24 evader headings |

**Claim.** In the chauffeur ordering with a *point* evader, the `T = 4` bridge is the capture disc
and nothing more: at three block lengths the largest reachable distance is `0.100` m for every
relative angle except directly ahead, where it is `0.13` m, against the `1.30` m a straight-line
chase would allow.

| `d*` at `ψ =` | 0° | 45° | 90° | 180° |
|---|---|---|---|---|
| closure-only (evader runs straight away) | 1.300 | 0.949 | 0.100 | 0.100 |
| value iteration, `Step 0.02` | 0.130 | 0.100 | 0.100 | 0.100 |
| value iteration, `Step 0.05` | 0.155 | 0.100 | 0.100 | 0.100 |
| value iteration, `Step 0.1` | 0.100 | 0.100 | 0.100 | 0.100 |
| naive `81 × 81 × 48` 3D grid, `Step 0.1` | 0.100 | 0.100 | 0.100 | 0.100 |

The evader wins by curvature. It can hold a circle of any radius, in particular `v_e/ω̄ = 0.7` m at
exactly the pursuer's maximum angular rate, while the pursuer's smallest curved path has radius
`v_p/ω̄ = 1` m. A carden with a larger turning radius cannot follow that circle, so it cannot be
within `r` of a target on it, and passing through the centre does not help: the target is never
there. The state `(2, 0, 0)` therefore has infinite value at every resolution tested, and the
`1.30` m comes from assuming the evader cooperates.

Two consequences, both bigger than the number. The rung as specified has no interior to study —
a bridge equal to its capture set is not a game with a front, and no discretization argument can
make the nonlinear pipeline interesting on it. And the reduction used to get here is a correction
to the P1 design: rotating position and heading together leaves the dynamics unchanged, so the
chauffeur game is `(d, ψ)` and the *two-turn-limited* game is `(d, ψ_rel, ψ_e)` — three numbers,
not the four that made it `P1x`. With two angles weighted by their turning radii, the existing
110-direction table applies, so P1x does not need a new direction construction after all.

**Evidence.** `python tools/p1_grid.py` and its `--step`, `--nd`, `--nu`, `--nbeta` options; the
3D grid that was tried first is in the same tool's history and is not part of the claim.

**Limits.** Value iteration on sampled controls, with no certificate and no executed evader
strategy — the orbit is an argument, not a verified witness (track 4's job). `min_u max_β` per
block is conservative; it is the block length, not the ordering, that the three steps rule out.
One speed ratio: `v_e/v_p = 0.7`. Nothing here says a slower or slower-turning evader is safe.

**Reuse.** Cite before quoting any P1 interior, before building P1's nonlinear pipeline, and
whenever P1x is called 4D.

**Links.** `02-p1-case-definition.md` (the case this voids), `proposals/02-nonlinear-games.md`,
`README.md` (the ladder).
