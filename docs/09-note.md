# 09-note — bands for direction-table instruments, and the predictions that missed

A measurement note, not a paper with a theorem. It states what was measured, what the errors were, and
what survives a prior-art check. Companion to `08-brief.md`; the numbers live in `kb/INDEX.md` and the
claims in `kb/entries/`.

## Abstract

A finite direction table approximates a convex body twice: from outside by intersecting the sampled
support half-spaces, and from inside by shifting the same half-spaces inward by their one-term
allowance. The gap between the two is a band that can be reported with every figure the instrument
produces. On four 2D cases it is `2.8160%` of the diameter (`3.7170%` when the recursion's shortfall is
charged to the inner side); in 3D it is `1.999277` and `1.307773` at `11 x 10` and `16 x 15`, and the
3D source stays labelled `estimate`. Twenty-two predictions missed, and the record of why is as much
the asset as the bands.

## 1. The construction

For a table $\psi_1 \dots \psi_m$ with supports $h_i = h(\psi_i)$:

- outer: $O = \{x : \langle \psi_i, x\rangle \le h_i\}$;
- inner: $I = \{x : \langle \psi_i, x\rangle \le h_i - e_i\}$, $e_i = h_i \tan \rho_i + R(1/\cos \rho_i - 1)$;
- $\rho_i$ the covering radius of the cell of $\psi_i$, `R` the body's circumradius;
- band $= \max_\psi (h_O(\psi) - h_I(\psi)) / 2R$.

The two sides differ in kind: `O` is what a support-function reconstruction gives for free, `I` is the
part that requires measuring whether the shift is inside the body. It is, on 24 of 24 probes across
two analytic twins, by a margin that thins from `0.2` to `0.02` of the allowance as density rises.

## 2. Results

**2D bracket.** Widest from `30.3–42.4%` of the diameter to `2.83%` of the outer radius at `110`
directions (`1.42%` at `220`) — `10.8×` on four cases. The band of record is `2.8160%`, `3.7170%`
folded inward.

**Exact covering radius.** The tie test in the enumeration discarded every genuine Voronoi vertex,
leaving `ρ` a strict lower bound: `19.3787° → 19.8733°` at `11 x 10`, `13.2754° → 13.3967°` at
`16 x 15`, `100` of `100` cells larger. The test is a regular tetrahedron, whose cells meet at the
antipode of the fourth sample: `acos 1/3 = 70.5288°`, not the half gap `54.7356°`. A degenerate triple has two poles and only one was credited: four equatorial
directions plus a pole read `54.7356°` against a true `90.0000°`, so both are credited now, `4` of `5` cells
move, and the shipped tables' radii do not.

**3D violation and alignment.** At `2525` probes the cube exceeds its one-term allowance at `14 x 13`
(`1.016`) and `16 x 15` (`1.046`) and not at `11 x 10`, `15 x 14`, `18 x 12` or `20 x 12`; a table
containing the facet normals (`13 x 12`) reconstructs the cube to `6.3e-14`. Density without alignment
does not repair it — `1.046` survives at `240` directions.

**Cost.** The `2%` rung at block 1 is crossed between `205` and `254` directions, and `2c`'s last block
stays at `18.2%` at `254` because `92.6%` of its band is the recursion's own shortfall.

## 3. The errors

| pattern | n | example | check |
|---|---|---|---|
| dimension carry | 4 | covering radius, pair bound, triple ties | measure the quantity in both dimensions |
| instrument for quantity | 6 | `1440`-angle band, band maximum, `support_at`, inside test | two known-point tests per predicate |
| scaling law assumed | 6 | `e_in ∝ tan ρ`, `2c` tail as density, cap shrink | fit the exponent at four densities |
| coarser-instrument expectation | 3 | `0.005–0.03` shortfall, `1e-6` extents | put the instrument's tolerance in the prediction |
| sound but not tight | 1 | inner gap as a repair, triple's maximum | price a bound as a ratio first |
| method-output assumption | 2 | "the printed set is uniformly outer" | count both inclusions |

The last five rows are one lesson in five forms: the allowance fraction was another case's number,
the outside vertex's `excess` was read as the shortfall, the chart residual was assumed quadratic
and falls as `half^0.44`, the step's Euler error was assumed under budget and is `8.4-20×` over, and
the enumeration's one-sided centre was read as the radius and came back at half the truth on a
degenerate vertex. Every one is a measurement error in the instrument, not a fact about the game.

## 4. What is not new

Both novelty candidates collapsed in a prior-art check. Spherical Voronoi vertices are the facet
normals of the generators' convex hull (Fortune; Renka's `STRIPACK`; SciPy `SphericalVoronoi`), so
crediting ties is a known identity and our step was a bug fix with a one-assertion test. A polytope is
the intersection of its facet half-spaces, and outer approximation with Hausdorff error bounds is
classical (Dudley 1974; Bronshtein–Ivanov 1976; Kamenev's optimal counts; `LazySets`), so the
alignment criterion is a design rule. The bracket is interval arithmetic on a support function.

What is left is a measurement record: the bands, the scan, the alignment numbers and the error log,
each reproducible from a part of `tools/covering_bound.py` or `tools/inner_between_samples.py`.

## 5. Open

- The `205` violation is explained: the shifted cap lands `1.06e-3` above a chamfer facet and the
  vertex spends `45%` of that gap.
- `2c`'s block-8 drift is not a table cost and no count moves it.
- P1 has no affordable affine bridge: the step's Euler error is `8.4-257×` the per-block budget and the
  charting needs `4.9e16` charts, so the `0.150` margin needs a discrete-time reformulation, not a finer
  table.
- P1's gate is `13.04%` of the interior on the `d` axis and `4.77%` on the angle axis, not `11.5%` of
  the support; the nearest measured table is `2.1×` over the loosest reading.

## 6. Reproduction

`cargo build --release --manifest-path engine/Cargo.toml`; the five committed outputs re-run to
`../examples/out/*.out.txt` byte for byte via the identity loop; `python tools/covering_bound.py`
(parts J, M, N, O, P); `python tools/inner_between_samples.py`; `python tools/exact_polygon.py` part F
(memory-heavy: `2a` takes about `100 s`).
