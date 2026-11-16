# The algorithm as implemented

Chapters 1–3 gave the mathematics; this one describes the machine that computes it, stage by
stage, and points at the module that implements each stage.

## 1. The pipeline

One run performs four stages.

```
problem description ──parse──> A, B, C, π, M, P, Q, T, Step
                            │
                            ├─ build Ψ: 110 directions                 (stage 1)
                            ├─ c(M, Ψ): terminal support vector        (stage 2)
                            │
      for k = 1 .. floor(T/Step):
          advance the support vector over one step                   (stage 3)
          turn c into vertices and print them                        (stage 4)
```

Stages 1 and 2 run once. The loop runs once per printed block, and each block is one call of
the driver, which internally does 110 sub-steps (chapter 5).

## 2. Stage 1 — the direction grid

Between blocks a set is never represented by its facets or its vertices, only by 110 support
values. The directions are fixed at compile time (`m = 110`).

**2D.** Uniform in angle:

$$
\psi_i = (\cos \theta_i, \sin \theta_i), \qquad \theta_i = \frac{2\pi i}{110},
\qquad i = 0,\dots,109 .
$$

**3D.** A latitude/longitude grid with 11 azimuths and 10 elevation levels,
$m = 10\cdot 11 = 110$:

$$
x = \cos\gamma_i \cos\theta_j,\quad
y = \cos\theta_j \sin\gamma_i,\quad
z = \sin\theta_j,
$$

$$
\gamma_i = \frac{i\pi}{10} - \frac{\pi}{2}\ (i = 0..10), \qquad
\theta_j = \frac{2\pi j}{10}\ (j = 0..9).
$$

The first two components are scaled by $\cos \theta_j$ and the third is $\sin \theta_j$, so the 10
elevation levels fix `z` and the 11 azimuths sweep the equator. A direction table exists for
these two dimensions only; anything else is a `SetupError`.

The 3D grid is the reason every 3D result in this repository is looser than its 2D
counterpart: 10 elevation levels put the nearest sample 18° away from the `z` poles, so
nothing caps a shell along that axis (see [results](06-results.md)).

## 3. Stage 2 — the terminal support vector

`M` arrives as inequalities $\langle a_j, x\rangle \le b_j$. Two things have to happen: the set has to be
turned into vertices, and the vertices into 110 support values.

**Boundedness guard.** For every direction, compute $\max_j \langle a_j, \psi_i\rangle$ and then the minimum
over `i`; the polyhedron is accepted only if that minimum is positive. Geometrically: a
polyhedron is bounded iff no nonzero direction is non-positive against every normal, and the
check samples that condition. It is sufficient, not necessary — a bounded `M` can be rejected —
and it is evaluated on the 110 directions rather than all of $\mathbb{R}^{n}$.

**Vertices.** Every $C(m_{\text{rows}}, n)$ combination of `n` rows is solved as an $n\times n$ linear system
by Gaussian elimination with partial pivoting. A candidate is accepted when it satisfies all the
other rows within the `0.01` slack, and duplicates are rejected with the same slack. The
vertices of $\{x : Ax \le b\}$ are exactly the feasible solutions of its $n\times n$ subsystems, so the
enumeration is complete up to the slack.

**Supports.** $c(M, \psi_i) = \max$ over the accepted vertices of $\langle \psi_i, v\rangle$.

The `0.01` slack is four orders of magnitude coarser than double precision and is the dominant
numerical artefact of the method: it admits near-duplicate vertices, and it is why an output
block can contain crowded runs of almost identical points. It is a documented tolerance, not a
bug — see the constants table below.

## 4. Stage 3 — advancing one step

Per block the driver applies the interval map and the recurrence of chapter 3 in dual space.
The stages, in the order the code runs them:

1. **Flow and integral.** $E = e^{Ah}$ and $\Phi_h = \int_{0}^{h} e^{sA}ds$, both by a truncated power
   series ($A^k h^k / k!$, accumulated with integer factorials) that stops when the largest
   entry of the new term is at most `1e-5`.
2. **The interval map.** $G = \pi \cdot \Phi_h\cdot E$, the flow from the interval to the terminal instant
   times the integral over the interval (chapter 3 §1).
3. **Pushing the directions through.** $GB = G\cdot B$ and $GC = G\cdot C$, and `GB` is negated as a
   whole. Negating the matrix flips the sign of every support value of `P` at once; that is the
   minus in $c(P_k) - c(Q_k)$, applied structurally rather than per term.
4. **The two support images.** $c_P = c(P, (GB)^{T}\Psi)$ and $c_Q = c(Q, (GC)^{T}\Psi)$ — stage 2's
   enumeration again, on the transformed directions.
5. **Sum, then difference.** $\Sigma = c_{\text{prev}} + c_P$, then the dual update with $c_Q$.

## 5. The dual update

Given `Σ` and $c_Q$, one call produces the next support vector without ever forming the
difference set in primal space:

```
diff[i] = Σ[i] − c_Q[i]                                  # the signed values           (1)
v       = (n/m) · Σ_i diff[i] · ψ_i                      # centre                        (2)
g[i]    = diff[i] − ⟨ψ_i, v⟩                             # recentred values              (3)
λ[i]    = min { g[j] / ⟨ψ_i, ψ_j⟩ : ⟨ψ_i, ψ_j⟩ > 0 }     # radius along ψ_i, active set  (4)
out[i]  = diff[i]                                        if i is active
        = ⟨ψ_i, v⟩ + max_j λ[j] ⟨ψ_i, ψ_j⟩               otherwise                     (5)
```

Steps (2), (4) and (5) are chapter 2 §4: the centre from the mean-support integral, the
min-ratio radii, and the support of the reconstruction's convex hull. Step (4) also records the
argmin indices — the *active set* — and step (5) keeps the exact sampled value there, where the
hull formula is exact anyway.

Two subtleties worth naming:

* The min-ratio scan is `O(m²)` and every term needs a Gram entry. The entries are recomputed
  inside the loop rather than stored, so the summation order stays fixed.
* `λ` can come out negative when $\Sigma - c_Q$ is not a support function. There is no guard: the
  hull step then produces a value below the sampled data, and `co` absorbs it. This is the
  numerical shadow of the outer approximation of chapter 2.

## 6. Stage 4 — printing the set

A support vector `c` defines the polytope

$$
\{\, x : \langle \psi_i, x\rangle \le c_i,\ i = 1..m \,\},
$$

which is a superset of the polytope those support values describe and has at most `m` facets.
Whether it is a superset of the *true* bridge depends on the stored values being true supports,
which is not guaranteed: `example-1` prints a shell 2.3% inside its analytic answer, so the
`y` half-width is `2.931` where the symmetry of the problem forces `3.000`
([`kb/entries/01-example1-deficit.md`](../kb/entries/01-example1-deficit.md)). Its vertices are
found by
the same `C(m, n)` enumeration as `M`: solve each `n`-subset of the half-space boundaries, keep
the solutions that satisfy all the others within the slack, and print them.

Nothing sorts the result. The printed order is the enumeration order of the binomial
coefficients, which is why consecutive lines jump across the shape and why the viewer derives
its outline from a convex hull rather than from the file order
([`viewer/README.md`](../viewer/README.md)). Each block ends with a `separator` line; numbers
are `%.9lf`, space separated.

The cost here dominates everything else. For `n = 3` there are `C(110,3) = 215 820` candidate
systems, each needing a solve plus `m` feasibility checks, per block.

## 7. Module map

| Module | Role |
|---|---|
| `main.rs` | parse, terminal support, block loop, output |
| `input_parser.rs` | the problem grammar and its shape checks |
| `directions.rs` | the 110 directions |
| `polytope.rs` | boundedness guard, vertices, supports |
| `linear_system.rs` | Gaussian elimination with partial pivoting |
| `combinations.rs` | `C(m,n)` enumeration order |
| `matrix.rs` | dense matrix arithmetic |
| `support_function.rs` | $\max_v \langle \psi, v\rangle$ over vertices |
| `pontryagin_difference.rs` | the dual update of §5 |
| `minkowski_sum.rs` | support-vector sum |
| `stable_bridge.rs` | series, interval map, driver |
| `convex_polygon.rs` | vertices from a support vector |
| `output_writer.rs` | `%.9lf`, `separator`, the document header |

## 8. Constants

| Quantity | Value | Consequence |
|---|---|---|
| directions `m` | 110 | accuracy ceiling in every direction |
| series tolerance | `1e-5` | $e^{Ah}$ and its integral are truncated, not machine-accurate |
| dedup / feasibility slack | `0.01` | near-duplicate vertices, loose acceptance |
| sub-steps per block | 110 | the inner refinement of chapter 5 |
| output precision | `%.9lf` | nine decimals, fixed |
| state dimension | 2 or 3 | no direction table exists for other `n` |
