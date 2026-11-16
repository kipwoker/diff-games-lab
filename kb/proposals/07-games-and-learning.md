# Track 7 — Games and learning

| field | value |
|---|---|
| state | planned; no code |
| depends on | tracks 1, 6; tracks 4 and 5 for the P4 loop |
| feeds | rung P4 of the ladder |
| artifacts | `ml/` (training, outside the engine), `engine/src/accept.rs`, `docs/11-learning.md` |

## The question

Compute the solvability set exactly, then learn its boundary or the corresponding cost function so
a strategy can be produced in real time. Which representation is learnable — the 110 support
values, a level-set function, the boundary itself? What error does the network have against the
exact bridge? And what does it cost when a cheap exact check refuses an answer?

## Why it matters for the target problem

A bridge recomputed per engagement is not a strategy a vehicle can carry. Real-time is the last
rung: the answer has to arrive in milliseconds, and the safety guarantee must not move. The
scientific object is therefore not the network — it is the *contract* around it: propose fast,
check exactly, refuse when the check fails, and state the price of a refusal.

## Proposal

* **E1 — the dataset.** Sweep the exact engine over a grid of P1/P2 cases (initial state, horizon,
  control authority, `ΔA` box) and store `(problem, output document)` pairs, with the exact
  outputs committed. The label is the support vector per block, so the dataset is the same object
  the engine already writes — no parallel schema.
* **E2 — three representations.** (a) the 110 support values per block as a vector regression;
  (b) an implicit level-set function over the state space, DeepReach-style, trained on the
  boundary; (c) the value function itself, as a PDE residual plus boundary data. Compare at equal
  parameter count and equal training time, not at equal epochs.
* **E3 — the exact check.** Accept a predicted answer only if, for every direction, the predicted
  support lies inside the certified bracket from track 6. That is `O(m)` support evaluations, no
  network verifier needed, and it is the check the guarantee rests on. Two conditions keep it
  honest: the bracket must be *certified* (`e_out` and `e_in`, not a sampled set), and an accepted
  prediction is used only as a bound on the side the bracket guarantees — never as "the set".
  Measure the acceptance rate and the latency of the check itself.
* **E4 — the refusal path.** On refusal, fall back to the exact solver. Measure end-to-end
  latency with refusals included, and the capture rate of strategies driven by accepted answers
  against the exact-only baseline.
* **E5 — where learning stops paying.** Sweep the state dimension and report the crossover: below
  some dimension the exact solver is faster than propose-plus-check. A negative result there is a
  result, and it protects the lab from publishing an unfounded speedup.

## Build

Training lives in `ml/`, Python, with its dependencies, deliberately outside the dependency-free
engine; the engine gains only an `accept` module (the check) and an optional input mode that reads
a predicted support vector instead of computing it. Model weights are artifacts, listed in the kb
entry with a hash, never committed as blobs.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | acceptance above 99% at P1/P2 with sub-10 ms queries | P4 is reachable; PC4 is real |
| O2 | refusals cluster near the boundary, where the set is hardest | error-targeted sampling matters more than architecture; a concrete next step |
| O3 | the level-set representation generalises across control authority, the support vector does not | the representation question is answered, and it changes E2's default |
| O4 | acceptance is high but check latency exceeds the exact solver at P1 | learning pays only above a dimension; publish the crossover |
| O5 | accepted answers still produce escaped trajectories | the check's bracket is too wide — a track 6 failure surfaced by track 7, not a learning failure |

## How we would read them

* Network error alone is never reported. Every number comes with acceptance rate, check latency
  and refusal cost, because a predictor that is right 90% of the time and cannot say when is not
  usable.
* Adopt only if accepted-answer latency is at least `3×` faster than the exact solver *including*
  refusals, and the capture rate is not below the exact baseline by more than the bracket allows.
  A speedup that trades away a proven guarantee is not a speedup for this lab.
* If O5 occurs, the fixing happens in track 6 and the entry cross-links. Blaming the network for a
  wide bracket is the easiest mistake this proposal can make.
* A refusal is not a failure of the system; it is the system working. Refusal rate is reported as
  a first-class metric, next to accuracy.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `07-dataset.md` | dataset | sweep definition, case count, label schema, and the committed artifact paths |
| `07-representations.md` | measurement | three representations at matched budget: accuracy, sample efficiency, boundary behaviour |
| `07-acceptance.md` | measurement | acceptance rate, check latency, bracket width used, per case |
| `07-refusal-cost.md` | measurement | end-to-end latency with refusals, and capture rate against the exact baseline |
| `07-crossover.md` | measurement | dimension against exact-vs-learned cost, with the crossover stated |

## Publication candidates

* **PC4 — refusal-aware real-time strategies.** Learned reachability is usually verified offline or
  probabilistically; a runtime exact check with a measured refusal rate and a stated price is a
  different contract, and it is the one a vehicle would need. Prior-art check: neural reachable
  tubes with scenario or conformal verification (Lin et al., L4DC 2024), verified neural value
  functions (JAIR 2025), and POLAR-style runtime verification of neural controllers. The claim
  survives only if the check is genuinely cheaper than re-solving, at the dimension claimed.
* Secondary: a documented crossover dimension for exact-versus-learned reachability is a useful
  practical result, and an honest negative one if it lands low.

## Stopping rule

Stop when one representation has been trained on a fixed dataset, its acceptance and refusal
numbers exist for two cases, and the crossover has at least three dimensions. Do not train a
second architecture for the same representation before the first check has run end to end.
