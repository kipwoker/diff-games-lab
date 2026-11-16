"""The exact 2D recursion, for the block-8 support `2c` never had.

The engine's substep is a support-function map: `c <- dual_update(c_Q, c + c_P)` with
`c_P = h_P(-G B psi)` and `c_Q = h_Q(G C psi)`, where `G = Ia(h) · exp(A · remaining)` and
`Ia(h) = int_0^h exp(A s) ds`. `Polytope::support` maps the *directions* by the matrix and takes
the support there, so `h_P(G psi) = h_{(G)^T P}(psi)` and the primal recursion carried by those
numbers is

    W <- (W + (-G B)^T P) - (G C)^T Q,

a Minkowski sum and then a Pontryagin difference. The transpose is not cosmetic: with the
untransposed `G` the same recursion reproduces every extent with the two state axes exchanged.
For convex polygons both operations are exact: a Minkowski sum is the hull of the pairwise sums,
and `P - Q = {x : <u,x> <= h_P(u) - h_Q(u)}` over the facet normals `u` of `P`, so an erosion is
`P` clipped by its own shifted half-spaces. Nothing here samples a direction, so the result is
the game's answer rather than the table's.

`2c` is `A = [[0,0],[1,0]]`, `B = C = pi = I`, `M = {|x| <= 0.5}`, `P = {|u| <= 1}`,
`Q = {|v| <= 2}`, `T = 0.4`, `Step = 0.05`. `exp(A t) = [[1,0],[t,1]]` and
`int_0^h exp(A s) ds = [[h,0],[h^2/2,h]]`, so both are written out rather than series-expanded.
`P` and `Q` are centrally symmetric, which is what lets `-G B P` be carried as `G P`.

Parts:

A. The kernel against answers known by hand: sums and erosions of squares, and the support at a
   direction the table does not contain.
B. The eight blocks, against the committed output's extents for every block — the validation the
   `2c` entry asks for before the exact support is asked anything.
C. Direction 27 at block 8: the exact support beside the printed one, which is the shortfall the
   `0.065902` slack was split into a drift and an unknown.
D. Block 8 at `m` directions: the true excess against the one-term allowance and the two-plane
   bound, per cell. Prediction in that part's docstring.
E. Density at blocks 1 and 8 for `m` from 110 to 254: whether both terms track `tan rho`, and what
   `158` would buy. Prediction in that part's docstring.
F. The same recursion from a problem file, for `example-1`, `2a`, `2b` and `2c`: the extents
   against each committed output, then the inner-side support violation per block at `110`
   directions. Prediction in that part's docstring.

Prediction, written before the run: the printed support at direction 27 is short of the exact one
by between `0.005` and `0.03` — the constant part of the slack is real but the smaller half, and
the drift stays the larger one. A shortfall above `0.047695` would falsify its own bound.

    python tools/exact_polygon.py

Requires numpy. Reads `examples/example-2c.txt` and, in part F, every case file; writes the
`--trace` dumps to `engine/target/trace/` and nothing else.
"""

import math
import os
import re
import sys

import numpy as np

STEP = 0.05
BLOCKS = 8
SUBSTEPS = 110
DIRECTIONS = 110
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
OUT = os.path.join(ROOT, "examples", "out", "example-2c.out.txt")
ENGINE = os.path.join(ROOT, "engine", "target", "release", "stable_bridge")
TRACE = re.compile(
    "block ([0-9]+) direction ([0-9]+) inward ([^ ]+) support ([^ ]+) allowance ([^ ]+)"
)


def hull(points):
    """Convex hull of a point set, counter-clockwise, collinear points dropped."""
    pts = np.unique(np.round(np.asarray(points, dtype=float), 12), axis=0)
    if len(pts) < 3:
        return pts
    pts = pts[np.lexsort((pts[:, 1], pts[:, 0]))]

    def chain(seq):
        out = []
        for point in seq:
            while len(out) >= 2:
                a, b = out[-2], out[-1]
                cross = (b[0] - a[0]) * (point[1] - a[1]) - (b[1] - a[1]) * (point[0] - a[0])
                if cross > 1e-14:
                    break
                out.pop()
            out.append(point)
        return out

    lower = chain(pts)
    upper = chain(pts[::-1])
    return np.array(lower[:-1] + upper[:-1], dtype=float)


def square(half):
    """`{|x| <= half}` as a counter-clockwise polygon."""
    return np.array([[half, -half], [half, half], [-half, half], [-half, -half]], dtype=float)


def affine(map_matrix, polygon):
    """The image of a convex polygon under a linear map, which is still its vertex image."""
    return hull(np.asarray(polygon, dtype=float) @ np.asarray(map_matrix, dtype=float).T)


def minkowski(first, second):
    """`{a + b}` for two convex polygons, as the hull of the pairwise sums."""
    a = np.asarray(first, dtype=float)
    b = np.asarray(second, dtype=float)
    return hull((a[:, None, :] + b[None, :, :]).reshape(-1, 2))


def clip(polygon, normal, rhs):
    """Sutherland-Hodgman: keep `normal . x <= rhs` of a convex polygon, in order.

    Vectorised: the exact set grows a facet per substep, so the per-vertex Python loop made the
    recursion quadratic in wall time rather than in work. Entry `2i` keeps vertex `i`, entry
    `2i + 1` is edge `i`'s crossing, which preserves the polygon's order for the next clip.
    """
    points = np.asarray(polygon, dtype=float).reshape(-1, 2)
    count = len(points)
    if count == 0:
        return points
    following = np.roll(points, -1, axis=0)
    values = points @ np.asarray(normal, dtype=float) - rhs
    next_values = following @ np.asarray(normal, dtype=float) - rhs
    inside = values <= 0.0
    crossing = inside != (next_values <= 0.0)
    ratio = np.divide(values, values - next_values, out=np.zeros(count), where=crossing)
    cross_points = points + ratio[:, None] * (following - points)
    entries = np.empty((2 * count, 2))
    entries[0::2] = points
    entries[1::2] = cross_points
    mask = np.empty(2 * count, dtype=bool)
    mask[0::2] = inside
    mask[1::2] = crossing
    return entries[mask]


def erode(polygon, other):
    """`{x : x + other <= polygon}`, i.e. the polygon clipped by its own shifted facets."""
    polygon = np.asarray(polygon, dtype=float)
    if len(polygon) < 3:
        return np.zeros((0, 2))
    points = np.asarray(other, dtype=float)
    result = polygon
    for index in range(len(polygon)):
        a = polygon[index]
        b = polygon[(index + 1) % len(polygon)]
        edge = b - a
        normal = np.array([edge[1], -edge[0]]) / np.linalg.norm(edge)
        height = float((points @ normal).max())
        result = clip(result, normal, float(normal @ a) - height)
        if len(result) < 3:
            return np.zeros((0, 2))
    result = hull(result)
    return simplify(result, 1e-9 * max(1.0, float(np.abs(result).max())))


def simplify(polygon, tol):
    """Drop vertices within `tol` of the chord between their neighbours, then re-hull.

    The exact set gains facets at every substep (each Minkowski sum adds the rotated `P`'s), so at
    `110` substeps it carries hundreds of nearly parallel edges whose vertex count makes the
    clipping quadratic. Dropping the ones that move the boundary by less than `tol` keeps the
    recursion exact to `tol` and linear in the number of real facets.
    """
    points = np.asarray(polygon, dtype=float)
    if len(points) < 3:
        return points
    keep = []
    for index in range(len(points)):
        previous = points[(index - 1) % len(points)]
        here = points[index]
        following = points[(index + 1) % len(points)]
        edge = following - previous
        length = float(np.hypot(*edge))
        if length <= tol:
            continue
        offset = abs(float(edge[0] * (here[1] - previous[1]) - edge[1] * (here[0] - previous[0])))
        if offset / length > tol:
            keep.append(here)
    if len(keep) < 3:
        return hull(points)
    return hull(np.asarray(keep, dtype=float))


def support(polygon, psi):
    return float((np.asarray(polygon, dtype=float) @ np.asarray(psi, dtype=float)).max())


def exp_a(t):
    """`exp(A t)` for `A = [[0,0],[1,0]]`."""
    return np.array([[1.0, 0.0], [t, 1.0]])


def integral_exp_a(h):
    """`int_0^h exp(A s) ds` for the same `A`."""
    return np.array([[h, 0.0], [0.5 * h * h, h]])


def directions():
    angles = np.arange(DIRECTIONS) * 2.0 * math.pi / DIRECTIONS
    return np.stack([np.cos(angles), np.sin(angles)], axis=1)


def printed_blocks(case="example-2c"):
    """The committed output's vertices, one array per block."""
    lines = open(os.path.join(ROOT, "examples", "out", f"{case}.out.txt"), encoding="utf-8").read().splitlines()
    starts = [i for i, line in enumerate(lines) if line.startswith("block ")]
    blocks = []
    for start in starts:
        end = lines.index("separator", start)
        blocks.append(
            np.array([[float(v) for v in line.split()] for line in lines[start + 1 : end]])
        )
    return blocks


P_SET = square(1.0)
Q_SET = square(2.0)


def two_c():
    """The eight blocks, exactly: `W <- (W + G P) - (G Q)` at each of the 110 substeps."""
    h = STEP / SUBSTEPS
    integral = integral_exp_a(h)
    w = square(0.5)
    trace = []
    for block in range(1, BLOCKS + 1):
        for index in range(SUBSTEPS):
            remaining = block * STEP - (index + 1) * h
            # `Polytope::support` maps directions by `G`, so the set it carries is `G^T P`.
            g = (integral @ exp_a(remaining)).T
            w = erode(minkowski(w, affine(g, P_SET)), affine(g, Q_SET))
            if len(w) < 3:
                raise SystemExit(f"exact bridge is empty in block {block}, substep {index}")
        trace.append(w.copy())
    return trace


def part_a():
    print("A. the kernel against answers known by hand")
    one = square(1.0)
    two = square(2.0)
    print(f"   square 1 + square 1        x extent {support(minkowski(one, one), [1, 0]):.9f}  (2)")
    print(f"   square 2 - square 1        x extent {support(erode(two, one), [1, 0]):.9f}  (1)")
    print(f"   square 1 - square 0.5      x extent {support(erode(one, square(0.5)), [1, 0]):.9f}  (0.5)")
    shear = np.array([[1.0, 0.0], [0.3, 1.0]])
    print(f"   shear 0.3 on square 1      y extent {support(affine(shear, one), [0, 1]):.9f}  (1.3)")
    return None


def part_b(trace):
    print()
    print("B. every block against the committed output's extents")
    print(f"   {'block':>6}{'exact x lo':>12}{'exact x hi':>12}{'printed x lo':>13}"
          f"{'printed x hi':>13}{'exact y':>10}{'printed y':>11}")
    printed = printed_blocks()
    worst = 0.0
    for index, (exact, shown) in enumerate(zip(trace, printed), start=1):
        x_low, y_low = exact.min(axis=0)
        x_high, y_high = exact.max(axis=0)
        p_low, p_high = shown.min(axis=0), shown.max(axis=0)
        worst = max(worst, abs(x_high - p_high[0]), abs(p_low[0] - x_low))
        print(f"   {index:6}{x_low:12.6f}{x_high:12.6f}{p_low[0]:13.6f}{p_high[0]:13.6f}"
              f"{y_high:10.6f}{p_high[1]:11.6f}")
    print(f"   largest x-extent disagreement over the eight blocks: {worst:.6f}")
    return worst


def part_c(trace):
    print()
    print("C. direction 27 at block 8, the exact support beside the printed one")
    table = directions()
    psi = table[27]
    print(f"   direction 27 = {math.degrees(math.atan2(psi[1], psi[0])):.4f} deg")
    exact = support(trace[-1], psi)
    shown = printed_blocks()[-1]
    got = support(shown, psi)
    print(f"   exact support        {exact:.6f}")
    print(f"   printed support      {got:.6f}")
    print(f"   printed - exact      {got - exact:+.6f}   (positive is short)")
    print(f"   share of the 0.065902 certificate slack: {100.0 * (exact - got) / 0.065902:+.1f}%")
    print()
    print("   the same comparison over all 110 directions, as a check that 27 is not special:")
    gaps = np.array([support(shown, p) - support(trace[-1], p) for p in table])
    print(f"   printed - exact: min {gaps.min():+.6f}  max {gaps.max():+.6f}"
          f"  at {int(np.argmax(np.abs(gaps)))}")
    return exact, got


def clip_set(dirs, caps, radius):
    """`{x : <psi_i, x> <= caps_i}` as a polygon, clipped from a box of half-width `radius`."""
    poly = square(radius)
    for psi, cap in zip(np.asarray(dirs, dtype=float), np.asarray(caps, dtype=float)):
        poly = clip(poly, psi, float(cap))
        if len(poly) < 3:
            return np.zeros((0, 2))
    return hull(poly)


def pair_extent(a, c_a, b, c_b, psi):
    """Extent of two half-spaces along `psi`, the engine's closed form."""
    ab = float(np.asarray(a) @ np.asarray(b))
    det = 1.0 - ab * ab
    if det <= 1e-14:
        return math.inf
    alpha = (c_a - ab * c_b) / det
    beta = (c_b - ab * c_a) / det
    return float(np.asarray(psi) @ (alpha * np.asarray(a) + beta * np.asarray(b)))


def measures(w, m):
    """Per-probe excess, one-term allowance and two-plane excess bound for one block.

    `a2` is `min over the owning cell's neighbour pairs of pair_extent` **minus** `h_W(psi)`: the
    pair bound is on the support, and the excess is what the allowance has to cover.
    """
    angles = np.arange(m) * 2.0 * math.pi / m
    dirs = np.stack([np.cos(angles), np.sin(angles)], axis=1)
    caps = np.array([support(w, psi) for psi in dirs])
    rho = math.pi / m
    radius = float(caps.max())
    p_hat = clip_set(dirs, caps, radius + 1.0)

    probes = [dirs[i] for i in range(m)]
    probes += [np.array([math.cos(a + math.pi / m), math.sin(a + math.pi / m)]) for a in angles]
    for k in range(len(w)):
        edge = w[(k + 1) % len(w)] - w[k]
        normal = np.array([edge[1], -edge[0]])
        probes.append(normal / np.linalg.norm(normal))
    probes += [
        np.array([math.cos(t), math.sin(t)])
        for t in np.linspace(0.0, 2.0 * math.pi, 4024, endpoint=False)
    ]

    rows = []
    for psi in probes:
        owner = int(np.argmax(dirs @ psi))
        exact_support = support(w, psi)
        excess = support(p_hat, psi) - exact_support
        one = caps[owner] * math.tan(rho) + radius * (1.0 / math.cos(rho) - 1.0)
        left = (owner - 1) % m
        right = (owner + 1) % m
        # Only the pair straddling the probe bounds it: for any other pair the two-plane set is
        # unbounded along `psi`, and `pair_extent` returns that pair's vertex projection, which is
        # a number below the support rather than a bound on it.
        two = pair_extent(dirs[left], caps[left], dirs[right], caps[right], psi) - exact_support
        rows.append((excess, one, two, owner, psi))
    return radius, rho, rows


def part_d(trace, m):
    """
    D. Every block at `m` directions: the excess against both candidate bounds, per cell.

    The table's supports at the samples are exact (`c_i = h_W(psi_i)`), so the reconstruction
    `P^ = {<psi_i, x> <= c_i}` carries the entire interpolation error:

        e  = h_P^(psi) - h_W(psi)              the excess the outer side has to cover
        a1 = c_i tan rho + R (1/cos rho - 1)   the engine's one-term allowance at the owner
        a2 = min over the owner's neighbour pairs of pair_extent(psi) - h_W(psi)

    Prediction, written before the run: `a2` dominates `e` at every probe and is within `1.3x` of it,
    while `a1` runs `1.5-4x`. If `a2` is violated anywhere, the two-plane term is not the tight one
    and the idea is dead in 2D, where it should be easiest.
    """
    print()
    print(f"D. every block at m = {m}: the excess against the one-term and two-plane bounds")
    print("   the worst row per block is chosen by excess over that block's radius; `a1/e` and `a2/e`")
    print("   are the largest ratios over probes with a non-negligible excess (> 1e-6).")
    print(f"   {'block':>6}{'R':>10}{'worst e':>11}{'e/R':>8}{'a1/e':>8}{'a2/e':>8}{'a2 short':>10}")
    worst = None
    for number, w in enumerate(trace, start=1):
        radius, rho, rows = measures(w, m)
        top = max(rows, key=lambda row: row[0] / radius)
        wider = max((row[1] for row in rows), default=float("nan"))
        a1 = top[1] / top[0] if top[0] > 1e-12 else float("nan")
        a2 = top[2] / top[0] if top[0] > 1e-12 else float("nan")
        short = sum(1 for row in rows if row[2] < row[0] - 1e-12)
        print(f"   {number:6}{radius:10.6f}{top[0]:11.6f}{top[0] / radius:8.4f}"
              f"{a1:8.3f}{a2:8.3f}{short:10d}   (max a1 {wider:.6f})")
        if worst is None or top[0] / radius > worst[0]:
            worst = (top[0] / radius, number, top, radius)
    ratio, number, top, radius = worst
    print(f"   widest: block {number}, cell {top[3]}, excess {top[0]:.6f} = "
          f"{100.0 * top[0] / radius:.4f}% of the radius, {100.0 * top[0] / radius:.4f}% of half "
          f"the diameter")
    print(f"   there: a1 {top[1]:.6f} ({top[1] / top[0]:.3f}x), a2 {top[2]:.6f} ({top[2] / top[0]:.3f}x)")
    return worst


def part_e(trace):
    """
    E. Density at blocks 1 and 8: does `tan rho` carry both terms?

    `A = a1 / R` is the allowance's two-sided share of the diameter and `T = e / R` the true excess's
    — both are already shares of the diameter, because the diameter is `2R` and each is doubled.

    Prediction, written before the run: `A / T` stays `1.051` at every `m` (the over-charge is a
    property of the cell, not the density), both scale as `tan rho`, and at `158` `A = 1.475%` with
    `T = 1.403%`. So the outward side meets `2%` at `158` while the printed band, at `2c`'s recorded
    `f = 1.30`, is `2.59%` and does not.
    """
    print()
    print("E. density: the allowance share `A = a1/R` and the true excess share `T = e/R` of the diameter")
    print(f"   {'m':>5}{'block':>6}{'rho deg':>9}{'tan rho %':>11}{'A %':>8}{'T %':>8}{'A/T':>7}"
          f"{'A/tan':>7}{'T/tan':>7}")
    for m in (110, 135, 158, 205, 254):
        rho = math.pi / m
        tangent = 100.0 * math.tan(rho)
        for number, w in ((1, trace[0]), (8, trace[-1])):
            radius, rho_used, rows = measures(w, m)
            top = max(rows, key=lambda row: row[0] / radius)
            a = 100.0 * top[1] / radius
            t = 100.0 * top[0] / radius
            print(f"   {m:5}{number:6}{math.degrees(rho):9.4f}{tangent:11.4f}{a:8.4f}{t:8.4f}"
                  f"{a / t:7.3f}{a / tangent:7.3f}{t / tangent:7.3f}")
    return None


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


def load(case):
    """A case file: `constraints M/P/Q` rows, matrices, `time`, `step`."""
    data = {"constraints": {}, "matrices": {}}
    current = None
    path = os.path.join(ROOT, "examples", f"{case}.txt")
    for raw in open(path, encoding="utf-8"):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        pieces = line.split()
        if pieces[0] == "constraints":
            current = ("constraints", pieces[1])
            data["constraints"][pieces[1]] = []
        elif pieces[0] == "matrix":
            current = ("matrices", pieces[1])
            data["matrices"][pieces[1]] = []
        elif pieces[0] in ("time", "step", "dimension") or pieces[0] == "":
            if pieces[0] != "":
                data[pieces[0]] = float(pieces[-1]) if pieces[0] != "dimension" else int(pieces[-1])
            current = None
        elif current is not None:
            kind, name = current
            values = [float(value) for value in pieces]
            if kind == "constraints":
                data["constraints"][name].append(values)
            else:
                data["matrices"][name].append(values)
    for name, rows in data["matrices"].items():
        data["matrices"][name] = np.asarray(rows, dtype=float)
    return data


def polytope_of(rows):
    """`{x : n . x <= rhs}` from the constraint rows, clipped from a box that contains it."""
    normals = np.asarray([row[:-1] for row in rows], dtype=float)
    caps = np.asarray([row[-1] for row in rows], dtype=float)
    span = 4.0 * float(np.abs(caps).max()) + 1.0
    return clip_set(normals, caps, span)


def expm(a, t, terms=24):
    """`exp(A t)` by its series; `A` is 2 x 2 and `|A t|` is small in these cases."""
    result = np.eye(2)
    term = np.eye(2)
    for order in range(1, terms):
        term = term @ (a * t) / order
        result = result + term
    return result


def integral_expm(a, h, terms=24):
    """`int_0^h exp(A s) ds` by its series."""
    result = np.zeros((2, 2))
    power = np.eye(2)
    for order in range(terms):
        result = result + power * (h ** (order + 1) / math.factorial(order + 1))
        power = power @ a
    return result


def blocks_of(data, substeps=SUBSTEPS):
    """The blocks of a case read from file: `W <- (W + B^T G^T P) - (C^T G^T Q)`.

    `(-G B)^T P` is `-(B^T G^T) P`, which is `(B^T G^T) P` because every case here has a
    centrally symmetric `P`; `(G C)^T Q` is `C^T G^T Q`. `G = pi · Ie · E`.
    """
    matrices = data["matrices"]
    a = matrices["A"]
    b = matrices["B"]
    c = matrices["C"]
    pi = matrices["pi"]
    step = data["step"]
    count = int(round(data["time"] / step))
    h = step / substeps
    p_set = polytope_of(data["constraints"]["P"])
    q_set = polytope_of(data["constraints"]["Q"])
    for vertex in p_set:
        if support_violation(-np.asarray(vertex, dtype=float), p_set) > 1e-9:
            raise SystemExit("P is not centrally symmetric; the sign flip is not absorbed")
    w = polytope_of(data["constraints"]["M"])
    integral = integral_expm(a, h)
    trace = []
    for block in range(1, count + 1):
        for index in range(substeps):
            remaining = block * step - (index + 1) * h
            g = (pi @ integral @ expm(a, remaining)).T
            w = erode(minkowski(w, affine(b.T @ g, p_set)), affine(c.T @ g, q_set))
            if len(w) < 3:
                raise SystemExit(f"exact bridge is empty in block {block}, substep {index}")
        trace.append(w.copy())
    return trace


def dumped(case, count=DIRECTIONS):
    """`(block, direction) -> (charge, support, allowance)` for one case from `--trace`."""
    path = os.path.join(ROOT, "engine", "target", "trace", f"{case}-{count}-clean.txt")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    import subprocess

    subprocess.run(
        [ENGINE, os.path.join(ROOT, "examples", f"{case}.txt"), "--trace", path,
         "--directions", str(count)],
        check=True,
        capture_output=True,
        text=True,
    )
    rows = {}
    for line in open(path, encoding="utf-8"):
        found = TRACE.match(line.strip())
        if found:
            block, direction, charge, support, allowance = found.groups()
            rows[(int(block), int(direction))] = (float(charge), float(support), float(allowance))
    return rows


def part_f():
    """
    F. Every case from its file: extents against the committed output, then the inner side.

    Prediction, written before the run: the extents of all four cases match their committed outputs
to `1e-6` (the 2c case is the control, already validated in part B), and at `110` directions the
inner side has zero support violation at every block of `example-1`, `2a` and `2b`, as it does at
`2c`'s `110`. A nonzero violation would put the `205`-style failure on a case whose band is tighter.
    """
    print()
    print("F. the recursion read from each case file: extents against the committed output, then")
    print("the inner polytope's worst support violation against the exact block polygons.")
    print(f"   {'case':>12}{'blocks':>7}{'extent diff':>13}{'worst viol':>12}{'at':>5}{'R':>10}")
    for case in ("example-1", "example-2a", "example-2b", "example-2c"):
        data = load(case)
        trace = blocks_of(data)
        shown = printed_blocks(case)
        if len(shown) != len(trace):
            print(f"   {case:>12}{len(trace):7}  output has {len(shown)} blocks")
            continue
        diff = max(
            max(
                abs(float(exact[:, 0].min()) - float(print_[:, 0].min())),
                abs(float(exact[:, 0].max()) - float(print_[:, 0].max())),
            )
            for exact, print_ in zip(trace, shown)
        )
        rows = dumped(case)
        dirs = directions()
        worst, where, radius = 0.0, 0, 0.0
        for block, exact in enumerate(trace, start=1):
            caps = np.array(
                [
                    rows[(block, i)][1] - rows[(block, i)][2] - rows[(block, i)][0]
                    for i in range(len(dirs))
                ]
            )
            inner = clip_set(dirs, caps, float(caps.max()) + 1.0)
            if len(inner) < 3:
                print(f"   {case:>12}{len(trace):7}  inner empty at block {block}")
                worst, where = math.inf, block
                break
            violation = max(support_violation(vertex, exact) for vertex in inner)
            if violation > worst:
                worst, where = violation, block
                radius = float(np.hypot(inner[:, 0], inner[:, 1]).max())
        print(f"   {case:>12}{len(trace):7}{diff:13.3e}{worst:12.6f}{where:5}{radius:10.6f}")
    return None


def main():
    part_a()
    trace = two_c()
    part_b(trace)
    part_c(trace)
    part_d(trace, 110)
    part_e(trace)
    part_f()
    return 0


if __name__ == "__main__":
    sys.exit(main())
