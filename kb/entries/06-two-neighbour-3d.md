# 06-two-neighbour-3d — the pair form stops at 2D, so the 3D allowance is labelled an estimate

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | octahedron `{|x|+|y|+|z| <= 1}`, cube `1.5`, `example-3d` |
| resolution | `11 x 10` and `12 x 15` tables; 110 directions in the engine; `Step 0.25` |

**Claim.** The two-neighbour formula does not generalise to 3D — applied to the nearest
three samples it lands *below* the measured extent (`0.5800` against `0.6688` on the
octahedron), because dropping constraints in 3D leaves a direction out of the pair's span
unbounded — so the engine computes its 3D allowance from the one-term form and reports it
as an estimate, not a bound.

| case | table | `ρ` to facet normal | pair over the cell | three-sample vertex | measured `h_P` |
|---|---|---|---|---|---|
| octahedron | `11 x 10` | `7.3492°` | `0.5800` | `0.5964` | `0.6688` |
| octahedron | `12 x 15` | `11.8089°` | `0.7032` | `0.7520` | `0.7344` |
| cube `1.5` | `11 x 10` | `14.9418°` | `2.0751` | `2.3864` | `2.3864` |
| cube `1.5` | `12 x 15` | `12.6343°` | `2.3514` | `2.3864` | `2.3864` |

In 2D two planes bound the set: their intersection is a vertex and the extent along any
direction between them is that vertex. In 3D three planes still leave the set unbounded
along `a × b`, so neither the pair's vertex nor the three-sample vertex is an upper bound,
and the second row shows the good case only because the cube's facets are sampled. The
measured extent is therefore still the only reference in 3D, exactly as
`06-two-neighbour-bound.md` said, with the stronger reading that no local formula replaces it.

| certificate, calculated allowance | `example-2a` | `example-2b` | `example-2c` | `example-3d` |
|---|---|---|---|---|
| allowance | `0.037483` | `0.020218` | `0.018186` | `0.322693` |
| allowance source | validated | validated | validated | estimate |
| worst slack | `-0.464237` | `-0.442675` | `+0.065902` | `+0.113155` |
| verdict | certified | certified | not certified | certified as estimate |

`2c` stays the informative failure: its allowance (`0.018`) is a quarter of its slack
(`0.066`), so the deficit is visible against the discretisation. On `3d` the computed
allowance is `0.323` — nearly three times the `0.113` it is meant to detect — so the run
passes on paper and the entry says so in the document header. The typed `0.12` that
certified `3d` before now sits *below* the computed figure; it was a number, not a bound,
which is what this step removed.

**Evidence.** `./target/release/stable_bridge ../examples/example-3d.txt --certify
1.4,1.4,1.4` prints `allowance 0.322693`, source `two-neighbour, estimate`, verdict
`certified yes (estimate, not a bound)`; `python tools/covering_bound.py` part G.

**Limits.** `ρ` in 3D is half the nearest-neighbour gap, an estimate of the cell's covering
radius rather than its supremum. `R` is the printed set's circumradius, which can sit below
the true one when the stored supports are short. Both errors are in the generous direction
for the allowance and the conservative one for the claim, which is why 3D is never called
certified. The 2D figure is validated only on the tracked cases (`0.1%` above the square).

**Reuse.** Cite before quoting any 3D `certified`, and as the reason a 3D claim needs a
denser table or an inner bound rather than a smaller allowance.

**Links.** `06-two-neighbour-bound.md`, `04-certificate-scope.md`,
`proposals/06-method-evolution.md` (E1, E4).
