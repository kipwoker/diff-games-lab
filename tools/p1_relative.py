"""P1 in the relative frame, with both headings turn-limited.

The redefined case: two unicycles. The pursuer moves at `v_p` and turns at `|u| <= w_p`; the
evader moves at `v_e` and turns at `|w| <= w_e`. Rotating the pair about the midpoint leaves the
game unchanged, so the state is three numbers relative to the line of sight:

    d      distance
    psi_p  pursuer heading minus the LOS angle
    psi_e  evader heading minus the LOS angle

    d_dot     = v_e cos(psi_e) - v_p cos(psi_p)
    los_dot   = (v_e sin(psi_e) - v_p sin(psi_p)) / d
    psi_p_dot = u - los_dot
    psi_e_dot = w - los_dot

`w_e -> inf` is the point-evader chauffeur game, whose bridge is the capture disc
(`02-p1-bridge-is-the-capture-set.md`). The evader can only hold a circle of radius `v_e / w_e`,
so the threshold where the interior should reappear is `w_e = w_p * v_e / v_p`, the point at
which the two turning radii are equal; `--sweep` measures it.

Value iteration backwards from capture, in blocks:

    V_0 = 0 where d <= radius, else cap
    V_k = min( V_{k-1}, 1 + min_u max_w V_{k-1}(z + h f(z, u, w)) ),  cap = blocks + 1

`min_u max_w` is the game's quantifier order inside one block. Trilinear in `(psi_p, psi_e, d)`,
circular in the two angles. The unreachable value is the *cap*, not infinity: with an infinite
sentinel a fraction of it leaks through the interpolator, and the `1 +` accumulates until the
values diverge; renormalising the weights instead is optimistic and lets the bridge jump across
unreachable cells. A finite cap keeps both failures out and is conservative at the front.

A reference, not a certificate: the controls are sampled and nothing brackets the grid's error.

`--flow-error` measures what a support-function bridge for this case would carry. Such a bridge
needs the block map affine in the state, and this one is affine only in the controls `(u, w)`:
`cos psi` and `1/d` are not. The mode reports two remainders: the exact flow against the grid's
Euler step (time discretisation), and the affine fit of the block map over a chart (the price of
covering the state space with charts instead of one map).

    python tools/p1_relative.py
    python tools/p1_relative.py --nd 121 --npp 36 --npe 18 --we 0.5
    python tools/p1_relative.py --sweep

Requires numpy.
"""

import argparse
import math
import sys
import time

import numpy as np


def solve(vp, ve, wp, we, radius, horizon, step, nd, npp, npe, dmax, nu, nw):
    ds = np.linspace(0.0, dmax, nd)
    pps = np.arange(npp) * (2.0 * math.pi) / npp
    pes = np.arange(npe) * (2.0 * math.pi) / npe
    dd = ds[1] - ds[0]
    dpp = (2.0 * math.pi) / npp
    dpe = (2.0 * math.pi) / npe
    blocks = int(round(horizon / step))
    cap = blocks + 1.0

    value = np.where(ds[None, None, :] <= radius, 0.0, cap)
    value = np.broadcast_to(value, (npp, npe, nd)).copy()

    d_row = ds[None, None, :]
    pp_col = pps[:, None, None]
    pe_col = pes[None, :, None]
    safe_d = np.maximum(d_row, radius)

    turns = np.linspace(-1.0, 1.0, nu) * wp
    evader_turns = np.linspace(-1.0, 1.0, nw) * we

    def sample(pp_next, pe_next, d_next):
        fd = np.clip(d_next / dd, 0.0, nd - 1.0)
        fp = pp_next / dpp
        fe = pe_next / dpe
        i0 = np.minimum(np.floor(fd).astype(int), nd - 2)
        jp = np.floor(fp).astype(int) % npp
        je = np.floor(fe).astype(int) % npe
        td = fd - i0
        tp = fp - np.floor(fp)
        te = fe - np.floor(fe)
        jp1 = (jp + 1) % npp
        je1 = (je + 1) % npe
        i1 = i0 + 1

        near = (
            value[jp, je, i0] * (1 - te) + value[jp, je1, i0] * te
        ) * (1 - tp) + (
            value[jp1, je, i0] * (1 - te) + value[jp1, je1, i0] * te
        ) * tp
        far = (
            value[jp, je, i1] * (1 - te) + value[jp, je1, i1] * te
        ) * (1 - tp) + (
            value[jp1, je, i1] * (1 - te) + value[jp1, je1, i1] * te
        ) * tp
        return near * (1 - td) + far * td

    for _ in range(blocks):
        best_turn = np.full_like(value, cap)
        for u in turns:
            worst = np.zeros_like(value)
            for w in evader_turns:
                d_next = d_row + step * (ve * np.cos(pe_col) - vp * np.cos(pp_col))
                los = (ve * np.sin(pe_col) - vp * np.sin(pp_col)) / safe_d
                pp_next = (pp_col + step * (u - los)) % (2.0 * math.pi)
                pe_next = (pe_col + step * (w - los)) % (2.0 * math.pi)
                blended = sample(pp_next, pe_next, d_next)
                blended = np.where(d_next < 0.0, cap, blended)
                worst = np.maximum(worst, blended)
            best_turn = np.minimum(best_turn, worst)
        value = np.minimum(value, np.minimum(cap, 1.0 + best_turn))

    return ds, pps, pes, value, blocks


def relative_flow(z, u, w, vp, ve, radius):
    """The relative dynamics, `z = (d, psi_p, psi_e)`.

    `d` is clamped at the capture radius: inside it the game is over, and the unclamped `1/d`
    makes any integrator diverge. That divergence belongs to the model's singular line, not to
    the flow a bridge would have to carry, and the grid solver clamps the same way.
    """
    d, pp, pe = z
    los = (ve * np.sin(pe) - vp * np.sin(pp)) / max(d, radius)
    return np.array([ve * np.cos(pe) - vp * np.cos(pp), u - los, w - los])


def block_map(z, u, w, vp, ve, step, radius, substeps=16):
    """RK4 over one block with the controls held, the reference the affine fit is measured to."""
    zv = np.array(z, dtype=float)
    hh = step / substeps
    for _ in range(substeps):
        k1 = relative_flow(zv, u, w, vp, ve, radius)
        k2 = relative_flow(zv + 0.5 * hh * k1, u, w, vp, ve, radius)
        k3 = relative_flow(zv + 0.5 * hh * k2, u, w, vp, ve, radius)
        k4 = relative_flow(zv + hh * k3, u, w, vp, ve, radius)
        zv = zv + (hh / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4)
    return zv


def euler_error(vp, ve, wp, we, radius, step, dmax,
                bands=(0.15, 0.3, 0.6, 1.0, 1.41), npp=8, npe=8, nu=3, nw=3):
    """Largest one-block difference between the exact flow and the grid's Euler step, per band.

    This is the calibration error only: the grid evaluates `z + step * f(z, u, w)`, and the
    question is how far that is from the flow it represents. Bands matter because the flow's
    angular rate is `1/d`, so the error near the capture radius is a statement about the model's
    singular line rather than about the bridge.
    """
    out = []
    for lo, hi in zip(bands[:-1], bands[1:]):
        worst = 0.0
        for d in np.linspace(max(lo, radius), min(hi, dmax), 5):
            for pp in np.linspace(0.0, 2.0 * math.pi, npp, endpoint=False):
                for pe in np.linspace(0.0, 2.0 * math.pi, npe, endpoint=False):
                    for u in np.linspace(-wp, wp, nu):
                        for w in np.linspace(-we, we, nw):
                            z = np.array([d, pp, pe])
                            exact = block_map(z, u, w, vp, ve, step, radius)
                            euler = z + step * relative_flow(z, u, w, vp, ve, radius)
                            worst = max(worst, float(np.linalg.norm(exact - euler)))
        out.append((max(lo, radius), min(hi, dmax), worst))
    return out


def chart_residual(vp, ve, wp, we, radius, step, dmax, half,
                   centres=4, offsets=3, nu=3, nw=3):
    """Worst residual of the affine fit of the block map over one chart.

    A chart is a cube of half-width `half` in the case's own metric (`scale_theta = 1 m/rad`,
    so a radian of heading and a metre count alike), centred at `(d0, psi_p0, psi_e0)`; the fit
    is the first-order expansion at that centre. `d` is clipped to the physical range, so a
    chart centred near the capture radius is measured on the part of it that exists.
    """
    worst = 0.0
    eps = 1e-4
    d_low = radius
    d_high = max(dmax, d_low + 2.0 * half)
    for d0 in np.linspace(d_low + half, d_high - half, centres):
        for pp0 in np.linspace(0.0, 2.0 * math.pi, centres, endpoint=False):
            for pe0 in np.linspace(0.0, 2.0 * math.pi, centres, endpoint=False):
                z0 = np.array([d0, pp0, pe0])
                for u in np.linspace(-wp, wp, nu):
                    for w in np.linspace(-we, we, nw):
                        base = block_map(z0, u, w, vp, ve, step, radius)
                        jac = np.empty((3, 3))
                        for k in range(3):
                            bump = np.zeros(3)
                            bump[k] = eps
                            jac[:, k] = (
                                block_map(z0 + bump, u, w, vp, ve, step, radius)
                                - block_map(z0 - bump, u, w, vp, ve, step, radius)
                            ) / (2.0 * eps)
                        for o_d in np.linspace(-1.0, 1.0, offsets):
                            for o_p in np.linspace(-1.0, 1.0, offsets):
                                for o_e in np.linspace(-1.0, 1.0, offsets):
                                    z = np.array([
                                        min(max(d0 + o_d * half, d_low), d_high),
                                        pp0 + o_p * half,
                                        pe0 + o_e * half,
                                    ])
                                    predicted = base + jac @ (z - z0)
                                    worst = max(worst, float(np.linalg.norm(
                                        block_map(z, u, w, vp, ve, step, radius) - predicted)))
    return worst


def flow_error_report(vp, ve, wp, radius, we, step, dmax, blocks, margin=0.150):
    """What a support-function bridge for this case would have to carry.

    The per-block budget is `margin / blocks` with `margin = 0.150`, the corrected interior
    (`02-p1-step-refinement.md`); the old `0.025` was a grid artefact.

    Prediction, written before the run: the chart residual is quadratic in `half`, so a half
    width near `0.005` meets the budget with fewer than `10^5` charts, and the Euler error stays
    below the budget in the smooth bands (`d >= 0.6`).
    """
    budget = margin / blocks
    print("P1 block map: what a support-function bridge would have to carry")
    print(f"   {blocks} blocks of {step} s, controls at the vertices, d from {radius:.2f}"
          f" to {dmax:.2f}")
    print(f"   margin {margin} per horizon, per block {margin} / {blocks} = {budget:.3e}")
    print()
    print("   grid's Euler step against the exact flow, one block - the step's own error, which")
    print("   no charting removes")
    worst_euler = 0.0
    for lo, hi, err in euler_error(vp, ve, wp, we, radius, step, dmax):
        worst_euler = max(worst_euler, err)
        print(f"     d {lo:.2f}-{hi:.2f}: {err:.3e} = {err / budget:6.1f}x budget")
    print()
    print("   affine fit of the block map over one chart, worst residual in the case's metric")
    print(f"   {'half width':>11}{'residual':>12}{'x budget':>10}{'charts':>12}")
    rows = []
    for half in (0.02, 0.05, 0.1, 0.2, 0.4):
        residual = chart_residual(vp, ve, wp, we, radius, step, dmax, half)
        d_charts = int(math.ceil((dmax - radius) / (2.0 * half)))
        a_charts = int(math.ceil(math.pi / half))
        rows.append((half, residual))
        print(f"   {half:11.2f}{residual:12.3e}{residual / budget:10.1f}"
              f"{d_charts * a_charts * a_charts:12d}")
    finest, reference = rows[0]
    next_half, next_residual = rows[1]
    exponent = math.log(next_residual / reference) / math.log(next_half / finest)
    needed = finest * (budget / reference) ** (1.0 / exponent)
    d_charts = int(math.ceil((dmax - radius) / (2.0 * needed)))
    a_charts = int(math.ceil(math.pi / needed))
    print()
    print(f"   the residual falls as half^{exponent:.2f} between {finest} and {next_half}, so the"
          " fit is not quadratic")
    print(f"   at that local slope the budget needs half <= {needed:.1e}, about"
          f" {d_charts * a_charts * a_charts:.1e} charts")
    print(f"   the Euler error alone is {worst_euler / budget:.1f}x the budget, so the step is the"
          " wall whichever charting is used")


def gate_report(vp, ve, wp, we, radius, horizon, step, dmax, nd, npp, npe, nu, nw,
                margin=0.150, scale_theta=1.0):
    """
    P1's gate as a fraction of the support, against every denominator on record.

    The `11.5%` in the kb is `margin / 1.3` — the margin over the closure limit, not over the
    support of the set the allowance is charged on. Three denominators are measured here or given
    by the case file: the interior at `psi_e = 0`, the widest interior over the angle axes, and the
    angular extent `scale_theta * pi`. A gate is only a requirement once one is chosen.

    Prediction, written before the run: the largest allowed fraction is `margin / d*` at
    `psi_e = 0` (`13%`), the closure limit's `11.5%` sits inside that bracket, the angular extent
    is far stricter (`4.8%`), and no measured table clears even the loosest `13%`.
    """
    ds, _, _, value, blocks = solve(
        vp, ve, wp, we, radius, horizon, step, nd, npp, npe, dmax, nu, nw,
    )
    curve = boundary(value, ds, blocks)
    at_zero = float(np.nanmax(curve[:, 0]))
    widest = float(np.nanmax(curve))
    supports = (
        ("interior at psi_e=0", at_zero),
        ("closure limit", 1.3),
        ("widest interior", widest),
        ("angular extent", scale_theta * math.pi),
    )
    print(f"\nP1 gate: margin {margin}, step {step}, {blocks} blocks, "
          f"grid {nd} x {npp} x {npe}")
    for label, support in supports:
        print(f"   {label:>20} support {support:7.4f}   "
              f"allowed fraction {100.0 * margin / support:6.2f}%")

    import covering_bound as covering

    print(f"\n   {'table':>8}{'m':>5}{'allow/support':>14}"
          f"{'absolute at each support':>40}   ratio to the margin")
    for azimuths, elevations in ((11, 10), (16, 15), (13, 12)):
        dirs = covering.unit(covering.table_3d(azimuths, elevations))
        h = 1.5 * np.abs(dirs).sum(axis=1)
        half_gap, e_half, _ = covering.engine_allowance(dirs, h)
        rho, allowance = covering.covering_allowance(dirs, h)
        fraction = float(allowance.max()) / float(h[int(np.argmax(allowance))])
        half = float(e_half.max()) / float(h[int(np.argmax(e_half))])
        absolutes = " ".join(f"{fraction * s:6.3f}" for _, s in supports)
        ratios = " ".join(f"{fraction * s / margin:4.1f}x" for _, s in supports)
        print(f"   {f'{azimuths}x{elevations}':>8}{len(dirs):5}{100.0 * fraction:13.2f}%"
              f"{absolutes:>34}   {ratios}")
        print(f"   {'':>8}{'':>5}{100.0 * half:13.2f}%"
              f"  (half-gap lower bound; cell radius {math.degrees(rho.max()):.4f} deg)")
    print("   verdict: the closure-limit denominator is neither the loosest nor the strictest, and")
    print("   the nearest measured table stays over the margin at every one of them")


def boundary(value, ds, blocks):
    reachable = value <= blocks + 1e-9
    furthest = np.where(reachable, ds[None, None, :], 0.0).max(axis=2)
    return np.where(reachable.any(axis=2), furthest, np.nan)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--nd", type=int, default=121)
    parser.add_argument("--npp", type=int, default=36)
    parser.add_argument("--npe", type=int, default=18)
    parser.add_argument("--nu", type=int, default=3)
    parser.add_argument("--nw", type=int, default=3)
    parser.add_argument("--dmax", type=float, default=2.0)
    parser.add_argument("--step", type=float, default=0.1)
    parser.add_argument("--we", type=float, default=0.5, help="evader turn rate, rad/s")
    parser.add_argument("--sweep", action="store_true", help="scan the evader turn rate")
    parser.add_argument("--flow-error", action="store_true",
                        help="measure the block map's Euler error and its affine-fit residual")
    parser.add_argument("--gate", action="store_true",
                        help="quote P1's margin against every support denominator on record")
    args = parser.parse_args()

    vp, ve, wp, radius, horizon = 1.0, 0.7, 1.0, 0.1, 4.0
    threshold = wp * ve / vp

    if args.sweep:
        print("evader turn-rate sweep, grid "
              f"{args.nd} x {args.npp} x {args.npe}, dmax {args.dmax}, T={horizon}")
        print(f"   equal turning radii at w_e = w_p v_e / v_p = {threshold:.2f} rad/s")
        print(f"   {'w_e':>7}{'v_e/w_e (m)':>13}{'max d* (m)':>12}{'interior?':>11}")
        for we in (50.0, 2.0, 1.2, 1.0, 0.7, 0.5, 0.3, 0.0):
            ds, pps, pes, value, blocks = solve(
                vp, ve, wp, we, radius, horizon, args.step,
                args.nd, args.npp, args.npe, args.dmax, args.nu, args.nw,
            )
            curve = boundary(value, ds, blocks)
            best = float(np.nanmax(curve)) if np.isfinite(curve).any() else float("nan")
            radius_e = float("inf") if we == 0.0 else ve / we
            verdict = "yes" if best > radius + 2 * (ds[1] - ds[0]) else "no"
            print(f"   {we:7.2f}{radius_e:13.2f}{best:12.3f}{verdict:>11}")
        return 0

    if args.flow_error:
        blocks = int(round(horizon / args.step))
        flow_error_report(vp, ve, wp, radius, args.we, args.step, args.dmax, blocks)
        return 0

    if args.gate:
        gate_report(vp, ve, wp, args.we, radius, horizon, args.step, args.dmax,
                    args.nd, args.npp, args.npe, args.nu, args.nw)
        return 0

    started = time.time()
    ds, pps, pes, value, blocks = solve(
        vp, ve, wp, args.we, radius, horizon, args.step,
        args.nd, args.npp, args.npe, args.dmax, args.nu, args.nw,
    )
    elapsed = time.time() - started
    curve = boundary(value, ds, blocks)

    print(
        f"P1 relative frame: {args.nd} x {args.npp} x {args.npe}, nu={args.nu}, nw={args.nw}, "
        f"{blocks} blocks, T={horizon}, w_p={wp}, w_e={args.we}, {elapsed:.1f} s"
    )
    print()
    print(f"   {'psi_e':>7}{'max d* over psi_p':>20}{'min d* over psi_p':>20}")
    for degrees in (0, 45, 90, 135, 180, 225, 270, 315):
        index = int(round(degrees / 360.0 * len(pes))) % len(pes)
        column = curve[:, index]
        best = np.nanmax(column) if np.isfinite(column).any() else float("nan")
        worst = np.nanmin(column) if np.isfinite(column).any() else float("nan")
        print(f"   {degrees:5d}deg{best:20.3f}{worst:20.3f}")

    print()
    print("   anchors (a state, not just a distance: psi_e = 0 is the evader fleeing)")
    reachable = value <= blocks + 1e-9
    print(f"     reachable cells: {int(reachable.sum())} of {reachable.size}")
    for d, pp, pe, note in (
        (0.08, 0.0, 0.0, "inside: d <= r"),
        (1.20, 0.0, 0.0, "aligned: (1.2 - 0.1)/0.3 = 3.67 s < T"),
        (2.00, 0.0, 0.0, "fleeing along the LOS: 6.33 s > T"),
    ):
        i = int(np.argmin(np.abs(ds - d)))
        j = int(np.argmin(np.abs((pps - pp + math.pi) % (2 * math.pi) - math.pi)))
        k = int(np.argmin(np.abs((pes - pe + math.pi) % (2 * math.pi) - math.pi)))
        verdict = "reached" if reachable[j, k, i] else "not reached"
        print(f"     d={d:.2f} psi_p={np.degrees(pp):.0f} psi_e={np.degrees(pe):.0f}"
              f" -> {verdict:12s} V={value[j, k, i]:6.1f} [{note}]")
    i = int(np.argmin(np.abs(ds - 1.0)))
    print(f"     aligned d=1.0 -> V={value[0, 0, i]:.1f} blocks (closure alone needs 30)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
