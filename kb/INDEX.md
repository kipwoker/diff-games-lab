# Index

The only file that must be read every session. Everything else is loaded on demand.

## Proposals

| track | file | question | depends on | state |
|---|---|---|---|---|
| 1 | [`proposals/01-linear-bridges.md`](proposals/01-linear-bridges.md) | what the reference implementation can and cannot be trusted to measure | — | proposal |
| 2 | [`proposals/02-nonlinear-games.md`](proposals/02-nonlinear-games.md) | which steps survive `ẋ = f(x, u, v)`, and what a certified linearization costs | 1, 6 | proposal |
| 3 | [`proposals/03-alternative-constructions.md`](proposals/03-alternative-constructions.md) | whether ellipsoids, zonotopes and grids agree with the bridge, and where they do not | 1, 6 | proposal |
| 4 | [`proposals/04-winning-strategies.md`](proposals/04-winning-strategies.md) | which positional strategies realise the bridge, and can they be certified | 1, 6 | proposal |
| 5 | [`proposals/05-unstable-parameters.md`](proposals/05-unstable-parameters.md) | what a parameter box does to the bridge, and how the two readings of `ΔA` differ | 1, 4 | proposal |
| 6 | [`proposals/06-method-evolution.md`](proposals/06-method-evolution.md) | a direction error, adaptive refinement, a support-function error, an inclusion check | 1 | proposal |
| 7 | [`proposals/07-games-and-learning.md`](proposals/07-games-and-learning.md) | a network that answers in milliseconds, and a check that may refuse it | 1, 6 (4, 5 for P4) | proposal |

## Entries

| id | track | claim | date |
|---|---|---|---|
| [`06-degenerate-vertices-need-the-antipode.md`](entries/06-degenerate-vertices-need-the-antipode.md) | 6 | one side of a degenerate triple's great circle is half the radius: the equator-plus-pole table read `54.7356°` against `90.0000°`, and both poles move `4` cells while the shipped tables' maxima do not move | 2026-11-16 |
| [`02-p1-gate-is-a-permissive-label.md`](entries/02-p1-gate-is-a-permissive-label.md) | 2 | P1's `11.5%` is `margin / 1.3`; measured denominators give `13.04%` at the interior, `11.54%` at the closure limit and `4.77%` on the angle axis, and the nearest measured table is `2.1×` over even the loosest | 2026-11-16 |
| [`02-no-affine-chart-bridge-closes-p1.md`](entries/02-no-affine-chart-bridge-closes-p1.md) | 2 | P1's step is the wall: its Euler error is `8.4-257×` the per-block budget and the chart residual needs `5.7e-6` half and `4.9e16` charts, so no affine-chart bridge is affordable | 2026-11-14 |
| [`06-inner-side-violation-is-a-chamfer-shortfall.md`](entries/06-inner-side-violation-is-a-chamfer-shortfall.md) | 6 | at `205` block 1 the shifted cap sits `1.06e-3` above the exact chamfer facet and the vertex pokes `4.8e-4` of it, so the inner test is `allowance > cap − h_facet` | 2026-11-14 |
| [`06-allowance-fractions-need-one-convention.md`](entries/06-allowance-fractions-need-one-convention.md) | 6 | an allowance needs its `ρ`: the shipped `11 x 10` reads `17.90%` with the half gap and `44.65%` at the cell radius, and P1's `12.4%` was another case's absolute allowance | 2026-11-14 |
| [`06-triple-is-a-bound-but-not-a-tight-one.md`](entries/06-triple-is-a-bound-but-not-a-tight-one.md) | 6 | the triple is exact at samples and loose elsewhere: charging its maximum widens the 3D band `1.999277 → 4.631082`, so the band stays one-term | 2026-11-13 |
| [`06-three-d-max-bound-leaves-the-flank-unproved.md`](entries/06-three-d-max-bound-leaves-the-flank-unproved.md) | 6 | the triple bounds `84–89%` of probes and is tight at the one-term's only failure; `11–16%` have no triple | 2026-11-13 |
| [`06-three-d-table-shape-is-a-parameter.md`](entries/06-three-d-table-shape-is-a-parameter.md) | 6 | `16 x 15` halves the 3D band (`1.999277 → 1.307773`) and the source is still `estimate` | 2026-11-13 |
| [`06-rung-is-254-at-the-exact-band.md`](entries/06-rung-is-254-at-the-exact-band.md) | 6 | at the exact band `205` misses for all four cases and `254` passes for all four, at block 1 | 2026-11-13 |
| [`06-three-d-violation-is-alignment-not-density.md`](entries/06-three-d-violation-is-alignment-not-density.md) | 6 | the one-term violation is grid alignment: `14 x 13` and `16 x 15` fail, five shapes are clean, and `13 x 12` is exact because it samples the facet normals | 2026-11-13 |
| [`06-alignment-is-sound-not-tight.md`](entries/06-alignment-is-sound-not-tight.md) | 6 | alignment removes the violation and not the allowance: `44.6%` at `11 x 10` falls to `27.5%` at `16 x 15`, the aligned `13 x 12` still reads `35.8%`, and the P1 entry's `12.4%` does not reproduce | 2026-11-13 |
| [`06-prior-art-retires-the-two-candidates.md`](entries/06-prior-art-retires-the-two-candidates.md) | 6 | the prior-art check retires both novelty candidates: spherical Voronoi vertices are facet normals (Fortune, STRIPACK, SciPy) and a polytope is its facet half-spaces (Dudley; Bronshtein–Ivanov), so the assets are the measurements | 2026-11-13 |
| [`06-what-the-falsifications-say.md`](entries/06-what-the-falsifications-say.md) | 6 | the `16` misses fall into six patterns — dimension carry, instrument for quantity, scaling law, coarser-instrument expectation, sound-but-loose, method assumption — each with a one-run check | 2026-11-13 |
| [`06-inner-side-holds-on-the-other-cases.md`](entries/06-inner-side-holds-on-the-other-cases.md) | 6 | the exact recursion reads a case file and reproduces `1`/`2a`/`2b` extents to `2e-10`; the inner side is inside at every block of all four cases | 2026-11-13 |
| [`06-chamfering-gap-is-not-the-table.md`](entries/06-chamfering-gap-is-not-the-table.md) | 6 | `gap / tan ρ` runs `0.38–1.55` and rises at `254`, so the inner gap is not a table term; a cap shrink empties block 8 | 2026-11-13 |
| [`06-inner-set-leaves-the-exact-body.md`](entries/06-inner-set-leaves-the-exact-body.md) | 6 | corrected: the `0.0109–0.0256` was distance to the boundary, not a violation; the vertices are inside except `4.8e-4` at `205` | 2026-11-13 |
| [`06-band-maximum-and-the-reconstruction-slack.md`](entries/06-band-maximum-and-the-reconstruction-slack.md) | 6 | the band is `1.001–1.033×` the sampled width, not `1.13×`: the rest was the `0.01` vertex slack | 2026-11-13 |
| [`06-band-direction-is-a-short-direction.md`](entries/06-band-direction-is-a-short-direction.md) | 6 | the inner side is sound at all `880` samples, and block 8's band lies inside the exact body by `0.003756` | 2026-11-13 |
| [`06-band-between-the-samples.md`](entries/06-band-between-the-samples.md) | 6 | the printed band is up to `13%` narrow: probed `1.00–1.13×` the sampled width | 2026-11-13 |
| [`06-cell-bound-was-not-a-bound.md`](entries/06-cell-bound-was-not-a-bound.md) | 6 | `support_at` gave `−1.414214` at `45°` where the support is `1.414214`, and a finite 3D probe bound | 2026-11-13 |
| [`06-last-block-band-is-drift.md`](entries/06-last-block-band-is-drift.md) | 6 | at `2c`'s last block `92.6%` of the band is the shortfall, while three cases' last blocks are `0–6.4%` | 2026-11-13 |
| [`06-inward-share-is-not-a-tan-rho-law.md`](entries/06-inward-share-is-not-a-tan-rho-law.md) | 6 | the inward share runs `0.56–0.84 tan ρ`, so `f` is `1.56` at `205`, not `1.30` | 2026-11-13 |
| [`06-density-sweep-measured.md`](entries/06-density-sweep-measured.md) | 6 | at four table sizes `158` misses `2%` even at `f = 1`, and `2c` needs `254`, not `205` | 2026-11-13 |
| [`06-two-d-table-is-a-parameter.md`](entries/06-two-d-table-is-a-parameter.md) | 6 | the 2D count is a run parameter for any `n ≥ 3`, and `110` is bit-identical | 2026-11-13 |
| [`06-three-d-bound-fails-at-a-cell-vertex.md`](entries/06-three-d-bound-fails-at-a-cell-vertex.md) | 6 | the 3D allowance is exceeded only at a cell vertex, where a three-plane vertex bounds and is tight | 2026-11-13 |
| [`06-density-scaling-on-two-c.md`](entries/06-density-scaling-on-two-c.md) | 6 | both band terms track `tan ρ`, but `2c` prints `2.567%` at `158` and needs `205` | 2026-11-13 |
| [`06-two-plane-bound-loses-to-the-allowance.md`](entries/06-two-plane-bound-loses-to-the-allowance.md) | 6 | on `2c` the one-term allowance is `1.05×` the true excess and the pair bound `1.50×`, so the pair widens the band | 2026-11-13 |
| [`06-three-d-radius-is-exact.md`](entries/06-three-d-radius-is-exact.md) | 6 | the tie test dropped every Voronoi vertex; `ρ` is now `19.8733°` and the one-term form fails at one probe | 2026-11-13 |
| [`06-rung-tolerance-is-a-table-size.md`](entries/06-rung-tolerance-is-a-table-size.md) | 6 | `2%` of diameter needs `m ≥ 158` in 2D and `3.2·10⁴` in 3D, so the tolerance is a table size | 2026-11-13 |
| [`04-two-c-slack-is-drift.md`](entries/04-two-c-slack-is-drift.md) | 4 | the exact block-8 support is `0.0014` under the printed one, so all of `0.0659` is the rule | 2026-11-13 |
| [`06-three-d-radius-is-a-lower-bound.md`](entries/06-three-d-radius-is-a-lower-bound.md) | 6 | the shipped 3D table repeats ten directions, those cells had `ρ = 0`, and `ρ` is a lower bound | 2026-11-13 |
| [`06-three-d-allowance-is-loose.md`](entries/06-three-d-allowance-is-loose.md) | 6 | the measured 3D excess is `27–69%` of the allowance, so P1's gate is `2.8×` away on the error itself | 2026-11-13 |
| [`06-adaptive-table-under-the-covering-radius.md`](entries/06-adaptive-table-under-the-covering-radius.md) | 6 | the greedy table's radius is `39.38°` against the shipped `19.38°`, so adaptive is worse | 2026-11-13 |
| [`06-bracket-folds-inward.md`](entries/06-bracket-folds-inward.md) | 6 | with the shortfall on the inner side the worst 2D band is `3.7170%`, not a uniform `2.8160%` | 2026-11-13 |
| [`02-p1-step-refinement.md`](entries/02-p1-step-refinement.md) | 2 | refining `Step` settles P1's `d*` at `1.150`, so the margin is `0.150`, not `0.025` | 2026-11-13 |
| [`04-two-c-slack-is-not-constant.md`](entries/04-two-c-slack-is-not-constant.md) | 4 | `2c`'s active direction settles and the slack decelerates, so at least `0.0182` of it is drift | 2026-11-13 |
| [`06-bracket-and-cost.md`](entries/06-bracket-and-cost.md) | 6 | the 2D band is `2.8160%` of the diameter on four cases, `3d` is `29.2%`, and the band costs `0.7%` of a run | 2026-11-13 |
| [`02-p1-bridge-is-not-affordable.md`](entries/02-p1-bridge-is-not-affordable.md) | 2 | a linearised P1 bridge's remainder is `200×` the per-block budget, and the grid's own block error is `50×` | 2026-11-13 |
| [`06-three-d-covering-radius.md`](entries/06-three-d-covering-radius.md) | 6 | 3D's `ρ` was half the nearest gap, which is only a 2D identity: the allowance was `2.93×` optimistic | 2026-11-13 |
| [`06-two-sidedness.md`](entries/06-two-sidedness.md) | 6 | the recursion's own shortfall is zero where `2c` fails, so that failure is not the deficit | 2026-11-13 |
| [`02-nonlinear-grammar.md`](entries/02-nonlinear-grammar.md) | 2 | the P1 case parses, `w_e < w_p` is enforced, and the driver refuses it by naming E4 | 2026-11-13 |
| [`02-p1-relative-interior.md`](entries/02-p1-relative-interior.md) | 2 | bounding the evader's turn rate restores P1's interior: `d* = 1.233` against the `1.3` closure limit | 2026-11-13 |
| [`06-three-d-adaptive-allowance.md`](entries/06-three-d-adaptive-allowance.md) | 6 | 3D's allowance is the polar spacing: `4×` smaller at the same 110 adaptive directions | 2026-11-13 |
| [`06-two-sided-bracket.md`](entries/06-two-sided-bracket.md) | 6 | shifting each half-space by its allowance gives a `2.83%` band measured inside the true body | 2026-11-13 |
| [`06-bracket-is-one-sided.md`](entries/06-bracket-is-one-sided.md) | 6 | the inner side is the inscribed ball, so the widest bracket is `30-42%` of the diameter | 2026-11-13 |
| [`02-p1-is-not-yet-certifiable.md`](entries/02-p1-is-not-yet-certifiable.md) | 2 | P1's interior survives the angle axes; the certificate is blocked by the bridge and the table | 2026-11-13 |
| [`02-p1-refinement.md`](entries/02-p1-refinement.md) | 2 | P1's interior rises `1.233 → 1.275` with `nd`; the coarse grid overstated the margin | 2026-11-13 |
| [`06-adaptive-refinement.md`](entries/06-adaptive-refinement.md) | 6 | bisecting the widest gap cuts the 2D excess `4.3×` at a matched `110` directions | 2026-11-13 |
| [`06-two-neighbour-3d.md`](entries/06-two-neighbour-3d.md) | 6 | the pair form is 2D-only; the computed 3D allowance is 0.323 and `certified` says "estimate" | 2026-11-13 |
| [`06-two-neighbour-bound.md`](entries/06-two-neighbour-bound.md) | 6 | the extent bound needs both straddling samples; the single-term form is false by up to 0.021 | 2026-11-13 |
| [`04-predictive-rule.md`](entries/04-predictive-rule.md) | 4 | scoring every pursuer vertex one block ahead removes the `3d` escape | 2026-11-13 |
| [`04-strategy-rule.md`](entries/04-strategy-rule.md) | 4 | the counter-direction vertex certifies three of five cases; the maximising vertex leaves the bridge by 4.83 | 2026-11-13 |
| [`04-adversarial-search.md`](entries/04-adversarial-search.md) | 4 | a sequence search beats the extremal evader everywhere and escapes `3d` with `0 0 1 0` | 2026-11-13 |
| [`04-certificate-scope.md`](entries/04-certificate-scope.md) | 4 | the certificate's two failures differ: a constant `y` offset on `2c`, a short support before any control on `3d` | 2026-11-13 |
| [`06-direction-table-explains-extents.md`](entries/06-direction-table-explains-extents.md) | 6 | the 110 directions alone reproduce the tracked 2D and 3D anomalies to four decimals | 2026-11-13 |
| [`06-direction-bound.md`](entries/06-direction-bound.md) | 6 | `h·tan ρ + R(1/cos ρ − 1)` sits 0.1% and 11% above the two tracked bulges and above every blind case | 2026-11-13 |
| [`01-deficit-is-chamfering.md`](entries/01-deficit-is-chamfering.md) | 1 | the error is over at facet normals and short at vertex normals, so neither inclusion holds | 2026-11-13 |
| [`01-dual-update-is-the-deficit.md`](entries/01-dual-update-is-the-deficit.md) | 1 | the 2.3% shortfall is the dual update's primal reconstruction: step-independent, alignment-dependent, two-sided | 2026-11-13 |
| [`01-example1-deficit.md`](entries/01-example1-deficit.md) | 1 | a 90°-symmetric case is printed 2.3% short, so the sampled set is not uniformly outer | 2026-11-13 |
| [`02-p1-case-definition.md`](entries/02-p1-case-definition.md) | 2 | P1 is 3D because the table is 2D/3D only, and `(2,0,0)` is outside at `T = 4` by 2.33 s | 2026-11-13 |
| [`02-p1-bridge-is-the-capture-set.md`](entries/02-p1-bridge-is-the-capture-set.md) | 2 | the point evader's curvature makes the `T = 4` bridge the capture disc, and the relative state is 3D | 2026-11-13 |

## State

Track 6 supplies the instruments and is the binding constraint. The 2D band is `2.8160%` of the
diameter (`3.7170%` once the recursion's shortfall is on the inner side) on four cases; the 3D band
is `1.999277` at `11 x 10` and `1.307773` at `16 x 15`, and 3D is `estimate` at every shape because
the one-term form is short at a cell vertex. The `2%` rung needs `254` directions at block 1. The
inner side is inside the exact body at `110` directions on all four cases, with one `4.8e-4`
violation at `205`, a `1.06e-3` chamfer shortfall. The radius enumeration now credits both poles of a degenerate triple. Twenty-two predictions are recorded as misses and classified in
`entries/06-what-the-falsifications-say.md`; the entry table below is the full record. A short reading
of the intermediate results, with what is ours and what is classical, is `docs/08-brief.md`; the
write-up of record is `docs/09-note.md`, and both are measurement records rather than novelty claims.

## Live numbers

| quantity | value | entry |
|---|---|---|
| 2D band, four cases | `2.8160%`; `3.7170%` folded inward | `06-bracket-and-cost.md` |
| 3D band, `11 x 10` / `16 x 15` | `1.999277` / `1.307773` | `06-three-d-table-shape-is-a-parameter.md` |
| one-term violation, cube `1.5` | out at `14 x 13` and `16 x 15`, clean at five shapes | `06-three-d-violation-is-alignment-not-density.md` |
| one-term allowance, cube | `44.6%` at `11 x 10` → `27.5%` at `16 x 15`; aligned `13 x 12` is `35.8%` | `06-alignment-is-sound-not-tight.md` |
| allowance convention | `17.90%` half-gap vs `44.65%` cell radius at `11 x 10`; no measured table under `13.04%` (`d`) or `4.77%` (angle) | `06-allowance-fractions-need-one-convention.md` |
| inner side at `110` | inside, all four cases | `06-inner-side-holds-on-the-other-cases.md` |
| inner side at `205` | out by `4.8e-4`, on a `1.06e-3` chamfer shortfall | `06-inner-side-violation-is-a-chamfer-shortfall.md` |
| P1 bridge feasibility | Euler `8.4-257×` budget; chart residual needs `5.7e-6` half, `4.9e16` charts | `02-no-affine-chart-bridge-closes-p1.md` |
| triple coverage at probes | `84–89%` | `06-three-d-max-bound-leaves-the-flank-unproved.md` |
| triple charged as a maximum | widens the 3D band `2.3×` | `06-triple-is-a-bound-but-not-a-tight-one.md` |
| degenerate circumcenters | one-sided radius reads `54.7356°` where the truth is `90.0000°`; shipped tables unchanged | `06-degenerate-vertices-need-the-antipode.md` |

## Superseded

| retired claim | replacement |
|---|---|
| `06-three-d-radius-is-a-lower-bound.md` — `ρ` is a lower bound | `06-three-d-radius-is-exact.md` — `19.8733°` / `13.3967°` |
| `06-band-between-the-samples.md` — the band is up to `13%` narrow | `06-band-maximum-and-the-reconstruction-slack.md` — `1.0008–1.0333×` at `1e-9` |
| `06-inner-set-leaves-the-exact-body.md` — vertices outside by `0.0109–0.0256` | `06-chamfering-gap-is-not-the-table.md` — a distance to the boundary, not a violation |

| task | track | settles | blocked by |
|---|---|---|---|
| a method for P1 that is not a linearised bridge | 2 | whether P1's interior can be certified rather than measured | track 6: no measured table is under `13.04%` (`d`) or `4.77%` (angle), the nearest is `2.1×` over, and the adaptive table is `118%` |
| an allowance that is not the one-term form, measured per cell | 6 | a 3D allowance under `11.5%` of the support, P1's gate | nothing |

## Decisions

| date | decision | because |
|---|---|---|
| 2026-11-13 | Findings live in `kb/entries/`, one claim per file, and proposals are stable plans | context is limited; a session must be able to load one claim without the conversation that produced it |
| 2026-11-13 | Success is defined as set + strategies + certificate, not as a set | a set answers "can it happen", the task is to give both players a winning strategy |
| 2026-11-13 | Tracks 3 and 6 are instruments, not rungs | agreement and error accounting decide whether a rung's claim is true |
| 2026-11-13 | Publication candidates are recorded with a prior-art check as a condition | a claim of novelty without a search is a liability, not an asset |
| 2026-11-13 | A failing certificate is reported with its localisation, never repaired by raising `--allowance` | the two failures are the measurement; the allowance belongs to track 6, it is not a knob |
