# 06-three-d-bound-fails-at-a-cell-vertex — one probe, on the vertex, where a triple does bound

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | octahedron `{|x|+|y|+|z| <= 1}`, cube `max|x| <= 1.5` |
| resolution | `11 x 10` and `16 x 15`; exact reconstruction by 3-subset enumeration; `2500` probes with every facet normal |

**Claim.** The one-term allowance fails in 3D at exactly the probes that sit on a cell's vertex. On
the cube at `16 x 15` one probe of `2500` is exceeded — `104.6%` of the allowance — and that probe's
distance to its nearest sample is `1.000` times its cell's covering radius, i.e. the vertex itself.
The cause is structural: at a vertex three samples are equidistant at angle `ρ`, so the support is
set by three planes, and `h tan ρ + R(1/cos ρ − 1)` is a one-plane statement. The three-plane vertex
*is* a bound — no probe in any row exceeds it — and it is exactly tight at the worst probes, but it
straddles only `89%` of them.

| body | table | `ρ` max | one-term exceeded | excess/allowance at the worst | gap/`ρ` there | triple exceeded | max excess−triple | no triple: excess/allowance | no triple |
|---|---|---|---|---|---|---|---|---|---|
| octahedron | `11 x 10` | `19.8733°` | `0` | `0.324` | `0.716` | `0` | `≤1e-9` | `0.313` | `401` |
| octahedron | `16 x 15` | `13.3967°` | `0` | `0.849` | `0.867` | `0` | `≤1e-9` | `0.241` | `283` |
| cube `1.5` | `11 x 10` | `19.8733°` | `0` | `0.694` | `0.960` | `0` | `≤1e-9` | `0.694` | `401` |
| cube `1.5` | `16 x 15` | `13.3967°` | `1` | `1.046` | `1.000` | `0` | `≤1e-9` | `0.492` | `283` |

The triple bound is tight rather than merely valid: at the worst probe `excess / triple = 1.000`,
because the reconstruction's support there is attained at that vertex.

**Prediction.** Written before part M ran: the one-term violations sit at `gap / ρ` near `1`, and
the triple is tighter there. Held — the only violation is at `1.000`, and the triple is exact.

**Why the pairs cannot reach it.** The engine's `cell_bound` minimises over *pairs* among the nearest
four. In 3D a pair bounds a probe only where the probe lies in the span of its two normals, which is
measure zero: part M finds no straddling triple at `11–16%` of probes, and a straddling pair is
rarer still. So the pair construction is not the repair, and in 2D the same code is sound only
because every probe's cell is an arc between two samples.

**What the repair costs.** A hybrid — triple where a triple straddles, one-term elsewhere — is not
violated on this probe set (`0.24–0.69` of the allowance on the uncovered probes), but the one-term
part is still unproved, so it is a measured improvement and not a certificate. The honest 3D status
is *estimate*, with a valid and tight bound on the vertex probes and an open rule for the rest.

**Limits.** Two analytic bodies, one probe set, and exact supports at the table, so the dual update's
inward loss is not measured here. "No violation" is a statement about `2500` directions, not a proof;
only the triple half is a bound by construction, and only where it straddles.

**Regression.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as cb;
cb.part_m()"` reprints the one-term and triple violation counts, as part K's does. A unit test that pins the `16 x 15` cube needs a `16 x 15` table in the engine, which
`directions::build` cannot build yet — it belongs with the table-size parameter, not here.

**Evidence.** Part M of `tools/covering_bound.py`; the `104.6%` is part K's.

**Reuse.** Do not quote a 3D allowance as a certificate. Where a triple straddles a probe, quote the
triple vertex instead: it is the excess itself at the worst probes.

**Links.** `06-three-d-allowance-is-loose.md` (the falsification), `06-direction-bound.md` (the form),
`06-three-d-covering-radius.md` (`ρ`), `06-two-neighbour-bound.md` (the pair form),
`06-three-d-adaptive-allowance.md` (the per-cell figures).
