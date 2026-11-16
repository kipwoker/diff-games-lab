# 06-two-sidedness — the recursion's own shortfall is zero where `2c` fails

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | `example-1`, `example-2b`, `example-2c` from `(0, 0.09)`, `example-3d` from `(1.4, 1.4, 1.4)` |
| resolution | 110 directions; every substep of every block; the allowance as computed |

**Claim.** `e_in` exists and is computed rather than typed: stage 5 of the dual update returns the
support of a minorant, so `max(0, diff − out)` is how far a stored support may sit below the
recurrence's own dual sum at a sampled direction, and its sum over a block is that block's inward
allowance. It does not rescue `example-2c`. The largest shortfall on `2c` is `0.038166`, but at the
block and direction where the run leaves the set (block 8, direction 27) it is `0.000000`, so the
`+0.065902` slack is real under either explanation left open.

| case | inward at the deciding block | largest anywhere | raw slack | adjusted | verdict |
|---|---|---|---|---|---|
| `example-1` | `0.000000` | `0.028829` | `-0.440376` | `-0.440376` | certified |
| `example-2b` | `0.000000` | `0.012771` | `-0.100000` | `-0.100000` | certified |
| `example-2c` | `0.000000` | `0.038166` | `+0.065902` | `+0.065902` | not certified |
| `example-3d` | `0.047489` (2.07%) | `0.142468` | `+0.113155` | `+0.065665` | certified, estimate, weak |

Predicted before the run: the shortfall is largest where the true body's facets fall between
samples, and `2c` fails at a facet normal of its square target, where the printed set reads *over*
rather than short — so the term would not change that verdict. That is what happened. The argument
does not need the outward reading to be measured on `2c` itself: an inward allowance could only
excuse a state outside the printed set if the true support were *above* the stored one there, and
the stored support is exact at the recurrence's value (loss zero). One qualification, which this
entry first overstepped: loss zero says stored equals the *recurrence*, and `2c` has `A ≠ 0`, so the
recurrence is not the true bridge. What the certificate can say is that no shortfall was
generated at that direction; how much of the slack is a constant offset is
`04-two-c-slack-is-not-constant.md`'s question.

`3d` is the one case where the term bites: `2.07%` of the support at the deciding direction. A pass
whose inward allowance exceeds `2%` of the support is now labelled weak, the same `2%` a rung's
bracket is allowed, because it says the state is inside a set whose own support is that uncertain.
The verdict line can carry both caveats — on `3d` it reads estimate and weak.

**Evidence.** `./target/release/stable_bridge ../examples/example-2c.txt --certify 0,0.09` prints
`inward`, `inward of support`, `inward max` and `adjusted slack`;
`./target/release/stable_bridge ../examples/example-3d.txt --certify 1.4,1.4,1.4` the same;
`engine/tests/inward.rs` holds the regression (traced supports equal printed ones bit for bit, `2c`
unchanged by the term, `3d` labelled weak).

**Limits.** The shortfall is relative to the recurrence's own dual sum, not to the game's true
bridge: on `A = 0` the two coincide, on `2c` (`A ≠ 0`) the second is a linearisation, so `e_in` is a
measurement there and not a bound. It is one-sided in the useful direction only — it excuses a state
outside the printed set when the support was short and never tightens a pass. It is per direction
throughout: `inward max` is reported and never subtracted, because a shortfall at one direction
cannot excuse a state that is outside at another.

**Reuse.** Cite before calling a certificate failure a deficit; cite `2c` as the case where it is
not. Any inward claim must be this computation, not a typed fraction.

**Links.** `01-deficit-is-chamfering.md` (the sign), `06-two-sided-bracket.md` (the other side),
`04-certificate-scope.md` (the ambiguity this closes), `proposals/06-method-evolution.md` (E4).
