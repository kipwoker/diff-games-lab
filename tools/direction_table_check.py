"""What the 110 directions alone do to a known set.

Reconstructs the polyhedron {x : <psi, x> <= h(psi) for psi in the table} for sets whose
support function is known exactly, and reports its extents. This is the check behind
kb/entries/06-direction-table-explains-extents.md and kb/entries/01-example1-deficit.md:
it needs no solver run, only the table.

    python tools/direction_table_check.py

Requires numpy. Lives in tools/ because the engine itself stays dependency-free.
"""

import itertools
import math
import os
import sys

import numpy as np

DIRECTIONS = 110
OUT = os.path.join("examples", "out")


def directions_2d():
    return [
        np.array([math.cos(i * 2 * math.pi / DIRECTIONS), math.sin(i * 2 * math.pi / DIRECTIONS)])
        for i in range(DIRECTIONS)
    ]


def directions_3d():
    """The engine's table: 11 azimuths over 180 degrees, 10 elevations over 360."""
    out = []
    for i in range(11):
        azimuth = i * math.pi / 10 - math.pi / 2
        for j in range(10):
            elevation = j * 2 * math.pi / 10
            out.append(
                np.array(
                    [
                        math.cos(azimuth) * math.cos(elevation),
                        math.cos(elevation) * math.sin(azimuth),
                        math.sin(elevation),
                    ]
                )
            )
    return out


def extents(directions, support, slack=0.0):
    """Max coordinate over the intersection of the half-spaces, by vertex enumeration."""
    table = np.array(directions)
    rhs = np.array([support(p) for p in directions]) + slack
    dim = table.shape[1]
    best = np.full(dim, -np.inf)
    for indices in itertools.combinations(range(len(table)), dim):
        matrix = table[list(indices)]
        try:
            point = np.linalg.solve(matrix, rhs[list(indices)])
        except np.linalg.LinAlgError:
            continue
        if np.all(table @ point <= rhs + 1e-9):
            best = np.maximum(best, point)
    return best


def tracked_extents(name):
    """Extents of the last block of a tracked output, or None if it is not present."""
    path = os.path.join(OUT, name)
    if not os.path.exists(path):
        return None
    lines = open(path, encoding="utf-8").read().splitlines()
    starts = [i for i, line in enumerate(lines) if line.startswith("block ")]
    if not starts:
        return None
    begin = starts[-1]
    end = lines.index("separator", begin)
    points = np.array([[float(v) for v in line.split()] for line in lines[begin + 1 : end]])
    return points.min(axis=0), points.max(axis=0), len(points)


def main():
    cube = lambda half: (lambda p: half * (abs(p[0]) + abs(p[1]) + abs(p[2])))
    square = lambda half: (lambda p: half * (abs(p[0]) + abs(p[1])))

    print("3D cube, half-width 1.5 (the example-3d shell at t = 1)")
    for slack in (0.0, 0.01):
        print(f"  slack {slack:<5}: extents {extents(directions_3d(), cube(1.5), slack).round(4)}")
    print("  tracked     : [1.5    1.5    1.9874]  (examples/out/example-3d.out.txt, block 4)")
    print("  ratio       : %.4f  (documented as 1.325)" % (1.9874 / 1.5))

    print()
    print("2D square, half-width 0.5 (the example-2b shell, equal authority)")
    for slack in (0.0, 0.01):
        print(f"  slack {slack:<5}: extents {extents(directions_2d(), square(0.5), slack).round(6)}")
    print("  tracked     : [0.5      0.5143  ]  (examples/out/example-2b.out.txt, block 8)")

    half_gap = 180.0 / DIRECTIONS
    print()
    print(f"2D angular half-gap {half_gap:.4f} deg -> circumscribed factor {1 / math.cos(math.radians(half_gap)):.6f}")
    print("3D gaps: azimuth 18 deg, elevation 36 deg")

    print()
    print("example-1 symmetry probe: target diamond r = 1, controls r = 2 and r = 1, A = 0")
    print("  exact bridge at T = 2 is the diamond 1 + 2*2 - 2*1 = 3, so both extents are 3.000")
    got = tracked_extents("example-1.out.txt")
    if got is None:
        print("  tracked file not found; skip")
    else:
        low, high, count = got
        print(f"  tracked block: x {low[0]:+.3f}..{high[0]:+.3f}  y {low[1]:+.3f}..{high[1]:+.3f}  ({count} vertices)")
        deficit = 3.0 - high[1]
        print(f"  y deficit   : {deficit:.3f}  ({100 * deficit / 3.0:.1f}% of the analytic half-width)")
        print("  the set is symmetric, so the deficit is the sampler's, not the game's")


if __name__ == "__main__":
    sys.exit(main())
