"""Whether the inner polytope stays inside the exact body between the samples.

The bracket's inner side is `c - a - s` at each sampled direction, and the inner polytope is the
intersection of those shifted half-spaces. Step 1 of `06-band-direction-is-a-short-direction.md`
tested the inequality at the 110 samples; the set is defined by them, so the only way it can
leave the exact body is at a vertex that sits between two samples. Those vertices are checked
here against the exact `2c` polygons.

    python tools/inner_between_samples.py

Prediction, written before the run: every inner vertex is inside the exact body, so the worst
outside distance is zero. Between samples the inner polytope's support falls along chords while
the exact body's does not, which makes the sampled directions the worst places to check — if the
prediction fails, the failure is in the caps and not in the sampling.

The first run of this tool reported "every vertex outside" because its inside test was inverted:
`outside_distance` returned the euclidean distance to the nearest edge for interior points too
(the centroid of the exact block-1 polygon read `0.448750`). `support_violation` replaces it — the
largest facet-normal excess over the polygon's outward normals, which is zero exactly when a point
is inside.

Part 2 asks whether the gap is a table term that can repair the inner side: if the inner polytope
leaves the body because its caps are chamfered by the table's own spacing, the distance should
scale with `tan rho` — `gap / tan rho` constant across counts — and a cap shrink of `k tan rho`
fitted at `110` should leave the `254` run's vertices inside. Prediction, written before the run:
`gap / tan rho` is constant within `15%` at blocks 1 and 8, and the `110` fit covers `254` with a
zero residual outside distance.

Requires the release binary and numpy.
"""

import math
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


COUNTS = (110, 158, 205, 254)


def table(count):
    angles = np.arange(count) * 2.0 * math.pi / count
    return np.stack([np.cos(angles), np.sin(angles)], axis=1)


def dump(count=110):
    """`(block, direction) -> (charge, support, allowance)` from the recursion's dump."""
    path = os.path.join("engine", "target", "trace", f"2c-inner-{count}.txt")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    subprocess.run(
        [ENGINE, "examples/example-2c.txt", "--trace", path, "--directions", str(count)],
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


def support_violation(point, polygon):
    """Largest `max_u(<u, point> - h_P(u))` over the polygon's outward facet normals.

    Independent of vertex order: each normal is flipped to point away from the centroid. It equals
    the distance to the nearest supporting line and is at least the euclidean distance, so a zero
    here is a sound "inside".
    """
    points = np.asarray(polygon, dtype=float)
    if len(points) < 3:
        return math.inf
    center = points.mean(axis=0)
    worst = 0.0
    for index in range(len(points)):
        start = points[index]
        edge = points[(index + 1) % len(points)] - start
        normal = np.array([edge[1], -edge[0]])
        length = float(np.hypot(*normal))
        if length <= 0.0:
            continue
        normal = normal / length
        if float(normal @ (start - center)) < 0.0:
            normal = -normal
        worst = max(worst, float(normal @ point) - float(normal @ start))
    return worst


def explain_205(counts=COUNTS, block=1):
    """Where the one outside vertex sits: at a sample, or between two.

    Prediction, written before the run: at `205` the outside vertex sits at a between-sample
    normal (`offset` near `0.5`) with `excess` equal to `bulge`, the amount by which the exact
    body outgrows the shifted caps at that normal — i.e. the charge and allowance are short at a
    chamfer, not inconsistent at a sample. `offset` near `0` instead would mean the sampled data
    disagree at a direction the set is defined by, which no chamfer argument repairs.
    """
    print()
    print("the worst inner vertex of block 1 against the exact polygon, and the offset of its")
    print("violated normal between the straddling samples (`0` at a sample, `0.5` mid-cell)")
    print(
        f"   {'m':>5}{'vertices':>9}{'out':>5}{'excess':>10}{'offset':>8}{'h':>10}"
        f"{'cap':>10}{'allow':>9}{'charge':>9}{'bulge':>10}"
    )
    polygons = ex.two_c()
    for count in counts:
        dirs = table(count)
        rows = dump(count)
        caps = np.array(
            [rows[(block, i)][1] - rows[(block, i)][2] - rows[(block, i)][0] for i in range(count)]
        )
        allowance = np.array([rows[(block, i)][2] for i in range(count)])
        charge = np.array([rows[(block, i)][0] for i in range(count)])
        support = np.array([rows[(block, i)][1] for i in range(count)])
        inner = ex.clip_set(dirs, caps, float(caps.max()) + 1.0)
        points = np.asarray(polygons[block - 1], dtype=float)
        center = points.mean(axis=0)
        excess, vertex, normal, height = -math.inf, None, None, 0.0
        for index in range(len(points)):
            start = points[index]
            edge = points[(index + 1) % len(points)] - start
            candidate = np.array([edge[1], -edge[0]])
            length = float(np.hypot(*candidate))
            if length <= 0.0:
                continue
            candidate = candidate / length
            if float(candidate @ (start - center)) < 0.0:
                candidate = -candidate
            reach = float(candidate @ start)
            for point in inner:
                gap = float(candidate @ point) - reach
                if gap > excess:
                    excess, vertex, normal, height = gap, point, candidate, reach
        outside = sum(1 for point in inner if support_violation(point, points) > 1e-12)
        phi = math.atan2(normal[1], normal[0]) % (2.0 * math.pi)
        slot = phi * count / (2.0 * math.pi)
        offset = abs(slot - round(slot))
        left = int(math.floor(slot)) % count
        right = (left + 1) % count
        bulge = max(height - caps[left], height - caps[right])
        print(
            f"   {count:5}{len(inner):9}{outside:5}{excess:10.6f}{offset:8.3f}{height:10.6f}"
            f"{max(support[left], support[right]):10.6f}"
            f"{max(allowance[left], allowance[right]):9.6f}"
            f"{max(charge[left], charge[right]):9.6f}{bulge:10.6f}"
        )


def main():
    table = ex.directions()
    count = len(table)
    dumped = dump()
    exact = ex.two_c()
    print("the inner polytope of each block, its vertices, and its support violation against the")
    print("exact polygon (zero means inside). Prediction: zero at every block.")
    print(f"   {'block':>6}{'vertices':>10}{'violating':>10}{'worst viol':>12}{'inner R':>10}")
    worst_all = 0.0
    for block in range(1, len(exact) + 1):
        caps = np.array(
            [
                dumped[(block, i)][1] - dumped[(block, i)][2] - dumped[(block, i)][0]
                for i in range(count)
            ]
        )
        inner = ex.clip_set(table, caps, float(caps.max()) + 1.0)
        if len(inner) < 3:
            print(f"   {block:6}{'empty':>10}")
            continue
        distances = [support_violation(vertex, exact[block - 1]) for vertex in inner]
        outside = sum(1 for distance in distances if distance > 1e-12)
        worst = max(distances)
        worst_all = max(worst_all, worst)
        radius = float(np.hypot(inner[:, 0], inner[:, 1]).max())
        print(
            f"   {block:6}{len(inner):10}{outside:10}{worst:12.6f}{radius:10.6f}"
        )
    print()
    print(f"   worst support violation over every inner vertex: {worst_all:.6f}")
    density()
    explain_205()
    return 0


def density():
    """The gap against the table spacing, and whether a `tan rho` shrink repairs it."""
    exact = ex.two_c()
    print()
    print("the same gap at four table sizes: `gap / tan rho` constant means the gap is the table's")
    print("chamfer; `repair` shrinks every cap by that block's own gap, `fit` by `k tan rho` from 110.")
    print(
        f"   {'m':>5}{'block':>6}{'rho deg':>9}{'tan %':>8}{'vertices':>9}{'outside':>8}"
        f"{'gap':>10}{'viol':>10}{'gap/tan':>9}{'repair':>10}{'R':>9}"
    )
    fitted = {}
    for count in COUNTS:
        rows = dump(count)
        dirs = table(count)
        tangent = math.tan(math.pi / count)
        for block in (1, 8):
            caps = np.array(
                [
                    rows[(block, i)][1] - rows[(block, i)][2] - rows[(block, i)][0]
                    for i in range(count)
                ]
            )
            inner = ex.clip_set(dirs, caps, float(caps.max()) + 1.0)
            distances = [outside_distance(vertex, exact[block - 1]) for vertex in inner]
            worst = max(distances)
            violation = max(support_violation(vertex, exact[block - 1]) for vertex in inner)
            outside = sum(1 for distance in distances if distance > 0.0)
            radius = float(np.hypot(inner[:, 0], inner[:, 1]).max())
            shifted = caps - worst
            repaired = ex.clip_set(dirs, shifted, float(shifted.max()) + 1.0)
            residual = (
                max(support_violation(vertex, exact[block - 1]) for vertex in repaired)
                if len(repaired) >= 3
                else float("inf")
            )
            if count == 110:
                fitted[block] = worst / tangent
            print(
                f"   {count:5}{block:6}{math.degrees(math.pi / count):9.4f}"
                f"{100.0 * tangent:8.4f}{len(inner):9}{outside:8}{worst:10.6f}"
                f"{violation:10.6f}{worst / tangent:9.4f}{residual:10.6f}{radius:9.6f}"
            )
    print()
    print("cross-density: shrink the 254 caps by the `k tan rho` fitted at 110")
    rows = dump(254)
    dirs = table(254)
    tangent = math.tan(math.pi / 254)
    for block in (1, 8):
        caps = np.array(
            [
                rows[(block, i)][1] - rows[(block, i)][2] - rows[(block, i)][0]
                for i in range(254)
            ]
        )
        inner = ex.clip_set(dirs, caps, float(caps.max()) + 1.0)
        before = max(support_violation(vertex, exact[block - 1]) for vertex in inner)
        shifted = caps - fitted[block] * tangent
        repaired = ex.clip_set(dirs, shifted, float(shifted.max()) + 1.0)
        after = (
            max(support_violation(vertex, exact[block - 1]) for vertex in repaired)
            if len(repaired) >= 3
            else float("inf")
        )
        print(
            f"   block {block}: k {fitted[block]:.4f}, shift {fitted[block] * tangent:.6f},"
            f" outside {before:.6f} -> {after:.6f}"
        )


if __name__ == "__main__":
    sys.exit(main())
