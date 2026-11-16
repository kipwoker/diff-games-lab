# 04-adversarial-search — the extremal evader is not the adversary, and `3d` escapes

| field | value |
|---|---|
| type | counterexample |
| track | 4 |
| date | 2026-11-13 |
| case | the five tracked closed loops; `example-3d` from `(1.4, 1.4, 1.4)` |
| resolution | 3000 random sequences plus coordinate descent; `3d` confirmed at 20000 trials, seeds 1–3 |

**Claim.** An evader sequence search raises the worst slack above the extremal run on all five
tracked cases, and on `example-3d` finds the sequence `0 0 1 0` — worst slack `0.411668` at block 4,
no capture — which leaves the printed bridge by more than its allowance, so "certified" describes
one trajectory and not the worst one.

| case | start | extremal slack | search slack | captured | escape |
|---|---|---|---|---|---|
| `example-1` | `(1.4, 0)` | `-0.440376` | `-0.294696` | at `1.2` | no |
| `example-2a` | `(0.05, 0.05)` | `-0.464237` | `-0.351250` | at `0.05` | no |
| `example-2b` | `(0.05, 0.05)` | `-0.442675` | `-0.250000` | at `0.05` | no |
| `example-2c` | `(0, 0.09)` | `+0.065902` | `+0.065902` | at `0.05` | no |
| `example-3d` | `(1.4, 1.4, 1.4)` | `+0.113155` | `+0.411668` | none | found |

Every row improves, which is the first finding: the evader's best block-by-block vertex is not its
best sequence. The pursuer's rule is a function of the state through the active direction, so a
deviation at block 3 changes which direction is active at block 4 and with it the pursuer's vertex;
the evader buys more by moving earlier than by maximising each step. `3d` is where that crosses the
allowance, and `0 0 1 0` is deterministic: seeds `1`, `2` and `3` and `20000` trials return the same
sequence and the same `0.411668`. The run captures on every other case, so no escape is claimed
there — a positive slack with capture is a certificate failure without a lost game.

**Evidence.** `./target/release/stable_bridge ../examples/example-3d.txt --certify 1.4,1.4,1.4
--search 20000 --seed 1`; the other rows are the same command with `--search 3000`; the trajectory
is written with `--play` or the search's own output argument.

**Limits.** The search is a heuristic, not an exhaustive game solve: `3000` sampled sequences and two
coordinate-descent passes can miss a worse sequence, and finding one does not mean a better one does
not exist. "Escape" means the state left the *printed* set; the true bridge may still contain it, so
this is a counterexample to the certificate, not a proof that the pursuer loses. The pursuer's rule
was held at `counter-direction`; a rule that anticipated the deviation might survive. `3d`'s
allowance is the 3D estimate, so its escape rests on that figure (`06-two-neighbour-3d.md`).

**Reuse.** Cite before quoting `certified` for a single trajectory, and when asked whether the
counter-direction rule is optimal.

**Resolved.** The escape was the rule's, not the bridge's: scoring every pursuer vertex by
the worst excess it leaves over all directions at the end of the block finds no escaping
sequence in 20 000 trials at seeds 1-3 and captures at `t = 1` (`04-predictive-rule.md`).

**Links.** `04-strategy-rule.md`, `04-certificate-scope.md`, `proposals/04-winning-strategies.md`
(E3).
