# Discretization

This chapter turns the continuous bridge of chapter 1 into the finite recurrence of chapter 2: fix a partition, work out the exact map of one
interval, and write the alternating sum that the program iterates.

## 1. The partition and the interval map

Take a uniform partition of `[0,T]`,

$$
0 = t_0 < t_1 < \dots < t_N = T, \qquad t_i = i\,h, \qquad h = T/N ,
$$

and count the intervals **from the terminal instant**, because that is the direction the
construction runs in. The `k`-th interval from `T` is `[t_{N-k}, t_{N-k+1}]`, and its exact
contribution to a condition at time `T` is the weighted integral

$$
A_k \;=\; \int_{t_{N-k}}^{t_{N-k+1}} e^{(T-s)A}\,ds .
$$

Substituting `s = T - v`, or equivalently changing variables to the interval's own offset
`u = s - t_{N-k}`, gives two equivalent forms:

$$
A_k = e^{(T-t_{N-k})A}\int_0^{h} e^{-sA}\,ds
    = e^{(k-1)hA}\int_0^{h} e^{uA}\,du
    = e^{khA}\int_0^{h} e^{-sA}\,ds .
$$

The last form is the definition of `A_k`; the middle one is the form the code evaluates,
because `e^{khA}∫_0^h e^{-sA}ds = e^{(k−1)hA}∫_0^h e^{uA}du` by the change of
variable `u = h - s`. Both say the same thing:

> the map of an interval is **the flow from the interval to `T`**, times **the integral of the
> flow over the interval**.

That factorization is why the program needs only two matrix series — `e^{hA}` and
`∫_0^{Δ} e^{sA}ds` — and why the flow factor changes from block to block while the integral
does not. It also shows how the position of the interval enters: only through `k`. The first
interval (nearest `T`) has `k = 1` and map `∫_0^h e^{uA}du`; the last has `k = N` and map
`e^{(N-1)hA}∫_0^h e^{uA}du`.

## 2. The images of the control sets

Under the transformed dynamics of chapter 1, the first player contributes over the `k`-th
interval the set

$$
P_k = A_k B P = \{\, A_k B u : u \in P \,\},
$$

and the second player contributes `Q_k = A_k C Q`. Both are compact convex sets, computed once
per interval, and their support values come from chapter 2's transpose identity:

$$
c(P_k, \psi) = c(P, B^{\mathsf T} A_k^{\mathsf T} \psi), \qquad
c(Q_k, \psi) = c(Q, C^{\mathsf T} A_k^{\mathsf T} \psi).
$$

No matrix ever touches `P` or `Q`. The 110 directions are pushed through the matrix, and the
polytope's support is read in the 110 resulting directions — which is exactly the operation
`polytope::support` performs in the code.

## 3. The recurrence

With `W^0 = M` at `T`, the bridge over `k` intervals is

$$
W^{0}_{N} = M, \qquad
W^{k}_{N} = \bigl(W^{k-1}_{N} \ominus Q_k\bigr) + P_k,
\qquad k = 1, \dots, N .
$$

Read right to left: the second player's reachable set is removed from the already-computed
bridge (so that no choice of `v` can escape), then the first player's is added back (so that
some choice of `u` can reach it). Composition over `N` intervals gives the set of states from
which `N` interval-sized decisions suffice to force `z(T) ∈ M`.

The support-value form of the same recurrence is chapter 2's

$$
c(W^k, \psi_i) = \operatorname{co}\bigl(c(W^{k-1}, \psi_i) + c(P_k,\psi_i) - c(Q_k,\psi_i)\bigr),
\qquad i = 1..m .
$$

## 4. What the limit buys

Refining the partition shrinks `W^{N}_{N}`:

$$
N_1 \preceq N_2 \;\Longrightarrow\; W^{N_1}_{N_1} \supset W^{N_2}_{N_2},
$$

so `W = lim W^{N}_{N}` exists and is the maximal stable bridge of chapter 1. The argument is
that the interval maps converge to the identity as `h → 0`, so the alternating sum tends to the
alternating integral, which is the bridge. Nothing in it requires the partition to be uniform;
it requires the mesh diameter to vanish, and each interval's map to be the *exact* weighted
integral over that interval. Chapter 5 shows how a step is split to satisfy both.

## 5. The capture test

For a fixed `x_0`, once `W^k` is known the question "is the target reachable by `k·h`?" is a
support-function comparison against the transformed initial point:

$$
\min_{\psi \in S_1}\Bigl\{\, c(W^k, \psi) - \langle e^{khA} x_0, \psi\rangle \,\Bigr\} \ge 0
\quad\Longleftrightarrow\quad z(0) \in W^{k},
$$

i.e. `z(0)` is in the bridge exactly when every support value of the bridge dominates the
corresponding value of the single point `z(0)`. The smallest `k` for which this holds is the
capture time, `T = k h`.

Two practical notes:

* The minimum runs over the whole unit sphere, but the solver evaluates it on the sampled
  grid. On a grid, `min_i` is an upper bound of the true minimum, so a positive test is
  reliable and a negative one is not — the sampling can only make capture look *easier*.
* The test is a point-vs-set operation and needs no vertex enumeration: it is `m` inner
  products per block. The solver computes every support it needs and prints every block, so
  membership is left to the reader.
