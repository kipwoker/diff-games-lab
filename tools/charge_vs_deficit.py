"""Whether the bracket's inner side is inside the exact body, and what sets the margin.

The bracket's inner side is `c - a - s`: the printed support, minus the printed boundary's
slide `a`, minus the recursion's charge `s`. Being inside the true set at a sampled direction
needs `c - a - s <= h_W`, so the slack of that inequality is

    slack_i = a_i + s_i - (c_i - h_W(psi_i))

and it is positive exactly when the inner side is inside the exact body there. On `2c` the
exact per-block polygons exist (`exact_polygon.py`), so the check can be made at all 110
directions instead of at the widest one.

`--trace` carries the stored support, the charge and the per-direction allowance, so all three
come from the engine and only `h_W` comes from the exact recursion.

    python tools/charge_vs_deficit.py

Prediction, written before the run: `slack` is positive at every direction of every block — the
2D allowance covers the printed set's excess, which is what makes the inner side sound — and at
each block's band direction the printed support is short of the exact one, so no charge is
needed there. If a slack is negative, the *allowance* is what fails, not the charge.

Requires the release binary and numpy.
"""

import os
import re
import subprocess
import sys

import numpy as np

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import exact_polygon as ex

ENGINE = os.path.join("engine", "target", "release", "stable_bridge")
TRACE = re.compile(
    "block ([0-9]+) direction ([0-9]+) inward ([^ ]+) support ([^ ]+) allowance ([^ ]+)"
)
BRACKET = re.compile("block ([0-9]+) direction ([0-9]+) inner ([^ ]+) outer ([^ ]+) width ([^ ]+)")


def dump():
    """`(block, direction) -> (charge, support, allowance)` from the recursion's dump."""
    path = os.path.join("engine", "target", "trace", "2c-charge.txt")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    subprocess.run(
        [ENGINE, "examples/example-2c.txt", "--trace", path],
        check=True,
        capture_output=True,
        text=True,
    )
    rows = {}
    for line in open(path, encoding="utf-8"):
        found = TRACE.match(line.strip())
        if found:
            block, direction, charge, support, allowance = found.groups()
            rows[(int(block), int(direction))] = (
                float(charge),
                float(support),
                float(allowance),
            )
    return rows


def bands():
    """`block -> direction` at the widest sampled width."""
    result = subprocess.run(
        [ENGINE, "examples/example-2c.txt", "--bracket"],
        check=True,
        capture_output=True,
        text=True,
    )
    rows = {}
    for line in result.stdout.splitlines():
        found = BRACKET.match(line.strip())
        if found:
            block, direction = found.groups()[:2]
            rows[int(block)] = int(direction)
    return rows


def main():
    table = ex.directions()
    count = len(table)
    dumped = dump()
    widest = bands()
    exact = ex.two_c()
    print("per block: `over` is printed minus exact at the band direction (negative is inside the")
    print("true set), `slack` is `a + s - over`, and the last columns are the tightest direction")
    print("over all 110 and how many directions have a negative slack.")
    print(
        f"   {'block':>6}{'dir':>5}{'c':>10}{'h':>10}{'over':>10}{'a':>9}{'s':>9}"
        f"{'slack':>10}{'min slack':>11}{'at':>5}{'neg':>5}"
    )
    negative = []
    lowest = (float("inf"), 0, 0)
    for block in range(1, len(exact) + 1):
        true = np.array([ex.support(exact[block - 1], psi) for psi in table])
        shown = np.array([dumped[(block, i)][1] for i in range(count)])
        charge = np.array([dumped[(block, i)][0] for i in range(count)])
        slide = np.array([dumped[(block, i)][2] for i in range(count)])
        over = shown - true
        slack = slide + charge - over
        tight = int(np.argmin(slack))
        direction = widest[block]
        bad = int(np.sum(slack < 0.0))
        if bad:
            negative.append((block, bad))
        if slack[tight] < lowest[0]:
            lowest = (slack[tight], block, tight)
        print(
            f"   {block:6}{direction:5}{shown[direction]:10.6f}{true[direction]:10.6f}"
            f"{over[direction]:10.6f}{slide[direction]:9.6f}{charge[direction]:9.6f}"
            f"{slack[direction]:10.6f}{slack[tight]:11.6f}{tight:5}{bad:5}"
        )
    print()
    print(
        f"   tightest slack over all blocks and directions: {lowest[0]:.6f} "
        f"at block {lowest[1]} direction {lowest[2]}"
    )
    print(f"   blocks with a negative slack: {len(negative)} {negative}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
