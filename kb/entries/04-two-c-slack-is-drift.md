# 04-two-c-slack-is-drift — the exact support leaves all of `0.0659` to the rule

| field | value |
|---|---|
| type | measurement |
| track | 4 |
| date | 2026-11-13 |
| case | `example-2c` from `(0, 0.09)`, block 8, the certificate of `06-two-sidedness.md` |
| resolution | exact polygon arithmetic, 8 blocks × 110 substeps, no direction table |

**Claim.** The exact block-8 bridge's support at direction 27 is `0.100530`; the printed support is
`0.099143`, i.e. `0.001387` *over* the exact one. So none of the `0.065902` certificate slack at that
direction is a support shortfall: the trajectory stands `0.064515` outside the true bridge, and the
earlier lower bound of `0.018207` on the drift was `3.5×` too small.

| quantity | value |
|---|---|
| exact support at `88.3636°`, block 8 | `0.100530` |
| printed support there | `0.099143` |
| printed − exact | `−0.001387` (over) |
| trajectory − exact, the drift | `0.064515` |
| slack the certificate reports | `0.065902` |

The scan over the whole table says direction 27 is ordinary: printed − exact runs from `−0.020049` to
`+0.009936` across the 110 directions, so the largest *shortfall* anywhere is the `0.01`
vertex-acceptance slack and the printed set is never short by more than the method already allows.
`2c`'s failure was never the table.

**Validation.** The same recursion reproduces the committed output: the `x` extents agree to six
decimals on blocks 1–5 (`0.448750`, `0.395000`, `0.338750`, `0.280000`, `0.218750`) and disagree by
at most `0.009976` on blocks 6–8, while the printed `y` extent is `2.87%` over the exact one at every
block — the chamfering sign of `01-deficit-is-chamfering.md` in one column.

Predicted before the run: the printed support would be short by `0.005–0.03`, so the constant part
would be real but the smaller half. Measured: it is not short at all, and the constant part is zero.

**Limits.** One case: `A` nilpotent, `B = C = pi = I`, `P` and `Q` symmetric, so the negation of
`−G B P` is absorbed. The exact recursion uses the engine's own 110 substeps of size `Step/110`, so
the comparison isolates the direction sampling and nothing else. The `0.01` acceptance slack sits
inside the `0.009936`, which is why that figure is quoted as the slack rather than as a measurement
of the loss.

**Method note.** `Polytope::support` maps *directions* by the transform, so the set a support vector
carries is `Gᵀ·P` and not `G·P`. With the untransposed `G` the same recursion reproduces every extent
with `x` and `y` exchanged (`0.45` where the output has `0.44875`), which is how the convention was
caught.

**Evidence.** `python tools/exact_polygon.py` — part A the kernel against hand answers, part B the
eight blocks against the committed extents, part C the direction-27 comparison and the scan.

**Reuse.** Cite before calling `2c` a support deficit: the printed set is over the exact one at the
direction it fails in, and the whole slack is the rule.

**Links.** `04-two-c-slack-is-not-constant.md` (the bound this replaces), `06-two-sidedness.md` (the
slack), `04-certificate-scope.md`, `proposals/04-winning-strategies.md`.
