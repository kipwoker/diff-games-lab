# 02-nonlinear-grammar — the case parses, and the engine names the method it lacks

| field | value |
|---|---|
| type | method |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1b.txt` |
| resolution | parser and field check only; no bridge is computed |

**Claim.** `nonlinear unicycle-pursuit` parses into eight named fields and three grid axes, its
defining inequality `turn_rate_evader < turn_rate_pursuer` is enforced, and the driver refuses the
case by naming track 2's E4 — so the spec file stops being a parse error and the gap is stated in
one place instead of leaking out as "unexpected non-linear".

| object | what the engine does |
|---|---|
| `nonlinear <kind>` + `key value` lines | parsed; a duplicate key is a line-tagged error |
| `grid <name> <low> <high> <count>` | parsed; count at least `2` |
| `unicycle-pursuit` | eight fields required, `w_e < w_p` enforced, positives checked |
| advancing the bridge | refused, exit `1`: "the interval map for it is track 2's E4 and is not built" |

The metric is the one piece of track 1 that survives unchanged: `scale_theta` is a diagonal metric
`diag(1, 1, 1/scale)` on the direction table, so a heading becomes a length (`v_p/ω̄`) and the
existing inner products are defined. `directions::apply_metric` applies it and leaves a unit table
unchanged at `1`; the table itself is not rebuilt, which is what the track-2 plan claims.

**Evidence.** `cargo test --test nonlinear` (six tests); `./target/release/stable_bridge
../examples/p1/example-p1b.txt` prints the refusal and exits `1`. The five linear outputs are
byte-identical after the change, and the old case still reports its own missing field
(`turn_rate_pursuer`), which is the record of the degenerate definition.

**Limits.** Parsing and a guard, not a solver: no bridge, no linearisation remainder, no dynamics
evaluation. The refusal is a property of this engine, not a statement that the case is unsolvable —
`tools/p1_relative.py` solves it on a grid.

**Reuse.** Cite when a non-linear case has to be identified as one, and before claiming the parser
cannot read P1.

**Links.** `02-p1-relative-interior.md`, `proposals/02-nonlinear-games.md` (E1).
