# 02-p1-case-definition — P1 is a 3D game, and its first case is out by 2.33 s

| field | value |
|---|---|
| type | dataset |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1.txt` |
| resolution | not computed yet; proposed grid `101 × 101 × 72` over `[-2,2]² × [0,2π)`, `Step 0.1` |

**Claim.** The first P1 case is three-dimensional — one turn-limited heading against a point
evader — because the direction table is built only for `n ∈ {2,3}`, and its state `(2, 0, 0)` is
outside the bridge at `T = 4` because closure is capped at `v_p − v_e = 0.3 m/s`.

| quantity | value | source |
|---|---|---|
| pursuer speed `v_p` | 1.0 m/s | case |
| evader speed `v_e` | 0.7 m/s | case |
| turn rate `ω̄` | 1.0 rad/s | case |
| turning radius `v_p/ω̄` | 1.0 m | the `θ` scale |
| capture radius `r` | 0.1 m | case |
| horizon `T` | 4.0 s | case |
| closure rate | 0.3 m/s | `v_p − v_e` |
| radial capture from `d = 2` | `(2 − 0.1)/0.3 = 6.33 s` | `> T`, so outside |
| capture at `d ≤ r` | immediate | inside |

Two structural facts are settled before any solver runs. The two-unicycle game has state
`(x, y, θ₁, θ₂)`, four-dimensional, and `directions::build` returns `None` for `n ∉ {2,3}`
(`engine/src/directions.rs`), so that game is `P1x` and needs a direction construction of its own.
And `θ` is an angle: a support function over `(x, y, θ)` needs a metric, here `θ · v_p/ω̄`, so the
existing 110 directions apply to `(ψ_x, ψ_y, ψ_θ / scale_theta)` without a new table.

**Evidence.** `sed -n '1,45p' examples/p1/example-p1.txt`; both anchors are arithmetic on the
file's numbers. No run exists yet — `tools/direction_table_check.py` does not read this case.

**Limits.** Each anchor bounds the bridge at one point, and the grid found nothing between them:
every state outside the capture disc is outside the bridge, because the point evader escapes by
curvature (`02-p1-bridge-is-the-capture-set.md`). The closure rate assumes the pursuer heads
straight at the evader, so it is a bound on how fast the gap can close, not a strategy. What
survives of this entry is the `(2, 0, 0)` anchor and the `step`/grid sizes the next case reuses.

**Reuse.** Cite before any P1 run: it fixes the case, rules out a two-unicycle reading of P1, and
gives the first grid a falsifiable outside point.

**Links.** `proposals/02-nonlinear-games.md`, `README.md` (the ladder).
