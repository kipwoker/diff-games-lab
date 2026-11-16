# 06-degenerate-vertices-need-the-antipode — one side of the great circle is half the radius

| field | value |
|---|---|
| type | method, one prediction falsified |
| track | 6 |
| date | 2026-11-16 |
| case | four equatorial directions plus the north pole; the shipped `11 x 10` and `16 x 15` tables |
| resolution | exhaustive triple enumeration, one side against both sides of each centre |

**Claim.** A triple whose plane passes through the origin has no unique circumcenter: both poles of
its great circle are equidistant from the three, and the sign convention in the enumeration kept
one. The table `{4 equator + north pole}` puts a vertex at the **south** pole, `90°` from each
equatorial site, and only degenerate triples define it, so one-sided credit read `54.7356°`; both
sides read `90.0000°` and move four cells. The shipped tables are untouched — no distinct cell of
either moves — so `19.8733°` and `13.3967°` stand.

| table | half gap | one side | both sides | cells moved |
|---|---|---|---|---|
| equator `4` + pole | `45.0000°` | `54.7356°` | `90.0000°` | 4 of 5 |
| `11 x 10`, 100 distinct | `9.0000°` | `19.8733°` | `19.8733°` | 0 of 100 |
| `16 x 15`, 240 distinct | `6.0000°` | `13.3967°` | `13.3967°` | 0 of 240 |

**Prediction.** Written before the run: one-sided `45°` against a true `90°`, and the shipped
tables unchanged. Measured `54.7356°`, not `45°` — falsified: the pole's own cell and the
non-degenerate triples that include the pole already reach `acos(1/sqrt 3)` — and the other two
held, `90.0000°` with `0` cells moved on either shipped table.

**Why both sides are sound.** Each pole is a genuine point of the sphere, and the credit assigns it
only to its nearest samples, so the distance is a genuine extent of their cells: the second side
can raise `rho` to the truth and cannot overshoot it. What it closes is specifically the
degenerate-vertex hole, not a general lower-bound gap — the one-sided figure was already a lower
bound, and on tables whose vertices are all non-degenerate the two figures coincide exactly.

**Limits.** Exactness is up to floating point, and the hole needs a table with an equatorial ring
and only one pole; a hemisphere hole of that shape is the case tested. The cost is one extra block
of dot products per centre, which is why the radius is still the instrument that breaks first at
large tables. Degenerate triples are now handled by their second side rather than skipped, but a
tie is still detected at `1e-12`.

**Evidence.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as c;
c.part_p()"` prints the three rows above; `cargo test --release --test error` includes
`a_degenerate_vertex_is_found_at_its_antipode`; the five committed outputs re-run byte-identical
after the change, because the shipped tables' radii do not move.

**Reuse.** Before quoting a 3D radius on any table with an equatorial ring and one pole, and before
calling the enumeration exact: the antipode is what makes "exact" defensible.

**Links.** `06-three-d-radius-is-exact.md` (the tie-credit fix this completes),
`06-three-d-radius-is-a-lower-bound.md` (the defect that fix settled),
`06-adaptive-table-under-the-covering-radius.md` (why the radius is the breaking instrument).
