# 06-what-the-falsifications-say — sixteen misses, six patterns, and the check that would have caught each

| field | value |
|---|---|
| type | meta-analysis |
| track | 6 |
| date | 2026-11-13 |
| case | the `16` rows of `docs/07-falsifications.md`, against `52` entries |
| resolution | classify every miss by what it shares with the others, not by its subject |

**Claim.** Sixteen predictions missed, and they are not sixteen unrelated mistakes: four are a 2D
identity carried into 3D, four are an instrument mistaken for the quantity it measures, four assume a
scaling law where the recursion's own loss dominates, two copy an expectation from a coarser
instrument, one prices a sound bound as if it were tight, and one assumes the method's own output is
unbiased. Each pattern has a check that costs one run, and none of the sixteen was found by the check
— each was found by the next measurement.

| pattern | n | examples | the check that would have caught it |
|---|---|---|---|
| dimension carry | 4 | covering radius, pair bound, triple tie test | measure the quantity at both dimensions before reusing the form |
| instrument for quantity | 4 | 1440-angle band, band maximum, `support_at`, inside test | known-point test: centroid inside, offset point outside |
| scaling law assumed | 4 | `e_in` as `c tan ρ`, `2c`'s tail as density, cap shrink | fit the exponent at four densities and report the spread |
| expectation from a coarser instrument | 2 | `0.005–0.03` shortfall, `1e-6` extent match | quote the instrument's own tolerance in the prediction |
| sound but not tight | 1 | the inner gap as a repair; the triple's maximum | price the bound as a ratio before adopting it |
| method-output assumption | 1 | "the printed set is uniformly outer" | test both inclusions, not the convenient one |

**What the ratio says.** `16` misses in `52` entries is `31%`: the predictions are doing work, and
the work is mostly in the instrument. The two instrument patterns plus the tolerance one are `7` of
the `16`, and the dimension carries are `4` more — so eleven misses are about the *measurement*, not
the game. That is also the shape of track 6: it exists to give the other tracks error bars, and its
own error bars are what keep missing.

**The rules this buys.** Six, one per pattern: put the instrument's tolerance in the prediction;
give every new geometric predicate the two known-point tests; count both sides of an inclusion;
price a bound as a ratio before charging it; measure a claimed law at four densities and report the
ratio's spread; and never let an identity cross dimensions without its own measurement.

**Why it is worth a file.** The misses are the only place the method's assumptions are stated
explicitly, and three of them were corrected *within one session* while the narrative in `INDEX.md`
still quoted the retired numbers. The compacted index and the `Superseded` notes exist so that a
later session cannot cite `0.0109–0.0256` as a violation, or `13%` as the band's own error, without
meeting the correction.

**Limits.** The classification is mine rather than mechanical, and several rows fit two patterns —
`06-two-plane-bound-loses-to-the-allowance` is both a dimension carry and a bound that lost on
price. The page holds only the predictions we wrote down, so the rate is a floor: a prediction that
was never phrased as one cannot miss.

**Evidence.** `docs/07-falsifications.md` (the sixteen rows), `kb/entries/` (the claims they correct).

**Reuse.** Before adopting a new instrument, name which of the six patterns it could be and run that
pattern's check first.

**Links.** `docs/07-falsifications.md` (the rows), `06-chamfering-gap-is-not-the-table.md` (the
instrument and the gap), `06-what-the-falsifications-say` (this file),
`proposals/06-method-evolution.md` (E1 and E4 are the instruments this constrains).
