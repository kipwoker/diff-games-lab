# 06-three-d-covering-radius — 3D's `ρ` was half a gap, and the allowance was 2.9× optimistic

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | the shipped 3D `11 × 10` table; `example-3d`; the shipped 2D table as the control |
| resolution | analytic tables; exhaustive triple enumeration for the Voronoi vertices |

**Claim.** `ρ` in the one-term allowance is the covering radius of a direction's cell — the largest
angle from it to any point of its Voronoi cell. In 2D that equals half the nearest-neighbour gap; in
3D it does not. The shipped 3D table's cells reach `19.3787°` against a `9°` half gap, so its
allowance was `0.322693` where it should be `0.945986` — `2.93×` understated — and the widest 3D
band widens from `14.82%` to `29.22%` of the diameter.

| table | half gap (max) | covering radius (max) | allowance on `3d` | band, of diameter |
|---|---|---|---|---|
| 2D shipped | `1.6364°` | `1.6364°` | `0.086927` (unchanged) | `2.83%` (unchanged) |
| 3D shipped `11 × 10` | `9.0000°` | `19.3787°` | `0.322693 → 0.945986` | `14.82% → 29.22%` |

Predicted before the run: the 2D numbers come back bit-identical, because an arc cell's circumradius
*is* half its length; the 3D figure rises by a factor between `2` and `4`, with the maxima at the
poles where the `11 × 10` construction stacks azimuths and leaves long thin cells. Measured: `2.93×`
at `19.3787°`, and every one of the 110 cells is at least its half gap, most strictly wider.

The arithmetic for P1 gets worse, not better, which is the point of measuring it:

| quantity | before | after |
|---|---|---|
| `3d` allowance | `0.322693` (`12.4%` of the support) | `0.945986` (`36.3%`) |
| for a `1.3` set | `0.16` | `0.47` |
| against a `0.025` margin | `6×` too large | `19×` too large |

**Evidence.** `./target/release/stable_bridge ../examples/example-3d.txt --bracket` prints
`covering radius max 19.3787 deg` in the header and `widest 1.891973 at block 4` (`29.2171%`);
`../examples/example-1.txt --bracket` prints `1.6364 deg` and the unchanged `2.83%`.
`engine/tests/error.rs` holds the invariants: 2D equals the half gap to `1e-12`, 3D is at least it
for every cell and strictly larger for more than half.

**Limits.** This corrects `ρ`, not the form: `h tan ρ + R(1/cos ρ − 1)` is still unproven in 3D,
which is why the certificate keeps saying `estimate`. The enumeration assumes a cell is a spherical
polygon whose vertices are triple circumcenters — true for the Voronoi diagram of distinct points,
with both poles of a degenerate triple credited — and it is exhaustive over triples (`215 820` of them).

**Corrected.** The maximum is a *lower bound*, not the exact radius this entry claimed: degenerate
circumcenters are skipped, and the ten repeated directions of the `11 × 10` table were given
`ρ = 0` here. The scalar maxima above (`19.3787°`, `0.945986`) are unchanged, because no repeated
cell set them. See `06-three-d-radius-is-a-lower-bound.md`. `06-three-d-adaptive-allowance.md`'s fractions were computed with the half gap
and are therefore lower bounds; its `4×` same-count ratio is the claim that survives. Both figures
now stand at `19.8733°` and `1.999277` / `30.5916%`; `06-three-d-radius-is-exact.md`.

**Reuse.** Cite before quoting any 3D allowance figure, and as the reason a 3D table change is the
only route to P1's `0.025` rather than a nicety.

**Links.** `06-direction-bound.md` (the form), `06-three-d-adaptive-allowance.md` (the figures this
corrects), `06-two-neighbour-3d.md` (the estimate label),
`02-p1-is-not-yet-certifiable.md` (the margin).
