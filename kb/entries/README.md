# Entries

One finding per file. The filename is the id — track number and slug, as in
`06-direction-error-2d.md` — and the claim is falsifiable and contains its number.

The directory is empty until the first result: the file you are reading is the format note, not an
entry. Read the schema in [`../README.md`](../README.md#entry-format) before writing the first one.

```md
# 06-direction-bound — sampled support error is O(R δ²)

| field | value |
|---|---|
| type | method |
| track | 6 |
| date | 2026-11-20 |
| case | example-3d |
| resolution | 110 directions, Step 0.25 |

**Claim.** For a convex set of radius R sampled on directions with local gap δ, the outer
reconstruction error is at most R δ²/8, and the 3D table's 18° pole gap predicts the measured
1.325 factor to within 2%.

| m | predicted z factor | measured | error |
|---|---|---|---|
| 110 | 1.33 | 1.325 | 0.4% |

**Evidence.** `cd engine && cargo test --release error::` and `examples/out/example-3d.out.txt`.

**Limits.** Convex sets only; the diameter is taken at the block, not over the tube.

**Reuse.** Cite for any claim about the 3D extent; rules out attributing 1.325 to the dynamics.

**Links.** `proposals/06-method-evolution.md`, `docs/06-results.md`.
```

Keep it under about 60 lines. If the finding needs two claims, it needs two entries.
