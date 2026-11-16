# Track 6 — Method evolution

| field | value |
|---|---|
| state | E1 corrected, E2 measured in 2D, E4 built in 2D (parts A–G) |
| depends on | track 1 (cases, tracked numbers, the deficit counterexample) |
| feeds | every track: it supplies the bracket and the inclusion test |
| artifacts | `engine/src/error.rs`, `engine/src/refine.rs`, `docs/10-errors.md`, `tools/direction_table_check.py`, `tools/covering_bound.py` |

## The question

What does a modern version of this approach owe? Four things, and track 1 supplies the numbers to
beat:

* a **direction-discretization error** that predicts the measured floor instead of discovering it
  after the fact — `1.6°` in 2D, `18°` at the 3D poles, the `1.325×` `z` factor of `example-3d`
  and the `2.9%` bulge of `example-2b`;
* **adaptive refinement** — directions added where the support function bends, at equal total
  count;
* an **error for the support function itself** — the `0.01` half-space slack and the `co` envelope
  are different approximations and, as it turns out, act on different sides;
* a **guaranteed inclusion check** — the printed `W^k` should arrive with a proven outer bound and
  a certified inner set.

## Why it matters for the target problem

Without this track every other result is a number with no error bar, and the lab's own success
criterion — a certificate — is unreachable. One measured fact makes it sharper than the proposal
originally assumed: the printed set is not reliably outer. `example-1` is 90°-symmetric with an
analytic half-width of `3.000`, and its tracked `y` is `±2.931`. A strategy certified against a set
that may be 2.3% short is not certified at all.

## Proposal

* **E1 — the bound.** *(Written.)* `entries/06-direction-bound.md`. With `ρ` the table's covering
  radius — `1.6364°` in 2D, `19.84°` for the shipped 3D table — and `h`, `R` the support and
  circumradius of the set, the excess of the reconstruction is at most
  `h·tan ρ + R(1/cos ρ − 1)` per direction. *(Correction: the first term is exact only when the two nearest samples straddle
  the normal symmetrically; `06-two-neighbour-bound.md` replaces the form with a two-sample
  formula that matches the measurement, the single term being false by `0.021`.)* First term: a
  facet normal `ρ` off `ψ` lets the
  intersection slide by `h tan ρ`, which is exact on a box and is the flat-facet drift the
  `R δ²/8` model could not see. Second term: curvature insurance, unexercised by the tracked
  cases. Measured: 0.1% over the `0.514284` square, 11% over the `1.9874` cube, above two blind
  cases predicted before they ran, and above the reconstruction at every direction count from
  `110` to `374`. Loose on vertex-degenerate sets (2.9% on a diamond) by construction. What is
  left of E1 is a proof of the first term for general polytopes and a real `ρ` for 3D.
  *(The 3D route through the pair form is closed: it is exact in 2D only, and the engine now
  computes the one-term allowance and labels 3D an estimate — `06-two-neighbour-3d.md`.)*
* **E2 — adaptive refinement.** Rank directions by the discrete curvature of the support function
  or by the gap between the reconstructed facet and the sampled plane; insert directions where
  that is largest, by bisection of the angular gap in 2D and subdivision of the spherical cell in
  3D. Compare equal-count adaptive against uniform doubling at matched vertex counts. *(2D built: `4.3×` at `110`.
  3D built as an allowance-greedy table: `4×` at the same `110` directions and `1.6%` of the support
  at `240` — `06-three-d-adaptive-allowance.md`.)* This is the
  one experiment that can shrink the E1 error rather than only report it.
* **E3 — the support-function error, split in two.** The slack and the envelope are different
  suspects. The slack is already excluded from the tracked bulges: a slack-free reconstruction
  reproduces them exactly and adding `0.01` does not. The `co` envelope is a minorant of the
  sampled support values by construction, and it was the prime suspect for the `±2.931`
  undershoot — which is now traced to stages 2–5 of the dual update instead
  (`01-dual-update-is-the-deficit.md`), step-independently. So E3's remaining job is the envelope
  on its own: difference `co` against the raw sampled values per direction and bound what it
  contributes, rather than assuming which side of the discrepancy it explains.
* **E4 — the bracket, with each side established rather than assumed.** A sampled set is not
  automatically outer, so both sides are claims to prove. Outward:
  `h_true(ψ) ≤ h_stored(ψ) + e_out(ψ)` with `e_out` from E1. Inward: an ellipsoid `E` is
  contained in the true set when `max_ψ (h_E(ψ) + e_in(ψ) − h_stored(ψ)) ≤ 0`, where `e_in`
  allows for the stored support being short. The naive test without `e_in` is unsound, and that
  mistake is what this experiment exists to avoid. Report the width as a fraction of diameter for
  every tracked example, and say plainly when only one side exists.
  *(Built: the shifted intersection is `10.8×` narrower than the ball and inside the true body on both twins — `06-two-sided-bracket.md`. 3D stays an estimate.)*
* **E5 — the cost of certainty.** Per-block wall time, peak memory and bracket width over a grid
  of direction counts, as a Pareto table. Certainty that costs more than the answer is a finding
  too.

## Build

An `error` module that carries, alongside each support vector, an allowed deviation per direction
in each sense, and a `bracket` mode in the writer: an optional header block stating both
allowances. The viewer gains a translucent band drawn from them, so the uncertainty is visible —
which is the point of having a viewer. Documents without the block stay byte-identical, so track
1's tracked files do not move.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | the covering-radius bound predicts `1.9874` and `0.514284` within 5% | the theory is right; PC1 becomes the track's product |
| O2 | the bound is right in 3D and 2× off in 2D, or the reverse | the table's two constructions differ in kind; a correction term is needed and must be named |
| O3 | adaptive refinement reaches the same bracket with under half the directions | adopt adaptive as the default in every later track |
| O4 | the envelope explains the undershoot and sampling the overshoot | the method is genuinely two-sided; PC1 requires both terms, and `e_in` is not optional |
| O5 | no inner certificate as tight as the outer bound is affordable | bracket stays one-sided for inner claims; borrow track 3's ellipsoid or zonotope |
| O6 | the tracked examples are outer by more than 2% in one direction | track 1's published extents gain an uncertainty column and `docs/06-results.md` is revised |

Recorded: O1 in part and O2's first branch — the bound is 0.1% above the 2D measurement and 11%
above the 3D one, far inside the 5% rule in 2D and outside it in 3D. O4 does not hold as written:
the undershoot is not the envelope but the dual update's stages 2–5, step-independent, and the
overshoot is the reconstruction.

## How we would read them

* The four obligations are pass/fail *per rung*: a rung's set claim must carry a bracket narrower
  than its tolerance (2% of diameter at P1, 5% at P2, a stated figure at P0). A track that cannot
  meet the number does not get to claim the rung.
* A predicted number is accepted only after it is written down first. The
  prediction-before-measurement order is the whole experiment; post-hoc agreement is curve
  fitting. `1.9874` and `0.514284` are already on record for this purpose.
* The bracket is always reported as a pair of one-sided claims with their own allowances. A
  single number labelled "error" is replaced by the pair, even when the inner set is empty and the
  reason is stated.
* Because the sign is not established, a certificate that uses `h_stored` without `e_out` is not a
  certificate. That rule propagates to tracks 4, 5 and 7 and is quoted there.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `06-direction-bound.md` | method | the bound, its derivation, pre-registered predictions and measured values — written |
| `06-direction-table-explains-extents.md` | measurement | the reconstruction that reproduces `1.9874` and `0.514284` with no slack |
| `06-adaptive-refinement.md` | measurement | bracket width and vertex count, adaptive against uniform, 2D and 3D |
| `06-envelope-and-slack.md` | measurement | the two support-function allowances, separated, with `co` differenced against raw values |
| `06-two-sidedness.md` | method | `e_out` and `e_in`, the test that uses each, and the `example-1` deficit as the trial |
| `06-bracket-examples.md` | measurement | both bounds for all five cases, as fractions of diameter |
| `06-certificate-cost.md` | measurement | time and memory per block against bracket width |

## Publication candidates

* **PC1 — a two-sided, a priori error theory for support-function bridges, with adaptive
  refinement that certifies a Hausdorff bracket.** The strongest candidate, and the measurement
  above makes it concrete: the support-function literature for linear games is rich on algebra and
  thin on sampled error bounds, while the level-set literature has elaborate grid error control.
  A bound that predicts a measured factor before it is observed, plus an inner certificate and a
  named minorant (`co`) that explains the sign, is a genuine contribution. Prior-art check:
  sampled support functions, polyhedral approximation of convex bodies, error analysis for
  Pontryagin-difference schemes.
* Secondary: the inner certificate by ellipsoid verification is a small reusable technique, worth
  a paragraph if it is cheap.

## Stopping rule

Stop when the bound has been pre-registered and tested at 2D and 3D, adaptive refinement has one
matched-cost comparison, and both sides exist for at least three cases — or when the sign is
proven to be uniform, which would be a stronger result than the bound. If the bound cannot be made
to work, the track still delivers the measured bracket and the negative result.
