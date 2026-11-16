# The game and the stable bridge

This chapter fixes the objects the rest of the documentation computes: the players, what counts as a strategy, what "the first player can force the target"
means, and the change of variables that turns a moving terminal instant into a fixed one.

## 1. Dynamics

On a fixed horizon `[0, T]` the state `x(t) ∈ ℝⁿ` obeys

$$
\dot x(t) = A\,x(t) + B\,u(t) + C\,v(t), \qquad x(0) = x_0, \qquad t \in [0,T],
$$

with constant matrices `A ∈ ℝ^{n×n}`, `B ∈ ℝ^{n×p}`, `C ∈ ℝ^{n×q}`. The first player steers
`u(t) ∈ P ⊂ ℝ^p`, the second steers `v(t) ∈ Q ⊂ ℝ^q`. Both `P` and `Q` are compact and
convex, and both contain the origin in every example here.

A control is *admissible* if it is measurable and takes values in its constraint set almost
everywhere. A *positional strategy* is a map `U : [0,T] × ℝⁿ → P`; it is
admissible when it is piecewise constant on the intervals of a partition, so `u(t) = U(t_i, x(t_i))`
on `[t_i, t_{i+1})`. The second player's objects are defined the same way
with `Q` and `V`.

For a partition `𝓝 = {t_i}_{i=0}^{N}` with `0 = t_0 < … < t_N = T`, whose diameter is
`d(𝓝) = max_i (t_{i+1} − t_i)`, and an admissible `U`, the guided equation has a
unique solution `x_𝓝(· | 0, x_0, U, v(·))` for every admissible `v(·)`. The set of all such
trajectories, over all admissible `v`, is written `X_𝓝(0, x_0, U)`.

## 2. What must be guaranteed

The target is a polyhedron `M` in the image of a projection `π`:

$$
\pi\,x(T) \in M .
$$

In the examples `π = I` and `M` is a square or a diamond, but the solver accepts a general matrix
`π` and describes `M` by rows `⟨a_j, x⟩ ≤ b_j`, so a projection is folded into the terminal set.

**Stable bridge.** A point `(0, x_0)` is in the bridge if the first player
can guarantee the terminal condition *uniformly*: for every `ε > 0` there is `δ > 0` such that
for every partition `𝓝` with `d(𝓝) < δ` and every admissible positional strategy `U`, every
trajectory in `X_𝓝(0, x_0, U)` ends in the `ε`-neighbourhood `M^ε` of the target.

Three things about this definition matter for the code:

* **It is about a family of partitions, not one partition.** The guarantee has to survive
  refinement, which is why the construction below is an alternating sum over a partition and
  why refining the mesh must not change the answer in the limit.
* **The order of quantifiers is the game's order.** The first player chooses a *strategy*
  `U(t,x)`, i.e. a rule that reacts to the state, and the second player chooses a control
  afterwards, trajectory by trajectory. That asymmetry — not a simultaneous choice — is what
  the geometric difference encodes (chapter 2).
* **`ε` and `δ` are separate.** `ε` is slack in the terminal condition, `δ` is fineness of the
  partition. Numerical implementations keep only the partition, so "capture" always means
  "capture up to the discretization and the direction grid".

The *maximal* stable bridge, written `W`, is the set of `z(0)` (below) from which this holds.
The method is built on its membership criterion: `x_0` can be forced to the target at `T`
precisely when `z(0) ∈ W`, and cannot when `z(0) ∉ W`.

## 3. Removing the drift

The dynamics carry `A` in every trajectory, which makes the terminal condition depend on the
whole path. The substitution

$$
z(t) = e^{(T-t)A}\,x(t)
$$

differentiates to

$$
\dot z(t) = -A\,e^{(T-t)A}x(t) + e^{(T-t)A}\bigl(Ax(t) + Bu(t) + Cv(t)\bigr)
          = e^{(T-t)A}B\,u(t) + e^{(T-t)A}C\,v(t),
$$

because the two `A`-terms cancel — `A` commutes with its own exponential. The initial and
terminal values are

$$
z(0) = e^{TA}x_0, \qquad z(T) = x(T),
$$

so `x(T) ∈ M ⟺ z(T) ∈ M`, while the drift has disappeared from the equation and only the
*weights* `e^{(T-t)A}B` and `e^{(T-t)A}C` remain.

This is the single most important structural fact in the method. After the substitution:

* the dynamics are `ż = w(t)` where `w` ranges over a set that moves with time through a known
  linear map — no free `Az` term;
* the terminal condition is evaluated at the fixed instant `T`, so the backward construction
  can start from `M` at `T`;
* the only place `A` still appears is in the interval maps `A_k`, which are computed once per
  interval (chapter 3).

The useful form groups the weights as `π·(flow)·B` and `π·(flow)·C`, where the flow is
`e^{(T-t)A}` at the interval: that is what the solver evaluates (chapter 4).

## 4. The family of approximations

`W` is defined by a limit over partitions, so it is built as the limit of a sequence of sets
computed on finer and finer meshes:

$$
W = \lim_{\substack{\mathfrak{N} \\ d(\mathfrak{N}) \to 0}} W^{N}_{N},
$$

where `W^{N}_{N}` is the `N`-th iterate of the recurrence of chapter 3 on a partition with `N`
intervals. Refining the partition shrinks the approximation, so the sets decrease as the mesh
is refined and approach `W` from outside. `W` is convex and compact, because every `W^{N}_{N}` is.

**The solver never computes the limit.** It computes one `W^{N}_{k}` per block on a fixed mesh
and prints all of them, so a run is a film of the backward construction rather than a single
set. That is why the output has one block per time step and why the viewer draws a sequence of
shells: reading a run means watching `W^k` grow (or shrink) as `k` runs from 1 to `N`.

## 5. Capture time

The horizon `T` is part of the input, but the interesting question is usually the *first* time
at which capture becomes possible. The answer is repetition: slice `[0,T]` at
`s, 2s, 3s, …` and compute the bridge over each prefix `[0, i·s]`; the smallest `i` for which
the initial point is in the bridge is the capture time, to the resolution `s`.

The solver does not implement this scan, and it does not implement the membership test either. What it produces is the raw material for both: the support vectors of
all shells, from which membership of a given `z(0)` and the minimal time can be read off
afterwards.
