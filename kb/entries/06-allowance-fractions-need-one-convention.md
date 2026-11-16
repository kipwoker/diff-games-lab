# 06-allowance-fractions-need-one-convention — and P1's `12.4%` came from another case

| field | value |
|---|---|
| type | measurement, one correction |
| track | 6 |
| date | 2026-11-14 |
| case | octahedron `1`, cube `1.5`; shipped `11 x 10` and `16 x 15`, adaptive `110` and `240` |
| resolution | `covering_bound.py` part J: `e = h tan ρ + R(1/cos ρ − 1)` at each cell's covering radius |

**Claim.** An allowance fraction means nothing without its `ρ`: the same table reads `17.90%` of the
support with half the nearest gap and `44.65%` with the cell's covering radius. P1's gate was quoted
with neither — its `12.4%` is `example-3d`'s absolute engine allowance `0.322693`
(`06-two-neighbour-3d.md`) divided by `2.603`, so it is a different case's number, not this table's
fraction.

| body | table | half-gap `ρ` | cell `ρ` | `e/h`, half gap | `e/h`, cell |
|---|---|---|---|---|---|
| cube `1.5` | `11 x 10` | 9.0000° | 19.8733° | `17.90%` | `44.65%` |
| cube `1.5` | `16 x 15` | 6.0000° | 13.3967° | `11.28%` | `27.14%` |
| octahedron `1` | `11 x 10` | 9.0000° | 19.8733° | `17.48%` | `44.49%` |
| octahedron `1` | `16 x 15` | 6.0000° | 13.3967° | `11.28%` | `27.62%` |
| cube `1.5` | adaptive `110` | 2.7398° | 38.5872° | `5.00%` | `118.13%` |
| octahedron `1` | adaptive `240` | 0.9048° | 39.6981° | `1.60%` | `120.53%` |

**Prediction.** Written before the run: `12.4%` would turn out to be the cell-radius fraction on an
earlier probe set, so the repair was a convention and the P1 ordering stood. Falsified on provenance:
the numerator is another case's absolute allowance, so the error was an absolute number used as a
fraction — a plain mix-up, not a radius variant — and the ordering does not survive the cell radius
either.

**What the numbers say now.**

1. The gate still fails under every convention: `0.4465 × 1.3 = 0.580` against `0.150` is `3.9×` too
   large, and even the half-gap lower bound (`0.233`) is `1.6×` too large. The verdict is
   convention-free; the ratio is not, which is why it has to be quoted with its `ρ`.
2. The adaptive route is not headroom. The greedy clusters and leaves a `39.7°` cell
   (`06-adaptive-table-under-the-covering-radius.md`), so its allowance *exceeds* the support:
   `118–120%`, not `1.60%`. That `1.60%` was the half-gap lower bound, quoted as if it were the
   quantity — the "instrument for quantity" pattern for a third time.
3. Nothing measured clears `11.5%`: `44.65%` (`11 x 10`), `27.14%` (`16 x 15`), `35.8%` probe-local at
   the aligned `13 x 12` (`06-alignment-is-sound-not-tight.md`). So P1's table requirement is unproven
   under any covering-radius convention, while the missing bridge is a separate and independent block.

**Limits.** `e/h` pairs an allowance with the support at the same cell; the two need not share a
direction, which is exact on the cube and approximate on the octahedron. The adaptive rows use this
tool's clustered greedy, not the best table of that count, and `circumradius` warns on the nan pairs a
duplicate direction creates.

**Evidence.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as c;
c.part_j()"` — the four cell fractions and their `ρ` per table.

**Reuse.** Quote an allowance fraction only with the radius that produced it, and never quote
`0.322693` as a cube number: it is `example-3d`'s engine estimate.

**Links.** `06-two-neighbour-3d.md` (the `0.322693`), `02-p1-is-not-yet-certifiable.md` (the rebuilt
table), `06-three-d-adaptive-allowance.md` (the half-gap figures), `06-three-d-covering-radius.md`
(why the two radii differ by `2.93×`).
