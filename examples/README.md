# Example problems

Track 1's five problems, each with a known answer, written in the input grammar of
[`../engine/README.md`](../engine/README.md). Their tracked outputs are in `out/`, so every
number quoted in the documentation can be checked without re-running anything.

| input | `α` (P) | `β` (Q) | `T` | `Step` | blocks | last block extent |
|---|---|---|---|---|---|---|
| `example-1.txt` | 2 (diamond) | 1 (diamond) | 2 | 0.1 | 20 | x ±3.000, y ±2.931 |
| `example-2a.txt` | 2 | 1 | 0.4 | 0.05 | 8 | x ±0.980, y ±0.926 |
| `example-2b.txt` | 2 | 2 | 0.4 | 0.05 | 8 | x ±0.500, y ±0.514 |
| `example-2c.txt` | 1 | 2 | 0.4 | 0.05 | 8 | x −0.030…0.026, y ±0.100 |
| `example-3d.txt` | 2 (cube) | 1 (cube) | 1 | 0.25 | 4 | x ±1.500, y ±1.500, z ±1.987 |

Example 1 has `A = 0`, so its interval map is exact and only the direction grid contributes
error. Examples 2a–2c share `A = [[0, 0], [1, 0]]` and differ only in the control sizes: the
bridge grows when `P` is larger, stays at the target when they are equal, and shrinks when `Q`
is larger.

Example 3d is the same growth in three dimensions: `A = 0` with a cube target and cube controls,
so the half-width is exactly `0.5 + t` in `x` and `y` — 0.75, 1.00, 1.25, 1.50 — while `z` is
`1.325×` that, because no sampled direction is normal to a `z` face.

The small `y` bulge over `x` (`±0.514` for `±0.500` in 2b) is the grid gap: no sampled
direction is exactly normal to a square's edge, so the reconstructed shell may sit slightly
outside. Example 2c shrinks to a sliver rather than a point because the support values stop
being distinguishable from the `0.01` acceptance slack at that scale.

## Reading an output

Each file in `out/` is one document: header, then a block per step. Serve the repository and
open the viewer on one:

```sh
python -m http.server 8000
# http://localhost:8000/viewer/?src=../examples/out/example-1.out.txt
```

## Regenerate

```sh
cd engine
for f in ../examples/example-*.txt; do
  name=$(basename "$f" .txt)
  ./target/release/stable_bridge "$f" "../examples/out/$name.out.txt"
done
```

The runs are deterministic, so the tracked files are reproduced byte for byte.
