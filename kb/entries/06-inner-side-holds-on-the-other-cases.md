# 06-inner-side-holds-on-the-other-cases — the exact recursion reads a case file, and the inner side is inside everywhere

| field | value |
|---|---|
| type | measurement, prediction partly falsified |
| track | 6 |
| date | 2026-11-13 |
| case | `example-1` (20 blocks), `2a`, `2b`, `2c` (8 blocks), `110` directions |
| resolution | `exact_polygon.py` part F: `blocks_of` from the file, `support_violation` against it |

**Claim.** The exact recursion now reads a case file — `constraints M/P/Q`, matrices `A/B/C/pi`,
`time`, `step` — and maps `W <- (W + Bᵀ Gᵀ P) − (Cᵀ Gᵀ Q)` with `G = pi · Ie · E`. It reproduces the
committed extents of `example-1` to `2.0e-10`, `2a` to `2.0e-12` and `2b` exactly, and at `110`
directions the inner polytope `c − a − s` has zero support violation at **every** block of all four
cases. The inner-side failure is the `205` count's, not the cases'.

| case | blocks | extent diff | worst violation |
|---|---|---|---|
| `example-1` | 20 | 2.0e-10 | 0 |
| `2a` | 8 | 2.0e-12 | 0 |
| `2b` | 8 | 0 | 0 |
| `2c` | 8 | 9.976e-03 | 0 |

**Prediction.** Written before the run: all four extents match to `1e-6`, and the inner side has
zero violation at every block. Held for the three new cases and for every violation; falsified for
`2c`'s extents at `9.976e-3` — which is exactly part B's disagreement between `two_c()` and the
committed document, the output's own `0.01` vertex slack, not the generalisation.

**The two maps.** `example-1` has `C = −I`, so a single shared map would put `Q` on the wrong side:
`(-G B)ᵀ P` is `−(BᵀGᵀ)P`, absorbed into `(BᵀGᵀ)P` because every case's `P` is centrally symmetric
(`blocks_of` refuses a case whose `P` is not). `G` itself is general: `exp` and `∫exp` come from
their series, and `example-1` with `A = 0` stays a diamond at all `20` blocks.

**Cost.** `2a`'s exact set gains about two facets per substep — `1764` vertices at block 8 — because
each Minkowski sum adds the rotated `P`'s facets. That made the clipping quadratic in wall time, so
`clip` is vectorised and `erode` prunes vertices within `1e-9` of their chord; the recursion is
exact to that tolerance, and `2c`, `2b` and `example-1` keep four vertices.

**Consequence.** At `110` directions the bracket's inner side is inside the exact body for all four
cases at every block, so `06-chamfering-gap-is-not-the-table`'s `4.8e-4` violation is a property of
the `205` table, not of the shift or of a case. A certificate may treat the inner side as a set
where the table is `110`.

**Limits.** One direction count (`110`) and one probe instrument: the inner polytope's vertices at
the clips of the shifted caps, which is the only place it can leave the body. Pruning at `1e-9` is
below every figure quoted here. `2a` takes about `100 s`; the other three are seconds.

**Evidence.** `python tools/exact_polygon.py` (part F), or
`python -c "import sys; sys.path.insert(0,'tools'); import exact_polygon as e; e.part_f()"`.

**Reuse.** Quote the inner side as inside at `110` on all four cases, and cite `205` whenever the
violation is mentioned.

**Links.** `06-chamfering-gap-is-not-the-table.md` (the `205` violation),
`06-inner-set-leaves-the-exact-body.md` (the corrected entry),
`06-band-maximum-and-the-reconstruction-slack.md` (the `0.01` output slack),
`06-rung-is-254-at-the-exact-band.md` (the count the band needs).
