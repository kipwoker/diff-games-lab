# 06-three-d-adaptive-allowance — 3D's allowance is the polar spacing, not the 110 directions

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | octahedron `|x|+|y|+|z| = 1`, cube `1.5` |
| resolution | analytic supports; engine's `h tan ρ + R(1/cos ρ − 1)` with local ρ; nothing reconstructed |

**Claim.** The 3D allowance is dominated by the shipped grid's polar cells, not by its count. An
adaptive table of the *same* `110` directions gives a `6.75%` allowance on the octahedron where the
`11 x 10` shape gives `17.48%`, and at `240` directions the adaptive allowance is `1.60%` — below
the `0.113155` that `example-3d`'s deficit needs to become a non-vacuous statement.

| body | table | `m` | worst ρ | allowance | of `h` |
|---|---|---|---|---|---|
| octahedron | `11 x 10` | `110` | `9.0000°` | `0.174815` | `17.48%` |
| octahedron | `16 x 15` | `240` | `6.0000°` | `0.112831` | `11.28%` |
| octahedron | `20 x 20` | `400` | `4.7368°` | `0.087426` | `8.77%` |
| octahedron | `24 x 25` | `600` | `3.9130°` | `0.071537` | `7.17%` |
| octahedron | adaptive | `110` | `3.6194°` | `0.043393` | `6.75%` |
| octahedron | adaptive | `240` | `0.9048°` | `0.012327` | `1.60%` |
| octahedron | adaptive | `600` | `0.2262°` | `0.003148` | `0.40%` |
| cube `1.5` | `11 x 10` | `110` | `9.0000°` | `0.374971` | `17.90%` |
| cube `1.5` | adaptive | `110` | `2.7398°` | `0.094499` | `5.00%` |
| cube `1.5` | adaptive | `240` | `1.1250°` | `0.037520` | `1.60%` |
| cube `1.5` | adaptive | `600` | `0.2813°` | `0.009812` | `0.49%` |

The prediction was that a factor `3` in ρ needs a factor `9` in directions. That is right for the
uniform route — `600` directions buy `7.17%` against `17.48%`, less than `3×` for `5.5×` the count —
and wrong about the mechanism: the adaptive route buys `4×` at *the same count*, because the
`11 x 10` construction stacks azimuths at fixed elevations and leaves `9°` cells at the poles while
wasting directions near them. `110` adaptive directions already clear `0.113155` on both bodies.

**Evidence.** `python tools/covering_bound.py` part I: `adaptive_3d` starts from the `6 x 5` table
and repeatedly splits the direction with the largest allowance towards its nearest neighbour.

**Limits.** Only the allowance is measured — the reconstruction's excess at these tables is not, and
`extent_along_3d` enumerates 3-subsets, which is infeasible at `600`. The greedy can cluster: ρ falls
to `0.2262°` at `600`, so it is minimising a local functional rather than improving the covering, and
a clustered table is not evidence of a better set. The engine's 3D table is fixed in
`engine/src/directions.rs`, so this is headroom, not a shipped change, and the supports are analytic
bodies rather than bridge blocks. Every fraction here uses half the nearest-neighbour gap; with each
cell's covering radius the shipped table's allowance is `2.93×` larger
(`06-three-d-covering-radius.md`), so these are lower bounds — and under that radius the same-count
comparison reverses too: the greedy table's radius is `39.38°` against the shipped `19.38°`
(`06-adaptive-table-under-the-covering-radius.md`).

**Reuse.** Cite before calling 3D uncertifiable: the shipped grid is the constraint, and a geodesic
or allowance-greedy table at the same count is worth `4×`. Cite with the clustering caveat.

**Links.** `06-two-neighbour-3d.md` (the `0.323` this explains), `06-direction-bound.md` (the form),
`06-adaptive-refinement.md` (the 2D comparison), `proposals/06-method-evolution.md` (E2).
