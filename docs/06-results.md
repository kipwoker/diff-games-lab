# Results

The repository carries five problems with known analytic answers, plus the tracked outputs of
each. They exercise the three qualitative behaviours of the recurrence — growth, cancellation
and shrinkage — and they separate the two error sources: the diamond case removes the time
discretization entirely, while the square cases keep the drift and the grid.

## 1. The problems

**Example 1 — diamonds, no drift.** `A = 0`, `B = C = π = I`, target `|x₁| + |x₂| ≤ 1`, first
player's control set `|u₁| + |u₂| ≤ 2`, second player's `|v₁| + |v₂| ≤ 1`, `T = 2`, `Step = 0.1`.
No drift means every flow factor is the identity, so the interval map is exact and only the
direction grid remains.

**Examples 2a–2c — squares, with drift.** `A = [[0,0],[1,0]]`, `B = C = π = I`, target
`|x_i| ≤ 0.5`, `T = 0.4`, `Step = 0.05`, with `P` a square of radius `α` and `Q` a square of
radius `β`:

| case | α | β | what should happen |
|---|---|---|---|
| 2a | 2 | 1 | the bridge grows |
| 2b | 2 | 2 | `P = Q`, so the bridge is the target at every block |
| 2c | 1 | 2 | the bridge shrinks |

**Example 3d — cubes, no drift.** The same construction in three dimensions: `A = 0`,
`B = C = π = I`, target `|x_i| ≤ 0.5`, `P` a cube of radius 2, `Q` a cube of radius 1,
`T = 1`, `Step = 0.25`.

All five are in [`examples/`](../examples/), and every output is in
[`examples/out/`](../examples/out/).

## 2. What the runs produce

Last-block extents, from the tracked outputs:

| case | blocks | vertices | last-block extent |
|---|---|---|---|
| 1 | 20 | 498 | x ±3.000, y ±2.931 |
| 2a | 8 | 504 | x ±0.980, y ±0.926 |
| 2b | 8 | 168 | x ±0.500, y ±0.514 |
| 2c | 8 | 333 | x −0.030 … 0.026, y ±0.100 |
| 3d | 4 | 186 | x ±1.500, y ±1.500, z ±1.987 |

Three things to read out of the table:

* **Example 1 is exact on the diagonal.** With `A = 0` the only error left is the grid, and the
  diamond of radius 3 comes out at 3.000 to three decimals. The `y` extent of 2.931 is the grid
  discretization of the diamond's 45° facets: with 110 samples the nearest direction to a
  diagonal is at most 1.6° away, and the shell is a 110-gon inscribed in the support values.
* **2b is the cancellation case.** `P = Q` makes the two images equal, so the recurrence
  re-derives the previous set and the target square is reproduced. The `y` bulge to 0.514 is the
  same grid gap: no sampled direction is exactly normal to the square's edges, so the
  reconstructed shell may sit slightly outside.
* **2c shrinks to the grid's resolution.** A width of 0.056 and a height of 0.2 are not a
  mathematical point: at that scale the support values are no longer distinguishable from the
  `0.01` acceptance slack, so the printed set is a sliver rather than a single vertex.
* **3d is exact where the grid samples.** `A = 0` again, so the bridge widens by `Step` per
  block and the half-width is `0.5 + t` — 0.75, 1.00, 1.25, 1.50 — in `x` and `y`, whose normals
  are in the table. `z` is not sampled, so the printed shell is `1.987` tall instead of `1.500`:
  a factor `1.325`, the same in every block because all the support values scale together.

## 3. Mesh consistency

Example 2a is the useful convergence check. Halving `Step` from `0.05` to `0.025` and again to
`0.0125` at fixed `T` leaves `x = ±0.980` unchanged: the value is the scheme's limit for this
grid, not a mesh artifact. That is what an alternating sum should do — refine the partition and
the answer stops moving once the remaining error is the direction grid's.

The same check is in `engine/tests/examples.rs` (`refining_the_step_leaves_the_limit_alone`), so
a change to the interval map that breaks the limit shows up as a test failure rather than as a
slightly different number in a table.

## 4. Where the accuracy goes

Example 3d is the three-dimensional case, and it shows the floor of the method. The 110
directions sit on 10 elevation rings, so the nearest sample is 18° from the `z` poles. Nothing
bounds a shell along that axis: `z` comes out `1.325×` wider than the cube it is meant to be,
`±1.987` against `±1.500` in the last block, and the factor is the same in the earlier blocks
because the support values all scale together. `x` and `y` stay exactly at `0.5 + t`, since their
facet normals are in the table.

`engine/tests/pipeline.rs` carries the degenerate `P = Q` version of the same construction. With
nothing moving, the printed polytope still reaches `z = ±0.668` for a cube of half-width 0.5, and
the support values stay within `9.5e-2` of the cube's exact support function without growing
with the block index. That residual and the `1.325` factor belong to the direction grid, not to
the time step.

This is the honest characterization of the method's error: it is geometric, it is set by the
direction table, and refining time does not touch it.

## 5. Reproducing

```sh
cd engine
cargo build --release

for f in ../examples/example-*.txt; do
  name=$(basename "$f" .txt)
  ./target/release/stable_bridge "$f" "../examples/out/$name.out.txt"
done
```

Each run prints the block and vertex counts it wrote. To look at one, serve the repository and
open the viewer on any output file:

```sh
python -m http.server 8000
# http://localhost:8000/viewer/?src=examples/out/example-1.out.txt
```

The document states each block's time, so the timeline needs no input file; the 2D fill and
outline are derived from a convex hull, because the printed vertex order is an enumeration
rather than a traversal.
