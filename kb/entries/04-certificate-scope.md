# 04-certificate-scope — the certificate fails for two different reasons, and says which

| field | value |
|---|---|
| type | measurement |
| track | 4 |
| date | 2026-11-13 |
| case | `example-2c` (start sweep), `example-3d` |
| resolution | blockwise, `Step 0.05` and `0.25`, 110 directions, allowance `0` |

**Claim.** The blockwise test `max_i (⟨ψ_i, z_k⟩ - c_i^j) ≤ allowance` fails on `example-2c` by
`+0.0659` and on `example-3d` by `+0.1132`, and the two are not the same defect: on `2c` the slack
scales exactly with the start's `y` (`slack = y - 0.024`), so it is a constant offset, while on `3d`
it is present at block 1, before any control acts.

| `2c` start `(0, y)` | `0.09` | `0.07` | `0.05` | `0.03` | `0.01` |
|---|---|---|---|---|---|
| worst slack, all at block 8 | `+0.065902` | `+0.045910` | `+0.025918` | `+0.005927` | `-0.014065` |

| `3d` | value |
|---|---|
| worst slack, `(1.4, 1.4, 1.4)` | `+0.113155`, block 1, direction 71 |
| the same run with `--allowance 0.12` | certified, as a **manual** allowance |
| the same run with the calculated allowance | `0.322693`, source `estimate`: `certified yes (estimate, not a bound)` |
| `(1.3, 1.3, 1.3)` | `-0.038259`, certified |

On `2c` the slope is `1.000` and the intercept `-0.0241`: only the offset is wrong, the run tracks
the block chain, and the state is captured at `0.05` in every row — the target test passes, the
*family* test does not. Either the printed full-horizon set is outer by `0.076` in `y` or the rule
loses that much; the unit slope says it is a constant, not an accumulating divergence, and the
certificate alone cannot separate the two explanations.

On `3d` the failure cannot be the strategy: block 1 compares the start with `W^4` before the first
`u` or `v`. Direction 71 is `(0.6545, 0.4755, 0.5878)`, where the cube `1.5` has exact support
`2.5767` and the start has `2.4049`; the printed support is `0.113` below the start, so it sits
about `0.285` below the exact value. That is the 3D dual-update deficit sized on the `0.5` cube in
`06-direction-table-explains-extents.md`, scaled by the bridge's half-width. Moving the start to
`1.3`, which is `0.2` inside the printed cube, clears it: a support deficit localised at one facet.

**Evidence.** `./target/release/stable_bridge ../examples/example-2c.txt --certify 0,0.09`; the sweep
is the same command at the other `y`; `--allowance 0.12` and `--certify 1.3,1.3,1.3` for the `3d`
rows.

**Corrected.** The `0.12` row is a typed figure, and the calculated allowance is `0.322693`
(`06-two-neighbour-3d.md`), so `3d` passes on paper by a margin that cannot separate it from
the discretisation. The `+0.113` at block 1 remains the measurement; what changed is that the
certificate now labels a 3D pass an estimate instead of a claim, and `2c`'s two explanations are
separated (`06-two-sidedness.md`): the shortfall at its failing direction is zero.

**Limits.** The test is sampled: it evaluates the 110 stored directions, so a positive slack is
reliable and a negative one is not (`docs/03` §5). "Certified" therefore means "inside the printed
polytope at every block", not "winning against every evader control". No escaping-evader search has
run yet (E3), so nothing here says a state outside the printed set is winnable.

**Reuse.** Cite whenever "certified" is quoted, and before treating a printed set as a winning set.

**Links.** `04-strategy-rule.md`, `06-direction-table-explains-extents.md`,
`proposals/04-winning-strategies.md`.
