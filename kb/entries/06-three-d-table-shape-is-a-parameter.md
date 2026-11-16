# 06-three-d-table-shape-is-a-parameter — `16 x 15` halves the 3D band and still says estimate

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-3d` at `11 x 10` and `16 x 15` |
| resolution | `--table azxel`, `build_3d(11, 10)` against the shipped table, `--bracket` |

**Claim.** The 3D table's shape is a run parameter now: `build_3d(azimuths, elevations)` builds any
shape of two or more each, `build_3d(11, 10)` reproduces `build(3)` **bit for bit**, and
`--table 16x15` runs `example-3d` on 240 directions. The covering radius falls `19.8733 → 13.3967°`
— the figure part M measured for that shape — and the widest band falls `1.999277 → 1.307773`.
Both runs still say `two-neighbour, estimate`: density buys the band, not the certificate.

| table | directions | covering radius max | widest band | source |
|---|---|---|---|---|
| `11 x 10` | 110 | 19.8733° | 1.999277 | estimate |
| `16 x 15` | 240 | 13.3967° | 1.307773 | estimate |

**Prediction.** Written before the run: the radius repeats part M's `13.3967°` and the band falls
below `1.5`. Held exactly — `13.3967` is the same enumeration, and the band is `1.3078`.

**Why the label stands.** The one-term form is exceeded at one probe of `2500` on the cube at
`16 x 15` (`104.6%`), and the table's shape does not change the form
(`06-three-d-bound-fails-at-a-cell-vertex.md`). The triple vertex bounds that probe and is tight
there, but the engine's allowance is still the one-term function: `--table` changes the table, not
the bound.

**Limits.** `--table` is 3D-only and `--directions` stays 2D-only; the shipped `11 x 10` remains the
default, so the five committed outputs are unchanged. The `16 x 15` run is `example-3d`'s game, not
the analytic cube that falsifies the form, so it measures the radius and the band, not the bound.

**Evidence.** `cargo test --release` (the `table3d` suite pins the bit equality), then
`stable_bridge examples/example-3d.txt --bracket --table 16x15`.

**Reuse.** Cite the 3D band at its shape — `1.999277` is `11 x 10`, `1.307773` is `16 x 15` — and
never read the smaller number as a certificate.

**Links.** `06-three-d-bound-fails-at-a-cell-vertex.md` (the probe that keeps it an estimate),
`06-three-d-covering-radius.md` (the radius), `06-two-d-table-is-a-parameter.md` (the 2D
counterpart), `06-band-maximum-and-the-reconstruction-slack.md` (the band instrument).
