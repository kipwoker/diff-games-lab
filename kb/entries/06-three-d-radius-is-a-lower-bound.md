# 06-three-d-radius-is-a-lower-bound — the covering radius has ten zero cells and is not exact

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | the shipped 3D `11 x 10` table; `16 x 15` as the control |
| resolution | exhaustive triple enumeration; the reconstruction sweep of the allowance measurement |

**Claim.** The `ρ` in the allowance is not a covering radius. The shipped 3D table has 110 entries
but 100 distinct directions — the `±90°` azimuth stacks coincide — and the radius pass gave each
repeated site a nearest gap of zero, so ten cells carried `ρ = 0` and an allowance of exactly zero.
Merging the repeats closes that; a second defect remains: the enumerated maximum is a lower bound,
because the worst probe on the cube at `16 x 15` is `13.3967°` from its nearest sample while the
largest cell radius reported is `13.2754°`.

| quantity | before | after merging repeats |
|---|---|---|
| distinct directions | `100` of `110` entries | unchanged |
| cells with `ρ = 0` | `10` | `0` |
| allowance at the cube's `z` facet cell | `0.000000` | `0.625896` |
| scalar `max_allowance`, octahedron `11 x 10` | `0.408749` | `0.408749` |
| scalar `max_allowance` on `3d`'s cells | `0.945986` | `0.945986` |

Why the repeats matter: a repeated site has no separating bisector, so its cell is the cell it
repeats, and the four sites nearest a cube's `z` axis are two such pairs — the `z` facet normal was
the widest hole in the table. No scalar maximum came from a repeated cell, which is why every
certificate that subtracts one number was unaffected and the defect stayed invisible.

Why the maximum is not exact: the enumeration skipped triples whose plane contains the origin, and
its tie test dropped every genuine vertex, so `ρ` was a lower bound — `0.12°` on `16 x 15` was a
lower bound on the error, not a measurement of it. Closed below.

The consequence is one-sided and agrees with the independent measurements: the one-term form is
false by `0.021` in 2D and `0.0087` in 3D (`06-two-neighbour-bound.md`). The `estimate` label on 3D
was right, and it is now a measurement rather than a caution.

Predicted before the run: the cube's printed zero was a numerical accident of the probe.
Measured: the zero was structural and ten cells wide.

**Limits.** The 2D table repeats nothing and its cells are arcs, so this is 3D only. Merging
restores the cell without making the enumeration exact. The five `--bracket` widths are unchanged
after the fix (`0.173854`, `0.079583`, `0.050714`, `0.047131`, `1.939462`) because no widest
direction is a repeated cell — the per-direction shift was still a hole on ten of them.

**Superseded.** Repeats merge and the dropped tie was *every* Voronoi vertex, so `ρ` is exact to floating
point: `19.8733°` at `11 x 10`, `13.3967°` at `16 x 15` (the probe quoted above), and `example-3d`'s
width moves to `1.999277`, so "five widths unchanged" holds in 2D only — `06-three-d-radius-is-exact.md`.

**Evidence.** `engine/tests/error.rs::duplicated_directions_share_a_cell` — the two codes now agree
on the deduplicated table and every cell has a radius; part K's falsified-probe count falls from
`145` to `1` when the merge is applied.

**Reuse.** Cite before using any per-direction 3D allowance, and before calling `ρ` exact.

**Links.** `06-three-d-covering-radius.md` (the `ρ` this corrects),
`06-three-d-allowance-is-loose.md` (the measurement that exposed it), `06-two-neighbour-bound.md`,
`proposals/06-method-evolution.md` (E1, E4).
