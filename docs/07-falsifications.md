# Falsifications

Every finding in [`kb/entries/`](../kb/entries/) records what was predicted before the run, next
to what the run measured. This page collects the predictions that **missed**, because they did
more work than the ones that held: each is a place where the method's story changed.

The rule behind the page is that a prediction is written down first and kept afterwards. No
tolerance was widened and no `--allowance` was raised to turn a miss into a pass; where a claim
was wrong, the entry says so and a later entry corrects it.

| predicted | measured | what changed | entry |
|---|---|---|---|
| the one-term 3D allowance holds | exceeded at one probe in `2500` of the cube at `16 × 15`: `104.6%` of the bound, at a cell vertex | 3D is an estimate, not a certificate; the three-plane vertex is the named repair | [`06-three-d-radius-is-exact`](../kb/entries/06-three-d-radius-is-exact.md) |
| half the nearest-neighbour gap is the covering radius | it is a 2D identity; the 3D allowance was `2.93×` optimistic | the exact covering radius is enumerated from triple circumcenters | [`06-three-d-covering-radius`](../kb/entries/06-three-d-covering-radius.md) |
| the tie test only skips degenerate triples | it discarded *every* Voronoi vertex, so `ρ` was a lower bound; repeated directions had `ρ = 0` | ties are credited; the shipped table's `ρ` is `19.8733°` | [`06-three-d-radius-is-a-lower-bound`](../kb/entries/06-three-d-radius-is-a-lower-bound.md) |
| the pair bound is within `1.3×` of the excess and tighter than the allowance | `1.501×`, against the allowance's `1.051×` | replacing the allowance with the pair would widen `2c`'s band by half | [`06-two-plane-bound-loses-to-the-allowance`](../kb/entries/06-two-plane-bound-loses-to-the-allowance.md) |
| `2c` prints `2.0%` at `205` | `2.3920%` | its rung is `254`, not `205` | [`06-density-scaling-on-two-c`](../kb/entries/06-density-scaling-on-two-c.md), [`06-density-sweep-measured`](../kb/entries/06-density-sweep-measured.md) |
| `example-1` reaches `2%` at `158` | `2.0084%`, over by four parts in `10⁴` | even the `f = 1` rung is `205` | [`06-density-sweep-measured`](../kb/entries/06-density-sweep-measured.md) |
| the inward share is `c · tan ρ`, so `f ≈ 1.30` | `f = 1.56` at `205`; the share runs `0.56–0.84 tan ρ` | `e_in` is not a `tan ρ` law, and the `205` jump is alignment | [`06-inward-share-is-not-a-tan-rho-law`](../kb/entries/06-inward-share-is-not-a-tan-rho-law.md) |
| `2c`'s printed support at block 8 is short by `0.005–0.03` | it is `0.0014` **over** the exact one | none of that block's slack is a support deficit; it is the rule | [`04-two-c-slack-is-drift`](../kb/entries/04-two-c-slack-is-drift.md) |
| the last block's shortfall is within `2×` of block 1's | `3.34×` on `2c`, and the support collapses `12×` | `2c`'s last block is drift, which no table size moves | [`06-last-block-band-is-drift`](../kb/entries/06-last-block-band-is-drift.md) |
| the sample grid's band is within `1%` of the set's | `12.5%` — then `0.1–3.3%` once the reconstruction slack was fixed | the prediction was right; the first instrument was wrong | [`06-band-between-the-samples`](../kb/entries/06-band-between-the-samples.md), [`06-band-maximum-and-the-reconstruction-slack`](../kb/entries/06-band-maximum-and-the-reconstruction-slack.md) |
| the band's maximum is the diameter of the pairwise vertex differences | `25–34×` too large (`6.000299` against `0.173854`) | the maximum is `max_ψ (h_outer − h_inner)`, the largest outer-vertex distance to the inner polygon | [`06-band-maximum-and-the-reconstruction-slack`](../kb/entries/06-band-maximum-and-the-reconstruction-slack.md) |
| `support_at` is a bound between samples | `−1.414214` at `45°` on a unit square whose support is `1.414214` | non-straddling pairs were counted; they now return `INFINITY` | [`06-cell-bound-was-not-a-bound`](../kb/entries/06-cell-bound-was-not-a-bound.md) |
| the one-term violation survives every shape up to `240` directions and falls monotonically | three of seven shapes are clean, and `13 x 12` is exact to `6.3e-14` | the failure is the grid's alignment with the body's facet normals, not its density | [`06-three-d-violation-is-alignment-not-density`](../kb/entries/06-three-d-violation-is-alignment-not-density.md) |
| the generalised recursion matches all four committed outputs to `1e-6` | `2c` differs by `9.976e-3` | part B's figure for the same blocks: it is the output's `0.01` vertex slack, not the recursion | [`06-inner-side-holds-on-the-other-cases`](../kb/entries/06-inner-side-holds-on-the-other-cases.md) |
| every inner vertex of the shifted set is outside the exact body | the `0.0109–0.0256` was the euclidean distance to the nearest edge; the inside test was inverted (the centroid read `0.448750`) | the inner set is inside at `110/158/254`; only `205` is out, by `4.8e-4` at block 1 | [`06-inner-set-leaves-the-exact-body`](../kb/entries/06-inner-set-leaves-the-exact-body.md), [`06-chamfering-gap-is-not-the-table`](../kb/entries/06-chamfering-gap-is-not-the-table.md) |
| the inner gap is a `tan ρ` chamfer that a cap shrink removes | `gap / tan ρ` runs `0.38–1.55` and rises at `254`; the shrink empties block 8 | the gap is not a table term; the `205` violation needs the exact body | [`06-chamfering-gap-is-not-the-table`](../kb/entries/06-chamfering-gap-is-not-the-table.md) |
| the printed set is uniformly outer | `example-1` is printed `2.3%` short | neither inclusion holds; the allowance is the wrong shape for `e_in` | [`01-example1-deficit`](../kb/entries/01-example1-deficit.md), [`01-deficit-is-chamfering`](../kb/entries/01-deficit-is-chamfering.md) |
| `12.4%` is the cell-radius fraction on an earlier probe set | it is `example-3d`'s absolute allowance `0.322693` over `2.603`, another case's number used as a fraction; the same table reads `17.90%` half-gap and `44.65%` at the cell radius | P1's table is rebuilt under one convention; the verdict survives, the ratio does not | [`06-allowance-fractions-need-one-convention`](../kb/entries/06-allowance-fractions-need-one-convention.md) |
| one-sided credit finds every Voronoi vertex, and the degenerate table reads its half gap `45°` | it read `54.7356°` — the pole's own cell and the pole-including triples — against a true vertex at `90°` | both poles of a degenerate triple are credited; `4` of `5` cells move, the shipped tables' maxima do not | [`06-degenerate-vertices-need-the-antipode`](../kb/entries/06-degenerate-vertices-need-the-antipode.md) |
| the outside vertex's `excess` equals the body's bulge over the shifted caps | `excess 4.78e-4` against `bulge 2.14e-4`, while the real shortfall is `1.06e-3` | the poke is `45%` of the shortfall, so the inner test is `allowance > cap − h_facet` | [`06-inner-side-violation-is-a-chamfer-shortfall`](../kb/entries/06-inner-side-violation-is-a-chamfer-shortfall.md) |
| the chart residual is quadratic in the half width, so `0.005` meets the budget under `10^5` charts | `half^0.44` locally, needing `5.7e-6` and `4.9e16` charts | charting is not the affordable route: the residual saturates at the block's displacement | [`02-no-affine-chart-bridge-closes-p1`](../kb/entries/02-no-affine-chart-bridge-closes-p1.md) |
| the Euler error stays under the per-block budget for `d >= 0.6` | `20×` at `0.60-1.00`, `8.4×` at `1.00-1.41`, `257×` near capture | the step is the wall, so P1 needs a smaller step or a discrete-time reformulation | [`02-no-affine-chart-bridge-closes-p1`](../kb/entries/02-no-affine-chart-bridge-closes-p1.md) |

## What the misses have in common

Three patterns account for most of them. **An identity that held in 2D was carried into 3D**
(the covering radius, the pair bound) — the correction is always a different object, not a
constant. **An instrument was mistaken for the quantity it measures** (the sampled band, the
reconstruction's vertex slack, `support_at`), and the fix was to measure at a tighter tolerance
or to say `INFINITY`. And **a smooth scaling law was assumed where the recursion's own loss
dominates** (`tan ρ` for `e_in`, density for `2c`'s tail), which is why the last two months of
work keep returning to stage 5 rather than to the table.

## What did not fail

For balance, the checks that came back clean: the sampled band is the set's band to
`0.1–3.3%`; the 2D inner side is inside the exact body at all `880` sampled directions of `2c`;
the density trend is real at every count; and the five committed outputs are reproduced byte
for byte by the engine after every change on this page.
