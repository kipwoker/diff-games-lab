# Convex sets through their support functions

This chapter is the geometric engine: how a convex set is represented, how the game's two operations act on that
representation, and how a finite grid of directions is turned back into a polytope.

## 1. Support functions

For a compact convex set `X ⊂ ℝⁿ` the support function is

$$
c(X, \psi) = \sup_{x \in X} \langle x, \psi \rangle, \qquad \psi \in \mathbb{R}^n .
$$

It is positively homogeneous and sublinear in `ψ`, it is the same for `X` and `conv X`, and it
determines `X` uniquely. Four identities carry the whole method:

$$
c(X_1 + X_2, \psi) = c(X_1,\psi) + c(X_2,\psi),
$$

$$
c(LX, \psi) = c(X, L^{\mathsf T}\psi) \quad\text{for a linear map } L,
$$

$$
c(\alpha X, \psi) = \alpha\, c(X,\psi) \quad (\alpha \ge 0),
$$

$$
c(X, \psi) = \langle \pi^{\mathsf T}\psi, \cdot\rangle\text{-weighted support of }\pi X .
$$

The second identity is what lets the program apply a matrix to the *directions* instead of to
the set: the support of `P_k = A_k B P` in direction `ψ` is the support of `P` in direction
`B^{\mathsf T}A_k^{\mathsf T}\psi`. No polytope is ever transformed, only the 110 directions.

## 2. The two operations

**Minkowski sum.**

$$
X_1 + X_2 = \{x_1 + x_2 : x_1 \in X_1,\ x_2 \in X_2\}.
$$

In support space it is a pointwise sum — the first identity above. This is the "the first
player's control helps" half of the recurrence, and it costs `m` additions.

**Geometric (Pontryagin) difference.**

$$
X_1 \ominus X_2 = \{x \in \mathbb{R}^n : x + X_2 \subset X_1\}.
$$

It is *not* the inverse of the sum, and it is where the game's quantifier order enters: a point
`x` survives an opponent's move `X_2` only if every `x + y` with `y ∈ X_2` is still allowed.
The set is convex and compact when `X_1` is, and it can be empty.

What support functions give is an inequality rather than an identity:

$$
c(X_1 \ominus X_2, \psi) \;\le\; c(X_1,\psi) - c(X_2,\psi) \quad \text{for all } \psi,
$$

with equality when `X_2` is a singleton; for a general `X_2` the right-hand side is strictly
larger in the directions where `X_2` has extent. So the game's recurrence, written in support
values,

$$
c(W^k, \psi) = c(W^{k-1}, \psi) + c(P_k, \psi) - c(Q_k, \psi),
$$

produces numbers that need not be the support function of any set at all: the sampled
difference values are an upper bound, and they can fail sublinearity between samples. The
recurrence is therefore only formal until `co` is applied.

## 3. `co`: from grid values back to a set

Given values `f_i` at directions `ψ_i`, `i = 1..m`, the operator `co` produces the support
function of the polyhedron cut out by the corresponding half-spaces:

$$
\operatorname{co}(f)(\psi) \;=\; \min\Bigl\{\textstyle\sum_i \lambda_i f_i \;:\; \psi = \sum_i \lambda_i \psi_i,\ \lambda_i \ge 0 \Bigr\}
\;=\; c\bigl(\{\,x : \langle \psi_i, x\rangle \le f_i \ \forall i\,\},\ \psi\bigr).
$$

The equality is an LP duality statement: maximising `⟨ψ, x⟩` over the half-space intersection
is dual to writing `ψ` as a non-negative combination of the constraint normals. Two
consequences are used constantly below:

* `co(f)(ψ_i) ≤ f_i` — the reconstructed set is contained in each of its defining half-spaces,
  so the values can only go down;
* if `f` is already the support function of a set `X`, then `co(f)` is the support function of
  the intersection of the half-spaces, which contains `X`. Since the half-space bounds came
  from `c(X_1) − c(X_2) ≥ c(X_1 ⊖ X_2)`, the computed bridge is an **outer** approximation:
  every direction that is not sampled is a direction in which the set may be too large.

This is the fundamental accuracy budget of the method: the approximation is controlled by the
density of the direction grid (chapter 3). It is also why
`co` cannot simply be replaced by "sample more directions": the grid is a compile-time
constant of 110.

## 4. The reconstruction, step by step

Evaluating `co` by the LP above at every direction is too slow for a 110-direction grid. The
implementation uses an explicit construction that needs only inner products between the
sampled directions.
It takes the sampled values `c_i = c(X_1,ψ_i) − c(X_2,ψ_i)` of the would-be difference `D` and
returns values that are a genuine support function of a polytope `D̂ ⊂ …`.

Let `G` be the `m×m` Gram matrix `G_{ij} = ⟨ψ_i, ψ_j⟩`.

**Step 1 — centre.** The set's translation is recovered from its support values,

$$
dc = \frac{n}{m}\sum_{i=1}^{m} c_i\,\psi_i ,
$$

which is the discretization of `(1/v_n)∫_{S} c(D,ψ)\,ψ\,dS`. For a body `D = X + t` with `X`
centrally symmetric, that integral equals `v_n·t`: the mean of the support directions recovers
the body's centre. The grid version is exact for centrally symmetric sets and a good estimate
otherwise — and every set in the examples is centrally symmetric.

**Step 2 — recentre.** With `g_i = c_i − ⟨dc, ψ_i⟩` the values are those of `D − dc`, which
contains the origin (up to the grid error), so the radii below are non-negative.

**Step 3 — radii by min ratio.** The reconstruction is the convex hull of `m` points, one per
direction, at distance `λ_i` along `ψ_i`:

$$
\hat D = \operatorname{conv}\{\, dc + \lambda_i \psi_i \,\}.
$$

For `D̂` to respect the constraint in direction `j` we need, for every `i`,

$$
\langle \psi_j, dc + \lambda_i \psi_i \rangle \le c_j
\iff \lambda_i \,\langle \psi_j, \psi_i\rangle \le g_j .
$$

Each constraint `j` with `⟨ψ_j, ψ_i⟩ > 0` is therefore an upper bound on `λ_i`, and taking the
tightest one gives

$$
\lambda_i = \min_{\substack{j \\ \langle \psi_j,\psi_i\rangle > 0}} \frac{g_j}{\langle \psi_j,\psi_i\rangle},
\qquad i = 1..m .
$$

Constraints with `⟨ψ_j,ψ_i⟩ ≤ 0` do not bound the ray along `ψ_i` at all; and if some `g_j` is
negative the minimum is negative, which is how the construction signals that the sampled values
were not a support function to begin with.

**Step 4 — evaluate the hull.** The support function of `D̂` in the sampled directions is

$$
c(\hat D, \psi_i) = \langle dc, \psi_i\rangle + \max_{j} \lambda_j \langle \psi_j, \psi_i\rangle .
$$

**Step 5 — keep the active directions.** The hull formula is exact only where the constraint is
active. In the directions that attain a minimum in step 3 — the *active set* — the hull value
coincides with the sampled value, and the construction keeps `c_i` verbatim rather than
accepting the round trip's error. This is the `mask` of the implementation, and it is why the
printed polytopes have exact facets in the sampled directions and interpolated ones between
them.

The result of steps 1–5 is `co(c)`: the values `c_i` on the active set, the hull values
elsewhere. Applied to `c(W^{k−1}) + c(P_k) − c(Q_k)` it yields `c(W^k)`, which is a support
function of an actual polytope — the only kind of object a vertex enumeration can print.

## 5. Why the difference is the expensive half

The sum is `3m` flops. The difference is `O(m²)` for the min ratios plus `O(m²)` for the hull
evaluation, and the polytope that results from it must then be turned back into vertices. Every inner
product in the method is between two of the same 110 directions, so the entire Gram matrix `G`
is a constant of the problem — `110 × 110` doubles. The implementation recomputes its entries
in the inner loops, which is the main avoidable cost (chapter 4).
