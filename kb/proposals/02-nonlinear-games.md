# Track 2 — Non-linear games

| field | value |
|---|---|
| state | E1's grammar built; the case parses and is refused by name |
| depends on | track 1 (baseline), track 6 (error bound) |
| feeds | rung P1 of the ladder; tracks 3, 4, 5, 7 all assume a P1 case exists |
| artifacts | new `engine/src/nonlinear.rs`, `docs/07-nonlinear.md`, `examples/p1/example-p1.txt` |

## The question

For `ẋ = f(x, u, v)`, which steps of track 1 survive: the support-function algebra, the interval
map, the alternating sum? Concretely: at P1 (planar pursuit with speed and turn-rate limits), can a
blockwise linearization with a certified remainder stand in for the interval map — and if not,
what does the first honest answer look like?

## Why it matters for the target problem

P0 is linear because a double integrator is what makes the support algebra exact. A real pursuer
turns at a bounded rate and moves at a bounded speed; that is the first thing a practitioner would
refuse to accept about P0. If the method cannot leave linear dynamics, the lab can describe toy
games only, and "winning strategies closer to reality" stops at the first rung.

## Proposal

* **E1 — fix the P1 case.** *(Case fixed, then found degenerate.)*
  `examples/p1/example-p1.txt`: pursuer at `1 m/s` with turn rate `1 rad/s`, evader a point at
  `0.7 m/s` free to turn, capture at `r = 0.1`, `T = 4`, `Step = 0.1`. `θ` is an angle, so it
  needs a metric: `scale_theta = v_p/ω̄ = 1 m/rad`, with the 110 directions evaluated at
  `(ψ_x, ψ_y, ψ_θ / scale_theta)`. Two anchors are hand-checkable: `d ≤ r` is in the bridge, and
  `(2, 0, 0)` is out at `T = 4` because closure is capped at `0.3 m/s`, needing `6.33 s`.
  **The grid reference then emptied the case**: a point evader escapes by curvature, so the
  bridge is the capture disc (`02-p1-bridge-is-the-capture-set.md`). It also corrected the frame:
  two headings are `(d, ψ_rel, ψ_e)` — three numbers, not the four that `P1x` was deferred for —
  so once the evader's turn rate is bounded the table applies as it is. *(Grammar built: `nonlinear` and `grid` parse, `w_e < w_p` is enforced, and the
driver names the missing interval map — `02-nonlinear-grammar.md`. The case itself is
`examples/p1/example-p1b.txt`.)*
* **E2 — the exact answer, outside the engine.** Compute the same game's bridge on a 3D grid with
  an external HJI solver in `tools/` (helperOC, `hj_reachability` or OptimizedDP — any of them,
  recorded with version and grid). This is ground truth for the comparison, is never shipped in
  `engine/`, and its outputs are committed so a later session need not re-run it.
* **E3 — which operation breaks first.** Propagate a square through the unicycle flow for one
  step and measure how far the image is from convex. Convexity is the assumption the support
  representation cannot state; if the image is visibly non-convex, no amount of linearization
  saves the representation and the finding is structural rather than numerical.
* **E4 — certified linearization.** Linearize `f` per block about the centroid of the current
  set, with a remainder from interval arithmetic over the block's reachable box. Feed the
  linearized interval map to the existing pipeline. Compare with E2's grid, in Hausdorff on a
  common direction set.
* **E5 — the classical check.** Homicidal chauffeur (car with bounded turn rate against a
  pedestrian with bounded speed): the textbook pursuit–evasion game, 3D, with a long literature.
  If the pipeline cannot reproduce the known qualitative picture there, it will not be right
  anywhere.

## Build

A `dynamics` option in the grammar (`linear` default, `nonlinear` with named fields), a module
that evaluates `f`, its Jacobian and an interval remainder per block, and a branch in the driver
that chooses the interval map. The direction table is not new: `scale_theta` is a diagonal metric,
not a fourth table. The viewer needs no change: if the output document says the same things, it
draws the same things.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | linearization remainder is small at the P1 scale, bridge within 2% of the grid | P1 is answered by an inequality, not a grid; PC6 has a candidate |
| O2 | remainder exceeds the tolerance at every usable step | linearization is retired for P1; the grid becomes the P1 method |
| O3 | the one-step image is strongly non-convex | the representation, not the map, is the obstacle; convex over-approximation becomes an explicit, measurable error |
| O4 | grid ground truth is intractable at 3D within the budget | reduce to 2D fixed-heading geometry and say so in the entry; the rung waits |
| O5 | homicidal chauffeur reproduces qualitatively but not quantitatively | the pipeline is sound but the sampling is too coarse at P1 — a track 6 input |
| O6 | the bridge changes shape when `scale_theta` changes | the support representation is metric-dependent at P1; the scale must be justified physically (the turning radius), not tuned for a nicer picture |

## How we would read them

* A method is adopted for P1 only if its Hausdorff gap to the grid is below 2% of the set
  diameter at the finest resolution the grid allows. "Looks similar in the viewer" is not a
  reading; the number is.
* If O3 holds, the convex hull of a non-convex image is an over-approximation, so the P1 bridge
  becomes an *outer* bound only. That changes what may be claimed: a strategy reconstructed from
  it may be conservative, and the entry must state that the evader's side is then the trustworthy
  one.
* A failed qualitative reproduction (O5's bad case) invalidates the case, not the method, until
  the grid is refined once more. Two failures at the finest grid are a method finding.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `02-p1-case-definition.md` | dataset | the P1 file, its parameters, and what is known analytically or classically about it |
| `02-nonconvexity.md` | measurement | one-step image of a square, deviation from convex, as a function of step |
| `02-linearization-remainder.md` | method | the remainder formula, its interval evaluation, and the measured gap to the grid |
| `02-grid-setup.md` | dataset | solver, version, grid size, extent, and the committed ground-truth output |
| `02-homicidal-chauffeur.md` | external | what the classical game says, in our notation, and whether the pipeline matches |

## Publication candidates

* **PC6 — a certified linearization error for nonlinear games.** New only if the remainder is
  tight enough at P1 to replace a grid. The prior-art check is the blockwise-linearization and
  Taylor-model literature, where remainders are standard for *reachability* but not usually
  carried through a support-function game. If O2 holds, write the negative result instead: which
  nonlinearity defeats the dual representation, with numbers.
* A secondary candidate: the convexity break in E3, if it can be quantified per step, is a clean
  statement of why support-function games are linear-only in practice.

## Stopping rule

Stop when either a P1 bridge with a stated bracket exists, or two independent attempts at E4 have
failed and E3 explains why. A third attempt at the same idea is not research; it is hoping.
