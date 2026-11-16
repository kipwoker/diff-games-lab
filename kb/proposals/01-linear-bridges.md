# Track 1 — Linear bridges as an instrument

| field | value |
|---|---|
| state | implemented: `engine/`, `docs/`, `examples/`, `viewer/` |
| depends on | — |
| feeds | every other track; it is the regression baseline |
| artifacts | `engine/`, `examples/out/`, `docs/06-results.md`, `tools/direction_table_check.py`, `tools/trace_deficit.py` |

## The question

The bridge and its implementation exist. The open question is what they can be trusted to
measure. Given a set carried as support values on 110 directions, what is the error of that
representation as a function of the number of directions, the set's size and the tolerances —
and, first, on which side of the true set does the printed one lie?

Track 1 is not asked to become more accurate. It is asked to become calibratable, because every
other track compares itself to it.

## Why it matters for the target problem

A pursuit answer is only as good as the bracket around it. If the reference method's own error is
unknown, then a disagreement at P1 cannot be attributed: it may be the nonlinear dynamics, or it
may be the 110 directions. Worse, if the printed set can sit either side of the true one, then a
strategy certified against it can be certified wrongly. The instrument has to be characterised in
both directions before it is pointed at anything harder.

## Proposal

* **E1 — direction sweep.** `example-1` has `A = 0`, so its interval map is exact and the only
  error is in the directions. Its exact bridge at `T = 2` is the diamond of radius
  `1 + 2·2 − 2·1 = 3`, with support `h(ψ) = 3 max(|ψ₁|, |ψ₂|)` — a closed form to measure
  against. Run `m ∈ {30, 55, 110, 220, 440, 880}` evenly spaced directions, measure the largest
  support error and the vertex count, and fit against the angular gap. At `m = 110` the answer is
  already known: `x ±3.000`, `y ±2.931` (E6).
* **E2 — the out-of-sample test.** Every stored support value is checked at its own direction by
  construction. Evaluate the reconstructed polyhedron at directions *not* in the table (say 5000
  evenly spaced) and measure the largest error there. This is the honest measure of the
  reconstruction, and it is the one that predicts the 3D bulge.
* **E3 — the slack floor.** Sweep the `0.01` acceptance slack from `1e-4` to `0.1` on
  `example-2c`, the case that collapses to a sliver. Find the set diameter below which support
  values stop being distinguishable, as a function of the slack and the coordinate scale. The
  slack is already excluded from the tracked bulges: a slack-free reconstruction reproduces
  `1.9874` and `0.514284` exactly, while adding `0.01` to every half-space gives `1.9979` and
  `0.524288` (see the `06-direction-table-explains-extents` entry).
* **E4 — error separation.** At fixed `m`, halve `Step` repeatedly and measure the change in the
  final set. Since directions are held fixed, whatever moves is interval error; whatever does
  not is direction error. Confirm second order in `h`.
* **E5 — the reconstruction, predicted before it is run.** Reconstructing the cube of half-width
  `1.5` from its exact supports on the table gives `1.9874` in `z` and `1.500` in `x` and `y`;
  the tracked block 4 is `1.9874` and `1.500`. The same reconstruction gives `0.514284` for the
  `0.5` square against a tracked `0.5143`. This has been done
  (`tools/direction_table_check.py`), so E5 is no longer the phenomenon — it is the record of the
  pre-registered check, and the remaining task is a closed form for it (track 6, E1).
* **E6 — the monotonicity probe.** *(Answered.)* See
  `entries/01-dual-update-is-the-deficit.md`. The tracked `y` is `±2.931` against an analytic
  `3.000` under a 90° symmetry, and the loss does not move across `Step` 0.2 … 0.01 — 10 to 200
  blocks — so it is neither the envelope accumulating nor interval error. A numpy copy of the
  recursion reproduces `2.930950` and returns the exact `3.000000` when stages 2–5 are skipped:
  the primal round trip through `v`, `c2` and the hull is the whole of it. Rotating the table
  changes the magnitude and a direction on the axis removes it, so the sign is an alignment
  accident, and `example-2b` prints the other sign. What remains of E6 is the bound: track 6, E1.

## Build

No engine change for E1–E2: a harness in `tools/` that evaluates supports at an arbitrary
direction set, plus a script that runs the binary at several `m` values. `m` is a compile-time
constant today (`DIRECTION_COUNT`), so E1 either makes it a field on the problem (an optional
grammar block `directions`) or builds six binaries. The grammar extension is worth it: every later
track wants to vary the resolution per run. E6 may need an envelope bypass behind a flag — a
diagnostic build, not a shipped default.

## Potential outcomes

| id | outcome | reading |
|---|---|---|
| O1 | error tracks the angular gap, roughly proportional to the squared gap times set radius | a closed form exists; track 6 inherits it |
| O2 | out-of-sample error is much larger than at sampled directions | the reconstruction, not the sampling, is the binding error |
| O3 | the slack floor sits at uncomfortable set sizes for P1 (metre-scale sets, not unit) | tolerances must become relative before P2 |
| O4 | with `m` fixed, halving `Step` changes nothing at 9 decimals | confirms mesh consistency; interval error is second order and small |
| O5 | the reconstruction reproduces `1.9874` and `0.514284` but no closed form fits both | the phenomenon is real and linear in neither gap nor curvature; a two-term model is needed |
| O6 | the `y` deficit shrinks with `m` | the envelope is not the cause and the dual difference loses support between samples; either way the method is two-sided, not outer |

## How we would read them

* A bound is adopted for track 6 only if it predicts the measured floor within 30% at every `m`
  in the sweep. A formula that is right at one resolution is a coincidence.
* A sampled set is **not** automatically an outer approximation. Track 1 measured a case that is
  inside the analytic answer, so every later track states which side it believes and why. "Outer
  by construction" is true of the half-space intersection only when the stored supports are the
  true ones, which is exactly what is in question.
* If O2 holds, track 6's effort moves from sampling more directions to bounding the
  reconstruction; that reorders its experiments, and the entry says so explicitly.
* Nothing here is a novelty claim. Track 1's product is calibration, and a calibrated instrument
  is reported as a table, not as a result.

## What this stores in the kb

| entry id | type | what it must contain |
|---|---|---|
| `01-direction-sweep-2d.md` | measurement | support error and vertex count against `m`, with the fit and its residuals |
| `01-outsample-error.md` | measurement | largest reconstruction error at 5000 unsampled directions, 2D and 3D |
| `01-slack-floor.md` | measurement | smallest distinguishable set diameter against slack and coordinate scale |
| `01-table-reconstruction.md` | measurement | tracked extents against the reconstruction from the table alone, slack 0 and 0.01 |
| `01-example1-deficit.md` | counterexample | the symmetric case that comes out 2.3% short |
| `01-dual-update-is-the-deficit.md` | measurement | the `Step` sweep, the numpy reproduction, the stage-5 bypass and the alignment sweep |
| `01-baseline-table.md` | reference | the five tracked cases with extents, vertex counts and measured brackets — the table every later track quotes |

## Publication candidates

None of its own. Track 1 supplies PC5's case set, the numbers PC1 must beat, and the
counterexample that makes PC1's two-sided version necessary.

## Stopping rule

Stop when the direction sweep has at least five resolutions in 2D and three in 3D, the
out-of-sample error is characterised for both, and the slack floor is known in absolute setdiameter. The track does not close while a symmetric case is answered asymmetrically without an
  explanation: the `example-1` deficit is a stop condition, not a footnote, and it is now explained
  in `01-dual-update-is-the-deficit.md`. If a tracked example
fails to reproduce byte for byte at any point, that is a bug and the work stops until it is found.
