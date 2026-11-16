"""The covering-radius bound on the sampled reconstruction.

The table `D` reconstructs a convex set from its support values as

    P = {x : <psi_i, x> <= h(psi_i), i = 1..m}.

`S` is always inside `P`, so what is left is the excess in directions the table misses. Write
`rho` for the table's covering radius — the largest angle from any unit direction to the
nearest table direction — and `h`, `R` for the support and circumradius of `S`. The bound
tested here is

    h_P(psi) <= h(psi) + h(psi) * tan(rho) + R * (1 / cos(rho) - 1).

The first term is the flat-facet drift: a facet normal `rho` away from `psi` lets the
intersection slide by `h tan rho`. The second is the curvature term: on a smooth body the
excess goes as `R(1/cos rho - 1)`. The bound is outer and need not be tight; it is useful only
if it never sits below a measured extent. Parts D and E test the first term where no facet
normal is in the table at all — a rotated parallelogram in 2D, an octahedron in 3D — and part E
also measures `rho` against the size of the 3D table.

Part K asks the question those parts leave open: how far below the measurement is the bound
actually? It reconstructs the sampled polyhedron exactly, sweeps directions that include every
facet normal of the body, and prints the ratio of the measured excess to the allowance at the
cell that owns each probe — the quantity that is 35% of the support in 3D and therefore the
whole of P1's gate. Prediction, written before the run: the 2D control reproduces the known
result (a box tight at 100%, a diamond loose at 0%), the octahedron's worst ratio is at most
0.5, and the cube — whose `z` facet normal is 18 degrees from the nearest sample — is the tight
case at 0.6 to 1.0, so the 3D allowance is loose by a factor between 1.5 and 3 rather than by
the 30 that `rho^2/2` would suggest.


    python tools/covering_bound.py

Requires numpy. Part B needs the engine binary and writes only under engine/target/.
"""

import itertools
import math
import os
import subprocess
import sys

import numpy as np

from direction_table_check import directions_2d, directions_3d

ENGINE = os.path.join("engine", "target", "release", "stable_bridge")
SCRATCH = os.path.join("engine", "target", "trace")


def covering_radius(dirs, samples=200000, seed=1):
    """Largest angle from a sampled direction to its nearest table direction."""
    table = np.asarray(dirs, dtype=float)
    table /= np.linalg.norm(table, axis=1)[:, None]
    generator = np.random.default_rng(seed)
    points = generator.normal(size=(samples, table.shape[1]))
    points /= np.linalg.norm(points, axis=1)[:, None]
    worst = 0.0
    for start in range(0, samples, 5000):
        block = points[start : start + 5000]
        nearest = (block @ table.T).max(axis=1)
        worst = max(worst, float(np.arccos(np.clip(nearest.min(), -1.0, 1.0))))
    return worst


def bound(h, radius, rho):
    return h * math.tan(rho) + radius * (1.0 / math.cos(rho) - 1.0)


def tracked_cases(rho_2d, rho_3d):
    print(f"rho 2D = {math.degrees(rho_2d):.4f} deg (analytic half-gap {math.degrees(math.pi / 110):.4f})")
    print(f"rho 3D = {math.degrees(rho_3d):.4f} deg")
    print()
    print("A. reconstruction from exact supports, against the tracked bulges")
    print(f"   {'case':<30}{'h':>6}{'R':>7}{'excess':>9}{'bound':>9}{'tracked':>10}{'slack':>8}")
    cases = [
        ("3D cube 1.5, z missed", 1.5, 1.5 * math.sqrt(3.0), rho_3d, 1.9874),
        ("2D square 0.5, y missed", 0.5, 0.5 * math.sqrt(2.0), rho_2d, 0.514284),
        ("2D diamond 3.0, y missed", 3.0, 3.0 * math.sqrt(2.0), rho_2d, 3.0),
    ]
    for name, h, radius, rho, tracked in cases:
        excess = bound(h, radius, rho)
        slack = 100.0 * (h + excess - tracked) / tracked
        print(f"   {name:<30}{h:6.2f}{radius:7.3f}{excess:9.4f}{h + excess:9.4f}{tracked:10.4f}{slack:7.1f}%")
    print()
    print("   'excess' is the bound on h_P - h; 'slack' is how far the bound sits above the")
    print("   measured extent, so a negative slack is a falsification.")


def write_problem(name, target, first, second, square):
    """A L1 diamond or box case with A = 0, T = 2, Step = 0.1, exact radius target + 2*(first-second)."""
    if square:
        rows = lambda a: [[1, 0, a], [-1, 0, a], [0, 1, a], [0, -1, a]]
    else:
        rows = lambda a: [[1, 1, a], [-1, 1, a], [1, -1, a], [-1, -1, a]]
    block = lambda a: "\n".join("  ".join(f"{v:g}" for v in row) for row in rows(a))
    text = f"""dimension 2

constraints M
{block(target)}

matrix A
  0 0
  0 0

matrix B
  1 0
  0 1

matrix C
  -1  0
   0 -1

matrix pi
  1 0
  0 1

constraints P
{block(first)}

constraints Q
{block(second)}

time 2
step 0.1
"""
    path = os.path.join(SCRATCH, name + ".txt")
    open(path, "w", encoding="utf-8").write(text)
    return path


def run(path):
    out = path.replace(".txt", ".out.txt")
    binary = ENGINE if os.path.exists(ENGINE) else ENGINE + ".exe"
    subprocess.run([binary, path, out], check=True, capture_output=True)
    lines = open(out, encoding="utf-8").read().splitlines()
    begin = [i for i, line in enumerate(lines) if line.startswith("block ")][-1]
    end = lines.index("separator", begin)
    points = np.array([[float(v) for v in line.split()] for line in lines[begin + 1 : end]])
    return points[:, 0].max(), points[:, 1].max()


def blind_cases(rho_2d):
    print()
    print("B. blind: the bound is written down before the run, on cases not in examples/")
    os.makedirs(SCRATCH, exist_ok=True)
    target, first, second = 1.0, 3.0, 1.0
    exact = target + 2.0 * (first - second)  # 5.0
    for square, name in ((True, "blind-square"), (False, "blind-diamond")):
        radius = exact * math.sqrt(3.0 if False else 2.0)
        predicted = exact + bound(exact, radius, rho_2d)
        got_x, got_y = run(write_problem(name, target, first, second, square))
        verdict = "bound holds" if got_y <= predicted else "BOUND FALSIFIED"
        print(
            f"   {name:<14} exact {exact:.4f}  predicted y <= {predicted:.4f}"
            f"   measured x {got_x:.4f} y {got_y:.4f}   {verdict}"
        )


def max_coordinate(dirs, support, axis=1, eps=1e-9):
    """Largest `axis` coordinate of {x : <psi_i, x> <= support_i}, no acceptance slack."""
    count = len(dirs)
    rows, cols = np.triu_indices(count, k=1)
    coef = np.stack([dirs[rows], dirs[cols]], axis=1)
    rhs = np.stack([support[rows], support[cols]], axis=1)
    det = coef[:, 0, 0] * coef[:, 1, 1] - coef[:, 0, 1] * coef[:, 1, 0]
    good = np.abs(det) > 1e-12
    coef, rhs, det = coef[good], rhs[good], det[good]
    point = np.empty((len(det), 2))
    point[:, 0] = (rhs[:, 0] * coef[:, 1, 1] - coef[:, 0, 1] * rhs[:, 1]) / det
    point[:, 1] = (coef[:, 0, 0] * rhs[:, 1] - rhs[:, 0] * coef[:, 1, 0]) / det
    best = -np.inf
    for start in range(0, len(point), 20000):
        block = point[start : start + 20000]
        ok = ((block @ dirs.T - support[None, :]) <= eps).all(axis=1)
        if ok.any():
            best = max(best, block[ok, axis].max())
    return best


def m_sweep():
    print()
    print("C. exact supports, m swept, y axis never sampled (m not divisible by 4)")
    print(f"   {'m':>5}{'rho deg':>9}{'square':>9}{'bound':>9}{'over':>7}   {'diamond':>9}{'bound':>9}{'over':>7}")
    a = 1.0
    for m in (110, 154, 198, 242, 286, 330, 374):
        dirs = table_2d(m)
        rho = math.pi / m  # half-gap of an evenly spaced 2D table
        square = max_coordinate(dirs, a * np.abs(dirs).sum(axis=1))
        diamond = max_coordinate(dirs, a * np.abs(dirs).max(axis=1))
        radius = a * math.sqrt(2.0)
        b_square = a + bound(a, radius, rho)
        b_diamond = a + bound(a, radius, rho)
        print(
            f"   {m:5}{math.degrees(rho):9.4f}{square:9.4f}{b_square:9.4f}"
            f"{100 * (b_square - square) / square:6.1f}%"
            f"   {diamond:9.4f}{b_diamond:9.4f}{100 * (b_diamond - diamond) / diamond:6.1f}%"
        )


def table_2d(m):
    angles = np.arange(m) * (2.0 * math.pi) / m
    return np.stack([np.cos(angles), np.sin(angles)], axis=1)


def table_3d(azimuths, elevations):
    """The engine's 3D table, for a given pair of counts."""
    out = []
    for i in range(azimuths):
        azimuth = i * math.pi / (azimuths - 1) - math.pi / 2
        for j in range(elevations):
            elevation = j * 2.0 * math.pi / elevations
            out.append(
                [
                    math.cos(azimuth) * math.cos(elevation),
                    math.cos(elevation) * math.sin(azimuth),
                    math.sin(elevation),
                ]
            )
    return np.asarray(out, dtype=float)


def extent_along_3d(dirs, support, psi, chunk=100000):
    """`max <psi, x>` over `{x : <psi_i, x> <= support_i}`, by enumerating 3-subsets."""
    count = len(dirs)
    best = -np.inf
    pairs = list(itertools.combinations(range(count), 3))
    for start in range(0, len(pairs), chunk):
        block = np.asarray(pairs[start : start + chunk])
        coef = dirs[block]
        rhs = support[block]
        det = np.linalg.det(coef)
        keep = np.abs(det) > 1e-12
        if not keep.any():
            continue
        points = np.linalg.solve(coef[keep], rhs[keep][..., None])[..., 0]
        ok = ((points @ dirs.T - support[None, :]) <= 1e-9).all(axis=1)
        if ok.any():
            best = max(best, float((points[ok] @ psi).max()))
    return best


def part_d(phi_degrees):
    print()
    print("D. the first term on a rotated parallelogram: no facet normal is in the table")
    print(f"   {'face angle':>11}{'rho deg':>9}{'h':>7}{'h_P':>9}{'excess':>9}{'h tan rho':>11}{'bound':>9}")
    half = 3.0
    dirs = table_2d(110)
    step = 2.0 * math.pi / 110
    for phi_degrees in phi_degrees:
        phi = math.radians(phi_degrees)
        theta = np.arctan2(dirs[:, 1], dirs[:, 0]) - phi
        h = half * (np.abs(np.cos(theta)) + np.abs(np.sin(theta)))
        psi = np.array([math.cos(phi), math.sin(phi)])
        tangent = np.array([-psi[1], psi[0]])
        rotated = np.stack([dirs @ psi, dirs @ tangent], axis=1)
        measured = max_coordinate(rotated, h, axis=0)
        rho = abs(phi - round(phi / step) * step)
        first = half * math.tan(rho)
        print(
            f"   {phi_degrees:11.4f}{math.degrees(rho):9.4f}{half:7.3f}{measured:9.4f}"
            f"{measured - half:9.4f}{first:11.4f}{half + bound(half, half * math.sqrt(2.0), rho):9.4f}"
        )


def part_e():
    print()
    print("E. 3D: rho against table size, then the first term on an octahedron")
    print(f"   {'azimuth x elev':>16}{'m':>6}{'rho deg':>9}")
    for azimuths, elevations in ((6, 5), (8, 8), (11, 10), (12, 15), (16, 15), (20, 20)):
        table = table_3d(azimuths, elevations)
        rho = covering_radius(table, samples=100000)
        print(f"   {f'{azimuths} x {elevations}':>16}{len(table):6}{math.degrees(rho):9.4f}")

    print()
    print("   octahedron {|x|+|y|+|z| <= 1}: its facet normals are (1,1,1)/sqrt(3), unsampled")
    print(f"   {'table':>8}{'rho deg':>9}{'h':>8}{'h_P':>9}{'excess':>9}{'h tan rho':>11}{'bound':>9}")
    psi = np.array([1.0, 1.0, 1.0]) / math.sqrt(3.0)
    for azimuths, elevations in ((11, 10), (12, 15)):
        dirs = table_3d(azimuths, elevations)
        h = np.abs(dirs).max(axis=1)
        measured = extent_along_3d(dirs, h, psi)
        rho = math.acos(min(1.0, float((dirs @ psi).max())))
        exact = 1.0 / math.sqrt(3.0)
        first = exact * math.tan(rho)
        print(
            f"   {f'{azimuths} x {elevations}':>8}{math.degrees(rho):9.4f}{exact:8.4f}{measured:9.4f}"
            f"{measured - exact:9.4f}{first:11.4f}{exact + bound(exact, 1.0, rho):9.4f}"
        )


def part_f():
    print()
    print("F. the two-neighbour bound against a full turn of the same parallelogram")
    print("   t = (c_a sin rho_b + c_b sin rho_a) / sin(rho_a + rho_b), a and b the nearest samples")
    print("   on either side of the facet normal; the one-term bound uses the nearer of the two.")
    half = 3.0
    radius = half * math.sqrt(2.0)
    dirs = table_2d(110)
    step = 2.0 * math.pi / 110
    angles = np.arange(110) * step
    worst_pair = -np.inf
    worst_one = -np.inf
    row = None
    for phi_degrees in np.arange(0.0, 360.0, 1.0):
        phi = math.radians(phi_degrees)
        theta = np.arctan2(dirs[:, 1], dirs[:, 0]) - phi
        h = half * (np.abs(np.cos(theta)) + np.abs(np.sin(theta)))
        psi = np.array([math.cos(phi), math.sin(phi)])
        tangent = np.array([-psi[1], psi[0]])
        rotated = np.stack([dirs @ psi, dirs @ tangent], axis=1)
        measured = max_coordinate(rotated, h, axis=0)
        rel = ((angles - phi + math.pi) % (2.0 * math.pi)) - math.pi
        a = int(np.argmin(np.where(rel >= 0.0, rel, np.inf)))
        b = int(np.argmax(np.where(rel < 0.0, rel, -np.inf)))
        rho_a, rho_b = float(rel[a]), float(-rel[b])
        pair = (h[a] * math.sin(rho_b) + h[b] * math.sin(rho_a)) / math.sin(rho_a + rho_b)
        nearest = min(rho_a, rho_b)
        one = half + bound(half, radius, nearest)
        worst_pair = max(worst_pair, measured - pair)
        worst_one = max(worst_one, measured - one)
        if abs(phi_degrees - 17.0) < 1e-9:
            row = (math.degrees(rho_a), math.degrees(rho_b), measured, pair, one)
    if row:
        print(f"   face 17.0 deg: rho {row[0]:.4f}/{row[1]:.4f}  measured {row[2]:.4f}"
              f"  pair {row[3]:.4f}  one term {row[4]:.4f}")
    print(f"   worst over 360 orientations: measured - pair {worst_pair:+.6f},"
          f" measured - one term {worst_one:+.6f}")
    print("   a positive second figure is the one-term bound being false.")


def pair_extent(a, c_a, b, c_b, psi):
    """Extent of {<a,x> <= c_a, <b,x> <= c_b} along psi, in any dimension."""
    ab = float(a @ b)
    det = 1.0 - ab * ab
    if det <= 1e-14:
        return math.inf
    alpha = (c_a - ab * c_b) / det
    beta = (c_b - ab * c_a) / det
    return float(psi @ (alpha * a + beta * b))


def part_g():
    print()
    print("G. 3D: the same solve over the cell's nearest three samples")
    print("   predicted before measuring: the three-sample vertex >= measured, and the 2D pair")
    print("   form applied to each pair of them is *below* it, so the pair form is 2D only")
    print(f"   {'body':>11}{'table':>7}{'rho deg':>9}{'triple':>9}{'measured':>10}{'pair 2D':>9}{'one term':>10}   verdict")
    octa = np.array([1.0, 1.0, 1.0]) / math.sqrt(3.0)
    cube = np.array([1.0, 1.0, 0.3])
    cube = cube / np.linalg.norm(cube)
    bodies = (
        ("octahedron", lambda dirs: np.abs(dirs).max(axis=1), octa, 1.0 / math.sqrt(3.0), 1.0),
        ("cube 1.5", lambda dirs: 1.5 * np.abs(dirs).sum(axis=1), cube, 1.5 * float(np.abs(cube).sum()),
         1.5 * math.sqrt(3.0)),
    )
    for name, support, psi, h_true, radius in bodies:
        for azimuths, elevations in ((11, 10), (12, 15)):
            dirs = table_3d(azimuths, elevations)
            h = support(dirs)
            samples = np.argsort(-(dirs @ psi))[:3]
            vertex = np.linalg.solve(dirs[samples], h[samples])
            triple = float(psi @ vertex)
            pair = min(
                pair_extent(dirs[i], h[i], dirs[j], h[j], psi)
                for i, j in itertools.combinations(samples, 2)
            )
            measured = extent_along_3d(dirs, h, psi)
            rho = math.acos(min(1.0, float((dirs @ psi).max())))
            one = h_true + bound(h_true, radius, rho)
            verdict = "triple holds" if triple >= measured - 1e-9 else "TRIPLE FALSIFIED"
            print(
                f"   {name:>11}{f'{azimuths}x{elevations}':>7}{math.degrees(rho):9.4f}"
                f"{triple:9.4f}{measured:10.4f}{pair:9.4f}{one:10.4f}   {verdict}"
            )
    print("   a triple below the measurement is not a loose bound: with three planes the set is")
    print("   still unbounded out of their span, so the vertex is no bound at all. The pair form")
    print("   is exact in 2D only; 3D needs the whole reconstruction or the one-term form.")


def adaptive_2d(count):
    """Bisect the widest angular gap until `count` directions exist."""
    angles = [0.0, math.pi]
    while len(angles) < count:
        order = sorted(angles)
        limits = order + [order[0] + 2.0 * math.pi]
        widest, where = -1.0, 0
        for i in range(len(order)):
            gap = limits[i + 1] - limits[i]
            if gap > widest:
                widest, where = gap, i
        angles.append((limits[where] + 0.5 * widest) % (2.0 * math.pi))
    return np.asarray([[math.cos(a), math.sin(a)] for a in sorted(angles)], dtype=float)


def worst_excess(dirs, half, facet_deg, rotations=90):
    """Largest (reconstruction - exact) over a turn, for a square of half-width `half`."""
    theta = np.arctan2(dirs[:, 1], dirs[:, 0]) - math.radians(facet_deg)
    h = half * (np.abs(np.cos(theta)) + np.abs(np.sin(theta)))
    worst = 0.0
    for offset in np.arange(0.0, rotations, 1.0):
        phi = math.radians(facet_deg + offset)
        psi = np.array([math.cos(phi), math.sin(phi)])
        tangent = np.array([-psi[1], psi[0]])
        rotated = np.stack([dirs @ psi, dirs @ tangent], axis=1)
        measured = max_coordinate(rotated, h, axis=0)
        exact = half * (abs(math.cos(math.radians(offset))) + abs(math.sin(math.radians(offset))))
        worst = max(worst, measured - exact)
    return worst


def part_h():
    print()
    print("H. adaptive against uniform at matched direction count, 2D, exact supports")
    print("   worst (reconstruction - exact) over a 90-degree turn of the facet normal")
    print(f"   {'body':>18}{'m':>6}{'uniform':>11}{'adaptive':>11}{'ratio':>9}")
    for half, facet, label in ((0.5, 0.0, "square 0.5"), (3.0, 17.0, "parallelogram 3")):
        for m in (110, 220):
            uniform = worst_excess(table_2d(m), half, facet)
            adaptive = worst_excess(adaptive_2d(m), half, facet)
            ratio = uniform / adaptive if adaptive > 1e-12 else math.inf
            print(
                f"   {label:>18}{m:6}{uniform:11.5f}{adaptive:11.5f}{ratio:9.1f}"
            )


def unit(vectors):
    vectors = np.asarray(vectors, dtype=float)
    return vectors / np.linalg.norm(vectors, axis=1)[:, None]


def angle_matrix(dirs):
    return np.arccos(np.clip(dirs @ dirs.T, -1.0, 1.0))


def engine_allowance(dirs, h):
    """The engine's one-term allowance from a table and its supports (`engine/src/error.rs`)."""
    gap = angle_matrix(dirs)
    np.fill_diagonal(gap, np.inf)
    rho = 0.5 * gap.min(axis=1)
    radius = float(h.max())
    close = gap < math.pi / 2.0
    pair = (h[:, None] + h[None, :]) / (2.0 * np.cos(gap / 2.0))
    if close.any():
        radius = max(radius, float(pair[close].max()))
    return rho, h * np.tan(rho) + radius * (1.0 / np.cos(rho) - 1.0), gap


def adaptive_3d(count, support, azimuths=6, elevations=5):
    """Greedy: split the direction with the largest allowance towards its nearest neighbour."""
    dirs = unit(table_3d(azimuths, elevations))
    while len(dirs) < count:
        _, e, gap = engine_allowance(dirs, support(dirs))
        i = int(np.argmax(e))
        row = gap[i].copy()
        row[i] = np.inf
        j = int(np.argmin(row))
        dirs = unit(np.vstack([dirs, dirs[i] + dirs[j]]))
    return dirs


def part_i():
    print()
    print("I. 3D allowance against table size, uniform engine shape against greedy adaptive")
    print("   the allowance is `h tan rho + R (1/cos rho - 1)`; the 3d deficit it must beat is 0.113155")
    bodies = (
        ("octahedron", lambda d: np.abs(d).max(axis=1)),
        ("cube 1.5", lambda d: 1.5 * np.abs(d).sum(axis=1)),
    )
    print(f"   {'body':>11}{'table':>10}{'m':>6}{'rho deg':>9}{'allowance':>11}{'of h':>8}")
    for name, support in bodies:
        for label, shape in (("11x10", (11, 10)), ("16x15", (16, 15)), ("20x20", (20, 20)), ("24x25", (24, 25))):
            dirs = unit(table_3d(*shape))
            h = support(dirs)
            rho, e, _ = engine_allowance(dirs, h)
            worst = int(np.argmax(e))
            print(
                f"   {name:>11}{label:>10}{len(dirs):6}{math.degrees(rho.max()):9.4f}"
                f"{float(e.max()):11.6f}{100.0 * float(e.max()) / float(h[worst]):7.2f}%"
            )
        for count in (110, 240, 400, 600):
            dirs = adaptive_3d(count, support)
            h = support(dirs)
            rho, e, _ = engine_allowance(dirs, h)
            worst = int(np.argmax(e))
            print(
                f"   {name:>11}{'adaptive':>10}{len(dirs):6}{math.degrees(rho.max()):9.4f}"
                f"{float(e.max()):11.6f}{100.0 * float(e.max()) / float(h[worst]):7.2f}%"
            )
        print()


def triple_indices(m):
    """All `i < j < k` triples of `m` indices, built in blocks to keep memory bounded."""
    blocks = []
    for k in range(2, m):
        i, j = np.triu_indices(k, k=1)
        blocks.append(np.stack([i, j, np.full_like(i, k)], axis=1))
    return np.concatenate(blocks) if blocks else np.zeros((0, 3), dtype=int)


def unique_directions(dirs, tol=1e-9):
    """First occurrence of each distinct direction, and the slot every entry occupies."""
    dirs = np.asarray(dirs, dtype=float)
    first = []
    back = np.empty(len(dirs), dtype=int)
    for index, direction in enumerate(dirs):
        for slot, kept in enumerate(first):
            if abs(1.0 - float(dirs[kept] @ direction)) <= tol:
                back[index] = slot
                break
        else:
            back[index] = len(first)
            first.append(index)
    return np.asarray(first, dtype=int), back


def cell_radius(dirs):
    """Largest angle from each direction to a vertex of its spherical Voronoi cell.

    In 2D the cell is an arc, so this is half the nearest-neighbour gap.    In 3D the cell's vertices are the circumcenters of sample triples, and the farthest point of a
    long thin cell is one of them — well beyond half the gap. Each centre is credited to every
    sample tied at the maximum distance from it, because the three that generate it always tie.
    This is the `rho` the one-term allowance assumes.

    The shipped 3D table stacks the `±90°` azimuths and repeats ten of its directions. A
    repeated site has no separating bisector and its cell is the cell it repeats, so the radius
    is computed on the distinct directions and mapped back; otherwise the repeated entries get
    a nearest gap of zero and the allowance has a hole exactly where the cells are widest.
    """
    dirs = np.asarray(dirs, dtype=float)
    first, back = unique_directions(dirs)
    return _cell_radius_distinct(dirs[first])[back]


def _cell_radius_distinct(dirs, antipodes=True):
    """`cell_radius` for a table in which no direction is repeated.

    Each triple's centre is credited on both sides of its great-circle plane. A vertex defined only
    by a degenerate triple (its plane through the origin) is the antipode of the centre the sign
    convention lands on, and one-sided credit misses it; see `part_p`. Both sides are genuine sphere
    points credited only to their nearest samples, so the second side can raise `rho` to the truth
    and cannot overshoot it.
    """
    dirs = np.asarray(dirs, dtype=float)
    m = len(dirs)
    gap = angle_matrix(dirs)
    np.fill_diagonal(gap, np.inf)
    rho = 0.5 * gap.min(axis=1)
    if dirs.shape[1] != 3:
        return rho

    triples = triple_indices(m)
    a, b, c = triples[:, 0], triples[:, 1], triples[:, 2]
    normal = np.cross(dirs[b] - dirs[a], dirs[c] - dirs[a])
    norm = np.linalg.norm(normal, axis=1)
    keep = norm > 1e-12
    normal = normal[keep] / norm[keep, None]
    # A zero side means the triple's sum is orthogonal to its normal, so the sign is free and
    # either normal is the circumcenter. Multiplying by the raw sign would zero the centre, and a
    # zero centre is equidistant from every sample — the artifact that reads as `rho = 90`.
    side = np.sign((normal * (dirs[a[keep]] + dirs[b[keep]] + dirs[c[keep]])).sum(axis=1))
    normal = normal * np.where(side < 0.0, -1.0, 1.0)[:, None]

    # Every centre is a vertex of the cell of each sample at the maximum distance from it. An
    # earlier version kept only centres with a unique nearest sample, which discarded every
    # genuine vertex — the three generating samples tie there by construction — and left a
    # radius that was always a strict lower bound.
    signs = (1.0, -1.0) if antipodes else (1.0,)
    for start in range(0, len(normal), 20000):
        block = normal[start : start + 20000]
        for sign in signs:
            dots = (sign * block) @ dirs.T
            best = dots.max(axis=1)
            tied = (best[:, None] - dots) <= 1e-12
            angles = np.arccos(np.clip(best, -1.0, 1.0))
            owners, values = np.nonzero(tied)
            np.maximum.at(rho, values, angles[owners])
    return rho


def circumradius(dirs, h):
    """The `R` in the allowance: the largest sampled support or straddling-pair circumradius."""
    gap = angle_matrix(dirs)
    radius = float(h.max())
    close = gap < math.pi / 2.0
    pair = (h[:, None] + h[None, :]) / (2.0 * np.cos(gap / 2.0))
    if close.any():
        radius = max(radius, float(pair[close].max()))
    return radius


def covering_allowance(dirs, h):
    """The one-term allowance with the cell's covering radius, and the radius it used."""
    rho = cell_radius(dirs)
    radius = circumradius(dirs, h)
    return rho, h * np.tan(rho) + radius * (1.0 / np.cos(rho) - 1.0)


def part_j():
    print()
    print("J. the same tables with the cell covering radius instead of half the nearest gap")
    print("   the gate is 0.150 over a 1.3 closure limit: 11.54% there, 13.04% at P1's interior,")
    print("   4.77% on the angle axis (`02-p1-gate-is-a-permissive-label.md`)")
    print("   convention: `of h` is the allowance over the support at the same cell, with that cell's")
    print("   covering radius; the kb's half-gap figures use half the nearest gap and are lower bounds")
    bodies = (
        ("octahedron", lambda d: np.abs(d).max(axis=1)),
        ("cube 1.5", lambda d: 1.5 * np.abs(d).sum(axis=1)),
    )
    print(f"   {'body':>11}{'table':>10}{'m':>6}{'half gap':>10}{'cell':>10}"
          f"{'e half':>11}{'e cell':>11}{'of h':>8}")
    for name, support in bodies:
        tables = [("11x10", unit(table_3d(11, 10))), ("16x15", unit(table_3d(16, 15)))]
        tables += [(f"ad{count}", adaptive_3d(count, support)) for count in (110, 240)]
        for label, dirs in tables:
            h = support(dirs)
            half_gap, e_half, _ = engine_allowance(dirs, h)
            cell, e_cell = covering_allowance(dirs, h)
            worst = int(np.argmax(e_cell))
            print(
                f"   {name:>11}{label:>10}{len(dirs):6}"
                f"{math.degrees(half_gap.max()):10.4f}{math.degrees(cell.max()):10.4f}"
                f"{float(e_half.max()):11.6f}{float(e_cell.max()):11.6f}"
                f"{100.0 * float(e_cell.max()) / float(h[worst]):7.2f}%"
            )
        print()


def probe_directions(count, seed):
    """Random unit directions plus every facet normal the bodies below have."""
    generator = np.random.default_rng(seed)
    points = generator.normal(size=(count, 3))
    points /= np.linalg.norm(points, axis=1)[:, None]
    fixed = np.vstack(
        [
            np.eye(3),
            -np.eye(3),
            unit(
                [
                    [1, 1, 1], [1, 1, -1], [1, -1, 1], [-1, 1, 1],
                    [-1, -1, 1], [-1, 1, -1], [1, -1, -1], [-1, -1, -1],
                ]
            ),
        ]
    )
    return np.vstack([fixed, points])


def reconstruction_vertices(dirs, support, eps=1e-9, chunk=20000):
    """Vertices of `{x : <psi_i, x> <= support_i}`, i.e. every accepted 3-subset solve."""
    triples = triple_indices(len(dirs))
    kept = []
    for start in range(0, len(triples), chunk):
        block = triples[start : start + chunk]
        coef = dirs[block]
        det = np.linalg.det(coef)
        good = np.abs(det) > 1e-12
        if not good.any():
            continue
        points = np.linalg.solve(coef[good], support[block][good][..., None])[..., 0]
        ok = ((points @ dirs.T - support[None, :]) <= eps).all(axis=1)
        if ok.any():
            kept.append(points[ok])
    if not kept:
        return np.zeros((0, 3))
    return np.concatenate(kept)


def triple_vertex_bound(dirs, h, probes, near=6):
    """Min over straddling triples of the three-plane vertex support along each probe.

    Dropping to three half-spaces enlarges the set, so the extent of the three-plane
    intersection bounds `h_P`; its vertex solves `<psi_j, x> = c_j`, and a probe in the positive
    cone of the three normals makes that vertex the maximum along the probe. `inf` where no
    triple straddles it.
    """
    dots = probes @ dirs.T
    order = np.argsort(-dots, axis=1)[:, :near]
    best = np.full(len(probes), np.inf)
    for row in range(len(probes)):
        for triple in itertools.combinations(order[row].tolist(), 3):
            coef = dirs[list(triple)]
            if abs(float(np.linalg.det(coef))) < 1e-12:
                continue
            lam = np.linalg.solve(coef.T, probes[row])
            if lam.min() < -1e-12:
                continue
            best[row] = min(best[row], float(lam @ h[list(triple)]))
    return best


def part_m(count=2500, seed=11):
    print()
    print("M. where the one-term allowance stops being a bound, and whether a triple vertex bounds")
    print("   `local` is the allowance at the probe's own cell, `excess / local` is the ratio at the")
    print("   worst probe; `gap / rho` = 1 means the probe sits on the cell's vertex, which is where")
    print("   the covering radius is attained. `triple` is the three-plane vertex bound on the same")
    print("   excess, `ex/triple` how tight it is at the worst probe, and `max ex-triple` the largest")
    print("   absolute violation. `no-triple` is the max one-term ratio where no triple straddles.")
    bodies = (
        ("octahedron 1", lambda d: np.abs(d).max(axis=1)),
        ("cube 1.5", lambda d: 1.5 * np.abs(d).sum(axis=1)),
    )
    probes = probe_directions(count, seed)
    print(
        f"   {'body':>13}{'table':>7}{'rho':>9}{'1-term':>8}{'ex/local':>10}{'gap/rho':>9}"
        f"{'ex/triple':>10}{'triple':>8}{'max ex-triple':>14}{'no-triple':>11}{'no tri':>8}"
    )
    for name, support in bodies:
        for label, dirs in (("11x10", unit(table_3d(11, 10))), ("16x15", unit(table_3d(16, 15)))):
            h = support(dirs)
            exact = support(probes)
            extent = (reconstruction_vertices(dirs, h) @ probes.T).max(axis=0)
            excess = extent - exact
            rho, _ = covering_allowance(dirs, h)
            radius = circumradius(dirs, h)
            rho_at = rho[(probes @ dirs.T).argmax(axis=1)]
            local = exact * np.tan(rho_at) + radius * (1.0 / np.cos(rho_at) - 1.0)
            triple = triple_vertex_bound(dirs, h, probes) - exact
            finite = np.isfinite(triple)
            viol = int((excess > local + 1e-9).sum())
            broken = int((excess[finite] > triple[finite] + 1e-9).sum())
            worst = int(np.argmax(excess / np.maximum(local, 1e-12)))
            gap = math.degrees(math.acos(np.clip(float((dirs @ probes[worst]).max()), -1.0, 1.0)))
            over = float(np.max(excess[finite] - triple[finite]))
            uncovered = float(np.max(excess[~finite] / np.maximum(local[~finite], 1e-12)))
            at_worst = excess[worst] / triple[worst] if finite[worst] else float("nan")
            print(
                f"   {name:>13}{label:>7}{math.degrees(rho.max()):9.4f}{viol:8d}"
                f"{excess[worst] / local[worst]:10.3f}{gap / math.degrees(rho_at[worst]):9.3f}"
                f"{at_worst:10.3f}{broken:8d}{over:14.1e}{uncovered:11.3f}"
                f"{int((~finite).sum()):8d}"
            )
            if broken:
                print(f"   {name:>13}{label:>7}  TRIPLE BOUND FALSIFIED at {broken} probes")
    print("   A triple that straddles the probe is a valid bound by construction, so the question is")
    print("   whether the cell's vertex is where the one-term form fails and whether the triple is")
    print("   tighter there. Prediction, written before the run: the one-term violations sit at")
    print("   `gap / rho` near 1, and the triple is both valid and tight there.")


def part_k(count=4000, seed=7):
    print()
    print("K. the measured excess against the allowance that is meant to bound it")
    print("   exact supports at the table, reconstruction by 3-subset enumeration, sweep includes")
    print("   every facet normal; `bound` is the allowance at the cell that owns the probe.")
    bodies = (
        ("octahedron 1", lambda d: np.abs(d).max(axis=1)),
        ("cube 1.5", lambda d: 1.5 * np.abs(d).sum(axis=1)),
    )
    probe = probe_directions(count, seed)
    print(
        f"   {'body':>13}{'table':>7}{'m':>5}{'rho max':>9}{'excess':>10}{'gap':>8}"
        f"{'bound':>10}{'bound/h':>9}{'excess/bound':>13}{'excess/scalar':>14}"
    )
    for name, support in bodies:
        for label, dirs in (
            ("11x10", unit(table_3d(11, 10))),
            ("16x15", unit(table_3d(16, 15))),
        ):
            h = support(dirs)
            vertices = reconstruction_vertices(dirs, h)
            extent = (vertices @ probe.T).max(axis=0)
            exact = support(probe)
            excess = extent - exact
            rho, allowance = covering_allowance(dirs, h)
            radius = circumradius(dirs, h)
            owner = (probe @ dirs.T).argmax(axis=1)
            rho_at = rho[owner]
            local = exact * np.tan(rho_at) + radius * (1.0 / np.cos(rho_at) - 1.0)
            worst = int(np.argmax(excess))
            gap = math.degrees(
                math.acos(np.clip(float((dirs @ probe[worst]).max()), -1.0, 1.0))
            )
            violations = int((excess > local + 1e-9).sum())
            print(
                f"   {name:>13}{label:>7}{len(dirs):5}{math.degrees(rho.max()):9.4f}"
                f"{excess[worst]:10.6f}{gap:8.4f}{local[worst]:10.6f}"
                f"{100.0 * local[worst] / exact[worst]:8.2f}%"
                f"{100.0 * excess[worst] / local[worst]:12.1f}%"
                f"{100.0 * excess.max() / allowance.max():13.1f}%"
            )
            if violations:
                print(f"   {name:>13}{label:>7}  BOUND FALSIFIED at {violations} probes")
    print("   the last column is the comparison the certificate and the band actually make:")
    print("   the worst measured excess over the engine's single scalar allowance.")


def part_k_2d(m=110):
    print()
    print("L. control: the same ratio in 2D, where part A already knows the answer")
    print(f"   {'body':>13}{'m':>5}{'rho max':>9}{'excess':>10}{'bound':>10}{'excess/bound':>13}")
    dirs = table_2d(m)
    rho = 0.5 * (2.0 * math.pi / m)
    for name, half, square in (("square 0.5", 0.5, True), ("diamond 3.0", 3.0, False)):
        h = (
            half * np.abs(dirs).sum(axis=1)
            if square
            else half * np.abs(dirs).max(axis=1)
        )
        radius = circumradius(dirs, h)
        worst = 0.0
        worst_bound = 0.0
        worst_ratio = 0.0
        for step in np.arange(0.0, 360.0, 0.5):
            phi = math.radians(step)
            psi = np.array([math.cos(phi), math.sin(phi)])
            tangent = np.array([-psi[1], psi[0]])
            rotated = np.stack([dirs @ psi, dirs @ tangent], axis=1)
            extent = max_coordinate(rotated, h, axis=0)
            exact = float(
                half * (abs(math.cos(phi)) + abs(math.sin(phi)))
                if square
                else half * max(abs(math.cos(phi)), abs(math.sin(phi)))
            )
            local = exact * math.tan(rho) + radius * (1.0 / math.cos(rho) - 1.0)
            if extent - exact > worst:
                worst = extent - exact
                worst_bound = local
            worst_ratio = max(worst_ratio, (extent - exact) / local)
        print(
            f"   {name:>13}{m:5}{math.degrees(rho):9.4f}{worst:10.6f}{worst_bound:10.6f}"
            f"{100.0 * worst_ratio:12.1f}%"
        )
    print("   part A's box is tight (`0.0142839` against `0.014284`), so 97% of the bound is the")
    print("   right number there; the diamond is exact at its vertex normal and bulges by")
    print("   `0.045448` at its facet normal, which is the worst probe and 73% of the bound.")


def part_n():
    """
    N. The cube across table shapes: does density ever remove the one-term violation?

    Prediction, written before the run: the violation survives every shape up to `240` directions,
    because the shortfall is structural at a cell vertex and density only shrinks it. No shape is
    clean, the worst ratio falls from `1.046` towards `1.01`, and the excess share of `R` falls
    roughly with `tan ρ`.
    """
    print()
    print("N. the cube at `1.5` across table shapes: the one-term violation against density")
    print("   `viol` counts probes where the excess exceeds the local one-term allowance;")
    print("   `worst ex/loc` is that ratio's maximum, `max ex/R` the widest excess over the radius.")
    shapes = ((11, 10), (13, 12), (14, 13), (15, 14), (16, 15), (18, 12), (20, 12))
    probes = probe_directions(2500, 11)
    cube = lambda d: 1.5 * np.abs(d).sum(axis=1)
    exact = cube(probes)
    print(f"   {'table':>8}{'n':>5}{'rho deg':>9}{'viol':>6}{'worst ex/loc':>14}{'max ex/R %':>12}")
    for azimuths, elevations in shapes:
        dirs = unit(table_3d(azimuths, elevations))
        h = cube(dirs)
        extent = (reconstruction_vertices(dirs, h) @ probes.T).max(axis=0)
        excess = extent - exact
        rho, _ = covering_allowance(dirs, h)
        radius = circumradius(dirs, h)
        rho_at = rho[(probes @ dirs.T).argmax(axis=1)]
        local = exact * np.tan(rho_at) + radius * (1.0 / np.cos(rho_at) - 1.0)
        ratio = excess / np.maximum(local, 1e-12)
        violations = int((excess > local + 1e-9).sum())
        print(
            f"   {f'{azimuths}x{elevations}':>8}{len(dirs):5}{math.degrees(rho.max()):9.4f}"
            f"{violations:6d}{ratio.max():14.3f}{100.0 * excess.max() / radius:12.4f}"
        )
    return None


def part_o():
    """
    O. Alignment against allowance: does a sound table also buy P1's gate?

    Prediction, written before the run: the aligned tables (`13 x 12`, `18 x 12`) are sound, as part N
    found, and their allowance stays `ρ`-driven at `9-12%` of the support — far above the adaptive
    `1.60%` and within a factor of the `11.5%` margin P1's `0.150` on a `1.3` limit implies. So
    alignment removes the violation but not the cost: if `18 x 12` lands under `11.5%` it is the
    first sound 3D table that could carry P1, and if it lands over, the gate is still a table size.
    """
    print()
    print("O. one-term allowance as a fraction of the support, aligned against unaligned tables")
    print("   `allow/support` is the widest local allowance over the probes, over the widest support.")
    probes = probe_directions(2500, 11)
    bodies = (
        ("cube 1.5", lambda d: 1.5 * np.abs(d).sum(axis=1)),
        ("octahedron 1", lambda d: np.abs(d).max(axis=1)),
    )
    shapes = ((11, 10), (13, 12), (16, 15), (18, 12), (20, 12))
    print(f"   {'body':>13}{'table':>9}{'n':>5}{'rho deg':>9}{'allow/support':>15}{'viol':>6}")
    for name, support in bodies:
        exact = support(probes)
        top = exact.max()
        for azimuths, elevations in shapes:
            dirs = unit(table_3d(azimuths, elevations))
            h = support(dirs)
            rho, _ = covering_allowance(dirs, h)
            radius = circumradius(dirs, h)
            rho_at = rho[(probes @ dirs.T).argmax(axis=1)]
            allowance = exact * np.tan(rho_at) + radius * (1.0 / np.cos(rho_at) - 1.0)
            excess = (reconstruction_vertices(dirs, h) @ probes.T).max(axis=0) - exact
            violations = int((excess > allowance + 1e-9).sum())
            print(
                f"   {name:>13}{f'{azimuths}x{elevations}':>9}{len(dirs):5}{math.degrees(rho.max()):9.4f}"
                f"{100.0 * allowance.max() / top:14.4f}%{violations:6d}"
            )
    print("   for reference: under this same cell-radius convention the shipped `11 x 10` reads")
    print("   `44.65%` of the support (cube), and P1's margin `0.150` reads `11.54%` there,")
    print("   `13.04%` at the interior (`02-p1-gate-is-a-permissive-label.md`).")
    return None


def part_p():
    """
    P. Degenerate circumcenters: one side of the great-circle plane is not the whole story.

    A triple whose plane passes through the origin has no unique circumcenter: its centre is a pole
    of the great circle the triple lies on, and both poles are equally valid. The sign convention in
    `_cell_radius_distinct` keeps one of them. A table of four equatorial directions plus the north
    pole shows what that costs: the four equatorial cells meet at the *south* pole, `90 deg` from
    each, and only degenerate triples define it, so the one-sided enumeration lands on the north
    pole instead.

    Prediction, written before the run: one-sided `45 deg` against a true `90 deg`, and the shipped
    tables unchanged by the second side because their vertices are non-degenerate.
    """
    print()
    print("P. degenerate circumcenters: the antipode of a one-sided centre")
    equator = unit(np.array([
        [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0],
    ]))
    one = _cell_radius_distinct(equator, antipodes=False)
    both = _cell_radius_distinct(equator, antipodes=True)
    print(f"   {'table':>20}{'half gap':>11}{'one side':>11}{'antipodes':>11}{'cells moved':>13}")
    print(f"   {'equator 4 + pole':>20}{math.degrees(math.pi / 4.0):10.4f}deg"
          f"{math.degrees(one.max()):10.4f}deg{math.degrees(both.max()):10.4f}deg"
          f"{int((both > one + 1e-12).sum()):13d}")
    for label, dirs in (("11 x 10", unit(table_3d(11, 10))), ("16 x 15", unit(table_3d(16, 15)))):
        first, _ = unique_directions(dirs)
        one = _cell_radius_distinct(dirs[first], antipodes=False)
        both = _cell_radius_distinct(dirs[first], antipodes=True)
        gap = angle_matrix(dirs[first])
        np.fill_diagonal(gap, np.inf)
        half_gap = 0.5 * gap.min(axis=1).max()
        print(f"   {label:>20}{math.degrees(half_gap):10.4f}deg"
              f"{math.degrees(one.max()):10.4f}deg{math.degrees(both.max()):10.4f}deg"
              f"{int((both > one + 1e-12).sum()):13d}")
    print("   the one-sided figure is a lower bound by construction; the antipode closes the")
    print("   degenerate-vertex hole and leaves the shipped tables' maxima where they were")
    return None


def main():
    rho_2d = covering_radius(directions_2d())
    rho_3d = covering_radius(directions_3d())
    tracked_cases(rho_2d, rho_3d)
    blind_cases(rho_2d)
    m_sweep()
    part_d([17.0, math.degrees(math.pi / 110), 1.2])
    part_e()
    part_f()
    part_g()
    part_h()
    part_i()
    part_j()
    part_k()
    part_k_2d()
    part_m()
    part_n()
    part_p()
    part_o()
    return 0


if __name__ == "__main__":
    sys.exit(main())
