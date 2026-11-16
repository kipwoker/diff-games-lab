# 01-dual-update-is-the-deficit — the 2.3% shortfall is the dual update's primal reconstruction, and it is two-sided

| field | value |
|---|---|
| type | measurement |
| track | 1 |
| date | 2026-11-13 |
| case | `example-1` block 20, `example-2b` block 8 |
| resolution | 110 evenly spaced 2D directions unless the row says otherwise, `Step 0.1`, `T = 2` / `0.8` |

**Claim.** The shortfall is not the step and not the `co` envelope: it is unchanged at every
`Step` from `0.2` to `0.01`, is reproduced digit for digit by a numpy copy of the recursion, and
vanishes exactly when stages 2–5 of the dual update are replaced by the algebraic difference.

| quantity | value | resolution |
|---|---|---|
| printed `y` half-width, analytic 3.000 | 2.9309 | 110 dirs, 20 blocks |
| the same at `Step` 0.2 / 0.02 / 0.01 | 2.9309 / 2.9309 / 2.9309 | 10 / 100 / 200 blocks |
| numpy re-run, table as shipped | 2.930950 | 110 dirs, 20 blocks |
| numpy re-run, stages 2–5 skipped | 3.000000 | 110 dirs, 20 blocks |
| table rotated half a gap / a quarter gap | 2.964529 / 2.943652 | 110 dirs |
| table rotated 90°, a direction on the axis | 3.000000 | 110 dirs |
| quarter-gap offset, `m` 110 / 154 / 220 / 308 | 1.88 / 1.16 / 0.14 / 0.035% short | that table |
| quarter-gap offset, `m` 440 / 616 / 880 | 0.16 / 0.13 / 0.09% **over** | that table |
| `example-2b`, equal authority, analytic 0.5 | 0.514284, 2.86% over | 110 dirs, 8 blocks |

Five Step values from `0.2` down to `0.01` span 10 to 200 blocks and therefore 1 100 to 22 000
substeps, and all five print `2.9309`. A loss that does not move when the number of dual updates
grows twentyfold is not accumulating in the envelope; it is a property of the map that reaches a
fixed point immediately. The numpy copy reproduces the engine to six digits, which places the loss
in `pontryagin_difference::dual_update` and not in `convex_polygon::reconstruct`. Removing stages
2–5 — `v`, `c2`, and the hull — leaves the algebraic difference and returns the analytic `3.000000`
exactly, so those three stages are the entire source.

The magnitude is a property of the table's alignment, not of `m`. Rotating the same 110 directions
by half a gap halves the loss; putting a direction exactly on the axis removes it. The density
sweep is not monotone: it falls to 0.035% at `m = 308` and then the `0.01` reconstruction slack
takes over and flips the sign, which is why the same method prints 2.3% under here and 2.86% over
on `example-2b`. A gap-only error law cannot fit this: it has to be evaluated on the table in use.

**Evidence.** `python tools/trace_deficit.py` — part A drives the engine, parts B–D emulate the
recursion in numpy and print the table above. The emulation is validated by reproducing the tracked
`2.930950` before it is asked anything else.

**Sign.** The shortfall is not one-signed: the same body reads over at its facet normals and
short at its vertex normals, so neither inclusion holds (`01-deficit-is-chamfering.md`).

**Limits.** `A = 0`, `2D`, and `L1`-type sets only; nothing here is a 3D or moving-drift statement.
The `0.01` slack contaminates every row at `m ≥ 308` and `example-2b` entirely — on a `0.5` set it
is a 2% floor — so those rows measure the slack as much as the sampling. No bound is derived here.

**Reuse.** Cite before any one-sided certificate, and before fitting an error law in `m`: the
deficit's sign is an accident of alignment with the set's facets.

**Links.** `01-example1-deficit.md` (the observation this explains),
`proposals/01-linear-bridges.md` (E6), `proposals/06-method-evolution.md` (E1).
