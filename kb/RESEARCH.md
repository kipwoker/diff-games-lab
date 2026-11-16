# Research tracks

**Diff Games Lab** is a research playground for differential games, not a single-purpose solver.
The objects of study are stable bridges, the methods that approximate them, and the strategies
they imply. One track is built out — the linear support-function track in `engine/`, `docs/`,
`examples/` and `viewer/`. Track 4 carries a certificate over its shells, track 2 has its case, a
grid reference and a measured step wall, and track 6 has its instruments written up in `entries/`;
tracks 3, 5 and 7 are planned.

Paths below are written from the repository root.

## Tracks

| track | question | state |
|---|---|---|
| **1. Linear bridges, support functions** | for `ẋ = Ax + Bu + Cv` with compact convex `P`, `Q` and a polyhedral target, what is the maximal stable bridge, and how faithfully can a 110-direction support grid carry it? | implemented — `engine/`, `docs/`, `examples/`, `viewer/` |
| **2. Non-linear games** | for `ẋ = f(x, u, v)`, which steps of track 1 survive — the support-function algebra, the interval map, the alternating sum — and what replaces the ones that do not? | case, grid reference and a measured step wall; no engine code |
| **3. Alternative constructions** | can the same bridge be built by other means — ellipsoidal inner and outer bounds, gridding, solver-backed reachability — and do the answers agree on track 1's examples? | planned |
| **4. Winning strategies** | given a bridge, which positional strategy realises it, how is the partition chosen, and does the trajectory stay inside the bridge against adversarial controls? | certificate built — `--certify`; rule and scope in `kb/entries/04-*` |
| **5. Unstable parameters** | for `ẋ = (A + ΔA)x + Bu + Cv`, what happens to the bridge when `ΔA` is not known exactly — as another choice of the second player, and as a fixed but uncertain drift? | planned |
| **6. Method evolution** | what a modern version of the approach owes: a direction-discretization error, adaptive grid refinement, a support-function error, and a guaranteed inclusion check | measured instruments — bands, covering radius, alignment; no engine direction-error code |
| **7. Games and learning** | compute the solvability set exactly, then learn its boundary or cost function so a strategy can be evaluated in real time without giving up the safety guarantee | planned |

Track 1 is the reference: implementation, a theory that matches it, examples with analytic
answers, and a viewer. A track qualifies as implemented only when it has all four.

## Track briefs

The newer tracks are scoped here before any code exists, so their first result is a question
rather than a refactor. Each one's plan — its experiments, the outcomes that would count, how
they would be read, and the entries it must leave behind — is in [`proposals/`](proposals/).

### 5. Unstable parameters

$$\dot x = (A + \Delta A)\,x + B\,u + C\,v,$$

with `ΔA` not known exactly. Two readings must be kept apart. If `ΔA` is something the second
player may choose, it belongs in `C v` and track 1 already answers it. If `ΔA` is an unknown but
fixed parameter, the bridge is an intersection over a family of games, which track 1 does not
compute. The drift enters the recursion in exactly one place — the interval map `A_k` — so the
cheapest first experiment is the alternating sum over the extreme points of a box of `ΔA`,
intersected. Instability matters twice there: a drift with eigenvalues of positive real part
grows every shell, so the evidence worth having is where the nominal bridge is non-empty and the
perturbed one is not, and how fast that boundary moves with `‖ΔA‖`.

### 6. Method evolution

What a modern version of the approach owes, whichever track computes the bridge:

* **a discretization error for the directions** — a bound that predicts the measured 1.6° gap in
  2D and 18° at the poles in 3D, instead of a floor discovered after the fact;
* **adaptive grid refinement** — directions added where the support function bends, so a `z`
  face is capped by a sampled plane rather than bulging between the two nearest samples;
* **an error for the support function itself** — the dual update accepts half-spaces with `0.01`
  of slack and the `co` envelope is an outer approximation; both should be quantified per block,
  not left as constants of the method;
* **a guaranteed inclusion check** — the printed `W^k` should arrive with a certificate that it
  is contained in the true set, or contains it, tested rather than assumed.

This is cross-cutting: it applies to every other track, and track 1 is where the numbers to beat
already exist.

### 7. Differential games and machine learning

Compute the solvability set with the exact method, then train a network to approximate its
boundary or the corresponding cost function. The scientific objective is real-time strategy
computation: a game whose bridge is expensive to recompute becomes one query to a trained model.

The safety guarantee cannot move, so the architecture has to be a proposal and a check: the
network answers in milliseconds, and a cheap exact test — an inclusion check from track 6, or an
interval bound on the support values — either accepts the answer or refuses it. What is still
open is the representation that is learnable (the 110 support values, level sets, or the boundary
itself), how the error is measured against the exact bridge (Hausdorff, as in track 3), and what
a refusal costs in the loop that consumes the answer.

## What a track owes

| layer | obligation | lives in |
|---|---|---|
| theory | what is computed and why it is correct, chapter by chapter | `docs/` |
| implementation | the computation itself, with no dependency unless there is no alternative | `engine/` |
| evidence | problems whose answers are known independently, plus tracked outputs | `examples/` |
| inspection | a way to look at a result instead of only asserting it | `viewer/` |
| record | what was learned, one claim per file, so a later session need not re-derive it | `kb/` |

## Conventions that keep tracks comparable

* **One input format.** A game is a text file in the grammar of `engine/README.md`. A new track
  extends that grammar with optional blocks rather than inventing a second one, so any example
  loads in any implementation.
* **Tracked outputs are byte-stable.** `examples/out/` is committed and regenerated by the
  script in `examples/README.md`. A different method produces a diff, and the diff is the
  comparison.
* **The viewer reads documents, not methods.** It is handed an output file and draws what the
  header and blocks say; a new track becomes visible by making its output say the same things.
* **Tracks are additive.** A new track adds modules, chapters and examples. Track 1's numbers
  are the regression baseline and do not move.
* **Claims carry numbers.** An accuracy or cost statement names the case, the resolution and the
  measured error, as `docs/06-results.md` does for track 1.

## Open questions

* **Where does linearity actually enter?** The interval map `A_k` is a matrix exponential and its
  integral; everything downstream is support-function algebra. A non-linear track may reuse most
  of the engine and change only `A_k` — or it may not, and the answer is the track's first result.
* **How fine must the direction set be for a given tolerance?** Track 1's floor — 1.6° in 2D, 18°
  at the poles in 3D — is a property of 110 samples, not of the method. A track-3 construction
  should predict that floor instead of measuring it after the fact.
* **Can a strategy be certified, not just observed?** A simulated trajectory is evidence; an
  inequality over the partition, checked at every step, would be a proof.
* **Which comparisons are honest?** A bridge is a set, so agreement between two methods should be
  measured as a Hausdorff distance between the support functions both already carry, at a stated
  resolution.
* **How large may `ΔA` be before the bridge stops being the bridge?** The two readings of track 5
  give different sets, and the interesting failure is not a smaller bridge but a nominal one that
  is non-empty while the perturbed one is not.
* **What does a refusal cost?** A learned boundary is only usable with the exact method still able
  to check it, so track 7 has to report the per-answer cost of that check, not only the network's
  error.

## Status

| part | state |
|---|---|
| engine | builds; `cargo test` green; five tracked example outputs |
| docs | seven chapters, one per stage of the engine, plus a brief and a note of record |
| viewer | any 2D or 3D output; computes through the local server; example dropdown, drop zone, recording |
| next | P1 needs a discrete-time reformulation or a per-cell allowance under `13.04%` (`4.77%` on the angle axis); no measured table clears either |

## References

The theoretical and practical context the tracks are meant to be read against, grouped by what
this lab would take from each one.

**The value and the reachable set**

* S. Bansal, M. Chen, S. Herbert, C. J. Tomlin, “Hamilton-Jacobi reachability: A brief overview
  and recent advances”, *2017 IEEE 56th Annual Conference on Decision and Control (CDC)*,
  pp. 2242–2253, [doi:10.1109/CDC.2017.8263977](https://doi.org/10.1109/CDC.2017.8263977),
  [arXiv:1709.07523](https://arxiv.org/abs/1709.07523). The tutorial this lab's formulation
  follows: reachability as a value function, the two-player zero-sum game behind the Hamiltonian,
  and the level-set toolbox that discretises it. Its account of the state-variable curse is the
  baseline track 6 measures its own error bounds against.
* G. Barles, E. Chasseigne, *An Illustrated Guide of the Modern Approaches of Hamilton-Jacobi
  Equations and Control Problems with Discontinuities*, [arXiv:1812.09197](https://arxiv.org/abs/1812.09197)
  (book project, 2018–2023, v4). Viscosity and stratified solutions for control problems with
  discontinuities and state constraints — the analytic backdrop for carrying a bridge as a set
  rather than a value function.
* M. Althoff, G. Frehse, A. Girard, “Set Propagation Techniques for Reachability Analysis”,
  *Annual Review of Control, Robotics, and Autonomous Systems* 4:369–395 (2021),
  [doi:10.1146/annurev-control-071420-081941](https://www.annualreviews.org/content/journals/10.1146/annurev-control-071420-081941).
  How guaranteed over-approximations are propagated from set to set; the yardstick for track 6's
  error bounds and inclusion checks.

**Uncertain dynamics**

* Y. Huang, E. Luo, S. Bak, Y. Sun, “Reachability analysis for linear systems with uncertain
  parameters using polynomial zonotopes”, *Nonlinear Analysis: Hybrid Systems* (2025),
  [doi:10.1016/j.nahs.2024.101571](https://doi.org/10.1016/j.nahs.2024.101571)
  ([ScienceDirect](https://www.sciencedirect.com/science/article/abs/pii/S1751570X24001080)).
  Linear dynamics whose parameters are only known to lie in a set, propagated as polynomial
  zonotopes — the closest neighbour to track 5's reading of `ΔA` as a family.

**A game in an application**

* Z. Jia, D. Ye, Y. Xiao, Z. Sun, “Approximate Analytical Approach for Spacecraft Pursuit–Evasion
  Game With Reachability Analysis”, *IEEE Transactions on Aerospace and Electronic Systems* 61(4)
  (2025), [IEEE Xplore 10930437](https://ieeexplore.ieee.org/document/10930437). A spacecraft
  pursuit–evasion problem posed as a two-player zero-sum differential game and answered through
  reachability — what a computed bridge is for once the adversary is a real vehicle.

**Learning with a guarantee**

* M. Ganai, S. Gao, S. Herbert, “Hamilton-Jacobi Reachability in Reinforcement Learning: A
  Survey”, [arXiv:2407.09645](https://arxiv.org/abs/2407.09645), IEEE Open Journal of Control
  Systems (2024). How learning and exact reachability are already combined: a reachable set
  supervises and verifies the policy — the pattern track 7 proposes, with the certificate made
  part of the loop rather than a post-hoc check.
