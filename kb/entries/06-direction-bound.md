# 06-direction-bound — the covering-radius bound predicts the direction floor and never sits below it

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | `example-3d` block 4, `example-2b` block 8, two blind 2D cases |
| resolution | 110 directions except where a row says another count |

**Claim.** With `ρ = 1.6364°` in 2D and `ρ = 19.84°` for the shipped 3D table, the excess of the
sampled reconstruction over the true set is at most `h·tan ρ + R·(1/cos ρ − 1)` per direction: the
bound sits 0.1% above the `0.514284` square and 11% above the `1.9874` cube, and above both blind
cases, so `R δ²/8` is replaced by a one-sided bound that predicts the number instead of fitting it.

| quantity | value | resolution |
|---|---|---|
| `ρ` 2D | `1.6364° = π/110` | evenly spaced table |
| `ρ` 3D | `19.8428°`, sampled | the 11 × 10 table |
| square `0.5`: `h`, `R` | `0.5`, `0.7071` | at the unsampled `y` axis |
| bound / tracked | `0.5146` / `0.514284` — 0.1% over | 110 dirs |
| cube `1.5`: `h`, `R` | `1.5`, `2.5981` | at the unsampled `z` axis |
| bound / tracked | `2.2053` / `1.9874` — 11% over | 110 dirs |
| diamond `3.0`: bound / reconstruction | `3.0874` / `3.0000` — 2.9% over | vertex-exact case |
| blind box, `M=1, P=3, Q=1, T=2` | predicted `y ≤ 5.1457`, measured `5.1428` | engine, 110 dirs |
| blind diamond, same case | predicted `y ≤ 5.1457`, measured `4.8769` | engine, 110 dirs |
| square, `m` 110 → 374, axis unsampled | bound 0.1% → 0.0% above measured | exact supports |
| `R δ²/8` on the two tracked cases | `0.0006`, `0.0002` | retired |

The first term is the flat-facet drift: a facet whose normal is `ρ` away from `ψ` lets the
intersection slide by `h tan ρ`. On a box that term is exact — `0.5 tan 1.6364° = 0.0142839`
against a measured `0.014284`, five decimals — and across the `m` sweep it tracks the measured
excess to within 0.1% at every count. The second term is curvature insurance for smooth bodies; on
the cube the first term alone already covers the measurement (`0.5414` against `0.4874`), so it is
unexercised here and is carried rather than relied on. The diamond is the case where the first term
never acts: its support is attained at a vertex for a whole cone of directions, so the
reconstruction is exact and the bound is loose by 2.9%.

Two things the bound does not do. It is a bound on the reconstruction from *exact* supports, so it
says nothing about the engine's printed set when the stored supports have themselves drifted: the
blind diamond is printed `2.46%` inside the exact `5.0` by the dual update's own loss
(`01-dual-update-is-the-deficit.md`), which is an inward term this bound cannot cover. And `ρ` for
the 3D table is sampled, so `19.8428°` is a lower bound on the true covering radius and the bound is
optimistic by about `0.004·h` — under `0.01` on the cube.

**Evidence.** `python tools/covering_bound.py`: part A the tracked cases, part B the two blind runs
(prediction printed before the run), part C the `m` sweep with the axes never sampled.

**Limits.** One 3D case, two 2D families, no moving drift. The first term is a conjecture — it is
derived for a polytope facet and validated on boxes and diamonds, not proved for general polytopes.
The bound is outer only; it may not be used as `e_in`.

**Superseded in scope.** Symmetric spacing only, see `06-two-neighbour-bound.md` (`0.021` above it).

**Reuse.** The `e_out` allowance for tracks 4, 5 and 7, and the extent track 1 quotes, after
checking the neighbour spacing or using the two-neighbour form.

**Links.** `proposals/06-method-evolution.md` (E1),
`06-direction-table-explains-extents.md`, `01-dual-update-is-the-deficit.md`.
