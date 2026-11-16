"""Where a block's printed band goes: its allowance or its shortfall.

`--bracket` prints `width = 2 * allowance + shortfall` at each block's widest direction, which
is two terms in one number. `--trace` dumps the recursion's own per-direction shortfall, so the
band can be split at the block it was printed for and compared across blocks.

The question is `2c`'s last block, which reads `17-20%` of the diameter at every table size
while its first block reads `3.7%`. If the absolute shortfall stays near its first-block value
while the support collapses, the share is a ratio: no table size moves it, and the `2%` rung
has to be read per block. If instead the absolute shortfall shrinks with the set, the collapse
is not the explanation and something else is.

    python tools/inward_profile.py

Requires the release binary (`cargo build --release --manifest-path engine/Cargo.toml`).
"""

import os
import re
import subprocess

ENGINE = os.path.join("engine", "target", "release", "stable_bridge")
SCRATCH = os.path.join("engine", "target", "trace")
CASES = ("example-1", "example-2a", "example-2b", "example-2c")
COUNT = 110

BRACKET = re.compile(
    "block ([0-9]+) direction ([0-9]+) inner ([^ ]+) outer ([^ ]+) width ([^ ]+) "
    "of diameter ([0-9.]+)%"
)
TRACE = re.compile("block ([0-9]+) direction ([0-9]+) inward ([^ ]+) support ([^ ]+)")


def bracket(case):
    """Per block: `(direction, outer, width, local share)` at the block's widest direction."""
    result = subprocess.run(
        [ENGINE, f"examples/{case}.txt", "--bracket", "--directions", str(COUNT)],
        capture_output=True,
        text=True,
        check=True,
    )
    rows = {}
    for line in result.stdout.splitlines():
        found = BRACKET.match(line.strip())
        if found:
            block, direction, _, outer, width, share = found.groups()
            rows[int(block)] = (int(direction), float(outer), float(width), float(share))
    return rows


def trace(case):
    """Per `(block, direction)`: `(inward, support)` from the recursion's own dump."""
    os.makedirs(SCRATCH, exist_ok=True)
    path = os.path.join(SCRATCH, f"{case}-{COUNT}.txt")
    subprocess.run(
        [ENGINE, f"examples/{case}.txt", "--trace", path, "--directions", str(COUNT)],
        capture_output=True,
        text=True,
        check=True,
    )
    rows = {}
    for line in open(path, encoding="utf-8"):
        found = TRACE.match(line.strip())
        if found:
            block, direction, inward, support = found.groups()
            rows[(int(block), int(direction))] = (float(inward), float(support))
    return rows


def main():
    print("each case's band split at three blocks: `1`, the last, and the widest.")
    print("`inward` and `support` are read at the width's own direction, so `2 * allowance` is")
    print("what the width has left over. Prediction, written before the run: the last block's")
    print("absolute shortfall is within a factor of two of block 1's, so its `17-20%` share is")
    print("the support collapsing under a shortfall that did not.")
    print()
    print(
        f"   {'case':>12}{'block':>6}{'inward':>10}{'support':>9}{'inward %':>9}"
        f"{'width':>9}{'2 allow':>9}{'width/(2R) %':>13}"
    )
    for case in CASES:
        widths = bracket(case)
        dumps = trace(case)
        if len(widths) != max(dumps)[0]:
            raise SystemExit(f"{case}: {len(widths)} bracket blocks vs {max(dumps)[0]} traced")
        last = max(widths)
        widest = max(widths, key=lambda block: widths[block][2])
        picked = [1, last] if widest in (1, last) else [1, widest, last]
        read = {}
        for block in picked:
            direction, outer, width, share = widths[block]
            inward, support = dumps[(block, direction)]
            allowance = (width - inward) / 2.0
            read[block] = inward
            print(
                f"   {case:>12}{block:6}{inward:10.6f}{support:9.4f}"
                f"{100.0 * inward / support if support else 0.0:9.3f}"
                f"{width:9.6f}{2.0 * allowance:9.6f}{share:13.4f}"
            )
        if last != 1:
            print(f"   {'':>12}{'':>6}{'ratio last/1':>10}{read[last] / read[1]:>9.2f}")
    print()
    print("   A ratio above `1` with support far below block 1's is the collapse, not a worse")
    print("   recursion: the band denominator shrank and the boundary's shortfall did not.")


if __name__ == "__main__":
    main()
