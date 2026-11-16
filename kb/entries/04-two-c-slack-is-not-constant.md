# 04-two-c-slack-is-not-constant — the drift decelerates, so at least `0.0182` of `2c`'s slack is the rule

| field | value |
|---|---|
| type | measurement |
| track | 4 |
| date | 2026-11-13 |
| case | `example-2c` from `(0, 0.09)`, the certificate of `06-two-sidedness.md` |
| resolution | 8 blocks of `0.05 s`, 110 directions, the printed rule |

**Claim.** A support shortfall at a fixed direction is a constant, so it can only explain a slack
that is constant at that direction. `2c`'s active direction changes for four blocks and then settles
on 27, and from there the slack grows by `0.0085`, `0.0061`, `0.0036`: it decelerates. At least
`0.065902 − 0.047695 = 0.018207` of the final slack is therefore accumulated drift, and at most
`0.047695` could be a constant support offset.

| block | active direction | slack |
|---|---|---|
| 1 | 16 | `0.008185` |
| 2 | 22 | `0.015233` |
| 3 | 24 | `0.026335` |
| 4 | 26 | `0.037662` |
| 5 | 27 | `0.047695` |
| 6 | 27 | `0.056192` |
| 7 | 27 | `0.062261` |
| 8 | 27 | `0.065902` |

Predicted before the run: a linear drift at `0.008` per block after block 1, and a constant part of
at most `0.0082`. Both were wrong. The first four rows are at *different* directions, so they say
nothing about direction 27 and the constant part could be as large as `0.0477`; and the increments
fall rather than hold, which rules out a steady rate as well as a constant offset. What the sequence
does support is the weakest of the three readings: some of the failure is the rule and some is
unresolved.

The unresolved part is arithmetic. Closing it needs the true support of `2c`'s block-8 set at
direction 27, and `A ≠ 0` means the recurrence's own value is not that: `e_in` measures the
reconstruction's loss against the recurrence (`06-two-sidedness.md`), not against the game. Exact
polygon arithmetic — Minkowski sum, then the Pontryagin difference at the facet normals of the
larger set, then the block's flow — would give the missing number, and reproducing the printed
supports at 110 directions would validate it before it is asked anything.

**Limits.** The active direction moves with the state, so the table is not eight measurements of one
quantity. The `0.01` vertex-acceptance slack is a tenth of `2c`'s own `0.1` scale and contaminates
both the printed supports and any exact comparison at this size. The split is a bound on the drift
only from below: a constant offset anywhere from `0` to `0.0477` is consistent with the table.

**Settled.** The exact support is now computed, by polygon arithmetic rather than a table
(`04-two-c-slack-is-drift.md`): the printed support at direction 27 is `0.001387` *over* the exact
one, so the constant part is `0` and the whole slack is drift. The lower bound of `0.018207` here is
replaced by `0.064515`, and the `0–0.0477` range collapses to a point.

**Reuse.** Cite before calling `2c` a deficit or a rule failure without the exact support; the
dec(cl)eration is the part that is measured.

**Links.** `06-two-sidedness.md` (the inward term), `04-certificate-scope.md` (the constant-offset
reading this corrects), `04-predictive-rule.md` (`2c` unchanged by the lookahead rule),
`proposals/04-winning-strategies.md`.
