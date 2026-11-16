# 06-two-sided-bracket — shifting each half-space by its allowance gives a 2.8% band that is measured inside

| field | value |
|---|---|
| type | measurement |
| track | 6 |
| date | 2026-11-13 |
| case | the five tracked cases; analytic twins `example-1` (diamond `3`) and `example-2b` (square `0.5`) |
| resolution | 110 directions; `Step 0.1`; reconstructed with the `0.01` acceptance slack |

**Claim.** Shrinking every sampled half-space by its own allowance, `⟨ψ_i, x⟩ ≤ c_i − e_i`, produces
an inner set instead of the inscribed ball: the widest band falls from `30.3–42.4%` of the diameter
to `2.83%` of the outer radius (`1.42%` at 220 directions), and on both analytic cases the shifted
set stays inside the true body at every probe of three tables.

| case | inscribed ball | shifted intersection | ratio |
|---|---|---|---|
| `example-1` | `1.871` | `0.173854` | `10.8×` |
| `example-2a` | `0.816` | `0.074966` | `10.9×` |
| `example-2b` | `0.436` | `0.040437` | `10.8×` |
| `example-2c` | `0.394` | `0.036372` | `10.8×` |
| `example-3d` | `2.206` | `0.645385` | `3.4×`, estimate |

The inner side is now the support of a real set, `c_i − e_i` at every sampled direction, rather than
`min_i c_i`. Its inclusion status is the part that needed measuring, not assuming. Predicted before
the run: the shifted set would stick out of the true body where the printed set reads high.

| body | table | least negative miss | most negative miss |
|---|---|---|---|
| `example-1` | 110 shipped | `-0.018136` | `-0.154482` |
| `example-1` | 110 half gap | `-0.003416` | `-0.122225` |
| `example-1` | 220 half gap | `-0.008328` | `-0.057516` |
| `example-2b` | 110 shipped | `-0.000805` | `-0.023137` |
| `example-2b` | 110 half gap | `-0.001956` | `-0.028754` |
| `example-2b` | 220 half gap | `-0.000268` | `-0.010239` |

Every one of the 24 probes is inside, so the prediction was wrong: the outward read at a facet
normal reaches `4.5%` of the support while the allowance at 110 directions is `2.8%`, both first
order in `ρ`, so the allowance wins by `0.2` to `0.02`. That margin is thin — the square at 220 is
inside by `0.000268` — so the test should be re-run before a denser table is called two-sided.

**Evidence.** `./target/release/stable_bridge ../examples/example-2a.txt --bracket` prints
`block N direction I inner X outer Y width W of diameter P%` and `widest W at block B`;
`python tools/trace_deficit.py` part G runs the inclusion probe on the analytic twins. The engine's
widest for `example-1` is `0.173854`; the twin's is `0.174774`, the difference being the engine's
own supports versus the numpy recursion.

**Corrected.** With the exact radius `example-3d`'s shifted width is `0.999639` (`1.999277` of band),
so `2.206` against it is `2.2×`, not `3.4×`; the 2D rows stand. `06-three-d-radius-is-exact.md`.

**Limits.** 2D, `A = 0`, `L1` and square bodies. The allowance used is the 2D validated form, so
this says nothing about 3D, where `--bracket` still reports an estimate and the band stays one-sided
(`06-bracket-is-one-sided.md`). Twenty-four probes is a spot check, not a proof: the shifted set is
inside at the true body's binding directions, and the interior directions were not swept.

**Reuse.** Cite as the two-sided bracket of record in 2D, and as the answer to E4's inner question: an inner side does not need track 3's ellipsoid, only the same allowance inward.

**Links.** `06-bracket-is-one-sided.md` (the ball this replaces), `06-direction-bound.md` (the
allowance), `01-deficit-is-chamfering.md` (why the outward read is large at facet normals),
`proposals/06-method-evolution.md` (E4).
