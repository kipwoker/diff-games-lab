use stable_bridge::convex_polygon;
use stable_bridge::directions;
use stable_bridge::error::{
    allowance, angle, circumradius_bound, covering_radius, max_allowance, pair_extent, support_at,
    triple_bound,
};

const HALF: f64 = 3.0;

fn parallelogram_support(angle: f64, facet: f64) -> f64 {
    let delta = angle - facet;
    HALF * (delta.cos().abs() + delta.sin().abs())
}

/// The solve must agree with the formula the 2D experiment used.
#[test]
fn the_solve_reproduces_the_two_sample_formula() {
    let rho_a = 2.6364_f64.to_radians();
    let rho_b = 0.6364_f64.to_radians();
    let a = [rho_a.cos(), -rho_a.sin()];
    let b = [rho_b.cos(), rho_b.sin()];
    let psi = [1.0, 0.0];
    let (c_a, c_b) = (1.7, 0.9);
    let solved = pair_extent(&a, c_a, &b, c_b, &psi);
    let closed = (c_a * rho_b.sin() + c_b * rho_a.sin()) / (rho_a + rho_b).sin();
    assert!((solved - closed).abs() < 1e-12, "{solved} vs {closed}");
}

/// Symmetric neighbours reduce to `c / cos ρ`.
#[test]
fn symmetric_neighbours_give_c_over_cos_rho() {
    let rho = 1.6364_f64.to_radians();
    let a = [(-rho).cos(), (-rho).sin()];
    let b = [rho.cos(), rho.sin()];
    let solved = pair_extent(&a, 2.0, &b, 2.0, &[1.0, 0.0]);
    assert!((solved - 2.0 / rho.cos()).abs() < 1e-12, "{solved}");
}

/// The measured row of `06-two-neighbour-bound.md`, at a facet 17° off a sample.
#[test]
fn the_recorded_17_degree_row() {
    let facet = 17.0_f64.to_radians();
    let sample_a = facet - 2.6364_f64.to_radians();
    let sample_b = facet + 0.6364_f64.to_radians();
    let a = [sample_a.cos(), sample_a.sin()];
    let b = [sample_b.cos(), sample_b.sin()];
    let psi = [facet.cos(), facet.sin()];
    let extent = pair_extent(
        &a,
        parallelogram_support(sample_a, facet),
        &b,
        parallelogram_support(sample_b, facet),
        &psi,
    );
    assert!((extent - 3.0537).abs() < 5e-5, "{extent}");
}

/// In 2D a cell is an arc, so its circumradius is half the nearest-neighbour gap exactly.
#[test]
fn the_covering_radius_is_the_half_gap_in_two_dimensions() {
    let table = directions::build(2).expect("2D table");
    let radius = covering_radius(&table);
    assert_eq!(radius.len(), table.len());
    for index in 0..table.len() {
        let nearest = (0..table.len())
            .filter(|other| *other != index)
            .map(|other| angle(&table[index], &table[other]))
            .fold(f64::INFINITY, f64::min);
        assert!(
            (radius[index] - 0.5 * nearest).abs() < 1e-12,
            "{}: {} vs {}",
            index,
            radius[index],
            0.5 * nearest
        );
    }
}

/// In 3D the cell's vertices are circumcenters of sample triples, so the radius is at least the
/// half gap and the allowance built on it is larger — which is the correction.
#[test]
fn the_three_d_covering_radius_exceeds_the_half_gap() {
    let table = directions::build(3).expect("3D table");
    let radius = covering_radius(&table);
    let mut strictly_larger = 0;
    for index in 0..table.len() {
        let nearest = (0..table.len())
            .filter(|other| *other != index)
            .map(|other| angle(&table[index], &table[other]))
            .fold(f64::INFINITY, f64::min);
        assert!(
            radius[index] >= 0.5 * nearest - 1e-12,
            "{}: {} below the half gap {}",
            index,
            radius[index],
            0.5 * nearest
        );
        if radius[index] > 0.5 * nearest + 1e-9 {
            strictly_larger += 1;
        }
    }
    assert!(
        strictly_larger > table.len() / 2,
        "only {strictly_larger} cells are wider than their half gap"
    );
    assert!(
        radius.iter().all(|value| value.is_finite()),
        "a cell with no finite radius"
    );
}

/// The shipped 3D table repeats ten directions (110 entries, 100 distinct sites). A repeated
/// site has no separating bisector, so it must inherit the cell of the direction it repeats,
/// not a radius of zero.
#[test]
fn duplicated_directions_share_a_cell() {
    let table = directions::build(3).expect("3D table");
    let radius = covering_radius(&table);

    let mut distinct: Vec<Vec<f64>> = Vec::new();
    let mut slot_of = Vec::new();
    for psi in &table {
        let mut slot = distinct.len();
        for (index, kept) in distinct.iter().enumerate() {
            if angle(kept, psi) < 1e-6 {
                slot = index;
                break;
            }
        }
        if slot == distinct.len() {
            distinct.push(psi.clone());
        }
        slot_of.push(slot);
    }
    assert!(
        distinct.len() < table.len(),
        "the shipped table is expected to repeat directions"
    );

    let deduped = covering_radius(&distinct);
    for index in 0..table.len() {
        assert!(
            radius[index] > 0.0,
            "cell {index} has no radius, so its allowance is a hole"
        );
        assert!(
            (radius[index] - deduped[slot_of[index]]).abs() < 1e-9,
            "{}: {} vs the deduplicated {}",
            index,
            radius[index],
            deduped[slot_of[index]]
        );
    }
}

/// A regular tetrahedron's cells meet at the antipode of the fourth sample, so each radius is
/// `acos 1/3 = 70.53°` — past the half gap `54.74°`. Skipping the tied generating samples
/// returns the half gap and is exactly the defect this test pins.
#[test]
fn a_tetrahedron_has_a_known_covering_radius() {
    let s = 1.0 / 3.0_f64.sqrt();
    let table = vec![
        vec![s, s, s],
        vec![s, -s, -s],
        vec![-s, s, -s],
        vec![-s, -s, s],
    ];
    let radius = covering_radius(&table);
    let want = (1.0 / 3.0_f64).acos();
    for (index, value) in radius.iter().enumerate() {
        assert!((value - want).abs() < 1e-9, "{index}: {value} vs {want}");
    }
}

/// The radius must dominate every point of the sphere, not only the centres the enumeration
/// keeps: a probe past the reported maximum is a certificate that is not a bound.
#[test]
fn the_three_d_radius_dominates_sampled_probes() {
    let table = directions::build(3).expect("3D table");
    let radius = covering_radius(&table);
    let worst = radius.iter().copied().fold(0.0_f64, f64::max);

    // A fixed xorshift, so the probe set is the same on every run.
    let mut state = 0x2545_F491_4F6C_DD1D_u64;
    let mut uniform = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1_u64 << 53) as f64 * 2.0 - 1.0
    };

    // Each probe's own nearest-sample angle, so the largest of them is what the radius must
    // dominate. One `acos` per probe keeps the suite quick.
    let mut closest = f64::INFINITY;
    let mut accepted = 0;
    while accepted < 200_000 {
        let point = [uniform(), uniform(), uniform()];
        let norm = (point[0] * point[0] + point[1] * point[1] + point[2] * point[2]).sqrt();
        if !(1e-3..=1.0).contains(&norm) {
            continue;
        }
        let unit = [point[0] / norm, point[1] / norm, point[2] / norm];
        let mut best = f64::NEG_INFINITY;
        for psi in &table {
            best = best.max(psi[0] * unit[0] + psi[1] * unit[1] + psi[2] * unit[2]);
        }
        closest = closest.min(best);
        accepted += 1;
    }
    // `closest` is the smallest over probes of the largest dot over samples — the probe least
    // covered by the table — so it measures the covering radius from below.
    let sampled = closest.clamp(-1.0, 1.0).acos();
    assert!(
        sampled <= worst + 1e-9,
        "a probe is {sampled} from its nearest sample, past the radius {worst}"
    );
    assert!(
        worst - sampled < 0.02,
        "the radius {worst} is more than a degree past the sampled {sampled}"
    );
    println!("radius {:.6} deg, sampled {:.6} deg", worst.to_degrees(), sampled.to_degrees());
}

#[test]
fn parallel_planes_have_no_bound() {    let psi = [1.0, 0.0];
    assert!(pair_extent(&psi, 1.0, &psi, 2.0, &psi).is_infinite());
}

/// At a sampled direction the reconstruction has exactly the stored support.
#[test]
fn the_support_is_recovered_at_a_sample() {
    let table = directions::build(2).expect("2D table");
    let square: Vec<f64> = table
        .iter()
        .map(|psi| psi[0].abs() + psi[1].abs())
        .collect();
    for index in [0, 17, 55, 109] {
        let recovered = support_at(&table, &square, &table[index]);
        assert!((recovered - square[index]).abs() < 1e-9, "{index}: {recovered}");
    }
}

/// The circumradius bound covers the set it is computed from.
#[test]
fn the_circumradius_bound_covers_the_square() {
    let table = directions::build(2).expect("2D table");
    let square: Vec<f64> = table
        .iter()
        .map(|psi| psi[0].abs() + psi[1].abs())
        .collect();
    let radius = circumradius_bound(&table, &square);
    let largest = square.iter().copied().fold(0.0_f64, f64::max);
    assert!(radius >= largest - 1e-9, "{radius} vs {largest}");
    assert!(radius < 1.02 * largest, "{radius} is a bound, not a blow-up");
}

/// The allowance is local, so a dense table gets a small one and the 3D table a large one.
#[test]
fn the_allowance_grows_with_the_local_gap() {
    let table_2d = directions::build(2).expect("2D table");
    let square: Vec<f64> = table_2d
        .iter()
        .map(|psi| psi[0].abs() + psi[1].abs())
        .collect();
    let flat = max_allowance(&table_2d, &square);
    assert!(flat > 0.0 && flat < 0.05, "2D allowance {flat}");

    let table_3d = directions::build(3).expect("3D table");
    let octahedron: Vec<f64> = table_3d
        .iter()
        .map(|psi| psi[0].abs().max(psi[1].abs()).max(psi[2].abs()))
        .collect();
    let solid = max_allowance(&table_3d, &octahedron);
    assert!(solid > 4.0 * flat, "3D {solid} should dwarf 2D {flat}");
    assert!(solid.is_finite(), "3D allowance {solid}");

    let values = allowance(&table_3d, &octahedron);
    assert!(values.iter().all(|value| value.is_finite() && *value >= 0.0));
    assert_eq!(values.len(), table_3d.len());
}

/// A pair whose two planes do not straddle the probe leaves the set unbounded along it, so
/// `pair_extent` projects that pair's vertex and calls it an extent. On a unit square at `45°`
/// the pairs that do not straddle project to `0`, and a minimum over them is not a bound.
#[test]
fn a_non_straddling_pair_is_not_a_bound() {
    let table = vec![
        vec![1.0, 0.0],
        vec![0.0, 1.0],
        vec![-1.0, 0.0],
        vec![0.0, -1.0],
    ];
    let square = vec![1.0, 1.0, 1.0, 1.0];
    let half = std::f64::consts::FRAC_1_SQRT_2;
    let bound = support_at(&table, &square, &[half, half]);
    assert!(
        (bound - std::f64::consts::SQRT_2).abs() < 1e-12,
        "support at 45 degrees is {bound}, the square's is {}",
        std::f64::consts::SQRT_2
    );
}

/// In 3D a generic probe is in no pair's positive cone: two planes bound nothing there, and the
/// honest answer is `INFINITY` rather than the small number a vertex projection gives.
#[test]
fn a_generic_three_d_probe_has_no_pair_bound() {
    let table = directions::build(3).expect("3D table");
    let octahedron: Vec<f64> = table
        .iter()
        .map(|psi| psi[0].abs().max(psi[1].abs()).max(psi[2].abs()))
        .collect();
    let raw = [0.3_f64, 0.5, 0.81];
    let norm = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt();
    let psi = [raw[0] / norm, raw[1] / norm, raw[2] / norm];
    assert!(
        support_at(&table, &octahedron, &psi).is_infinite(),
        "a generic 3D probe got a finite pair bound"
    );
    // At a sample the pair plane is exact, so two planes do bound the support there.
    for index in [0, 7, 55, 109] {
        let recovered = support_at(&table, &octahedron, &table[index]);
        assert!(
            (recovered - octahedron[index]).abs() < 1e-9,
            "{index}: {recovered} vs {}",
            octahedron[index]
        );
    }
}

/// A straddling triple is exact at a sampled direction, and a measurable fraction of probes has
/// none — which is the `11-16%` the entries quote.
#[test]
fn a_triple_is_tight_on_a_sample_and_absent_elsewhere() {
    let table = directions::build(3).expect("3D table");
    let cube: Vec<f64> = table
        .iter()
        .map(|psi| 1.5 * (psi[0].abs() + psi[1].abs() + psi[2].abs()))
        .collect();
    for index in [0, 37, 109] {
        let value = triple_bound(&table, &cube, &table[index]);
        assert!(value.is_finite(), "{index} has no triple");
        assert!(
            (value - cube[index]).abs() < 1e-9,
            "{index}: {value} vs {}",
            cube[index]
        );
    }

    let mut state = 0x9E37_79B9_7F4A_7C15_u64;
    let mut uniform = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 11) as f64 / (1_u64 << 53) as f64 * 2.0 - 1.0
    };
    let mut missing = 0;
    let mut total = 0;
    while total < 2000 {
        let raw = [uniform(), uniform(), uniform()];
        let norm = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt();
        if !(0.2..=1.0).contains(&norm) {
            continue;
        }
        let psi = [raw[0] / norm, raw[1] / norm, raw[2] / norm];
        if !triple_bound(&table, &cube, &psi).is_finite() {
            missing += 1;
        }
        total += 1;
    }
    assert!(missing > 0, "no probe lacked a triple");
    assert!(missing < total / 2, "{missing} of {total} lacked a triple");
    println!("no triple at {missing} of {total} probes");
}

/// Where a triple exists it bounds the reconstruction itself, not just the exact body.
#[test]
fn the_triple_bounds_the_reconstruction() {
    let table = directions::build_3d(6, 5).expect("6 x 5");
    let cube: Vec<f64> = table
        .iter()
        .map(|psi| 1.5 * (psi[0].abs() + psi[1].abs() + psi[2].abs()))
        .collect();
    let vertices = convex_polygon::reconstruct(&table, &cube);
    let mut checked = 0;
    for first in 0..table.len() {
        for second in (first + 1)..table.len() {
            let raw = [
                table[first][0] + table[second][0],
                table[first][1] + table[second][1],
                table[first][2] + table[second][2],
            ];
            let norm = (raw[0] * raw[0] + raw[1] * raw[1] + raw[2] * raw[2]).sqrt();
            if norm < 1e-9 {
                continue;
            }
            let psi = [raw[0] / norm, raw[1] / norm, raw[2] / norm];
            let bound = triple_bound(&table, &cube, &psi);
            if !bound.is_finite() {
                continue;
            }
            let reach = vertices
                .iter()
                .map(|vertex| vertex[0] * psi[0] + vertex[1] * psi[1] + vertex[2] * psi[2])
                .fold(f64::NEG_INFINITY, f64::max);
            assert!(
                reach <= bound + 1e-9,
                "reach {reach} above the triple {bound}"
            );
            checked += 1;
        }
    }
    assert!(checked > 100, "only {checked} probes had a triple");
}

/// A vertex defined only by a degenerate triple (its plane through the origin) is the antipode of
/// the centre the sign convention keeps. Four equatorial directions plus one pole put such a
/// vertex at the south pole, `90°` from each equatorial site; one-sided credit returns the half
/// gap `45°` instead, so the radius would be half the truth.
#[test]
fn a_degenerate_vertex_is_found_at_its_antipode() {
    let table = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![-1.0, 0.0, 0.0],
        vec![0.0, -1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    let radius = covering_radius(&table);
    for (index, value) in radius.iter().enumerate().take(4) {
        assert!(
            (value - std::f64::consts::FRAC_PI_2).abs() < 1e-9,
            "equator site {index}: {value} vs the south pole at half pi"
        );
    }
}
