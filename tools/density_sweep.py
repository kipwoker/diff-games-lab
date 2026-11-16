"""The printed band of a 2D case against its table size.

`--directions` makes the 2D count a run parameter, so the rung's `2%` question can be measured
instead of projected. For every case and count this runs the engine's `--bracket`, takes each
block's band `width = 2 * allowance + shortfall`, and divides it by `2 R`, where `R` is the
printed block's circumradius read from the vertex document of the same run.

That denominator matters. The bracket's own `of diameter` is `width / (2 * outer)` at the
widest direction, which is a local figure: at a block whose set has nearly collapsed it reads
`38%` for a band that is `3.7%` of the diameter. The entries' bands are `width / (2 R)`, and
this script reproduces them.

The sampled `width` is a grid figure. The bracket also prints the exact band,
`max_psi (h_outer - h_inner)`, the largest distance from an outer vertex to the inner polygon,
and this script reports its share beside the sampled one, so the rung can be read at the
instrument that is not a sample.

Part E of `covering_bound.py` supplies the outward share `A = allowance / R` at the same
tables, so the inward share is `band - A`.

    python tools/density_sweep.py

Requires the release binary (`cargo build --release --manifest-path engine/Cargo.toml`).
"""

import math
import os
import re
import subprocess

ENGINE = os.path.join("engine", "target", "release", "stable_bridge")
SCRATCH = os.path.join("engine", "target", "density")
CASES = ("example-1", "example-2a", "example-2b", "example-2c")
COUNTS = (110, 158, 205, 254)

PATTERN = re.compile(
    r"block (\d+) direction (\d+) inner (\S+) outer (\S+) width (\S+) of diameter ([\d.]+)%"
)
EXACT = re.compile(r"block (\d+) exact width (\S+)")


def bracket(case, count):
    """Every finite block of one bracket run, sampled and exact: `(rows, exact)`."""
    result = subprocess.run(
        [ENGINE, f"examples/{case}.txt", "--bracket", "--directions", str(count)],
        capture_output=True,
        text=True,
        check=True,
    )
    rows = []
    exact = {}
    for line in result.stdout.splitlines():
        stripped = line.strip()
        found = PATTERN.match(stripped)
        if found:
            block, direction, _, outer, width, share = found.groups()
            rows.append((int(block), int(direction), float(outer), float(width), float(share)))
        found = EXACT.match(stripped)
        if found:
            block, width = found.groups()
            exact[int(block)] = float(width)
    return rows, exact


def radii(case, count):
    """Per-block circumradius of the printed set, from the vertex document."""
    os.makedirs(SCRATCH, exist_ok=True)
    path = os.path.join(SCRATCH, f"{case}-{count}.out.txt")
    subprocess.run(
        [ENGINE, f"examples/{case}.txt", path, "--directions", str(count)],
        capture_output=True,
        text=True,
        check=True,
    )
    found = []
    current = None
    for line in open(path, encoding="utf-8"):
        line = line.strip()
        if line.startswith("block "):
            current = 0.0
        elif line == "separator":
            found.append(current)
            current = None
        elif current is not None and line:
            current = max(current, math.hypot(*[float(v) for v in line.split()]))
    return found


def main():
    print("the printed band of each case at each table size, `share = width / (2 R)`.")
    print("`block 1` is a fixed-block comparison; `widest` is the band the entries quote, which")
    print("follows the largest absolute width and can move between blocks as the count rises.")
    print("Prediction, written before the run: `2c`'s block-1 share falls from about 3.7% at 110")
    print("to 2.6% at 158 and 2.0% at 205, and `example-1` reaches 2.0% at 158.")
    print()
    print(
        f"   {'case':>12}{'m':>5}{'block 1 %':>11}{'last %':>9}{'widest':>10}{'at':>4}"
        f"{'widest %':>10}{'rho deg':>9}"
    )
    collected = []
    for case in CASES:
        for count in COUNTS:
            rows, exact = bracket(case, count)
            radius = radii(case, count)
            if len(rows) != len(radius):
                raise SystemExit(f"{case} {count}: {len(rows)} rows vs {len(radius)} blocks")
            shares = [
                100.0 * row[3] / (2.0 * r) if r > 0.0 else 0.0 for row, r in zip(rows, radius)
            ]
            widest = max(rows, key=lambda row: row[3])
            widest_share = shares[widest[0] - 1]
            exact_share = {
                block: 100.0 * exact[block] / (2.0 * radius[block - 1])
                if radius[block - 1] > 0.0
                else 0.0
                for block in exact
            }
            top = max(exact_share, key=exact_share.get)
            collected.append((case, count, exact_share[1], top, exact_share[top]))
            rho = math.degrees(math.pi / count)
            print(
                f"   {case:>12}{count:5}{shares[0]:11.4f}{shares[-1]:9.4f}{widest[3]:10.6f}"
                f"{widest[0]:4}{widest_share:10.4f}{rho:9.4f}"
            )
    print()
    print("   the exact band, `exact_width / (2 R)`, where the width is `max_psi (h_outer - h_inner)`")
    print("   rather than a figure at the samples:")
    print(f"   {'case':>12}{'m':>5}{'block 1 %':>11}{'widest %':>10}{'at':>4}")
    for case, count, one, top, share in collected:
        print(f"   {case:>12}{count:5}{one:11.4f}{share:10.4f}{top:4}")
    print()
    print("   A band that falls like `tan rho` is the allowance; a band that does not is the")
    print("   inward shortfall, which no table size fixes.")


if __name__ == "__main__":
    main()
