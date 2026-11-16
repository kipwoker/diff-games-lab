# 06-adaptive-table-under-the-covering-radius — the greedy table's radius is worse than the shipped one

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | octahedron `max|x|`, diamond `1.5·Σ|x|`, as in the earlier allowance table |
| resolution | covering radius by triple circumcenters; the tool's `adaptive_3d` tables |

**Claim.** Under the cell covering radius the allowance-greedy table is *worse* than the shipped
grid: its radius stays at `39.38°` however many directions are added, because the greedy splits an
existing direction towards its nearest neighbour and a hole contains no direction to split. The
shipped `11 × 10` table's radius is `19.38°`, so the adaptive allowance is `1.129666` against
`0.408749` — `118%` of the support against `41%`.

| body | table | `m` | half gap | cell | `e` half | `e` cell | of `h` |
|---|---|---|---|---|---|---|---|
| octahedron | `11x10` | 110 | `9.0000` | `19.3787` | `0.174815` | `0.408749` | `40.87%` |
| octahedron | `16x15` | 240 | `6.0000` | `13.2754` | `0.112831` | `0.273144` | `27.46%` |
| octahedron | `ad110` | 110 | `3.6194` | `39.3803` | `0.043393` | `1.129666` | `118.24%` |
| octahedron | `ad240` | 240 | `0.9048` | `39.6458` | `0.012327` | `1.142834` | `120.16%` |
| diamond `1.5` | `11x10` | 110 | `9.0000` | `19.3787` | `0.374971` | `1.114103` | `43.24%` |
| diamond `1.5` | `16x15` | 240 | `6.0000` | `13.2754` | `0.263875` | `0.697064` | `26.86%` |
| diamond `1.5` | `ad110` | 110 | `2.7398` | `38.5124` | `0.094499` | `3.032838` | `117.75%` |
| diamond `1.5` | `ad240` | 240 | `1.1250` | `38.5825` | `0.037520` | `3.047945` | `118.34%` |

The `39.38°` is the radius of the *initial* `6 × 5` table the greedy starts from. Adding
`dirs[i] + dirs[j]` where the half-gap allowance is largest can only refine clusters, so the radius
of a cell centred on a hole never moves: the half gap falls to `0.9048°` while the covering radius
falls by `0.27°` in total. The `4×` at matched count from `06-three-d-adaptive-allowance.md` was a
property of the metric that selected the splits, not of the table it produced.

Predicted before the run: the adaptive advantage would shrink but survive, and land under P1's
`11.5%` gate. Measured: it reverses, and *no* table here meets the gate — `40.87%` shipped,
`27.46%` at the uniform `240`, `118%` adaptive. The uniform route is closest and still `2.4×` over.

So the route to P1's gate is a table built for covering — max-min or geodesic — not an allowance
greedy, and "adaptive is `4×` better at the same count" is retracted rather than qualified.

**Evidence.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as cb;
cb.part_j()"`. The radius implementation is checked against the engine's: the `11 × 10` table gives
`19.3787°` here and in `--bracket`, the same number from two independent codes.

**Limits.** Two analytic bodies and no bridge block. The greedy is the tool's, not re-optimised for
the new metric — which is the point, since it cannot add a direction where there is none to split.
The `240` enumeration is `2.3M` circumcenters per table, so `600` was not run. The one-term form is
still unproven, so the percentages are allowances and not bounds.

**Reuse.** Cite before calling any table adaptive or better: the covering radius is the metric, and
an allowance greedy does not improve it.

**Links.** `06-three-d-adaptive-allowance.md` (the claim this reverses),
`06-three-d-covering-radius.md` (the correction), `02-p1-step-refinement.md` (the `11.5%` gate).
