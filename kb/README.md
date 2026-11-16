# The knowledge base

This is the lab's memory. Each research session starts with almost no context, so nothing that
matters may live only in a conversation. A proposal in [`proposals/`](proposals/) is a plan; an
entry in `entries/` is a finding. This file is the contract between the two.

## The target problem

Every track is aimed at one game, made progressively less academic: a pursuer and an evader, each
with its own bounded control, a capture set and a horizon. "Close to reality" is not a slogan
here; it is a ladder, and each rung names what the previous one simplified away.

| rung | game | what stops being academic | first result |
|---|---|---|---|
| **P0** | `ẋ = Ax + Bu + Cv`, linear, double integrator | nothing — the reference | track 1, implemented |
| **P1** | two turn-limited vehicles, in the relative frame `(d, ψ_rel, ψ_e)` | the dynamics stop being linear | track 2, grid reference |
| **P2** | P1 with a parameter box `ΔA` and unknown drag | the model stops being known | track 5 |
| **P3** | P2 with control sampled at a stated rate | the control stops being continuous | track 4 |
| **P4** | P3 answered by a learned map in milliseconds | the computation stops being offline | track 7 |

Two bounded headings are four numbers in the absolute frame, `(x, y, θ₁, θ₂)`, but rotating
position and heading together leaves the dynamics unchanged, so the game is three: `(d, ψ_rel,
ψ_e)`. The direction table applies to three, with the two angles weighted by their turning radii,
so no new construction is needed.

The chauffeur ordering — one unbounded-turn-rate heading against a point evader — was tried first
and is degenerate: the evader holds a circle of radius `v_e/ω̄` at the pursuer's maximum angular
rate, which the pursuer cannot follow, so the bridge is its capture set
([`entries/02-p1-bridge-is-the-capture-set.md`](entries/02-p1-bridge-is-the-capture-set.md)).
P1 therefore bounds the evader's turn rate too, and the case is redefined in
[`entries/02-p1-case-definition.md`](entries/02-p1-case-definition.md) up to the point the grid
reference overtook it.

Tracks 3 and 6 are instruments rather than rungs: track 3 says whether two methods agree, track 6
says how much of a claim is error. A rung is not climbed until both instruments can be pointed at
it. Beyond P4, in rough order of payoff: static obstacles (the game becomes reach-avoid), two
pursuers against one evader, measurement delay and partial state, and wind as a third disturbance
set. Each becomes a rung `P5+` when someone takes it, with the same obligations.

## What "solved" means here

A set is half an answer. The claim this lab wants to be able to make, per rung, is:

> from `x₀ ∈ W⁰` the pursuer has a strategy that reaches the capture set by `T` against every
> admissible evader control, and from `x₀ ∉ W⁰` the evader has a strategy that avoids capture
> until `T` against every admissible pursuer control.

Strategies are nonanticipative: at time `t` each player's control may depend on the history up to
`t`, not on the opponent's current or future control. Both players are assumed to know the game
and the state; uncertainty enters only where a rung puts it (P2 parameters, P3 sampling).

So a rung produces three things, and a result without all three is not a result:

* the **set** `W⁰`, as something with a claimed inner and outer bound;
* the **two strategies**, in a form that can be executed and checked;
* the **certificate**, an inequality that holds step by step, not a simulation that happened to
  stay inside.

## Layout

| path | what it is | who reads it |
|---|---|---|
| `README.md` | this contract: target, schema, rules | every session, first |
| [`INDEX.md`](INDEX.md) | one table per kind of record, and the decision log | every session, second |
| [`proposals/`](proposals/) | one plan per track, identical structure | only when starting that track |
| `entries/` | one finding per file, named by track | only when its claim is needed |

## Entry format

One claim per entry. The filename is the id: `entries/06-direction-error-2d.md` — track number,
slug, nothing else — so
`entries/06-direction-error-2d.md` belongs to track 6 and states one thing.

| field | rule |
|---|---|
| `type` | `measurement`, `method`, `counterexample`, `decision`, `external`, `dataset` |
| `track` | the track number it belongs to, or `cross` |
| `date` | ISO date of the run or reading |
| `case` | which example or configuration, named as in `examples/` or as a new case file |
| `resolution` | directions, step, grid, sample count — whatever the number depends on |
| `claim` | one sentence, falsifiable, with the number in it |
| `numbers` | a table; every figure carries its resolution in the same row |
| `evidence` | the command to reproduce it and the artifact path |
| `limits` | where the claim stops being true |
| `reuse` | one line: when to cite this, and what it rules out |

An `external` entry records what a paper actually says, in our notation, with the citation — so a
later session can use it without re-reading the paper.

## Rules that keep this library usable

* **Small.** An entry over ~60 lines, or a proposal over ~140, is a sign the claim is really two.
* **Numbers carry their resolution.** "1.6°" without "110 directions, 2D" is unusable later.
* **Lower bounds are labelled.** A Hausdorff distance sampled at 110 directions is a lower bound
  on the true distance; writing it as the distance is the most likely way this library lies.
* **Disagreement is the result.** When two methods differ, the difference is the entry; never
  average them into a third number.
* **Correct, do not overwrite.** A superseded entry keeps its file and gains a `superseded by`
  line. The decision log in `INDEX.md` says why.
* **Nothing is adopted without a bracket.** A method enters a rung only with an inner and an
  outer bound whose gap is below the rung's stated tolerance.
* **Sampled is not outer.** A reconstruction from sampled supports can fall on either side of the
  true set — track 1 has a 2.3% undershoot on a symmetric case and 2.9% overshoot on
  equal-authority squares — so each side is a claim to prove, and "outer by construction" is not
  available to any track. The sign follows how the table sits against the set's facets.

## Publication candidates

Not the goal, but a natural product. A candidate becomes real only when an entry backs it with
numbers and a prior-art note; until then it is a hypothesis with a name.

| id | candidate | track | what would make it new | status |
|---|---|---|---|---|
| PC1 | a priori direction-discretization error for support-function bridges, with adaptive refinement that certifies a Hausdorff bracket | 6 | dual/support methods are used for linear games but their sampling error is usually reported after the fact, not bounded before | retired — prior art (`entries/06-prior-art-retires-the-two-candidates.md`) |
| PC2 | the adaptivity gap: static-unknown `ΔA` (an intersection over a family) against time-varying adversarial `ΔA` (a game), measured on the same cases | 5 | the two readings are conflated in practice; the measured gap would price the assumption | hypothesis |
| PC3 | a strategy certificate that falls out of the bridge: blockwise invariance checked as an inequality, so both players' strategies come with a proof rather than a simulation | 4 | simulated trajectories are the usual evidence; a checkable certificate per step is stronger and cheap | hypothesis |
| PC4 | refusal-aware real-time strategies: a learned map plus a per-step exact test with a measured refusal rate and cost | 7 | learned reachability is usually verified offline or probabilistically; a runtime check with a stated price is a different contract | hypothesis |
| PC5 | a cross-method Hausdorff benchmark for pursuit–evasion, with byte-stable outputs and stated resolutions | 3 | comparison is usually qualitative; a fixed case set with numbers is reusable by others | hypothesis |
| PC6 | a certified linearization error for nonlinear games: the blockwise remainder that lets a linear bridge stand in for a nonlinear one | 2 | if the remainder is tight enough to be useful at P1, it replaces a grid with an inequality | hypothesis |

Before any of these is called new, the proposal's own prior-art check must run and be recorded as
an `external` entry. A candidate killed by prior art is still a successful entry.

## Reading order

1. This file, then `INDEX.md`.
2. If the question is about one track, read only that proposal, then only the entries it names.
3. If the question is cross-cutting, read the entries first and let them point at proposals.
4. Never load `proposals/` wholesale to "get context": their value is that they are written to be
   read one at a time.
