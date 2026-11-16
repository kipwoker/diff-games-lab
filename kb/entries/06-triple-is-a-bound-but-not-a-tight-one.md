# 06-triple-is-a-bound-but-not-a-tight-one — the maximum widens the 3D band `2.3×`

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | cube `1.5` on `11 x 10` and `16 x 15`; `example-3d` bracket |
| resolution | `triple_bound` in `engine/src/error.rs`, `cargo test`, `--bracket --table` |

**Claim.** The three-plane bound is a bound and is exactly tight at a sampled direction, but it is
loose everywhere else: charging `max(one-term, triple)` into the allowance widens the `example-3d`
band `1.999277 → 4.631082` at `11 x 10` and `1.307773 → 4.946911` at `16 x 15`, so the engine keeps
the one-term form and does not charge the triple at all.

| policy | `11 x 10` band | `16 x 15` band | source |
|---|---|---|---|
| one-term (unchanged) | 1.999277 | 1.307773 | estimate |
| `max(one-term, triple)` | **4.631082** | **4.946911** | estimate |

**Evidence it is a bound.** At the sampled directions `0`, `37`, `109` on the cube the triple's
value equals the support to `1e-9`; on `6 x 5` every pair-sum probe with a triple has the
reconstruction's reach at or under it (`>100` checked); and `299` of `2000` random probes (`15.0%`)
have no triple, the same flank part M's `401/2500` (`16.0%`) and `283/2500` (`11.3%`) count.

**One wrong solve.** The coefficients need `Nᵀ λ = ψ`; `linear_system::solve` takes rows as its
equations, so passing the normals as rows solved `N λ = ψ` and returned the wrong weights. That read
as `70 of 110` directions "covered", accepted triples with negative weights, and failed two tests
(`value` above the support at a sample, `reach` above the bound on `6 x 5`). With the transpose both
pass, and coverage at the table's own directions is `110 of 110` — true by construction, since every
sample is in its own triple, which is why the bracket prints no coverage line.

**Consequence.** The 3D band is the one-term figure it was, and the triple is a conditional bound:
valid where it straddles, absent on `11–16%`, far above the truth elsewhere. `max` is sound because
it is at least the triple, and useless because the apex is far away except at a cell vertex.

**Limits.** The triples are searched among the nearest six samples, so a straddling triple outside
that set is missed and the `15.0%` is an upper bound on the unproved flank, as in part M. The
engine's default table (`11 x 10`) and the five committed outputs are unchanged; `67` tests in `13`
suites pass.

**Evidence.** `cargo test --release` (the two new `error` tests; the probe share needs
`--nocapture`), then `stable_bridge examples/example-3d.txt --bracket` and the same with
`--table 16x15`.

**Reuse.** Quote the 3D band unchanged and the triple as "a bound where it straddles", never as
something the band may subtract.

**Links.** `06-three-d-max-bound-leaves-the-flank-unproved.md` (the tool's version of the same
figure), `06-three-d-bound-fails-at-a-cell-vertex.md` (the probe it repairs),
`06-three-d-table-shape-is-a-parameter.md` (the band it does not move),
`06-cell-bound-was-not-a-bound.md` (the earlier repair with the same shape of bug).
