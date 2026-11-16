# 02-p1-bridge-is-not-affordable — the remainder is larger than the margin before any table is used

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1b.txt`: `w_p = 1.0`, `w_e = 0.5`, `T = 4`, `Step 0.1` |
| resolution | 40 blocks; controls at their vertices; `d` from `0.1` to `1.4`; case metric `1 m/rad` |

**Claim.** A support-function bridge for P1 needs its block map affine in the *state*, and this map
is affine only in the controls: `cos ψ` and `1/d` are not. Measuring both remainders that a charted
linearisation would carry shows neither is affordable. The grid's own Euler step is already `0.031`
off the exact flow in the outer band, against a per-block budget of `0.025/40 = 6.25e-4`, and the
affine fit is `0.131` at best.

| `d` band | Euler step against the exact flow, one block |
|---|---|
| `1.00 - 1.40` | `3.138e-2` |
| `0.60 - 1.00` | `7.604e-2` |
| `0.30 - 0.60` | `2.836e-1` |
| `0.15 - 0.30` | `9.634e-1` |

| chart half-width | affine residual | within `6.25e-4`? | charts to cover the bridge |
|---|---|---|---|
| `0.02` | `1.306e-1` | no | `823 812` |
| `0.05` | `1.947e-1` | no | `51 597` |
| `0.10` | `3.507e-1` | no | `7 168` |
| `0.20` | `8.215e-1` | no | `1 024` |
| `0.40` | `1.522e+0` | no | `128` |

Predicted before the run: the residual would fall with the square of the half-width, and the Euler
error would be `1e-2` in the outer band. Both were wrong: the residual falls like the `0.6` power,
because a chart near the capture radius straddles `1/d² = 100`, and the Euler error is `3e-2`
because the angular rate reaches `2.7 rad/s`, which a `0.1 s` block cannot integrate to first order.

The consequence is about the margin, not the table. `1.275` was read as a distance from a value
iteration on `0.1 s` blocks whose one-block error is already `0.031` outside the singular band, so
the interior's `0.025` margin is *below the resolution of the method that produced it*. Refining
`Step` is now the first thing to try, before any bridge: the point-evader study did exactly that
(`Step 0.02` against `0.05` and `0.1`), and it is the cheapest way to learn whether the interior
survives.

**Evidence.** `python tools/p1_relative.py --flow-error --dmax 1.4`: charts are cubes of the given
half-width in the case metric, the fit is the first-order expansion at the centre, `d` is clamped at
the capture radius in both the flow and the fit, and the residual is the worst over charts, controls
and offsets.

**Limits.** The chart count is a covering count for the *state region*, not a bridge construction:
it says what a charted linearisation would need, not that one exists. The residual is measured on
the block map, so it does not include the direction table (step 2's corrected allowance is `0.47`
for a `1.3` set on top of this). Refining `Step` leaves the affine column untouched; the two blockers are
independent. Nothing here is a certificate.

**Reuse.** Cite before proposing a linearised bridge for P1, and before quoting `1.275` as though
its margin were resolved: the method's block error is larger than the margin.

**Links.** `02-p1-is-not-yet-certifiable.md` (the margin), `02-p1-refinement.md` (the value this
qualifies), `06-three-d-covering-radius.md` (the table's allowance, which enters on top),
`proposals/02-nonlinear-games.md` (E1).
