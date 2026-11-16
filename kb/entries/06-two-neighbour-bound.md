# 06-two-neighbour-bound — the extent bound uses both straddling samples

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-13 |
| case | rotated parallelogram half-width `3`, octahedron `{|x|+|y|+|z| <= 1}` |
| resolution | 110 directions 2D; `11 x 10` and `12 x 15` 3D; 360 facet orientations |

**Claim.** With `a` and `b` the nearest sampled directions on either side of `ψ`, at angles `ρ_a`,
`ρ_b` and supports `c_a`, `c_b`, the reconstruction's extent is at most
`(c_a·sin ρ_b + c_b·sin ρ_a)/sin(ρ_a + ρ_b)`. Over 360 orientations of a rotated parallelogram that
formula equals the measured extent (`measured - pair = 0.000000`), while
`h·tan ρ + R(1/cos ρ - 1)` is *false* there by up to `+0.0210`, `0.7%` of the support, and fails in
3D as well.

| facet angle | `ρ_a` | `ρ_b` | measured | pair | one term |
|---|---|---|---|---|---|
| `0.0°` (sampled) | `0.0000°` | `3.2727°` | `3.0000` | `3.0000` | `3.0000` |
| `0.655°` | `2.6182°` | `0.6545°` | `3.0548` | `3.0548` | `3.0346` |
| `1.309°` | `1.9636°` | `1.3091°` | `3.0823` | `3.0823` | `3.0697` |
| `17.0°` | `2.6364°` | `0.6364°` | `3.0537` | `3.0537` | `3.0336` |
| worst, 360 orientations | — | — | — | `+0.000000` | `+0.020998` |

| octahedron, 3D | `ρ` to facet normal | measured `h_P` | one term |
|---|---|---|---|
| `11 x 10` (110 dirs) | `7.3492°` | `0.6688` | `0.6601` — exceeded by `0.0087` |
| `12 x 15` (180 dirs) | `11.8089°` | `0.7344` | `0.7197` — exceeded by `0.0147` |

| 3D table | `30` | `64` | `110` | `180` | `240` | `400` |
|---|---|---|---|---|---|---|
| covering radius | `39.52°` | `25.26°` | `19.84°` | `14.45°` | `13.33°` | `10.11°` |

The one-term form assumes the best point lies on the facet normal and the nearest constraint binds
alone. With asymmetric neighbours the optimum slides off the normal, and the two constraints
intersect further out than either alone; solving the pair for equality gives the formula above, which
the measurements match. Symmetric spacing is the degenerate case where the two coincide, which is why
the boxes and the `m` sweep agreed to 0.1%: the earlier validation only ever placed the facet normal
halfway between two samples.

**Evidence.** `python tools/covering_bound.py`, parts D, E and F — part F is the 360-orientation
sweep, part E prints the 3D table radii and the octahedron rows.

**Limits.** Derived and tested for polygons in 2D. In 3D only the *failure* of the one-term form is
measured, and the pair form does not generalise: part G of `tools/covering_bound.py` puts it
`0.089` *below* the measured extent on the octahedron, because three planes still leave a
direction out of their span unbounded (`06-two-neighbour-3d.md`). Both
forms bound the reconstruction from *exact* supports: neither covers the inward loss of the engine's
stored supports (`01-dual-update-is-the-deficit.md`), so neither may be used as `e_in`.

**Reuse.** Replaces the one-term form wherever an `e_out` is needed — track 4's allowance, track 5's
outer reading — with the caveat that the 3D case still rests on a measurement.

**Links.** `06-direction-bound.md` (the form this corrects), `04-certificate-scope.md` (the consumer
that used an allowance), `proposals/06-method-evolution.md`.
