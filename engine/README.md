# The engine

Track 1's solver: a linear differential game's maximal stable bridge, computed by carrying convex
sets as support functions over 110 fixed directions — never as vertex lists. The mathematics is
narrated in [`../docs/`](../docs/README.md), the bench's other tracks in
[`../kb/RESEARCH.md`](../kb/RESEARCH.md); this file is the build and usage note.

## Build and run

```sh
cargo build --release
./target/release/stable_bridge ../examples/example-1.txt out.txt
```

With no arguments the program reads `problem.txt` and writes `out.txt` in the working
directory. Both paths can be overridden positionally; `--help` prints the usage line.

### The certificate

`--certify` runs the closed loop from a start state instead of printing the shells, and reports
how far outside the printed bridge the state gets at each block:

```sh
./target/release/stable_bridge ../examples/example-1.txt --certify 1.4,0
./target/release/stable_bridge ../examples/example-1.txt --certify 1.4,0 --rule max
./target/release/stable_bridge ../examples/example-3d.txt out.txt --certify 1.4,1.4,1.4 --allowance 0.12
```

| option | meaning |
|---|---|
| `--certify x1,x2,…` | start state; one value per state dimension |
| `--rule counter` | pursuer takes the vertex opposite the evader's (default) |
| `--rule max` | pursuer maximises its own support; the control experiment, it fails |
| `--allowance a` | outward slack subtracted before the run prints `certified yes` |
| `--directions n` | 2D direction count, default `110`; 3D is fixed at `110` |
| `--trace path` | per-block, per-direction shortfall of the recursion, for splitting a band |

The report is a plain document, not a `stable_bridge output`: a header, the worst slack, then one
line per block with the state. `worst slack` non-positive means the state stayed inside the set the
run printed for that many blocks left; `captured` is the target-membership test.

No dependencies: the linear algebra is six-by-six at most, and the geometry is its own.

## Input

One problem per file, directives in any order, `#` comments, blank lines between blocks:

```
# a square target driven by equal control sets
dimension 2

constraints M
   1  0  0.5
  -1  0  0.5
   0 -1  0.5
   0  1  0.5

matrix A
  0 0
  1 0

matrix B
  1 0
  0 1

matrix C
  1 0
  0 1

matrix pi
  1 0
  0 1

constraints P
   1  0  2
  -1  0  2
   0 -1  2
   0  1  2

constraints Q
   1  0  2
  -1  0  2
   0 -1  2
   0  1  2

time 2
step 0.2
```

* `constraints M`, `constraints P`, `constraints Q` take rows `coeff… rhs`, meaning
  `⟨a, x⟩ ≤ rhs`. `M` is in state coordinates, `P` in control coordinates, `Q` in disturbance
  coordinates.
* `matrix A`, `matrix B`, `matrix C`, `matrix pi` take one matrix row per line.
* `dimension`, `time` and `step` each take one value and are required, as is every block.
* Row and column counts are implied by the lines written; every shape is checked against the
  sizes the other blocks imply, so a mistyped row is an error with a line number.

### Non-linear descriptions

`nonlinear <kind>` opens a block of `key value` fields, and `grid <name> <low> <high> <count>`
records a reference grid:

```
nonlinear unicycle-pursuit
  speed_pursuer     1.0
  speed_evader      0.7
  turn_rate_pursuer 1.0
  turn_rate_evader  0.5
  scale_theta       1.0
  capture_radius    0.1
  horizon           4.0
  step              0.1

grid d     0.0  2.0  121
grid psi_p 0.0  6.283185307179586  36
grid psi_e 0.0  6.283185307179586  18
```

The engine parses and validates these — `unicycle-pursuit` needs its eight fields and
`turn_rate_evader < turn_rate_pursuer`, which is what keeps the case non-degenerate — and then
refuses to run them: the interval map is a support-function construction and `ẋ = f(x, u, v)` has
none. The refusal names the case's fields, its grid and the tool that does run it
(`../tools/p1_relative.py`). `scale_theta` is the metric a heading needs before a table's inner
products mean anything; `directions::apply_metric` applies it, and it leaves a unit table unchanged at `1`.

### Trajectories and the evader search

`--play <state>` writes a closed loop as `strategy trajectory v1`: one line per block with both
controls, the slack and the state, so the pursuer's and the evader's choices can be read side by
side. `--search <trials> --certify <state>` draws that many random evader vertex sequences, then
scans each block over every vertex, and reports the sequence with the worst slack next to the
extremal baseline. `--seed` makes the search reproducible. The search calls the same `run` as the
certificate, so a difference between them is a difference in evader choice, never in arithmetic.

## Output

A self-describing document: a six-line header, then one block per `Step` up to `floor(T/Step)`.
Each block starts with `block <k> time <t> vertices <n>`, holds one vertex per line — exactly
`n` numbers, nine decimals, single spaces — and ends with `separator`.

```
stable_bridge output v2
dimension 2
T 2
Step 0.2
blocks 10
directions 110

block 1 time 0.2 vertices 31
0.390574797 0.450000000
...
separator
```

The header and the block lines are labels, so reals are written with trailing zeros removed
(`T 2`, `Step 0.2`) while vertex lines keep nine decimals. A reader keys on the first line;
[`../viewer/`](../viewer/) reads the same document.

## Server

`bridge_server` is the same solver behind a small HTTP front end, so a page can ask for a run
without shelling out. It has no dependencies either: the listener, the request parser, the cache
and the job table are standard library only.

```sh
cargo run --release --bin bridge_server [port] [cache-dir]
# defaults: 8787, and `cache` in the working directory
```

| route | what it does |
|---|---|
| `GET /api/health` | `{ok, service, port, cache_dir}`; the viewer's startup probe |
| `POST /api/run[?invalidate=1]` | body is a problem file; answers `{id, cache}` with `cache` = `hit` or `miss` |
| `GET /api/progress?id=…` | `{state, blocks, total, elapsed_ms, cache, message}`; `state` is `running`, `done` or `error` |
| `GET /api/result?id=…` | the output document as `text/plain`; `409` while running, `500` with the message on failure |

The cache holds one `<key>.out.txt` per distinct input text, `key` being FNV-1a 64 bits of the
request body, so a comment-only edit is a different case. `?invalidate=1` deletes that case's
document before the run and recomputes; without it a hit is answered from disk and nothing is
recomputed. A request for a case that is already running follows the live job instead of
starting a second one.

While a run is in flight the job reports finished blocks against the total the input implies,
which is what a progress bar needs; `elapsed_ms` keeps moving even between blocks.

Both binaries share the library, so `stable_bridge` and `bridge_server` produce identical bytes
for the same input — the server only adds the cache, the job table and the routes.

## Modules

| Module | Role |
|---|---|
| `main.rs` | parse, terminal support, block loop, output |
| `input_parser.rs` | the grammar above and its shape checks |
| `directions.rs` | the 110 directions, 2D and 3D |
| `polytope.rs` | boundedness guard, vertices, support vectors |
| `linear_system.rs` | Gaussian elimination with partial pivoting |
| `combinations.rs` | `C(m,n)` enumeration order |
| `matrix.rs` | dense matrix arithmetic |
| `support_function.rs` | `max_v ⟨ψ, v⟩` over a vertex list |
| `pontryagin_difference.rs` | the dual update, i.e. the geometric difference |
| `minkowski_sum.rs` | support-vector sum |
| `nonlinear.rs` | the non-linear grammar, its field check, and the driver's refusal |
| `stable_bridge.rs` | matrix series, interval map, driver |
| `strategy.rs` | both players' rules and the blockwise certificate behind `--certify` |
| `convex_polygon.rs` | vertices from a support vector |
| `output_writer.rs` | vertex lines, block lines, header |

## Tolerances

These are properties of the method, not accidents. Changing one changes the numbers:

* **`1e-5`** truncation of the matrix series for `e^{Ah}` and its integral.
* **`0.01`** vertex de-duplication and half-space acceptance slack. Coarse by design: the set is
  a polyhedral approximation over 110 directions, and the slack absorbs the rounding of the
  solves that build it. It is why output blocks contain near-duplicate vertices.
* **110** directions and 110 sub-steps per block; both are the resolution at which the method
  samples a continuum.

State dimensions other than 2 and 3 are a typed error, not undefined behaviour.

## Tests

`cargo test` runs the unit and end-to-end suites:

* `format` — the grammar accepts any directive order and reports the line of a malformed block;
  the document header states dimension, time and vertex count while vertex lines stay
  byte-stable.
* `pipeline` — the sample parses; both direction tables are unit length and match their
  formulas; the terminal support equals the analytic support function of `M`; the series is
  exact on a nilpotent `A`; a full run has the specified text format; reconstructed vertices
  respect the support values they came from; the zero-dynamics case stays at the terminal set in
  2D and 3D.
* `search` — a replayed extremal sequence reproduces the certificate, the search never returns
  worse than the extremal run, the seed decides the search, and a trajectory carries both controls.
* `nonlinear` — the redefined P1 case parses into its eight fields and three grid axes, a
  free-turning evader is refused, a duplicate field is reported with its line, and the angle metric
  is the identity at scale one.
* `strategy` — the counter-direction rule stays inside the diamond and captures, the maximising rule
  leaves the same bridge, a start outside is reported at block one, and equal authority keeps the
  state in the target.
* `examples` — the five example problems reach their analytic extents — including the 3D growth
  case, whose `x` and `y` half-widths are exactly `0.5 + t` and whose `z` extent is the grid's
  `1.325×` — and halving the step at fixed `T` does not move the limit.
