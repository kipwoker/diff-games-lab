# 04-predictive-rule — scoring every pursuer vertex one block ahead removes the `3d` escape

| field | value |
|---|---|
| type | measurement |
| track | 4 |
| date | 2026-11-13 |
| case | all five tracked cases, `3d` from `1.4,1.4,1.4` |
| resolution | 110 directions; 20 000 trials plus two coordinate-descent passes; seeds 1–3 |

**Claim.** The `3d` escape was the rule's, not the bridge's. The counter-direction vertex maximises
the support in the direction that was active *before* the evader moved; scoring each pursuer vertex
by the worst excess it leaves over every direction at the end of the block returns `0.113155` with
capture at `t = 1` and finds no escaping sequence in 20 000 trials at seeds 1, 2 and 3.

| case | extremal, counter | extremal, predictive | search, counter | search, predictive |
|---|---|---|---|---|
| `example-1` | `-0.440376` | `-0.717132` | `-0.294696` | `-0.717132` |
| `example-2a` | `-0.464237` | `-0.466736` | `-0.351250` | `-0.451250` |
| `example-2b` | `-0.442675` | `-0.442675` | `-0.250000` | `-0.442675` |
| `example-2c` | `+0.065902` | `+0.065902` | `+0.065902` | `+0.065902` |
| `example-3d` | `+0.113155` | `+0.113155` | `+0.411668` escape | `+0.113155` |

The rule is the certificate's own quantity one block early: for each vertex of `P`, form the
predicted `z' = z + M(Bu + Cv)`, score it by `max_i (⟨ψ_i, z'⟩ − c_i)`, and take the smallest. It
stays blockwise and positional, and it costs one `4 × 110` scan per block. Because the evader's
vertex is already observed when the pursuer chooses, the gain is real but not free: this is a game
where the pursuer sees the evader's choice, which is also what the old opposition tie-break assumed,
so the two rules are compared fairly and neither is the simultaneous game.

Two rows say what the rule cannot fix. `2c` is unchanged at `0.065902` under both rules and both
searches, because its failure is the deficit measured outward at a facet normal
(`01-deficit-is-chamfering.md`), not a lagging reaction. `example-1` gains the most (`+0.277` at the
extremal evader) and captures at `0.4` instead of `1.2`, which is the same mechanism seen from the
other side: the old rule spent the first blocks aiming at a direction the evader had already left.

**Evidence.** `./target/release/stable_bridge ../examples/example-3d.txt --certify 1.4,1.4,1.4
--search 20000 --seed <1|2|3> --rule predictive`; the other rows are the same command with
`--rule counter`. `engine/tests/predictive.rs` holds the regression: the rule never does worse than
the counter-direction rule on the extremal evader, returns vertices of `P`, and reproduces from its
seed under search.

**Limits.** No escape found is not a proof of none: the search is a heuristic over vertex sequences
with two descent passes, and it still only sees the 110 sampled directions. The pursuer observes the
evader's block control, which a strict simultaneous game would not grant. Nothing here changes the
allowance, so `2c` and `3d` remain uncertified for the reasons track 6 gave.

**Reuse.** Cite as the rule of record before any strategy claim, and as the resolution of the
`3d` counterexample in `04-adversarial-search.md`.

**Links.** `04-adversarial-search.md` (the escape this answers), `04-strategy-rule.md` (the rule it
replaces), `04-certificate-scope.md` (the failures that remain), `01-deficit-is-chamfering.md`.
