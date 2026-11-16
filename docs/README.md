# How the method works

This documentation set explains the mathematics and the numerics of the solver: the problem, the
construction, its discretization, and the algorithm that runs. It is written to be read on its
own and next to the code.

These seven chapters belong to track 1 of the lab, the linear support-function track, with
[the brief](08-brief.md) and [the note of record](09-note.md) alongside them.
[`../kb/RESEARCH.md`](../kb/RESEARCH.md) is the map of the whole bench and of the tracks still
planned.

| Path | What it holds |
|---|---|
| `docs/` (here) | the method, chapter by chapter |
| [`engine/`](../engine/) | the implementation, one module per stage |
| [`examples/`](../examples/) | five problems with known answers, and their outputs |
| [`viewer/`](../viewer/) | three.js page that draws any output file |

## Reading order

1. [The game and the stable bridge](01-game-and-bridge.md) — dynamics, strategies, what a
   stable bridge is, and the reverse-time transformation the whole method rests on.
2. [Convex sets through their support functions](02-support-functions.md) — support functions,
   Minkowski sums, the geometric difference, and the dual reconstruction `co` that makes a
   finite direction grid usable.
3. [Discretization](03-discretization.md) — the partition, the interval map `A_k`, the
   recurrence `W^k = (W^{k−1} ⊖ Q_k) ⊕ P_k`, and the capture-time test.
4. [The algorithm as implemented](04-algorithm.md) — the 110 directions, the vertex
   enumeration, the dual update stage by stage, the tolerances, and the module map.
5. [The time step: interval map and refinement](05-time-step.md) — how a step is split into 110
   pieces, why that is the correct refinement, and where the error goes.
6. [Results](06-results.md) — the five examples, measured extents, mesh consistency, and the
   accuracy floor set by the grid.
7. [Falsifications](07-falsifications.md) — the predictions that missed, and what each changed.

Two further records sit beside the chapters: [the brief](08-brief.md) reads the intermediate
results against the literature, and [the note](09-note.md) is the write-up of record.

## Notation

| Symbol | Meaning |
|---|---|
| `n` | state dimension (2 or 3) |
| `A, B, C` | dynamics matrices, `ẋ = Ax + Bu + Cv` |
| `P, Q` | control constraint of the first player, of the second; compact convex |
| `M`, `π` | terminal set `πx(T) ∈ M`; `M` is a polyhedron, `π` a projection |
| `T`, `Step` | horizon and one interval of the partition |
| `ψ_i`, `m` | sampled direction `i` of `m = 110`, and their count |
| `c(X, ψ)` | support function `sup_{x∈X}⟨x, ψ⟩` |
| `X₁ ⊕ X₂`, `X₁ ⊖ X₂` | Minkowski sum, geometric (Pontryagin) difference |
| `co(f)` | the sublinear envelope of sampled support values; support of the half-space intersection |
| `W^k`, `W` | the `k`-th alternating sum, and its limit — the maximal stable bridge |
| `A_k` | the flow-and-integral map of the `k`-th interval counted back from `T` |

Vectors are columns; `⟨·,·⟩` is the Euclidean inner product; `e^{tA}` is the matrix
exponential. Everything is `double` in the code.
