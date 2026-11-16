"""Where the `example-1` deficit comes from.

A. Refine `Step` at fixed `T`. If the printed shortfall scales with the step it is the
   interval refinement; if it is constant it is geometry.
B. Re-run the *same* recursion in numpy with m evenly spaced directions (A = 0, so the
   substep is exactly `dual_update(c_Q, dirs, c + c_P)`). The engine's 110-direction answer
   must reproduce first; then the table is rotated and refined to separate alignment from
   density.
C. Skip stages 2-5 and use the algebraic difference: whatever vanishes then is the loss of
   the dual update's primal reconstruction.
D. The same two experiments on `example-2b` (squares, equal authority), where the exact
   bridge is the target and the printed answer is 2.9% *over* rather than 2.3% under.
E. The sign, asked as a question about sets rather than about one extent: the support mismatch
   at the true facet normals, with the angle to the nearest sampled direction.
G. The inner side of a bracket: shift every sampled half-space inward by its own allowance and
   ask whether the resulting set is inside the true body.
F. Both inclusion tests. Prediction, written before the run: neither inclusion holds in
   general. The deficit's sign follows the alignment of the true facets with the table, and
   `example-1` and `example-2b` disagree on it while sharing a table — so the printed set is
   neither reliably inside nor reliably outside the true one.

    python tools/trace_deficit.py

Requires numpy. The engine binary is used only for experiment A.
"""

import math
import os
import subprocess
import sys

import numpy as np

ENGINE = os.path.join("engine", "target", "release", "stable_bridge")
CASE = os.path.join("examples", "example-1.txt")
SCRATCH = os.path.join("engine", "target", "trace")

# Analytic answers: example-1 is a diamond of 1 + 2*(2 - 1) = 3; example-2b has equal
# authority, so its bridge is its target, a square of half-width 0.5.
EXACT_DIAMOND = 3.0
EXACT_SQUARE = 0.5


def table(m, offset=0.0):
    """m evenly spaced unit directions, rotated by `offset` radians."""
    angles = np.arange(m) * (2.0 * math.pi) / m + offset
    return np.stack([np.cos(angles), np.sin(angles)], axis=1)


def shape_support(dirs, half_width, square):
    """Support of a diamond {|x1|+|x2| <= a} or a square {|x| <= a}."""
    if square:
        return half_width * np.abs(dirs).sum(axis=1)
    return half_width * np.abs(dirs).max(axis=1)


def dual_update(cq, dirs, total, n):
    """The engine's five stages, vectorised (engine/src/pontryagin_difference.rs)."""
    count = len(dirs)
    diff = total - cq
    v = n / count * (dirs.T @ diff)
    g = diff - dirs @ v
    inner = dirs @ dirs.T

    # c2[i] = min { g[j] / <psi_i, psi_j> : <psi_i, psi_j> > 0 }; mask[argmin] set.
    positive = inner > 0.0
    with np.errstate(divide="ignore", invalid="ignore"):
        ratios = np.where(positive, g[None, :] / np.where(positive, inner, 1.0), np.inf)
    c2 = ratios.min(axis=1)
    active = ratios.argmin(axis=1)
    mask = np.zeros(count, dtype=bool)
    mask[active] = True

    hull = (c2[:, None] * dirs) @ dirs.T
    return np.where(mask, diff, dirs @ v + hull.max(axis=0))


def extent(dirs, support, axis=1, eps=0.01):
    """Largest `axis` coordinate of {x : <psi_i, x> <= support_i}, by pairwise vertices.

    The engine keeps a reconstruction point when it satisfies every half-space within 0.01,
    so the same slack is used here to make the two numbers comparable.
    """
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


def run(m, offset, target, p_half, q_half, blocks, step, square, exact_difference=False):
    """The `A = 0, pi = B = C = I` recursion of example-1 and example-2b."""
    dirs = table(m, offset)
    h = step / 110.0
    c_p = shape_support(dirs, p_half, square) * h
    c_q = shape_support(dirs, q_half, square) * h
    c = shape_support(dirs, target, square)
    for _ in range(blocks):
        for _ in range(110):
            c = c + c_p - c_q if exact_difference else dual_update(c_q, dirs, c + c_p, 2)
    return dirs, c


def example_1(m=110, offset=0.0, **kw):
    return run(m, offset, 1.0, 2.0, 1.0, 20, 0.1, False, **kw)


def example_2b(m=110, offset=0.0, **kw):
    return run(m, offset, 0.5, 2.0, 2.0, 8, 0.1, True, **kw)


def report(label, dirs, c, exact):
    got = extent(dirs, c)
    print(
        f"  {label:<28} {got:10.6f}   {exact - got:+.6f} ({100 * (exact - got) / exact:+.3f}%)"
    )
    return got


def experiment_a():
    print("A. refine Step at fixed T = 2 (example-1, engine)")
    binary = ENGINE if os.path.exists(ENGINE) else ENGINE + ".exe"
    if not os.path.exists(binary):
        print("  engine binary not built; skipping")
        return
    os.makedirs(SCRATCH, exist_ok=True)
    text = open(CASE, encoding="utf-8").read()
    for step in (0.2, 0.1, 0.05, 0.02, 0.01):
        problem = os.path.join(SCRATCH, f"e1-{step}.txt")
        out = os.path.join(SCRATCH, f"e1-{step}.out.txt")
        open(problem, "w", encoding="utf-8").write(text.replace("step 0.1", f"step {step}"))
        subprocess.run([binary, problem, out], check=True, capture_output=True)
        lines = open(out, encoding="utf-8").read().splitlines()
        begin = [i for i, line in enumerate(lines) if line.startswith("block ")][-1]
        end = lines.index("separator", begin)
        points = np.array([[float(v) for v in line.split()] for line in lines[begin + 1 : end]])
        got = points[:, 1].max()
        print(
            f"  Step {step:<5} blocks {len([l for l in lines if l.startswith('block ')]):>3}"
            f"  y {got:.4f}  short {EXACT_DIAMOND - got:.4f}"
            f" ({100 * (EXACT_DIAMOND - got) / EXACT_DIAMOND:.2f}%)"
        )


def experiment_b():
    print()
    print("B. example-1, same recursion, m evenly spaced directions")
    print(f"  {'directions':<28} {'y extent':>10}   shortfall")
    report("m 110, table as shipped", *example_1(), EXACT_DIAMOND)
    report("m 110, rotated half a gap", *example_1(offset=math.pi / 220.0), EXACT_DIAMOND)
    report("m 110, rotated a quarter gap", *example_1(offset=math.pi / 440.0), EXACT_DIAMOND)
    report("m 110, rotated 90 deg", *example_1(offset=math.pi / 2.0), EXACT_DIAMOND)
    report("m 110, algebraic difference", *example_1(exact_difference=True), EXACT_DIAMOND)


def experiment_c():
    print()
    print("C. density at fixed alignment (a quarter gap off every axis)")
    print(f"  {'directions':<28} {'y extent':>10}   shortfall")
    for m in (110, 154, 220, 308, 440, 616, 880):
        report(f"m {m}, quarter gap", *example_1(m=m, offset=math.pi / (4.0 * m)), EXACT_DIAMOND)


def experiment_d():
    print()
    print("D. example-2b, squares with equal authority: exact bridge is the target")
    print(f"  {'directions':<28} {'x extent':>10}   error")
    for m, offset, label in (
        (110, 0.0, "m 110, table as shipped"),
        (110, math.pi / 220.0, "m 110, rotated half a gap"),
        (220, math.pi / 440.0, "m 220, rotated half a gap"),
        (440, math.pi / 880.0, "m 440, rotated half a gap"),
    ):
        dirs, c = example_2b(m=m, offset=offset)
        report(label, dirs, c, EXACT_SQUARE)


def vertices(dirs, support, eps=0.01):
    """Points of {x : <psi_i, x> <= support_i} that satisfy every half-space within `eps`."""
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
    keep = np.zeros(len(point), dtype=bool)
    for start in range(0, len(point), 20000):
        block = point[start : start + 20000]
        keep[start : start + 20000] = ((block @ dirs.T - support[None, :]) <= eps).all(axis=1)
    return point[keep]


def support_of(points, psi):
    """Support of the reconstructed point set at `psi`."""
    return float((points @ psi).max())


def sample_gap(dirs, psi):
    """Angle in degrees from `psi` to the nearest table direction."""
    return math.degrees(math.acos(min(1.0, float((dirs @ psi).max()))))


PROBES = ((0.0, "0 deg"), (45.0, "45 deg"), (90.0, "90 deg"), (135.0, "135 deg"))

TABLES = (
    (110, 0.0, "110 shipped"),
    (110, math.pi / 220.0, "110 half gap"),
    (220, math.pi / 440.0, "220 half gap"),
)

BODIES = (
    ("example-1 diamond", example_1, EXACT_DIAMOND, False),
    ("example-2b square", example_2b, EXACT_SQUARE, True),
)


def experiment_e():
    print()
    print("E. the sign at the true facet normals, printed minus exact")
    print("   0 deg is a facet of the square and a vertex of the diamond, 45 deg the reverse;")
    print("   `gap` is the angle from that direction to the nearest sampled one.")
    print(
        f"   {'case':<22}{'table':<14}{'direction':>10}{'gap deg':>9}"
        f"{'printed':>10}{'exact':>9}{'diff':>10}"
    )
    for label, builder, exact, square in BODIES:
        for m, offset, tag in TABLES:
            dirs, c = builder(m=m, offset=offset)
            points = vertices(dirs, c)
            for degrees, name in PROBES:
                psi = np.array([math.cos(math.radians(degrees)), math.sin(math.radians(degrees))])
                got = support_of(points, psi)
                want = float(shape_support(psi[None, :], exact, square)[0])
                print(
                    f"   {label:<22}{tag:<14}{name:>10}{sample_gap(dirs, psi):9.4f}"
                    f"{got:10.6f}{want:9.6f}{got - want:+10.6f}"
                )
        print()


def experiment_f():
    print("F. inclusion, both ways")
    print("   outward = max over the true facet normals of h_printed - h_true, positive sticks out")
    print("   inward  = max over the sampled directions of h_true - c, positive sits short")
    normals = np.array([[1.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, -1.0]])
    normals /= np.linalg.norm(normals, axis=1)[:, None]
    print(f"   {'case':<22}{'table':<14}{'outward':>10}{'inward':>10}   verdict")
    for label, builder, exact, square in BODIES:
        for m, offset, tag in TABLES:
            dirs, c = builder(m=m, offset=offset)
            points = vertices(dirs, c)
            outward = max(
                support_of(points, a) - float(shape_support(a[None, :], exact, square)[0])
                for a in normals
            )
            inward = float((shape_support(dirs, exact, square) - c).max())
            if outward <= 1e-6 and inward <= 1e-6:
                verdict = "both inclusions"
            elif outward <= 1e-6:
                verdict = "printed inside the true set"
            elif inward <= 1e-6:
                verdict = "true set inside the printed one"
            else:
                verdict = "neither"
            print(f"   {label:<22}{tag:<14}{outward:+10.6f}{inward:+10.6f}   {verdict}")


def allowance_2d(dirs, support):
    """The engine's one-term allowance, `h tan rho + R (1/cos rho - 1)`, per direction."""
    inner = dirs @ dirs.T
    np.fill_diagonal(inner, -1.0)
    rho = 0.5 * np.arccos(np.clip(inner.max(axis=1), -1.0, 1.0))
    radius = float(support.max())
    for i in range(len(dirs)):
        for j in range(i + 1, len(dirs)):
            gap = math.acos(min(1.0, max(-1.0, float(inner[i, j]))))
            if gap < math.pi / 2.0:
                radius = max(
                    radius,
                    (float(support[i]) + float(support[j])) / (2.0 * math.cos(gap / 2.0)),
                )
    return radius, support * np.tan(rho) + radius * (1.0 / np.cos(rho) - 1.0)


def experiment_g():
    print()
    print("G. the inner side: is the shifted intersection inside the true set?")
    print("   support `c_i - e_i` at every sampled direction, `e_i` the engine's allowance;")
    print("   a positive miss means the inner candidate sticks out of the true body.")
    print(
        f"   {'case':<22}{'table':<14}{'direction':>10}{'shifted':>10}{'true':>9}{'miss':>10}"
    )
    for label, builder, exact, square in BODIES:
        for m, offset, tag in TABLES:
            dirs, c = builder(m=m, offset=offset)
            _, e = allowance_2d(dirs, c)
            shifted = vertices(dirs, c - e)
            for degrees, name in PROBES:
                psi = np.array([math.cos(math.radians(degrees)), math.sin(math.radians(degrees))])
                got = support_of(shifted, psi)
                want = float(shape_support(psi[None, :], exact, square)[0])
                print(
                    f"   {label:<22}{tag:<14}{name:>10}{got:10.6f}{want:9.6f}{got - want:+10.6f}"
                )
            outer = float((c + e).max())
            print(
                f"   {'':<22}{tag:<14}max e {float(e.max()):.6f}, widest bracket"
                f" {2.0 * float(e.max()):.6f} ({100.0 * float(e.max()) / outer:.2f}% of outer)"
            )
        print()


def main():
    experiment_a()
    experiment_b()
    experiment_c()
    experiment_d()
    experiment_e()
    experiment_f()
    experiment_g()
    return 0


if __name__ == "__main__":
    sys.exit(main())
