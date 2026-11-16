# 06-adaptive-refinement — bisecting the widest gap cuts the 2D excess fourfold at matched count

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | square half-width `0.5`, parallelogram half-width `3` rotated `17°` |
| resolution | `110` and `220` directions; exact supports; worst over a `90°` turn of the facet normal |

**Claim.** Placing directions by repeatedly bisecting the widest angular gap reduces the measured
reconstruction excess at a matched direction count — `0.05369 → 0.01251` on the rotated
parallelogram (`4.3×`) at `110`, `0.04073 → 0.01193` (`3.4×`) at `220` — and removes the square's
on-axis error entirely at `110` (`0.00556 → 0.00000`), because it puts a direction where the uniform
table has a gap.

| body | `m` | uniform | adaptive | ratio |
|---|---|---|---|---|
| square `0.5` | `110` | `0.00556` | `0.00000` | adaptive below `1e-5` |
| square `0.5` | `220` | `0.00000` | `0.00000` | both exact (`90°` sampled) |
| parallelogram `3` | `110` | `0.05369` | `0.01251` | `4.3×` |
| parallelogram `3` | `220` | `0.04073` | `0.01193` | `3.4×` |

The square row is the mechanism rather than a victory: a `90°` axis is unsampled at `110`, and
bisection lands on it. The parallelogram is the honest row, because its facet normal at `17°` is
never landed on exactly by bisection from `{0°, 180°}` — the adaptive table stalls at `0.0119`,
which is the residual of a normals-blind criterion. So the gain is real but bounded by what the
criterion can see: equal spacing is beaten at equal cost, and a curvature criterion should beat
bisection, which is untested.

**Evidence.** `python tools/covering_bound.py` part H: `adaptive_2d(m)` bisects the widest gap, and
`worst_excess` recomputes the reconstruction's extent over a turn with exact supports.

**Limits.** 2D only, exact supports, one query family (the facet normal swept `90°`), and a greedy
criterion that knows nothing about the body. Matched `m` is not matched cost: the adaptive table's
reconstruction has the same vertex count here but the vertex structure differs. No claim is made for
3D, where the criterion needs a spherical subdivision and the covering radius is `19.8°`.

**Reuse.** Cite before adding directions uniformly at a matched budget, and as the starting point
for a curvature criterion.

**Links.** `06-two-neighbour-bound.md`, `06-direction-bound.md`,
`proposals/06-method-evolution.md` (E2).
