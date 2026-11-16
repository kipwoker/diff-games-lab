# 06-three-d-radius-is-exact — the enumeration discarded every Voronoi vertex

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | the shipped 3D `11 x 10` table, `16 x 15`, and `example-3d` |
| resolution | exhaustive triple enumeration with ties credited; `200 000` probes; a tetrahedron by hand |

**Claim.** `spherical_circumcenter` returns the point equidistant from its three generators, so the
three inner products with that centre always agree. The enumeration read that agreement as a
degenerate tie and skipped the centre — which discarded *every* genuine Voronoi vertex, keeping only
circumcenters of non-Delaunay triples. Each of those is still a point of some cell, hence no larger
than the truth, so the reported `ρ` was a strict lower bound by construction. Crediting every sample
tied at the maximum distance closes it.

| quantity | tie skipped | ties credited |
|---|---|---|
| `ρ` max, `11 x 10`, 100 distinct sites | `19.3787°` | `19.8733°` |
| `ρ` max, `16 x 15`, 240 sites | `13.2754°` | `13.3967°` |
| cells that grew | — | `100` of `100` |
| largest single-cell gain | — | `+1.5525°` |
| `example-3d` covering radius in the header | `19.3787°` | `19.8733°` |
| `example-3d` widest bracket | `1.939462`, `29.9505%` | `1.999277`, `30.5916%` |

The `16 x 15` figure is the test: `13.3967°` is exactly the probe that had falsified the old radius,
so the missing vertex was the one the sweep found. No cell lost radius, and the allowance grows by
`2.55%` at `110` and `0.9%` at `240`.

Predicted before the change: `ρ` lands in `19.7–20.0°` and `13.38–13.45°`, with the four 2D widths
unmoved to the last digit. All three held; the five committed outputs re-run byte-identical and the
2D table's radius is untouched, because an arc cell's circumradius is half its gap.

**A second defect the fix exposed.** Both implementations normalise the circumcenter by the sign of
`⟨a + b + c, n⟩`. When that sum is orthogonal to the normal the sign is `0`, and multiplying by it
returns the *zero* vector — a point equidistant from every sample, which reads as `ρ = 90°` and puts
`tan 90°` into the allowance. The tie-skip had masked it, since a zero centre ties everywhere and was
therefore dropped. A zero sign is now taken as `+1`.

**Limits.** Degenerate triples (plane through the origin) have two poles and both are now credited
(`06-degenerate-vertices-need-the-antipode.md`); a tie is detected at `1e-12` in the inner product, so samples closer than `8·10⁻⁵°` merge. Exactness is
therefore up to floating point, not in principle. The pass costs twice the dot products of before and
is still `O(m³)`, which is why the radius remains the instrument that breaks first at larger tables.

**Evidence.** `engine/tests/error.rs::a_tetrahedron_has_a_known_covering_radius` — a regular
tetrahedron's cells meet at the antipode of the fourth sample, so the radius is `acos 1/3 = 70.5288°`
against a half gap of `54.7356°`; the old code returned the half gap. `::the_three_d_radius_dominates_sampled_probes`
sweeps `200 000` deterministic probes and asserts the radius dominates them while staying within a
degree of the sampled worst. `tools/covering_bound.py` parts E and K reproduce `19.8733°` and
`13.3967°` to four decimals.

**Reuse.** Quote `ρ` from this change onward; earlier figures are lower bounds and are noted as such.

**Links.** `06-three-d-radius-is-a-lower-bound.md` (the defect this settles),
`06-three-d-covering-radius.md` (the figure it corrects), `06-bracket-folds-inward.md` (`3d`'s band),
`06-three-d-allowance-is-loose.md` (the ratios this shifts), `06-rung-tolerance-is-a-table-size.md`
(`2%` in units of `tan ρ`).
