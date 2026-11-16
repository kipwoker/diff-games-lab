# 06-alignment-is-sound-not-tight — an aligned table has no violation and an unchanged allowance

| field | value |
|---|---|
| type | measurement, prediction falsified; an instrument mismatch recorded |
| track | 6 |
| date | 2026-11-13 |
| case | cube `1.5` and octahedron `1`, five shapes from `110` to `240` directions, `2525` probes |
| resolution | `covering_bound.py` part O: probe-local one-term allowance over the widest support |

**Claim.** Alignment removes the violation and nothing else. Under one definition applied to every
shape — the widest probe-local allowance divided by the widest support — the cube reads `44.58%` at
`11 x 10`, `35.76%` at `13 x 12`, `27.53%` at `16 x 15`, `33.75%` at `18 x 12` and `33.28%` at
`20 x 12`, with the octahedron within `0.3` points throughout. The aligned `13 x 12` has zero
violations and still a `35.8%` allowance, which is `3.1×` P1's `11.5%` margin: soundness and
tightness are separate axes, and the facet-normal table buys only the first.

| table | n | `ρ` | violations | allowance, cube | allowance, octahedron |
|---|---|---|---|---|---|
| `11 x 10` | 110 | 19.8733° | 0 | 44.58% | 44.49% |
| `13 x 12` | 156 | 16.6227° | 0 | 35.76% | 36.02% |
| `16 x 15` | 240 | 13.3967° | **1** | 27.53% | 27.74% |
| `18 x 12` | 216 | 15.8291° | 0 | 33.75% | 33.93% |
| `20 x 12` | 240 | 15.6672° | 0 | 33.28% | 33.50% |

**Prediction.** Written before the run: `9–12%` of the support, comparable to the P1 entry's `12.4%`,
with `18 x 12` possibly under `11.5%` and therefore the first sound 3D table that could carry P1.
Falsified: the measured range is `27–45%`, and `18 x 12` sits at `33.8%`, three times the margin.

**Why the guess was wrong, and what it exposed.** It was formed by comparing part O's quantity with a
different one: the P1 entry's `12.4%` is `0.322693` over `2.603`, while part O's peak allowance on the
same `2.603` support is `1.16`. A factor of `3.6` is more than any cell-radius convention explains, so
at least one of the two figures is mislabelled. The P1 entry's own corrected line uses `0.47`, which
is `1.3 · tan 19.8733°` — the `h tan ρ` term at a `1.3` support, consistent with part O's structure.
The gate's verdict does not depend on the dispute: `0.47 > 0.150` by `3.1×`, so the shipped table
still fails, but the entry's ratios should be re-derived before they are quoted again.

**Corrected.** Not a convention variant: the `12.4%` is `example-3d`'s absolute allowance `0.322693`
over `2.603`, i.e. another case's number, and under the cell radius this same table reads `44.65%`
(`06-allowance-fractions-need-one-convention.md`).

**Consequence.** No shape in this family carries P1. A facet-normal table is a correctness fix — it
removes the one-term violation by construction — not a budget fix; the `11.5%` margin needs the
adaptive density that reads `1.60%` at `240` directions. The two facts are independent, which is why
`16 x 15` can halve the band and stay `estimate` while `13 x 12` is exact at `35.8%` of the support.

**Limits.** One definition, probes not proofs, and the denominator is the widest support, so the ratio
is a worst case over the probe set rather than a norm. The allowance is the one-term form; charging
the triple changes the ratio, not the ordering.

**Evidence.** `python -c "import sys; sys.path.insert(0,'tools'); import covering_bound as c;
c.part_o()"` after the release build.

**Reuse.** Quote alignment as removing violations (`0` at five shapes) and never as reducing the
allowance; quote `44.6% → 27.5%` as the `ρ`-driven scan and cite `tan ρ` as its law.

**Links.** `06-three-d-violation-is-alignment-not-density.md` (the violations),
`02-p1-is-not-yet-certifiable.md` (the disputed ratio),
`06-rung-is-254-at-the-exact-band.md` (the density the gate needs), `docs/08-brief.md`.
