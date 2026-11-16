# 04-strategy-rule — the certifying rule is the vertex opposite the evader's

| field | value |
|---|---|
| type | measurement |
| track | 4 |
| date | 2026-11-13 |
| case | the five tracked examples, one start each |
| resolution | blockwise at each case's `Step`, 110 directions, allowance `0` |

**Claim.** With the pursuer taking the vertex of `P` that minimises `⟨ψ, Φ B u⟩` against the active
direction and the evader the vertex of `Q` that maximises `⟨ψ, Φ C v⟩`, the closed loop stays inside
the printed bridge on `example-1`, `2a` and `2b` — worst slacks `-0.440`, `-0.0013`, `-0.100` — and
taking the maximising vertex instead leaves the same bridge by `+4.83` and never captures.

| case | start | worst slack | captured at | verdict |
|---|---|---|---|---|
| `example-1` | `(1.4, 0)` | `-0.440376` | `1.0` | certified |
| `example-2a` | `(0.9, 0)` | `-0.001250` | `0.4` | certified |
| `example-2b` | `(0.4, 0)` | `-0.100000` | `0.05` | certified |
| `example-2c` | `(0, 0.09)` | `+0.065902` | `0.05` | no |
| `example-3d` | `(1.4, 1.4, 1.4)` | `+0.113155` | `1.0` | no |
| `example-1`, maximising | `(1.4, 0)` | `+4.826296` | none | no |
| `example-2b`, maximising | `(0.4, 0)` | `+1.901342` | none | no |
| `example-1`, outside the bridge | `(4, 0)` | `+1.000000` | none | no |

The sign is the thesis. In `c(W^k) = co(c(W^{k-1}) + c(P_k) - c(Q_k))` the pursuer's image enters
through the negated map `-(G·B)`, so the vertex that grows the bridge is the one the unflipped map
calls the minimum; `--rule max` implements the literal reading and is the control experiment. The
`2a` slack of `-0.00125` from a start on the boundary is the informative one: the rule sits on the
facet to the millimetre, which is what a saddle looks like numerically.

**The extremal control is a face.** On `example-2b` at direction `0` the weight is `(0.05, 0)`, so
`⟨w, u⟩` does not depend on `u₂` and the extremal set is an edge, not a point. The first
implementation took the first vertex of that edge for each player — `(-2, 2)` against `(2, 2)` — a
net `(0, 4)` that walked the state from `y = 0.16` to `0.56` in two blocks and out of the target.
Resolving the tie toward `-v` fixes it: every vertex of the face is admissible, but only the one
that opposes the evader cancels, and the slack then reads `-0.100`.

**Evidence.** `./target/release/stable_bridge ../examples/example-1.txt --certify 1.4,0`, and the
same with `--rule max`; `cargo test --test strategy` holds the three certified starts and both
maximising failures.

**Limits.** Five cases, one start each, chosen by hand — not a sample over the bridge. Controls are
constant per block, so a finer partition could only help. `-0.44` on `example-1` is a start at half
the bridge's radius, not a boundary claim. The plant uses `P` and `Q` directly; `C` enters only as
the direction map, which is faithful for the symmetric `Q` of every tracked case and would need
`C·Q` otherwise.

**Reuse.** Cite for the strategy rule, for why "maximise the support" is the wrong reading of the
recurrence, and before quoting a slack as a certificate.

**Links.** `04-certificate-scope.md` (where it fails and why), `proposals/04-winning-strategies.md`,
`06-direction-bound.md` (the allowance this check would use).
