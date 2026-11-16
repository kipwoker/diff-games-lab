# Track 4 — Winning strategies

| field | value |
|---|---|
| state | E1 revised, E3 and E4 built; the predictive rule leaves no escape on `3d` |
| depends on | track 1 (bridge, outputs), track 6 (bound for the continuum) |
| feeds | rung P3 of the ladder; track 5's adversarial reading; track 7's closed loop |
| artifacts | `engine/src/strategy.rs` (built), `examples/strategy-*.txt`, viewer strategy overlay |

## The question

Given a bridge, which positional strategy realises it, how is the partition chosen, and does the
trajectory stay inside the bridge against adversarial controls? Stated as the lab's success
criterion: can both players' strategies be *certified*, rather than demonstrated by a simulation
that happened to work?

## Why it matters for the target problem

This is the track where "winning strategies for both players" stops being a slogan. The bridge
already encodes both sides — inside it the pursuer can force capture, outside it the evader can
avoid — but that statement is about sets. Turning it into two executable controllers, with an
inequality that holds at every step, is the difference between an analysis and an answer.

## Proposal

* **E1 — the strategy rule.** From the dual form, at block `k` each player moves to maximise its
  own support in the block's active direction: the pursuer takes
  `argmax_{u∈P} ⟨ψ, A_k B u⟩`, the evader `argmax_{v∈Q} ⟨ψ, A_k C v⟩`, with `ψ` the direction in
  which the state is closest to the boundary. Write this as a proposition first: the  direction choice is where the subtlety is, and the experiment is whether the simple choice
  realises the bridge. *(Built: the choice that certifies is the counter-direction vertex, and
  the extremal control is a face, not a point — `04-strategy-rule.md`.)*
* **E2 — the partition.** Strategies are constant per block; test whether they need sub-block
  resolution by using the cells of the reconstructed polyhedron rather than the block as the unit.
  Measure the loss from a coarser partition at fixed `T`.
* **E3 — adversarial search.** Drive the closed loop against: extremal bang-bang evader;
  `10⁴` sampled evader sequences; a gradient or CMA-ES style search for an escaping sequence; and
  the support test over all 110 directions. The first three propose, the fourth decides.
  *(Built as `--search`: random sequences plus coordinate descent, deterministic from `--seed`.
  It beats the extremal evader on all five cases and escapes `3d` — `04-adversarial-search.md`.
  The escape was the rule's: `Rule::Predictive` removes it at seeds 1-3 — `04-predictive-rule.md`.)*
* **E4 — the certificate.** At every step check `x_k ∈ W^k` as
  `max_ψ (⟨ψ, x_k⟩ − h_{W^k}(ψ) − e_out(ψ)) ≤ 0` over the sampled directions, where `e_out` is
  track 6's outward allowance. Without it the test is optimistic: track 1 has a case whose stored
  supports are 2.3% short, so a state just outside the true bridge could pass. The test is
  necessary but not sufficient for the continuum until `e_out` covers the directions between
  samples, and every run says which of the two it is. Record the worst slack, not just pass/fail.
  *(Built for the sampled directions with `--certify`; two of five cases fail and the two failures
  are localised in `04-certificate-scope.md`.)*
* **E5 — sampled control (P3).** Recompute the strategy at 50 ms, 100 ms and 200 ms and measure
  the capture rate against the rate. A real autopilot updates at a rate; the curve says what the
  game tolerates.

## Build

A `strategy` module: given a block's support vector, return both controls and the active
direction; a driver mode that simulates the closed loop and writes a trajectory document (a
second output kind, with a header the viewer can draw as an overlay). The simulation is
double-precision and uses the same interval map, so a certificate failure cannot be blamed on
mismatched integrators.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | certificate holds at every step of every run, with slack that shrinks as directions grow | PC3 is real: the strategy comes with a proof, not a movie |
| O2 | failures only at corners of `P`/`Q` or at block boundaries | the loss is the dual update's, and track 6's refinement fixes it |
| O3 | adversarial search finds an escape inside the claimed bridge | the bridge is an outer approximation and the claim was too strong; a counterexample entry |
| O4 | capture rate falls sharply below a rate `f*` | a quantified control-rate requirement — the most directly useful number this lab can give a practitioner |
| O5 | sub-block strategies change nothing measurable | the block is the right partition; record it so nobody re-derives it |

## How we would read them

* "Winning" requires the certificate, not the simulation. A run that stayed inside is reported as
  *observed*, with the worst slack; the word certified is reserved for a check whose sampling gap
  is bounded by track 6.
* Any escape found is recorded as a counterexample with the exact state, block, direction and
  slack. It is never patched by loosening a tolerance; the counterexample is the result.
* The rate curve is reported as capture probability against `Δt` with the number of trials, not
  as a threshold. A single run at one rate is an anecdote.
* If O2 holds, this track's failure and track 3's corner finding are the same phenomenon, and the
  entries are cross-linked rather than duplicated.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `04-strategy-rule.md` | method | the rule, the direction choice, and the proposition it tests |
| `04-certificate-scope.md` | measurement | worst slack per example, and what remains unproven for the continuum |
| `04-adversarial-search.md` | method | search used, trials, and the escaping sequences found or excluded |
| `04-rate-curve.md` | measurement | capture rate against control period, trials per point, case |
| `04-escape-counterexample.md` | counterexample | state, block, violated direction, slack — if one is found |

## Publication candidates

* **PC3 — a strategy certificate that falls out of the bridge.** The blockwise invariance check is
  cheap, exact in the sampled directions, and gives both players their strategy in the same
  pass. Novelty rests on it being a certificate rather than a simulation; the prior-art check is
  viability-kernel and reachability-based controller synthesis, where similar inequalities appear
  for single-player invariance. The two-player, both-sides version is the claim.
* Secondary: O4's rate curve is a useful empirical contribution even without theory — "how fast
  must the loop run for a certified strategy to remain certified".

## Stopping rule

Stop when both strategies exist for the five tracked cases, the certificate has been checked on
all of them, at least one adversarial search has failed to find an escape, and the rate curve has
three points. An escape counterexample stops the track until it is explained.
