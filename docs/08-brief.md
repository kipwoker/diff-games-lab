# 08-brief — intermediate results: what improved, what might be new, what is not

Status of 2026-11-16. Tracks 1–7; track 6 (instruments) is the binder, and this brief is a reading of
it rather than a new claim. Every figure is from a `kb/entries/` file, and `kb/INDEX.md` carries the
live numbers.

## Verdict in one paragraph

One reusable method improvement — a two-sided bracket built from the same table that is `10.8×`
tighter than the inscribed ball and measured *inside* the body on every probe. Two algorithmic correctness results — crediting ties makes the covering-radius enumeration exact instead of a strict lower bound, and crediting both poles of a degenerate triple lets it see a cell vertex that only degenerate triples define. One structural criterion — the 3D one-term violation is decided by the table's alignment
with the body's facet normals, not by its density. Four measured negative results that saved real
work (the inner gap is not a chamfer term, the triple's maximum costs `2.3×`, adaptive tables are
worse, the pair form is 2D-only).**No new theorem.** Two items were tagged as possible novelty and the prior-art check retired both
(`kb/entries/06-prior-art-retires-the-two-candidates.md`).

## 1. Method: the two-sided bracket

The old inner side was the inscribed ball, because nothing said where the true body's boundary was
between samples. Shifting every sampled half-space inward by its own allowance,
$\langle \psi_i, x\rangle \le c_i - e_i$, gives an inner set that is a real polytope, not a ball.

| case | inscribed ball | shifted intersection | ratio |
|---|---|---|---|
| `example-1` | `1.871` | `0.173854` | `10.8×` |
| `example-2a` | `0.816` | `0.074966` | `10.9×` |
| `example-2b` | `0.436` | `0.040437` | `10.8×` |
| `example-2c` | `0.394` | `0.036372` | `10.8×` |

Widest bracket `30.3–42.4%` of the diameter → `2.83%` of the outer radius at `110` directions
(`1.42%` at `220`). The 2D band of record is `2.8160%` of the diameter, `3.7170%` once the
recursion's own shortfall is folded onto the inner side.

Why it matters: every later track-6 figure is now a band with a certified inner side rather than an
outer estimate with a guess behind it. Honest caveat: the inclusion was measured on 24 probes, and
the thin margin (`0.000268` at `220` on the square) means it is measured, not proved — the entry says
so and asks for a re-run before a denser table is called two-sided.

## 2. Correctness results: credit the ties and both poles

The 3D radius enumeration returned the point equidistant from its three generators, then treated the
resulting equality of inner products as a *degenerate tie* and skipped it. Every genuine Voronoi
vertex is exactly such a point, so the enumeration discarded all of them and kept only non-Delaunay
circumcenters — each still a point of some cell, hence no larger than the truth. `ρ` was a strict
lower bound by construction.

| quantity | tie skipped | ties credited |
|---|---|---|
| `ρ` max, `11 x 10` | `19.3787°` | `19.8733°` |
| `ρ` max, `16 x 15` | `13.2754°` | `13.3967°` |
| cells that grew | — | `100` of `100` |
| largest single-cell gain | — | `+1.5525°` |

The `16 x 15` value is exactly the probe that had falsified the old radius, so the missing vertex was
the one the sweep found. The fix also exposed a sign bug: both implementations normalise the
circumcenter by the sign of $\langle a + b + c, n\rangle$, which is `0` when that sum is orthogonal to the normal,
and multiplying by `0` returns the zero vector — a point equidistant from everything, read as
`ρ = 90°` with `tan 90°` in the allowance. The test that pins it: a regular tetrahedron's cells meet
at the antipode of the fourth sample, so `ρ = acos 1/3 = 70.5288°` against a half gap of `54.7356°`.

**Prior art: searched, and it is textbook.** Spherical Voronoi vertices are the facet normals of the
generators' convex hull (Fortune; Renka's `STRIPACK`; SciPy's `SphericalVoronoi`), so crediting ties
applies a known identity. Our contribution is finding that the tie test discarded every vertex — a
defect with a one-assertion test, not a theorem.

## 3. Structural criterion: alignment, not density

Seven `az × el` tables on the cube at `2525` probes:

| table | n | `ρ` | violations | worst `excess/local` |
|---|---|---|---|---|
| `11 x 10` | 110 | 19.8733° | 0 | 0.694 |
| `13 x 12` | 156 | 16.6227° | 0 (exact, `6.3e-14`) | 0.000 |
| `14 x 13` | 182 | 15.4502° | **1** | 1.016 |
| `15 x 14` | 210 | 14.2808° | 0 | 0.765 |
| `16 x 15` | 240 | 13.3967° | **1** | 1.046 |
| `18 x 12` | 216 | 15.8291° | 0 | 0.246 |
| `20 x 12` | 240 | 15.6672° | 0 | 0.223 |

`13 x 12` is exact because twelve elevations give `0°` and `90°` and thirteen azimuths give
`0`/`±90`, so $\pm e_i$ are sampled and the six facet caps alone pin the cube. The sufficiency of that
is trivial to prove; the useful part is the measurement that **density without alignment still leaves
`1.046` at 240 directions**, while alignment alone is exact to `6e-14`. That is why `16 x 15` halves
the band ($1.999277 \to 1.307773$) and the label still reads `estimate`.

**Prior art: searched, and the sufficiency is the definition.** A polytope *is* the intersection of
its facet half-spaces, and tangent-half-space outer approximation with Hausdorff bounds is standard
(Dudley 1974; Bronshtein–Ivanov 1976; Kamenev's optimal direction count; `LazySets` implements the
constraint form). The alignment result is a benchmark, not a discovery.

## 4. Measurements that changed decisions

| question asked | answer | number | what it changed |
|---|---|---|---|
| is the inner gap a chamfer term? | no | $\text{gap}/\tan \rho$ = `0.38–1.55`, spread `4×`, rises at `254` | the repair was abandoned; a published claim was corrected |
| is the triple worth charging? | no | sound, `84–89%` coverage, tight only at samples; `max` widens the band `2.3×` | band keeps the one-term form |
| does density buy the `2%` rung? | `254` at block 1 | `205` misses all four (`2.26–2.39%`), `254` passes all four (`1.57–1.82%`) | the tolerance is a table size |
| is adaptive better? | no | greedy table `39.38°` vs shipped `19.38°`; bisection at matched count cuts excess `4.3×` | refine within the shipped table |
| is the 3D band shape-fixed? | no | $1.999277 \to 1.307773$ at `16 x 15` | shape is a parameter |
| can P1 be certified? | not yet | margin `0.150`, not `0.025`; shipped allowance `44.65%` of support at the cell radius; the adaptive table is worse at `118%` | `Step` first, then a bridge; no measured table under `13.04%` (`d`) or `4.77%` (angle), nearest `2.1×` over |

The P1 row is the one that reshaped a track: the earlier `0.025` margin was a grid artefact, and with
the corrected `0.150` the table is no longer the dominant error — the missing nonlinear bridge is.

## 5. Engineering improvements

- `tools/exact_polygon.py` — generalised from one hard-coded case to any case file: reads
  `M/P/Q`, `A/B/C/pi`, `time`, `step`; applies `W ← (W + BᵀGᵀP) − (CᵀGᵀQ)` with $G = \pi \cdot \mathrm{Ie}\cdot E$;
  `expm`/`integral_expm` by series; reproduces committed extents to `2.0e-10`, `2.0e-12` and exactly.
  Vectorised `clip` plus `1e-9` pruning made `2a` tractable at `1764` facets (~`97 s` of a `100 s`
  run); `simplify` keeps the rest at four vertices.
- `tools/inner_between_samples.py` — `support_violation` (largest facet-normal excess) replaced a
  sign-inverted inside test. That bug produced the retracted `0.0109–0.0256` "violation" claim; the
  real violation is `4.8e-4` at `205` only.
- `tools/covering_bound.py` — `table_3d`, seeded `probe_directions`, chunked all-triple
  reconstruction, `covering_allowance`, `circumradius`, `triple_vertex_bound`, and parts M/N.
- `engine/` — `triple_bound` with the `Nᵀλ = ψ` cone solve, `nearest_to`, the one-term `allowance`;
  `67` tests; the five committed outputs re-run byte-identical after every change.
- Process — predictions written into tool docstrings *before* the run; `22` misses logged in
  `docs/07-falsifications.md` and classified into `6` patterns, the instrument patterns the largest
  of them.

## 6. What is not new

Covering radius, support-function duality, tangent-cap outer approximations, interval brackets,
Voronoi/Delaunay on the sphere — all classical, and the bracket is standard interval arithmetic
applied to a support function. Nothing here is a theorem: the two candidates (tie-credit exactness and the alignment criterion) both
collapsed to textbook identities in the search, which is why the usable asset is the measurement
record rather than a novelty claim (the `Decisions` table makes prior art a condition of being a
publication candidate). Everything is
polytopes and a cube; nothing transfers to the nonlinear $\dot{x} = f(x, u, v)$ world yet.

## 7. What to do next

1. A facet-normal-aware table, then re-check P1's gate — the alignment result says this is the cheap
   route to a sound 3D band, not more directions.
2. A per-cell allowance under `13.04%` (`4.77%` on the angle axis): the nearest measured table is
   `2.1×` over, so the remaining route is the form, not the count.
3. Prior-art search on the two candidates above, before any external write-up.
4. Cost curve: directions against band on the aligned family, to replace the `254`-style test with a
   prediction.
