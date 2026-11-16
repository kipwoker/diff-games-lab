# The time step: interval map and refinement

This chapter connects the partition of chapter 3 to what one call of the driver does, and
explains the two levels of discretization in the code: the outer step that produces a printed
block, and the inner refinement that approximates the interval map.

## 1. What one block must do

Block `k` advances the bridge from $(k-1)\cdot \text{Step}$ to $k\cdot \text{Step}$. Over that interval the alternating
sum needs the exact weighted map

$$
A_k = \int_{t_{k-1}}^{t_k} e^{(T-s)A}\,ds ,
$$

which by chapter 3 §1 factors into a flow and an integral:

$$
A_k = e^{(T-t_k)A}\int_0^{Step} e^{uA}\,du ,
\qquad T = N\cdot \text{Step} .
$$

Applied as a set operation, the block is

$$
W^{k} = (W^{k-1} \ominus Q_k) + P_k,
\qquad P_k = A_k B P,\quad Q_k = A_k C Q .
$$

Everything about the block is determined by $A_k$ and by the two support images. So the driver's
only numerical choices are how to evaluate $A_k$ and how to apply the difference.

## 2. Splitting the interval

$A_k$ is a single matrix, but the set recurrence is nonlinear in the support vector: applying
$(\cdot \ominus Q_k) + P_k$ once with $A_k$ is *not* the same as applying it to the two halves with the
corresponding half-maps. Refining the partition gives a different — and smaller — set, which is
exactly the monotonicity of chapter 3 §4.

The implementation therefore refines every step into `110` equal pieces of length
`h = Step/110` and applies the recurrence to each piece. Piece `i` sits at distance
$k\cdot \text{Step} - (i+1)h$ from the terminal instant, so its map is

$$
e^{(k\,Step-(i+1)h)A}\,\Phi_h, \qquad \Phi_h = \int_0^{h} e^{uA}\,du .
$$

Summing the pieces telescopes:

$$
\sum_{i=0}^{109} e^{(k\,Step-(i+1)h)A}\,\Phi_h
\;\longrightarrow\;
\int_0^{Step} e^{(k\,Step-s)A}\,ds
\;=\; A_k \quad (h \to 0),
$$

so the refined block still represents the interval map $A_k$, but with the difference taken
piece by piece, which is the alternating sum over a finer partition of the same interval. The
`110` is the same constant as the direction count, and for the same reason: it is the resolution
at which the method samples a continuum.

Two consequences:

* **`A = 0` removes the question.** Every flow factor is the identity, the pieces sum to
  $\text{Step}\cdot I$, and the result is exact for any refinement — which is why the diamond example hits
  its analytic radius to three decimals.
* **Refinement is not free.** Each piece costs a full dual update plus two support
  enumerations, so one block is `110` of those. The inner loop is where essentially all the
  running time goes, and the outer step count is `floor(T/Step)`.

## 3. What the code evaluates

```rust
let h = step / 110.0;
let phi = integrate_exp(a, h);          // Φ_h, computed once per block
for index in 0..110 {
    let remaining = block * step - (index + 1) * h;
    let e = expm(a, remaining);         // e^{(k·Step − (i+1)h)A}
    c = substep(c, &e, &phi)?;          // sum, difference, dual update
}
```

`expm` and `integrate_exp` are truncated at `1e-5`, which is the second numerical choice: the
series for $e^{Ah}$ needs only a few terms at these step sizes, and the truncation error sits
below the `0.01` vertex slack, so it never dominates. `remaining` is the *distance* from the
piece to the terminal instant, which is why it decreases across the loop and why the block
index enters the map at all.

## 4. Error budget

Three approximations act at once, and they are not interchangeable:

| Source | Magnitude | Behaviour under refinement |
|---|---|---|
| direction grid (110 samples) | up to 1.6° in 2D, 18° in 3D | unchanged — it is a constant |
| interval refinement (110 pieces) | second order in `h` | shrinks with `Step` |
| series truncation (`1e-5`) | fixed | independent of `Step` |
| vertex slack (`0.01`) | fixed | independent of `Step` |

Refining `Step` therefore improves only the middle row. The grid gap is the floor: a 3D shell
cannot be capped at the poles no matter how fine the time step, because no sampled direction
points there (see [results](06-results.md)). This is why the convergence test in
`engine/tests/examples.rs` asserts that the final set is unchanged when `Step` is halved at
fixed `T` — it checks the refinement is consistent, not that the answer is exact.

## 5. Capture time

The horizon `T` is an input, so a run answers "what is reachable at `T`". The *first* time at
which capture becomes possible is found by repetition: slice `[0,T]` and reuse the same blocks,
since block `k` is the bridge over $k\cdot \text{Step}$. The membership test of chapter 3 §5 is then `m`
inner products per block against $e^{k\cdot \text{Step}\cdot A}x_0$.

Neither is a separate mode: the printed blocks already carry everything both need, and leaving
them out of the program keeps the output the single source of truth for downstream tools such
as the viewer.
