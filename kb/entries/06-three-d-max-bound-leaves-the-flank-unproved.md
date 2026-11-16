# 06-three-d-max-bound-leaves-the-flank-unproved — a bound on `89%` of the sphere, an estimate on `11%`

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | octahedron `1` and cube `1.5`, `11 x 10` and `16 x 15`, 2500 probes |
| resolution | `covering_bound.py` part M: one-term allowance, three-plane vertex, exact support |

**Claim.** Where a triple straddles the probe the three-plane vertex is a bound — 0 violations in
every row, largest `excess − triple` of `1.4e-12` — and it is exactly tight at the one probe where
the one-term form fails: the cube at `16 x 15` has 1 one-term violation at `1.046`, with
`excess/triple = 1.000`. So the maximum of the two figures is violated nowhere a triple exists.
But a triple exists at only `84–89%` of probes: `401` of `2500` (`16.0%`) at `11 x 10` and `283`
(`11.3%`) at `16 x 15` have none.

| body | table | `ρ` | one-term violations | no triple | share | max `excess − triple` |
|---|---|---|---|---|---|---|
| octahedron 1 | `11 x 10` | 19.8733° | 0 | 401 | 16.0% | 9.8e-15 |
| octahedron 1 | `16 x 15` | 13.3967° | 0 | 283 | 11.3% | 6.1e-13 |
| cube 1.5 | `11 x 10` | 19.8733° | 0 | 401 | 16.0% | 2.5e-14 |
| cube 1.5 | `16 x 15` | 13.3967° | **1** at `1.046` | 283 | 11.3% | 1.4e-12 |

**Prediction.** From this step's plan: the `104.6%` violation disappears wherever a triple
straddles, and 3D becomes a bound on `89%` with an estimate on `11%`. Held — the violating probe
has a triple and the triple is tight there.

**Derived, not printed.** Part M prints the one-term and the triple columns, not their maximum. The
maximum's violation count follows from the two: 0, because the only probe where the one-term is
violated is one where the triple is valid.

**Consequence.** 3D is not a certificate. It is a bound on the straddling `84–89%` and an unproved
estimate on the rest, where the one-term is not violated in these probes (its ratio there is
`0.241–0.694`) but is still the form that was falsified. The engine's allowance remains the
one-term: `--table` changes the table, not the bound.

**Limits.** `2500` probes per row, seed 11. The triple is searched among the nearest samples'
triples, so a straddling triple outside that set would be missed and the `11%` is an upper bound on
the unproved flank. `example-3d` runs still report `estimate`.

**Evidence.** From `tools/`, `python -c "import covering_bound as c; c.part_m()"` after
`cargo build --release`.

**Reuse.** Quote 3D as "a bound on `89%`, an estimate on `11%`", never as certified.

**Links.** `06-three-d-bound-fails-at-a-cell-vertex.md` (the probe and the triple),
`06-three-d-table-shape-is-a-parameter.md` (the shapes), `06-three-d-allowance-is-loose.md` (how
loose the one-term is), `06-two-neighbour-3d.md` (why the engine says estimate).
