# Track 3 — Alternative constructions

| field | value |
|---|---|
| state | planned; no code |
| depends on | track 1 (cases and baseline), track 6 (bracket) |
| feeds | all engineering tracks — it decides which bound is trustworthy |
| artifacts | `tools/` comparison harness, `docs/08-comparisons.md` |

## The question

Can the same bridge be built by other means — ellipsoidal inner and outer bounds, zonotope
propagation, gridding — and do the answers agree on track 1's examples? Where they disagree, is
the disagreement explainable from the geometry of the sets, or is it a bug in one of them?

## Why it matters for the target problem

The lab needs a second opinion it can trust. A bridge that only one method produces is an
assertion; a bridge bracketed between an inner ellipsoid and an outer zonotope is evidence. The
same machinery later becomes track 4's certificate and track 7's acceptance test, so the brackets
have to exist before either can be built.

## Proposal

* **E1 — ellipsoidal bounds.** On `example-2a` and `example-2b`, compute inner and outer
  ellipsoidal approximations of the same recurrence with an LMI solver in `tools/` (Kurzhanski–
  Vályi style: the geometric difference of two ellipsoids is not an ellipsoid, so what exists is a
  family of ellipsoidal *approximations* indexed by a support constraint; the entry records which
  member bounds which side). Record the solver, tolerance, and the semi-axes per block.
* **E2 — zonotope propagation.** The same recurrence with zonotopes, using an existing library
  (CORA or JuliaReach) or a small own implementation: Minkowski sum is exact for zonotopes, the
  geometric difference is not. Record the order and the number of generators per block, since
  order growth is the cost that decides whether this scales past 2D.
* **E3 — grid third opinion.** Reuse track 2's HJI harness at 2D to produce a third set for the
  same cases; the grid is neither inner nor outer by construction but converges, so it arbitrates
  when E1 and E2 disagree.
* **E4 — the comparison metric.** All methods are evaluated as support functions on a common
  direction set of at least 2000 directions, never on their own vertices. Report
  `max_ψ |h_A(ψ) − h_B(ψ)|`, labelled as a lower bound on the Hausdorff distance, plus each
  method's own inner/outer status.
* **E5 — locate the disagreement.** For directions where the gap is largest, check whether they
  point at corners (where the support function is non-differentiable) or at flat faces. If the
  disagreement lives at corners, it is the dual update losing information there, not an arbitrary
  numerical difference.

## Build

`tools/compare/`: a Python harness that reads our output documents and each library's output,
evaluates support values on a shared direction set, and writes one table. No shipped code depends
on it. The engine stays dependency-free; the comparison is deliberately outside it, and that
separation is recorded in the entry so a later session does not "helpfully" merge them.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | all three agree within the direction-grid floor on both cases | the floor is track 1's, not the method's; the case set is cross-validated |
| O2 | ellipsoid is inner, zonotope is outer, gap is 5–30% of diameter | the gap is the price of the representation; it becomes the bracket every later track quotes |
| O3 | gap concentrated at the corners of `P` and `Q` | a specific, explainable loss in the dual update; PC5 plus a geometric explanation |
| O4 | zonotope order explodes beyond 2D at useful tightness | a cost wall; the method map says which construction to use at which dimension |
| O5 | the grid arbitrates against *both* algebraic methods | one of them is wrong, or the recurrence is; a bug hunt, not a comparison |

## How we would read them

* Agreement is stated as a bracket width as a fraction of set diameter, never as "the same set".
  At P0 the acceptance threshold is 3%, not 2%: track 1 measured the reference method's own
  symmetric case 2.3% short, so a 2% band would flag the baseline against itself. Anything wider
  than 3% is reported as a disagreement with a location, not rounded away.
* If O2 holds, the bracket from E1/E2 is adopted as the inclusion mechanism for track 6, and
  track 6's own construction is then checked against it. Two independent brackets that agree are
  worth more than one that is believed.
* If O3 holds, the finding is that the dual update is exact in the directions it samples and
  lossy at corners — which is the same statement track 6 must bound. The two tracks then share
  one entry rather than duplicating the work.
* A method that is only inner or only outer is still useful: the entry records which side it
  bounds, because a conservative inner bound can certify a strategy while an outer bound cannot.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `03-method-bracket-2a.md` | measurement | per-block semi-axes or generators, inner/outer status, bracket width vs diameter |
| `03-bracket-2b.md` | measurement | the same for the equal-authority case, where the bridge should not move |
| `03-corner-disagreement.md` | measurement | largest-gap directions, whether they aim at corners, and the gap there |
| `03-cost-scaling.md` | measurement | wall time and representation size per block for all three methods |
| `03-toolchain.md` | dataset | library, version, tolerance, and the exact command that produced each set |

## Publication candidates

* **PC5 — a cross-method Hausdorff benchmark for pursuit–evasion.** Modest but real: a fixed case
  set, byte-stable outputs, stated resolutions, and a bracket per method. The novelty is the
  discipline (numbers on a common direction set, lower bounds labelled), not a new algorithm.
* Secondary: if O3 isolates corner loss analytically, that is a small theoretical note on
  support-function games — the sampled dual is exact at sampled directions and the loss is
  controlled by the normal cone at vertices.

## Stopping rule

Stop when two cases have a bracket from at least two independent constructions, the corner
hypothesis has been tested once, and the cost table covers 2D and 3D. Do not chase a third
library for the same bracket.
