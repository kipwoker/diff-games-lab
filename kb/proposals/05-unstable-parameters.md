# Track 5 — Unstable parameters

| field | value |
|---|---|
| state | planned; no code |
| depends on | track 1 (recurrence), track 4 (adversarial strategies) |
| feeds | rung P2 of the ladder |
| artifacts | `engine/src/uncertain.rs`, `examples/example-p2.txt`, `docs/09-uncertain-parameters.md` |

## The question

For `ẋ = (A + ΔA)x + Bu + Cv` with `ΔA` known only to lie in a set, what happens to the bridge —
and are the two readings of `ΔA` interchangeable? The first reading is a fixed but unknown
parameter: one strategy must work for every element of the family, so the answer is an intersection
over games. The second is a time-varying adversary: `ΔA(t)` may be chosen against us at every
instant, which is a different game with a different answer. Both are called "uncertain `A`" in
practice. The track measures the difference.

## Why it matters for the target problem

Every real pursuit problem has coefficients that are only approximately known: mass, drag, turn
authority. A bridge computed for the nominal `A` is silent about whether it survives a 10% error
in those numbers. And the choice between the two readings is physical, not mathematical — a slowly
varying coefficient is static, a gust is time-varying — so the lab has to price the assumption
rather than pick one.

## Proposal

* **E1 — static reading by intersection.** Take a box of `ΔA` around track 1's `A`, enumerate its
  corners (`16` for a `2×2` with all four entries free, `2^k` if only `k` entries vary), compute
  `W(A + ΔA_c)` for each, and intersect. Record that the min of sampled support values is an
  *outer* approximation of the intersection unless the supports happen to be exact, so the
  intersection carries its own gap — measured in E4, not assumed away. Each corner set's own side
  is a claim too: track 1's `±2.931` case shows a sampled bridge can sit inside the true one, so
  the intersection is outer only once every corner is.
* **E2 — time-varying reading as a game.** Let `ΔA(t)` be chosen adversarially from the box.
  The Hamiltonian gains the term `sup_{ΔA} ⟨p, ΔA x⟩`, a rank-one disturbance that a level-set
  solver handles pointwise. Compute it on the P2 reduced state (3D) and compare with E1.
* **E3 — the gap.** Report `‖W_static − W_varying‖` in Hausdorff on a common direction set, as a
  fraction of set diameter, across a sweep of `‖ΔA‖/‖A‖`. The curve is the track's main object.
* **E4 — the intersection gap.** On cases small enough to compute the intersection exactly (vertex
  enumeration in 2D), compare the min-support construction with the true intersection. This
  decides whether the static reading is cheap or merely cheap-looking.
* **E5 — instability.** Sweep `ΔA` toward matrices with eigenvalues of positive real part and map
  where `W⁰` goes empty. The interesting number is not a smaller bridge but a nominal bridge that
  is non-empty while the perturbed one is empty, and how fast that boundary moves with `‖ΔA‖`.

## Build

An `uncertainty` block in the grammar (box on named entries of `A`), a driver mode that loops the
corners and intersects the resulting support vectors, and the game mode reusing track 2's
grid harness for the time-varying reading. The viewer draws the nominal and perturbed shells in
different colours so the fragility is visible, which is inspection, not evidence.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | static and time-varying agree within 5% across the box | the cheap reading is a valid surrogate at P2; adopt it and record the caveat |
| O2 | the gap grows with `‖ΔA‖`, reaching tens of percent | the readings are not interchangeable; the curve is the result (PC2) |
| O3 | min-support intersection is a loose outer bound | static reading needs exact intersection, so it is only cheap in low dimension |
| O4 | fragility: nominal non-empty, perturbed empty at small `‖ΔA‖` | the headline number for a practitioner: a robust solvability radius |
| O5 | growth follows `e^{λt}` with `λ` the perturbed real part, so the bridge empties at a predictable horizon | an analytic handle; the entry states the formula and the fit |

## How we would read them

* The gap is reported dimensionless, against `‖ΔA‖/‖A‖`, so it survives a change of units or a
  different vehicle. Absolute numbers alone are not stored.
* The static reading is adopted for P2 only if O1 holds across the whole box used at that rung,
  and only with the outer-intersection gap from E4 included in the bracket.
* If O4 holds, it outranks the gap curve: a fragility threshold is what makes the rung usable or
  not, and the entry says so in its first line.
* Any claim that a bridge "survives" a parameter error must name which reading it means. A
  sentence that does not is corrected, not footnoted.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `05-delta-a-readings.md` | decision | the two readings, the physical criterion for choosing, and the working definition used at P2 |
| `05-static-vs-varying.md` | measurement | gap vs `‖ΔA‖/‖A‖`, cases, resolutions, both sets' brackets |
| `05-intersection-gap.md` | measurement | min-support intersection against exact intersection, where exact is computable |
| `05-fragility-threshold.md` | measurement | the `ΔA` at which the bridge empties, nominal and perturbed, with the sweep |
| `05-growth-fit.md` | measurement | whether the emptiness horizon follows an exponential rate and with what residual |

## Publication candidates

* **PC2 — the adaptivity gap.** The two readings are conflated in practice; a measured gap curve
  prices the assumption and tells a practitioner which model they are actually solving. The
  prior-art check is robust reachability and set-membership literature, where static and
  time-varying uncertainty are both treated but rarely compared on the same game at the same
  resolution. If the literature already contains the comparison, the entry records that and the
  candidate is retired — a successful prior-art check, not a failure.
* Secondary: a robust solvability radius defined as the largest `‖ΔA‖` preserving non-emptiness is
  a clean object, and its dependence on the control sets may be monotone in a way worth stating.

## Stopping rule

Stop when the gap curve has at least five points, the fragility threshold is located for two
control-authority ratios, and the intersection gap is either measured or shown to be negligible.
Do not extend the box until the two readings have been separated at one box size.
