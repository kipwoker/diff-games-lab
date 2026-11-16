# 02-p1-relative-interior — bounding the evader's turn rate gives P1 an interior

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1b.txt`: `w_p = 1.0`, `w_e = 0.5`, `T = 4` |
| resolution | `121 x 36 x 18` in `(d, psi_p, psi_e)`, `Step 0.1`, 3 turn rates each |

**Claim.** With the evader's turn rate bounded below the pursuer's, the redefined P1 bridge is no
longer its capture disc: at `psi_e = 0`, both vehicles aligned along the line of sight, the largest
reachable distance is `1.233 m` against the `1.3 m` the closure rate `v_p - v_e = 0.3 m/s` allows,
so the interior the point-evader case lost is back.

| `psi_e` | 0 | 45 | 90 | 135 | 180 | 225 | 270 | 315 |
|---|---|---|---|---|---|---|---|---|
| `max d*` over `psi_p` | `1.233` | `1.317` | `1.700` | `2.000` | `2.000` | `2.000` | `1.700` | `1.317` |
| `min d*` over `psi_p` | `0.100` | `0.100` | `0.100` | `0.100` | `0.100` | `0.100` | `0.100` | `0.100` |

`2.000` is the edge of the grid, not a boundary of the bridge, and it appears where the evader's
heading points back along the line of sight: from `psi_e = 135°` on, the evader starts by flying at
the pursuer, so a large initial distance is already winning. The `min` over `psi_p` is the capture
radius everywhere, which is the honest reading of the state: a pursuer pointing away from the
evader cannot recover within `T` at any distance above `r`, so the bridge is thin along its worst
heading and thick along its best.

Two checks fix the machinery before the numbers mean anything. At the aligned state with
`d = 1.0`, the value is `30.3` blocks where closure alone needs `(1.0 - 0.1)/0.3/0.1 = 30`, so the
grid is not gaining time it should not have. And `(d, psi_p, psi_e) = (2, 0, 0)` — fleeing along the
line of sight, needing `6.33 s` — is outside, while `d = 2` with `psi_e = 180°` is inside, which is
the same statement about direction rather than distance.

**Evidence.** `python tools/p1_relative.py --we 0.5`; the anchors above are printed by the same run.
The solver is a value iteration with `BIG` replaced by the horizon cap: an infinite sentinel leaks
through the interpolator and diverges, and renormalising the reachable corners instead is optimistic
and lets the bridge jump across unreachable cells.

**Superseded in scope.** The `1.233 m` is the `nd 121` value; at `nd 481` it is `1.275`
(`02-p1-refinement.md`), so quote the range, not this number alone.

**Limits.** A reference, not a certificate: `3 x 3` sampled turn rates, `36` and `18` angle cells,
nothing bracketing the grid's error. The turn-rate sweep is **not** yet an instrument — at large
`w_e` the sampled turn `w_e·Step` is a jump of several radians, and `max d*` saturates the grid edge,
so the predicted threshold `w_e = w_p v_e / v_p = 0.7` (equal turning radii) stays a hypothesis.
Three dimensions, so if this case is ever run through the support-function pipeline it inherits the
`19.84°` table radius (`06-two-neighbour-bound.md`).

**Reuse.** Cite as the redefined P1's first grid reference, and for the reading that the chauffeur
degeneracy is the free-turning limit `w_e -> inf`, not a property of pursuit itself.

**Links.** `02-p1-bridge-is-the-capture-set.md` (the case this replaces),
`02-p1-case-definition.md`, `proposals/02-nonlinear-games.md`.
