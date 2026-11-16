# 06-prior-art-retires-the-two-candidates — both "maybe new" items are textbook, and one was a bug

| field | value |
|---|---|
| type | prior-art check |
| track | 6 |
| date | 2026-11-13 |
| case | the two candidates tagged in `docs/08-brief.md`, four searches |
| resolution | web search, snippets and abstracts only; no paper read in full |

**Claim.** Neither candidate is new. Spherical Voronoi vertices are the facet normals of the
generators' convex hull — stated as the standard construction by Fortune, Renka's `STRIPACK` and
SciPy's `SphericalVoronoi` — so crediting the ties is applying a known identity, not discovering one;
what is ours is noticing that the tie test discarded *every* vertex, which is a defect with a
one-assertion test. A polytope *is* the intersection of its facet half-spaces, so sampling the facet
normals reconstructs it exactly; tangent-half-space outer approximation with Hausdorff bounds is the
standard method (Dudley 1974, Bronshtein–Ivanov 1976, Kamenev's optimal direction count, implemented
by `LazySets`). The alignment measurement is therefore a benchmark, not a discovery.

| candidate | closest prior art | verdict |
|---|---|---|
| tie-credit exactness | spherical Voronoi = convex hull of the sites; facet normals = Voronoi vertices (Fortune; Renka `STRIPACK`; SciPy `SphericalVoronoi`) | textbook identity; our version was a bug |
| alignment criterion | `K` is the intersection of its facet half-spaces; outer approximation by tangent half-spaces with Hausdorff error (Dudley; Bronshtein–Ivanov; Kamenev) | trivial sufficiency; benchmark only |
| two-sided bracket | support-function sampling and interval/Hausdorff error bounds (`LazySets` polyhedral approximations; Kamenev) | standard interval arithmetic on a support function |

**What survives.** The measured numbers, not their novelty: the `2.83%` bracket at `110` directions,
the `27–45%` allowance scan, the `1.046` violation at 240 directions, and the `17` recorded misses.
That is a measurement record that someone else can re-run, which is a different asset from a theorem
and should be published as such.

**Prediction.** Written before the searches: at least the tie-credit item would not survive, because
the Voronoi/Delaunay equivalence is standard, while the alignment criterion might. Falsified on the
second half — the alignment sufficiency is the definition of a polytope in support-function form.

**Consequence.** No external write-up may use "new" for either item, and `docs/08-brief.md`'s two
"prior art: unchecked" tags now read as searched. The `Decisions` table already made a prior-art
check a condition of being a publication candidate, so this retires the candidates rather than the
results.

**Limits.** Four English queries, snippets and abstracts, no full-text read and no citation chasing;
absence of a hit is not proof of absence. The image/video literature and non-English work were not
searched, and "spherical covering radius" has a large coding-theory literature not touched here.

**Evidence.** Queries: "exact covering radius finite set of points on the sphere Voronoi vertices
Delaunay algorithm", "covering radius of a direction set support function samples inner and outer
approximation convex body bracket", "Bronshteyn Ivanov approximation of convex bodies by support
function samples error estimate", "tangent halfspace intersection outer approximation error bound
covering radius tan rho geometry".

**Links.** `docs/08-brief.md` (the tags this changes), `06-three-d-radius-is-exact.md` (the defect),
`06-three-d-violation-is-alignment-not-density.md` (the benchmark), `docs/07-falsifications.md`.
