"""The band between the samples, and the exact band, against the band at the samples.

`--bracket` defines a block's width at the widest of the 110 sampled directions, but the band
is a set: outer polytope minus inner polytope, and their support gap `h_outer - h_inner` is
defined at every angle. Two more figures are printed for 2D: the widest gap over 1440 probed
angles, and the exact maximum, `max_psi (h_outer - h_inner)`, which is the largest distance from
a vertex of the outer polygon to the inner one. The diameter of the pairwise vertex differences
is `25-34x` that figure and was the first, wrong, reading.

    python tools/band_between_samples.py

With the polytopes rebuilt at `1e-9` the exact maximum equals the 1440-angle probe to six
decimals, and the excess over the sampled width is `0.1-3.3%`. The earlier `2.5-12.5%` was the
document's `0.01` vertex slack, not the band (`06-band-maximum-and-the-reconstruction-slack.md`).

Requires the release binary (`cargo build --release --manifest-path engine/Cargo.toml`).
"""

import subprocess

ENGINE = "engine/target/release/stable_bridge"
CASES = ("example-1", "example-2a", "example-2b", "example-2c")


def rows(case):
    """`(sampled, probed, exact)` widths per block from one bracket run."""
    result = subprocess.run(
        [ENGINE, f"examples/{case}.txt", "--bracket"],
        capture_output=True,
        text=True,
        check=True,
    )
    sampled = {}
    probed = {}
    exact = {}
    for line in result.stdout.splitlines():
        pieces = line.strip().split()
        if len(pieces) >= 10 and pieces[0] == "block" and pieces[2] == "direction":
            sampled[int(pieces[1])] = float(pieces[9])
        elif len(pieces) >= 5 and pieces[0] == "block" and pieces[2] == "probed":
            probed[int(pieces[1])] = (
                float(pieces[4]) if pieces[4] != "infinite" else float("inf")
            )
        elif len(pieces) >= 5 and pieces[0] == "block" and pieces[2] == "exact":
            exact[int(pieces[1])] = (
                float(pieces[4]) if pieces[4] != "infinite" else float("inf")
            )
    return sampled, probed, exact


def main():
    print("sampled = widest of the 110 printed widths; probed = widest over 1440 angles;")
    print("exact = max_psi(h_outer - h_inner), the largest outer-vertex distance to the inner")
    print("widest is within 0.5% of the probed one, so the angle probe finds the worst direction.")
    print()
    print(
        f"   {'case':>12}{'block':>7}{'sampled':>11}{'probed':>11}{'exact':>11}"
        f"{'probed/samp':>13}{'exact/probed':>14}"
    )
    for case in CASES:
        sampled, probed, exact = rows(case)
        if not exact:
            raise SystemExit(f"{case}: no exact lines, is the build current?")
        block = max(sampled, key=lambda k: sampled[k])
        block_p = max(probed, key=lambda k: probed[k])
        block_e = max(exact, key=lambda k: exact[k])
        print(
            f"   {case:>12}{block:7}{sampled[block]:11.6f}{probed[block_p]:11.6f}"
            f"{exact[block_e]:11.6f}{probed[block_p] / sampled[block]:13.4f}"
            f"{exact[block_e] / probed[block_p]:14.4f}"
        )
    print()
    print("   An exact/probed ratio above 1.005 would mean 1440 angles still miss the worst")
    print("   direction; at or below it, the probe is enough for these cases.")


if __name__ == "__main__":
    main()
