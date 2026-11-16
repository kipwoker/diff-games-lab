# 06-density-sweep-measured — `158` misses `2%` even at `f = 1`, and `2c` needs `254`

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-1`, `2a`, `2b`, `2c` at `m = 110, 158, 205, 254` |
| resolution | the engine at each count; `width / (2R)` from `--bracket` and the block's vertex document |

**Claim.** Measured instead of projected, the `2%` rung is not reached at `158` by any case. Under
the widest-column convention the best is `example-1` at `2.0084%`, four parts in `10⁴` over the line,
and its first-block share is `2.9073%`. `2c` prints `2.6059%` at `158`, so the `2.567%` projection
held, but `2.3920%` at `205`, not the `1.967%` predicted: the widest-`f` rung is `254` (`1.6553%`),
not `205`. At the last block every table prints `17–20%`, so the verdict is about a fixed block and
the drift is a separate problem.

| case | `110` | `158` | `205` | `254` |
|---|---|---|---|---|
| `example-1` | `4.1320%` | `2.9073%` | `2.2620%` | `1.8199%` |
| `2a` | `3.6710%` | `2.5732%` | `2.2939%` | `1.6302%` |
| `2b` | `3.6034%` | `2.5079%` | `2.2841%` | `1.5660%` |
| `2c` | `3.7230%` | `2.6059%` | `2.3920%` | `1.6553%` |

First-block band, `width / (2R)`. The widest-column figures differ, and the difference is the point:

| case | last block at `110` / `254` | widest at `110` | widest at `205` |
|---|---|---|---|
| `example-1` | `2.8976%` / `1.2496%` | `2.8976%` (block 20) | `1.5462%` |
| `2a` | `3.0612%` / `1.3726%` | `3.0612%` (block 8) | `1.7572%` |
| `2b` | `2.8732%` / `1.2372%` | `3.6034%` (block 1) | `2.2841%` |
| `2c` | `20.4142%` / `17.3445%` | `3.7230%` (block 1) | `18.5285%` (block 8) |

**Prediction.** Written before the sweep: `2c` falls `3.7 → 2.6 → 2.0` at `110 / 158 / 205`, and
`example-1` reaches `2.0%` at `158`. The first two held (`3.7230%`, `2.6059%`); the third failed by
`21%`, and `example-1` lands at `2.0084%` — over, not under, so the `f = 1` rung is `205` too.

**The convention.** The five recorded bands are the widest absolute width over blocks over that
block's `2R`; the sweep reproduces them at `110` to `0.3–3%` (`2.8160 → 2.8976`,
`3.0090 → 3.0612`, `3.6259 → 3.6034`, `3.7170 → 3.7230`), the residual being the printed `R` against
the exact one. That column moves between blocks with the count — for `2c` from block `1` to block
`8` — so a density comparison has to fix a block, which is why the first table is block 1.

**The plateau.** Between `158` and `205` the first-block band falls `9%` while `tan ρ` falls `23%`,
and all four cases land at `2.28–2.39%`; at `254` they fall to `1.57–1.82%`. Something in the band
does not follow the table, so density does not buy the rung linearly.

**Limits.** One block reported, four 2D cases, and `2c`'s last block stays at `17–20%` at every
count, so no tested table clears the rung at the end of that trajectory. The band is the engine's
`2·allowance + shortfall`; the exact polygon's `A` is a different instrument and agrees at `110` to
`0.4%`.

**Evidence.** `python tools/density_sweep.py` after `cargo build --release`.

**Reuse.** Quote `2c`'s rung as `254` at the first block, never as `205`, and always with the block.

**Links.** `06-rung-tolerance-is-a-table-size.md` (the `158`/`205` table),
`06-density-scaling-on-two-c.md` (the projection), `06-two-d-table-is-a-parameter.md` (the flag),
`06-bracket-folds-inward.md` (the widest-width convention).
