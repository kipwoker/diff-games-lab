# 06-rung-is-254-at-the-exact-band — `205` misses for all four cases and `254` passes for all four

| field | value |
|---|---|
| type | measurement, filed from the exact table |
| track | 6 |
| date | 2026-11-13 |
| case | `example-1`, `2a`, `2b`, `2c` at `m = 110, 158, 205, 254` |
| resolution | exact band `max_ψ (h_outer − h_inner)`, block 1, `width / (2R)` |

**Claim.** At the instrument that is not a sample, the `2%` rung is `254` for every 2D case: at
`205` the block-1 shares are `2.2623`, `2.2939`, `2.2841`, `2.3920%`, all over, and at `254` they
are `1.8200`, `1.6305`, `1.5667`, `1.6561%`, all under. `254` is the smallest **tested** count that
passes; nothing between `205` and `254` was run.

| case | `110` | `158` | `205` | `254` | widest at `254` |
|---|---|---|---|---|---|
| `example-1` | 4.1336 | 2.9078 | 2.2623 | **1.8200** | 1.8200 (block 1) |
| `2a` | 3.6734 | 2.5746 | 2.2939 | **1.6305** | 1.6305 (block 1) |
| `2b` | 3.6074 | 2.5092 | 2.2841 | **1.5667** | 1.5667 (block 1) |
| `2c` | 3.7259 | 2.6078 | 2.3920 | **1.6561** | **18.2008** (block 8) |

**The convention.** These are block-1 shares. The recorded bands of the earlier entries are
widest by *absolute width* (`2.8160–3.7170%` at `110`), which puts a different block in the column
as the count rises; by *share*, block 1 is the worst at every count (`4.13–4.13%` at `110`).
A rung is a per-block statement, so it is quoted at a fixed block — block 1, as in the density
table — and never as "the band".

**What is left over.** At `254` the only block still over `2%` by share is `2c`'s last,
`18.2008%`. That is the drift of `06-last-block-band-is-drift.md`: `92.6%` of its band is the
recursion's shortfall, so it is not a table cost and no count in this table moves it.

**No new prediction.** This entry files the exact table the sweep already prints; the prediction
it tests is the old `2c`-needs-`205` claim, which `06-density-scaling-on-two-c.md` and
`06-density-sweep-measured.md` already falsified at `2.3920%`.

**Limits.** Block 1 only for the rung; `254` is a tested count, not a proved minimum; the exact
band is a support gap, not a Hausdorff distance.

**Evidence.** `python tools/density_sweep.py`, second table, after the release build.

**Reuse.** Quote the rung as "`254` at block 1, `205` for none", and quote `2c`'s tail separately
as drift.

**Links.** `06-density-sweep-measured.md` (the sampled table and the `254` verdict),
`06-band-maximum-and-the-reconstruction-slack.md` (the exact instrument),
`06-last-block-band-is-drift.md` (the leftover block), `06-rung-tolerance-is-a-table-size.md`
(the projection this replaces).
