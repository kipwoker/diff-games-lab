"""Grid reference solver for the P1 game, in polar coordinates.

The game is the chauffeur ordering of `examples/p1/example-p1.txt`: state `(x, y, theta)` is
the evader's position relative to the pursuer plus the pursuer's heading, the pursuer moves at
`v_p` along `theta` and turns at `|u| <= omega`, the evader is a point moving at `v_e` in any
direction.

    x_dot = v_e cos(phi) - v_p cos(theta)
    y_dot = v_e sin(phi) - v_p sin(theta)
    theta_dot = u

Rotating the position and the heading together leaves the dynamics unchanged, so the bridge is
a set in `d` (distance) and `psi = angle(position) - theta` only, and a 3D grid would be 72
copies of the same 2D one. With `beta = phi - angle(position)`:

    d_dot   = v_e cos(beta) - v_p cos(psi)
    psi_dot = (v_e sin(beta) + v_p sin(psi)) / d - u

The solver is a value iteration backwards from capture:

    V_0 = 0 where d <= radius, else infinity
    V_k(z) = min( V_{k-1}(z), 1 + min_u max_beta V_{k-1}( z + h f(z, u, beta) ) )

`min_u max_beta` within a block is the quantifier order of the game: the pursuer picks one
turn, then the evader picks a heading, so the turn has to work against every heading. The `min`
with `V_{k-1}` is capture being absorbing. Sampling is bilinear in `d`, circular-linear in
`psi`, because the closure rate `(v_p - v_e) h` is smaller than a cell.

This is a *reference*, not a certificate: the controls are sampled, and nothing brackets the
grid's own error.

    python tools/p1_grid.py
    python tools/p1_grid.py --nd 401 --npsi 144 --nu 5 --nbeta 24

Requires numpy.
"""

import argparse
import math
import sys
import time

import numpy as np

BIG = 1e6


def solve(vp, ve, omega, radius, horizon, step, nd, npsi, dmax, nu, nbeta):
    ds = np.linspace(0.0, dmax, nd)
    psis = np.arange(npsi) * (2.0 * math.pi) / npsi
    dd = ds[1] - ds[0]
    dpsi = (2.0 * math.pi) / npsi

    value = np.where(ds[None, :] <= radius, 0.0, BIG)
    value = np.broadcast_to(value, (npsi, nd)).copy()

    d_row = ds[None, :]
    psi_col = psis[:, None]
    betas = np.arange(nbeta) * (2.0 * math.pi) / nbeta
    turns = np.linspace(-1.0, 1.0, nu) * omega
    blocks = int(round(horizon / step))

    for _ in range(blocks):
        best_turn = np.full_like(value, BIG)
        for turn in turns:
            worst = np.zeros_like(value)
            for beta in betas:
                d_next = d_row + step * (ve * math.cos(beta) - vp * np.cos(psi_col))
                # psi_dot is singular at d = 0, which is inside capture and never propagated.
                safe_d = np.maximum(d_row, radius)
                psi_next = (
                    psi_col
                    + step * ((ve * math.sin(beta) + vp * np.sin(psi_col)) / safe_d - turn)
                ) % (2.0 * math.pi)

                fd = np.clip(d_next / dd, 0.0, nd - 1.0)  # (1, nd)
                fp = np.clip(psi_next / dpsi, 0.0, npsi - 1.0)  # (npsi, 1)
                i0 = np.minimum(np.floor(fd).astype(int), nd - 2)
                j0 = np.floor(fp).astype(int) % npsi
                wd = fd - i0
                wp = fp - np.floor(fp)
                j1 = (j0 + 1) % npsi
                i1 = i0 + 1

                left = value[j0, i0] * (1 - wd) + value[j0, i1] * wd
                right = value[j1, i0] * (1 - wd) + value[j1, i1] * wd
                blended = left * (1 - wp) + right * wp
                blended = np.where(d_next < 0.0, BIG, blended)
                worst = np.maximum(worst, blended)
            best_turn = np.minimum(best_turn, worst)
        value = np.minimum(value, 1.0 + best_turn)

    return ds, psis, value, blocks


def boundary(ds, psis, value, blocks):
    reachable = value <= blocks + 1e-9
    return np.array([ds[reachable[i]].max() if reachable[i].any() else np.nan for i in range(len(psis))])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--nd", type=int, default=201)
    parser.add_argument("--npsi", type=int, default=72)
    parser.add_argument("--nu", type=int, default=3)
    parser.add_argument("--nbeta", type=int, default=12)
    parser.add_argument("--dmax", type=float, default=2.0)
    parser.add_argument("--step", type=float, default=0.1, help="block length")
    args = parser.parse_args()

    vp, ve, omega, radius, horizon = 1.0, 0.7, 1.0, 0.1, 4.0
    step = args.step
    started = time.time()
    ds, psis, value, blocks = solve(
        vp, ve, omega, radius, horizon, step,
        args.nd, args.npsi, args.dmax, args.nu, args.nbeta,
    )
    elapsed = time.time() - started
    curve = boundary(ds, psis, value, blocks)

    print(
        f"P1 reference in (d, psi): {args.nd} x {args.npsi}, nu={args.nu}, nbeta={args.nbeta}, "
        f"{blocks} blocks, T={horizon}, {elapsed:.1f} s"
    )
    print()
    print(f"   {'psi':>8}{'d* (m)':>10}{'closure-only d*':>18}")
    for degrees in (0, 45, 90, 135, 180, 225, 270, 315):
        index = int(degrees / 360.0 * len(psis)) % len(psis)
        # The naive bound: capture radius plus the closure the straight-line chase allows.
        naive = radius + (vp - ve) * horizon * max(0.0, math.cos(math.radians(degrees)))
        print(f"   {degrees:6d}deg{curve[index]:10.3f}{naive:18.3f}")

    print()
    print("   checks")
    for d, psi, expected in (
        (0.08, 0.0, "inside: d <= r"),
        (0.08, math.pi, "inside: capture set, any heading"),
        (1.20, 0.0, "inside: (1.30 - 0.1) / 0.30 = 4.0 s"),
        (1.36, 0.0, "outside: (1.36 - 0.1) / 0.30 = 4.2 s > T"),
        (2.00, 0.0, "outside: (2.00 - 0.1) / 0.30 = 6.33 s > T"),
    ):
        i = int(np.argmin(np.abs(ds - d)))
        j = int(np.argmin(np.abs((psis - psi + math.pi) % (2.0 * math.pi) - math.pi)))
        verdict = "reached" if value[j, i] <= blocks else "not reached"
        print(f"     d={d:.2f} psi={math.degrees(psi):5.1f}deg -> {verdict:11s} V={value[j, i]:6.1f}  [{expected}]")
    return 0


if __name__ == "__main__":
    sys.exit(main())
