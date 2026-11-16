# 06-two-d-table-is-a-parameter — any 2D count from three up, and `110` unchanged

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | the five tracked examples; the shipped 2D and 3D tables |
| resolution | `directions::build_with`, `StableBridge::new_with`, `--directions n` |

**Claim.** The 2D direction count is a run parameter, not a compile-time constant: `build_with(2, n)`
returns `n` evenly spaced directions for any `n ≥ 3`, `--directions n` selects it, and `build(2)`,
`StableBridge::new` and a default run stay on `110` bit for bit. 3D keeps the shipped `11 x 10`
shape and accepts only `110`, because a different 3D size needs azimuth and elevation counts, which
the single-count signature cannot express.

**Most of it was already parameterised.** `dual_update_with_loss` takes its scale from
`directions.len()`, `StableBridge` carries `directions` as a field, and the reconstruction, support
evaluation and output header all follow it. The only hard-wired `110` was the builder, so the change
is the builder plus one constructor.

**Verification.** All five committed outputs re-run at the default and compared byte for byte:
`identical` for `example-1`, `2a`, `2b`, `2c` and `3d`. `build_with(2, 110) == build(2)` is asserted
as a table equality, not a tolerance. Two tests cover the new surface: spacing is exactly `2π/n` at
`n ∈ {3, 158, 205, 254}`, and a 2D problem runs at `158` with a 158-long support vector while `158`
is refused in 3D and `2` is refused in 2D.

**Limits.** `SUBSTEPS_PER_STEP` stays `110` and is not the table — it refines the interval map in
time. Tables in the `10⁴` range are untested, and the 3D radius enumeration is `O(m³)`, so a larger
3D table would break there first.

**Reuse.** Cite before quoting a band as "the 110-direction band": with `--directions` the number
travels with the table.

**Evidence.** `cargo test --release` — `a_two_d_table_can_be_any_count`,
`a_two_d_problem_runs_on_another_table`; the byte comparison is the five re-runs of `examples/out/`.

**Links.** `06-density-scaling-on-two-c.md` (the sweep this enables),
`06-rung-tolerance-is-a-table-size.md` (`m ≥ 158`), `06-three-d-radius-is-exact.md` (`O(m³)`).
