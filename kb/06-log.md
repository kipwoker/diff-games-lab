# Index log (archive)

The chronological narrative that `INDEX.md` carried through 2026-11-13, before it was compacted
into state, live numbers and superseded claims. Kept because the wording records what was
believed at each step; not read every session.

Track 4's rule and certificate are built (`entries/04-strategy-rule.md`,
`entries/04-certificate-scope.md`): three of five tracked cases certify, and both failures are
localised rather than repaired. The 3D bound is tightened and its one-term form corrected
(`entries/06-two-neighbour-bound.md`): the two-neighbour formula matches the measurement, the single
term is false by up to `0.021`. The `e_out` that the certificate would need therefore exists in 2D
and is still a measurement in 3D, which is P1's dimension. The redefined case now has its first grid reference
(`entries/02-p1-relative-interior.md`): bounding the evader's turn rate restores an interior,
`1.233 m` at `psi_e = 0` against the `1.3 m` closure limit, a figure that refining the `d`
axis moves to `1.275` (`entries/02-p1-refinement.md`) — the interior holds, its margin is smaller
than first reported. The adversarial search has run and
found the extremal evader wanting (`entries/04-adversarial-search.md`): a sequence raises the slack
on all five loops and escapes `3d`, so a certificate over one trajectory is no longer enough. The grammar now exists too: the case parses and the engine refuses it by
name (`entries/02-nonlinear-grammar.md`), so what is left is the method, not the input. Adaptive refinement has one matched-cost number: bisection beats uniform `4.3×` at `110`
directions, with a 3D subdivision still to build (`entries/06-adaptive-refinement.md`). The
certificate's allowance is no longer typed: it is computed from the table, and the document
says whether it is a validated bound (2D) or an estimate (3D), which is the difference between
`certified` and `certified as estimate` on `example-3d` (`entries/06-two-neighbour-3d.md`).
The deficit's sign is now measured rather than assumed: the printed set is over at the true body's
facet normals and short at its vertex normals, so neither inclusion holds on any of six tables
(`entries/01-deficit-is-chamfering.md`) — the certificate's single-signed allowance is the wrong
shape, and `e_in` is needed alongside `e_out`. The rule now answers a deviation:
scoring every pursuer vertex one block ahead removes the `3d` escape entirely
(`entries/04-predictive-rule.md`), so what remains uncertified is the allowance, not the strategy. The bracket is two-sided in 2D now:
shifting every sampled half-space inward by its own allowance narrows the band `10.8×` and stays
inside the true body at all 24 probes (`entries/06-two-sided-bracket.md`), while 3D remains a
`14.8%` estimate. The 3D estimate is the table's shape rather than its
count: an allowance-greedy table at the same `110` directions is `4×` smaller, and at `240` it is
`1.6%` of the support, under the `0.113` the `3d` deficit needs (`entries/06-three-d-adaptive-allowance.md`). P1 is where that bites: its interior survives
the angle axes unchanged (`entries/02-p1-is-not-yet-certifiable.md`), but a certificate needs a
bridge P1 does not have and an allowance under its `0.025` margin, which the shipped table misses
by `6x` and an adaptive `240` table would meet. The inward term is now computed rather than
assumed: it is the dual update's own minorisation loss, `0.000000` at the direction where `2c`
leaves the set and `2.07%` at `3d`'s (`entries/06-two-sidedness.md`), so `2c` is a rule failure and
`3d`'s pass is weak. The 3D allowance's `ρ` was wrong as well as unproven: half the
nearest-neighbour gap is a cell's covering radius in 2D only, and the shipped table's cells reach
`19.3787°`, so its allowance is `0.945986` rather than `0.322693` and P1's margin is `19×` short
rather than `6×` (`entries/06-three-d-covering-radius.md`). And the bridge P1 would need is priced:
a charted linearisation's affine residual is `0.131` at a `0.02` half-width against a `6.25e-4`
per-block budget, while the grid's own Euler step is already `0.031` in the outer band
(`entries/02-p1-bridge-is-not-affordable.md`) — so `Step` comes before the bridge. The band and its
price are now one table: `2.8160%` of the diameter on four 2D cases against `29.2171%` on `3d`, for
`0.7%` of a run (`entries/06-bracket-and-cost.md`). `2c`'s slack is not constant either: direction
27 takes over at block 5 and the increments fall from `0.0085` to `0.0036`, so at least `0.0182` of
the final `0.0659` is drift and the rest needs an exact support
(`entries/04-two-c-slack-is-not-constant.md`). P1's gate moved with its `Step`: `d*` converges to
`1.150` and the margin to `0.150`, so the table is `3×` out rather than `19×`
(`entries/02-p1-step-refinement.md`). The adaptive table does not survive its own metric: measured
with the covering radius it is worse than the shipped grid (`39.38°` against `19.38°`), because the
greedy can never place a direction in a hole (`entries/06-adaptive-table-under-the-covering-radius.md`).
The 3D allowance has now been measured against analytic bodies instead of believed: the worst excess
is `27–69%` of what it charges (`entries/06-three-d-allowance-is-loose.md`), so it is loose by
1.5–3.7× — and P1's gate is `2.8×` away on the *measured* error, which means the table is the
blocker and tightening the bound would not certify it. Measuring it exposed the instrument too: the
shipped 3D table has ten repeated directions whose cells carried `ρ = 0`, and the enumerated
maximum is a lower bound rather than the exact radius it was called
(`entries/06-three-d-radius-is-a-lower-bound.md`). `2c`'s open question is closed the same way:
exact polygon arithmetic reproduces the committed output to six decimals and puts the printed
support at direction 27 `0.0014` *over* the exact one, so the whole `0.0659` slack is drift and the
trajectory is `0.0645` outside the true bridge (`entries/04-two-c-slack-is-drift.md`), which makes
`2c` a rule failure and nothing else. The rung's `2%` has also been derived rather than measured
against: a band is `0.85–1.30` times `tan ρ` of the diameter, so `2%` costs `m ≥ 158` in 2D and about
`3.2·10⁴` in 3D at `O(m²)` a substep, and the tolerance belongs in units of `tan ρ` rather than in
percent (`entries/06-rung-tolerance-is-a-table-size.md`). The covering radius is now exact, and that
reverses a verdict: keeping only centres with a unique nearest sample had discarded every Voronoi
vertex, so `ρ` was a lower bound by construction. Crediting the ties gives `19.8733°` at `110` and
`13.3967°` at `16 x 15`, moves `example-3d`'s band to `1.999277` (`30.5916%`), and then the cube at
`16 x 15` **falsifies** the one-term allowance at one probe (`104.6%` of the bound) —loose by `1.5–4.0×` where it holds and simply false at that cell
(`entries/06-three-d-radius-is-exact.md`). The per-cell alternative has now been measured and lost:
on `2c`'s own cells the one-term allowance is `1.05×` the true excess where the band is set while the
two-plane bound is `1.50×`, so the pair would widen the band by half — and the true band at that cell
is already `2.017%` of the diameter, the remaining `1.6%` being inward
(`entries/06-two-plane-bound-loses-to-the-allowance.md`). Density has now been measured instead of
assumed: at `2c`'s band-setting block the allowance charges `0.723–0.742 tan ρ` of the diameter and
the true excess `0.706 tan ρ` flat, so `2c` prints `2.567%` at `158` and `1.967%` at `205` — its rung
needs `205`, and the `158` belongs to the `f = 1` cases
(`entries/06-density-scaling-on-two-c.md`). The 3D falsification now has a location and a repair: the
one-term allowance is exceeded only at a cell's vertex — one probe in `2500` on the cube at `16 x 15`,
at `1.000` times its covering radius — where three planes set the support, and the three-plane vertex
is a valid bound and exact there, though only `89%` of probes have a straddling triple, so 3D stays
an estimate (`entries/06-three-d-bound-fails-at-a-cell-vertex.md`). The 2D count is now a run
parameter: `--directions n` builds `n` evenly spaced directions for any `n ≥ 3`, `110` reproduces
the shipped table bit for bit and every committed output, while 3D stays on its `11 x 10` shape
(`entries/06-two-d-table-is-a-parameter.md`). The rung is now measured: at `158` even `example-1`
prints `2.0084%`, and `2c` prints `2.3920%` at `205` where `1.967%` was projected, so its rung is
`254` at the first block — while its last block stays at `17–20%` at every count, unreachable by
table size (`entries/06-density-sweep-measured.md`). The inward share is not a `tan ρ` law either: on
`2c` it reads `0.561`, `0.578`, `0.836`, `0.615` times `tan ρ` at the four counts, so `f` is
`1.30–1.34` except at `205`, where it is `1.56` — the table the sweep expected to pass
(`entries/06-inward-share-is-not-a-tan-rho-law.md`). The band's two terms are now separated at
the block they were printed for: `2c`'s last block is `92.6%` shortfall against `24.2%` at its
first, while the other three cases' last blocks are `0–6.4%` shortfall and follow the table — so
`--directions` is not the lever on `2c`'s tail (`entries/06-last-block-band-is-drift.md`). The cell
bound was repaired on the way: `support_at` minimised over pairs that do not straddle the probe,
returning `−1.414214` at `45°` on a unit square where the support is `1.414214`, and a finite number
on a generic 3D probe; it now counts only straddling pairs and says `INFINITY` otherwise
(`entries/06-cell-bound-was-not-a-bound.md`). The band is a sample figure: probing 1440 angles
finds the outer-inner gap up to `12.5%` wider than the widest sample, so a quoted band is a lower
bound and the `2%` rung is `3.28–3.87%` away at `110`, not `3.01–3.72%`
(`entries/06-band-between-the-samples.md`). The band direction is a short direction: the printed
support there is below the exact one at all eight `2c` blocks, so the inner side needs no charge and
is inside the exact body at all `880` sampled directions — while at block 8 the *outer* side is also
inside the truth, by `0.003756`, so the `20.4%` band is not an uncertainty about the true set
(`entries/06-band-direction-is-a-short-direction.md`). The between-sample excess was the
instrument: at a `1e-9` vertex tolerance the exact band is `1.0008–1.0333×` the sampled one, so the
earlier `12.5%` was the `0.01` acceptance slack, and `2c`'s rung stays `254` at the first block
(`entries/06-band-maximum-and-the-reconstruction-slack.md`). The inner side is sound as a set too, once measured with a facet-normal test: the earlier "every
vertex outside" was a distance to the boundary from an inverted inside test, and the vertices are
inside at `110/158/254` with a `4.8e-4–7.8e-4` violation only at `205`
(`entries/06-inner-set-leaves-the-exact-body.md`, `entries/06-chamfering-gap-is-not-the-table.md`).
Nor is the gap a table term: `gap / tan ρ` runs `0.38–1.55` and rises at `254`, so a cap shrink is
not the repair and the `205` violation needs the exact body, not the table
(`entries/06-chamfering-gap-is-not-the-table.md`). The exact recursion now reads a case file, and
the inner side holds on the other cases: zero support violation at every block of `example-1`,
`2a`, `2b` and `2c` at `110` directions, so the `205` violation is that count's rather than the
cases' (`entries/06-inner-side-holds-on-the-other-cases.md`). The rung is now filed at the exact instrument:
`205` misses for all four cases and `254` passes for all four at block 1, while the only block
still over `2%` at `254` is `2c`'s last, which is drift
(`entries/06-rung-is-254-at-the-exact-band.md`). 3D's table shape is a run parameter too: `--table
16x15` takes `example-3d`'s band from `1.999277` to `1.307773` at the same radius part M measured,
and the source stays `estimate` because the one-term form is falsified at a cube's cell vertex
whatever the shape (`entries/06-three-d-table-shape-is-a-parameter.md`). The triple's coverage is
now counted: it bounds `84–89%` of probes and is exactly tight at the one-term's only failure, so
the 3D statement is "a bound on `89%`, an estimate on `11%`" — the engine still uses the one-term
(`entries/06-three-d-max-bound-leaves-the-flank-unproved.md`). The triple is now in the engine, and
so is the measurement that it cannot be charged: `max(one-term, triple)` widens the 3D band from
`1.999277` to `4.631082` at `11 x 10`, because the three-plane apex is far everywhere except a cell
vertex, so the band stays the one-term form and the triple is a conditional bound
(`entries/06-triple-is-a-bound-but-not-a-tight-one.md`).

