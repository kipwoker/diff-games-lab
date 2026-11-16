# 01-example1-deficit — a symmetric bridge is printed 2.3% short, so the sampler is not outer

| field | value |
|---|---|
| type | counterexample |
| track | 1 |
| date | 2026-11-13 |
| case | `example-1`, block 20 |
| resolution | 110 directions, `Step 0.1`, `T = 2` |

**Claim.** `example-1` is invariant under a 90° rotation — diamond target of radius 1, diamond
controls of radius 2 and 1, `A = 0` — so its bridge at `T` must have equal `x` and `y`
half-widths, and the alternating sum gives the diamond of radius `1 + 2·2 − 2·1 = 3`. The tracked
output is `±3.000` in `x` and `±2.931` in `y`: the printed set lies **inside** the analytic
answer by `0.069`, or `2.3%`.

| quantity | analytic | tracked | difference |
|---|---|---|---|
| x half-width | 3.000 | 3.000 | 0.000 |
| y half-width | 3.000 | 2.931 | −0.069 (2.3%) |
| vertices | — | 498 | — |

`x` is sampled exactly (direction `i = 0` is `(1, 0)`); no direction is normal to `y` — the
nearest is `1.636°` away — but that predicts an **overshoot** of order `0.04%`, not an undershoot
of `2.3%`. So sampling geometry cannot explain the sign.

**Evidence.** `python tools/direction_table_check.py`, last section, which parses the tracked
block and prints the deficit; the symmetry is visible in `examples/example-1.txt`.

**Limits.** One case. The mechanism is not identified here: see the follow-up. The prime suspect was
`co`, the sublinear envelope; the tracked evidence retired it — the deficit does not move with
`Step`, so it is not accumulating per block.

**Resolved by.** `01-dual-update-is-the-deficit.md`: stages 2–5 of the dual update, alignment
dependent, and two-sided.

**Reuse.** Cite whenever a claim rests on the sampled set being an over-approximation. It is not
established; this case is 2.3% on the wrong side, which makes every one-sided certificate
(tracks 4, 5, 7) unsound until the sign is bounded.

**Links.** `proposals/01-linear-bridges.md` E6, `proposals/06-method-evolution.md` E4.
