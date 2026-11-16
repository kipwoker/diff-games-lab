# 02-p1-step-refinement — refining `Step` settles P1 at `1.150`, so the margin is `0.150`

| field | value |
|---|---|
| type | measurement |
| track | 2 |
| date | 2026-11-13 |
| case | `examples/p1/example-p1b.txt`: `w_p = 1.0`, `w_e = 0.5`, `T = 4` |
| resolution | `121 x 36 x 18`, 3 turn rates each, `Step` from `0.1` to `0.00625` |

**Claim.** At a fixed distance grid the reachable distance at `psi_e = 0` converges: `1.233` at
`Step 0.1`, then `1.217`, `1.167`, and `1.150` at both `0.0125` and `0.00625`. Against the `1.3` the
closure rate allows, the interior's margin is therefore `0.150`, six times the `0.025` that the
`0.1 s` blocks implied — the coarse block *understated* the interior rather than inflating it.

| `Step` | blocks | `d*` at `0°` | at `45°` | aligned `V` | closure `V` |
|---|---|---|---|---|---|
| `0.1` | 40 | `1.233` | `1.317` | `30.3` | `30` |
| `0.05` | 80 | `1.217` | `1.317` | `61.6` | `60` |
| `0.025` | 160 | `1.167` | `1.267` | `125.8` | `120` |
| `0.0125` | 320 | `1.150` | `1.250` | `253.9` | `240` |
| `0.00625` | 640 | `1.150` | `1.250` | `510.0` | `480` |

The last two rows are identical to the grid's resolution, and the aligned column tracks closure
`(1.0 - 0.1)/0.3` in every row, so the refinement is not losing time anywhere it should not. The
`nd` refinement of `02-p1-refinement.md` moved the same number the other way (`1.233 -> 1.275`), so
the two axes disagree in direction and `Step` dominates: the `1.275` was a `0.1 s` value on a finer
`d` axis, not a converged one.

Predicted before the run: a fall of `0.02-0.04` to `Step 0.05`, then under `0.005` per halving, and
a margin of `0.01-0.02`. Measured: `0.016`, then `0.050`, then `0.017`, then `0.000` — the first
halving was the cheap one, the correction arrived at `0.025`, and the margin went the other way from
my prediction, to `0.150`. Both the shape and the direction were wrong.

That changes P1's gate rather than its conclusion. A `0.150` margin admits a 3D allowance of
`0.150` on a `1.3` set — `11.5%` of the support — where the shipped table gives `0.47` (`3×` too
large). An adaptive `240` table's `0.021` predates the covering-radius correction and does not
survive it (`06-adaptive-table-under-the-covering-radius.md`), so the table is out of reach at the
shipped shape and the missing pieces are a covering-aware table and a method that is not a
linearised bridge (`02-p1-bridge-is-not-affordable.md`).

**Limits.** `121` distance cells give `0.0167` spacing, so the converged `1.150` is resolved to about
that; the angle grid is `36 x 18` and the `90°` column is still angle-limited
(`02-p1-is-not-yet-certifiable.md`). Controls are three turn rates each, and the value iteration is
a reference: nothing here brackets its grid error. The `Step 0.00625` run costs `64 s` on this host,
which is why the finer `d` axis was not repeated at it.

**Reuse.** Cite as the converged P1 reference (quote `1.150`, and `0.150` for the margin) and before
quoting any P1 gate: the `0.025` figure belongs to the `0.1 s` grid.

**Links.** `02-p1-is-not-yet-certifiable.md` (the margin this corrects), `02-p1-refinement.md` (the
`d` axis), `02-p1-bridge-is-not-affordable.md` (the method), `06-three-d-covering-radius.md` (the
allowance it must meet).
