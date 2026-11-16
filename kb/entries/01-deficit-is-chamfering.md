# 01-deficit-is-chamfering — the printed set bulges at flats and cuts corners, so neither inclusion holds

| field | value |
|---|---|
| type | measurement |
| track | 1 |
| date | 2026-11-13 |
| case | `example-1` diamond (exact `3.000`), `example-2b` square (exact `0.500`) |
| resolution | 110 and 220 directions, shipped and rotated; `Step 0.1`; `A = 0`; reconstruction slack `0.01` |

**Claim.** The deficit has no single sign. At the *binding* directions of the true body, the printed
set sits **over** at the body's facet normals and **short** at its vertex normals, on both bodies and
all three tables, so neither `printed ⊆ true` nor `true ⊆ printed` holds on any of the six cases.
The sign is predicted by the local geometry of the true body, not by the table's alignment.

| case | table | facet normal | diff | vertex normal | diff |
|---|---|---|---|---|---|
| diamond `3.0` | 110 shipped | `45°` | `+0.045448` | `90°` | `-0.069050` |
| diamond `3.0` | 110 half gap | `45°` | `+0.060601` | `90°` | `-0.035471` |
| diamond `3.0` | 220 half gap | `45°` | `+0.022720` | `90°` | `-0.014339` |
| square `0.5` | 110 shipped | `90°` | `+0.014284` | `45°` | `-0.003250` |
| square `0.5` | 110 half gap | `90°` | `+0.013100` | `45°` | `-0.008731` |
| square `0.5` | 220 half gap | `90°` | `+0.007071` | `45°` | `-0.000000` |

Both inclusion tests fail everywhere: outward `+0.007` to `+0.061`, inward `+0.007` to `+0.079`,
six rows out of six reading *neither*. That is not two measurements of one error — it is a printed
polygon whose flats are pushed out and whose corners are pulled in, i.e. the true body chamfered.

The likely mechanism is stage 5: inactive directions take the support of the hull
`conv{v + c2_j ψ_j}`, a maximum over the sampled directions, so the printed body's own vertices lie
along the table's directions and its facets between them. Cutting the true body's corners then shows
up as a short reading there, while the bulge between samples shows up as an over reading at flats.
This is offered as the reading that fits, not as a derivation; what is measured is the two columns.

**Evidence.** `python tools/trace_deficit.py` parts E and F. Part E prints `h_printed - h_exact` at
`0°/45°/90°/135°` with the angle to the nearest sample; part F prints the two inclusion maxima and
the verdict. The prediction that neither inclusion holds was written into the tool's header before
the run.

**Limits.** `A = 0`, 2D, and `L1`/square bodies only, so "facet" and "vertex" mean the four binding
directions of a polygon. Three tables, one offset family. The `0.01` reconstruction slack inflates
the outward column by up to `0.01`, which is why the smallest outward reading is not treated as a
sign flip. Nothing here is a bound: it is a sign law on two bodies.

**Reuse.** Cite before any one-sided certificate or bracket. It says the certificate needs *both*
`e_out` and `e_in` evaluated on the table in use, and that a bracket whose inner side is the printed
set is not a bracket on the true set.

**Links.** `01-dual-update-is-the-deficit.md` (which stage), `01-example1-deficit.md` (the original
observation), `06-bracket-is-one-sided.md` (the bracket this corrects),
`04-certificate-scope.md` (the failures this explains).
