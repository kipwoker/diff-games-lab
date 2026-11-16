# 06-rung-tolerance-is-a-table-size — `2%` of diameter is `m ≥ 158` in 2D and `3.2·10⁴` in 3D

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | the five tracked bands of the bracket entry; the rung's tolerance from the proposal |
| resolution | arithmetic on measured bands; `ρ = π/m` in 2D, the radius table in 3D |

**Claim.** A band is `f · tan ρ` of the diameter, with `f` measured between `0.85` and `1.30`, so the
rung's `2%` requires `ρ ≤ 1.146°`: `m ≥ 158` in 2D, where `ρ = π/m` exactly, and about `3.2·10⁴`
directions in 3D. At the shipped 110 the *outward half alone* is `1.43%` of the diameter and the band
`2.82%`, so no 2D case can meet `2%` at that table — the tolerance is a table-size requirement, and
every 2D case misses it by `1.4×` to `1.9×` because the table has `1.4×` too few directions.

| case | band, of diameter | `tan ρ` | band / `tan ρ` |
|---|---|---|---|
| `example-1` | `2.8160%` | `2.8564%` | `0.99` |
| `example-2a` | `3.0090%` | `2.8564%` | `1.05` |
| `example-2b` | `3.6259%` | `2.8564%` | `1.27` |
| `example-2c` | `3.7170%` | `2.8564%` | `1.30` |
| `example-3d` | `29.9505%` | `35.166%` at `ρ = 19.3787°` | `0.85` |

| target, of diameter | `ρ` | `m` 2D | `m` 3D |
|---|---|---|---|
| `2%` at `f = 1` | `1.146°` | `158` | `3.2·10⁴` |
| `2%` at the widest `f = 1.30` | `0.881°` | `205` | `5.4·10⁴` |
| `2%` at the tightest `f = 0.85` | `1.348°` | `134` | `2.3·10⁴` |

The 3D column uses `ρ ≈ 3.589 rad·√m` fitted on the two largest tables, and drifts at `10³`-fold
sizes but not by a factor; the substep is `O(m²)`, so `3.2·10⁴` directions is `8.5·10⁴×` the work of
110 — the 3D rung is a different method, not a bigger table.

**Restatement.** Quote the band as `band / tan ρ` and the five cases read `0.85` to `1.30`, which is
dimension-free, `m`-free and survives a change of table. Quote it as a percentage of diameter and the
number is a statement about the table, not the method — which is why the same success criterion called
`2.8%` acceptable on `example-1` and `3.7%` a failure on `2c`.

**Corrected by measurement.** `158` misses `2%` at `f = 1` too: `example-1` prints `2.0084%` there,
and `2c` prints `2.3920%` at `205`, so the widest-`f` row is `254` (`06-density-sweep-measured.md`).

**Corrected.** The exact radius gives `3d`'s band `30.5916%` against `tan 19.8733° = 36.145%`, so
`f = 0.846` and the spread `0.846–1.30`; the `m` columns move under `1%`, so `158` and `3.2·10⁴` stand.

**Limits.** `f` rests on five cases, three sharing `ρ = 1.6364°`, and on one 3D case at one radius, so
the `3D` column is an extrapolation in `m`. The 2D table is fixed at 110 in `directions::build`, so
"run it at 158" needs the builder changed — the restatement is the actionable outcome.

**Evidence.** `06-bracket-folds-inward.md` (the five bands), `06-three-d-covering-radius.md` (`ρ` in
3D), `06-direction-bound.md` (`ρ = π/110 = 1.6364°` in 2D), the radius table of
`06-two-neighbour-bound.md` for the 3D scaling.

**Reuse.** Cite before calling a band "out of tolerance": quote `band / tan ρ` unless the table size
travels with the number.

**Links.** `06-bracket-folds-inward.md` (the bands), `06-bracket-and-cost.md` (the tolerance this
replaces), `proposals/06-method-evolution.md` (the rung's `2%`), `02-p1-step-refinement.md` (the
`11.5%` P1 gate, which is a different number for a different question).
