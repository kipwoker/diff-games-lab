// The 110 sampled directions of the support-function grid.
//
// They are a fixed constant of the method, not data from any file, so a printed block can
// be measured against them after the fact: for direction `psi_i` the block's own vertices
// give `c_i = max_j <psi_i, v_j>`, which is the tight bound of the polytope that was
// actually printed. That is how the viewer draws the half-spaces without re-running the
// reconstruction.
//
// The formulas mirror `engine/src/directions.rs` exactly, including operation order:
//   2D: psi_i = (cos a, sin a),            a = i * 2pi / 110,          i = 0…109
//   3D: psi   = (cos g cos e, cos e sin g, sin e),
//                                          g = i * pi / 10 - pi/2,     i = 0…10
//                                          e = j * 2pi / 10,           j = 0…9
//
// The component order matters: the third component is `sin e`, so `e` sets the elevation in
// rings of constant z while `g` sweeps azimuth. Neither dimension samples its poles.

export const DIRECTION_COUNT = 110;

const AXIS_NAMES = ["x", "y", "z"];

/**
 * The direction table for a state dimension, or an empty list for any other dimension —
 * the original writes nothing in that case.
 * @param {number} dimension
 * @returns {number[][]}
 */
export function unitDirections(dimension) {
  if (dimension === 2) {
    const out = [];
    for (let i = 0; i < DIRECTION_COUNT; i += 1) {
      const angle = (i * (2 * Math.PI)) / 110;
      out.push([Math.cos(angle), Math.sin(angle)]);
    }
    return out;
  }

  if (dimension === 3) {
    const out = [];
    for (let i = 0; i < 11; i += 1) {
      const azimuth = (i * Math.PI) / 10 - Math.PI / 2;
      for (let j = 0; j < 10; j += 1) {
        const elevation = (j * (2 * Math.PI)) / 10;
        out.push([
          Math.cos(azimuth) * Math.cos(elevation),
          Math.cos(elevation) * Math.sin(azimuth),
          Math.sin(elevation),
        ]);
      }
    }
    return out;
  }

  return [];
}

/**
 * Boundary of the convex hull of a 2D point set, counter-clockwise.
 *
 * A block's vertices are printed as an enumeration, not as a traversal: 2D walks the
 * direction table, so consecutive points jump across the shape. The shell is convex, so its
 * outline is exactly the hull of the printed points, and deriving it is the only way to get
 * a shape that does not cross itself.
 *
 * Collinear points are dropped, duplicate points are merged, and a set that cannot enclose
 * an area comes back with fewer than three points.
 *
 * @param {number[][]} points
 * @param {number} [epsilon]
 * @returns {number[][]}
 */
export function convexHull2D(points, epsilon = 1e-12) {
  const unique = [];
  for (const point of points) {
    if (!unique.some((other) => Math.hypot(other[0] - point[0], other[1] - point[1]) <= epsilon)) {
      unique.push([point[0], point[1]]);
    }
  }
  if (unique.length < 3) return unique;

  const sorted = [...unique].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o, a, b) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);

  // Andrew's monotone chain: one pass for the lower edge, one for the upper.
  const half = (sequence) => {
    const chain = [];
    for (const point of sequence) {
      while (chain.length >= 2 && cross(chain[chain.length - 2], chain[chain.length - 1], point) <= epsilon) {
        chain.pop();
      }
      chain.push(point);
    }
    chain.pop();
    return chain;
  };

  const hull = [...half(sorted), ...half([...sorted].reverse())];
  // Collinear input yields the two endpoints, which callers treat as "no polygon".
  return hull.length >= 2 ? hull : unique;
}

/**
 * Bounds each direction's supporting half-space against one block.
 *
 * `touching` counts the vertices that attain the bound, which is what separates a plane
 * that carries a facet (three or more vertices in 3D, two or more in 2D) from one that
 * meets the polytope only at a vertex.
 *
 * @param {number[][]} points
 * @param {number[][]} directions
 * @param {number} [epsilon]
 * @returns {{psi: number[], offset: number, touching: number}[]}
 */
export function supportPlanes(points, directions, epsilon = 1e-8) {
  const dots = points.map(() => 0);

  return directions.map((psi) => {
    let offset = Number.NEGATIVE_INFINITY;
    for (let index = 0; index < points.length; index += 1) {
      const point = points[index];
      let dot = 0;
      for (let axis = 0; axis < psi.length; axis += 1) dot += psi[axis] * point[axis];
      dots[index] = dot;
      if (dot > offset) offset = dot;
    }

    let touching = 0;
    for (const dot of dots) {
      if (Math.abs(dot - offset) <= epsilon) touching += 1;
    }

    return { psi, offset, touching };
  });
}

/**
 * Picks the planes that carry a facet, falling back to all of them when none does.
 *
 * The fallback matters for 2D: a printed 2D block is a hull of pairwise line
 * intersections, so its edges generally do not lie on sampled lines and no direction
 * reaches two vertices. Without the fallback the facet filter would blank the view.
 *
 * @param {{psi: number[], offset: number, touching: number}[]} planes
 * @param {number} threshold
 * @returns {{selected: {psi: number[], offset: number, touching: number}[], fellBack: boolean}}
 */
export function selectPlanes(planes, threshold) {
  const selected = planes.filter((plane) => plane.touching >= threshold);
  if (selected.length > 0) return { selected, fellBack: false };
  return { selected: planes, fellBack: true };
}

/**
 * For each axis, how far the closest sampled direction is from that axis.
 *
 * A zero means the axis is sampled exactly, so a polytope edge lying along it can be
 * bounded tightly; a large angle means nothing bounds that direction directly and the
 * printed shell has to overshoot there.
 *
 * @param {number[][]} directions
 * @returns {{axis: string, degrees: number}[]}
 */
export function axisGaps(directions) {
  const dim = directions.length > 0 ? directions[0].length : 0;
  const gaps = [];

  for (let axis = 0; axis < dim; axis += 1) {
    let best = 0;
    for (const direction of directions) {
      const magnitude = Math.abs(direction[axis]);
      if (magnitude > best) best = magnitude;
    }
    gaps.push({
      axis: AXIS_NAMES[axis] ?? `#${axis}`,
      degrees: (Math.acos(Math.min(1, best)) * 180) / Math.PI,
    });
  }

  return gaps;
}

/**
 * Unit normal of a triangle, or `null` when the triangle has no area.
 *
 * The winding decides the sign; callers that know the solid's center should use
 * [`facetReport`] so every normal is oriented outwards instead of trusting the mesh.
 *
 * @param {number[]} a
 * @param {number[]} b
 * @param {number[]} c
 * @returns {number[]|null}
 */
export function triangleNormal(a, b, c) {
  const ux = b[0] - a[0];
  const uy = b[1] - a[1];
  const uz = b[2] - a[2];
  const vx = c[0] - a[0];
  const vy = c[1] - a[1];
  const vz = c[2] - a[2];

  const nx = uy * vz - uz * vy;
  const ny = uz * vx - ux * vz;
  const nz = ux * vy - uy * vx;
  const length = Math.hypot(nx, ny, nz);
  if (!(length > 1e-12)) return null;

  return [nx / length, ny / length, nz / length];
}

/**
 * The sampled direction closest to a unit normal, with the angle between them.
 *
 * This is the measurement the facet colors stand for: a face whose normal is itself in the
 * table is bounded exactly, while a face pointing at an unsampled direction (the z poles in
 * 3D) has to lean.
 *
 * @param {number[]} normal
 * @param {number[][]} directions
 * @returns {{index: number, psi: number[], degrees: number}}
 */
export function nearestDirection(normal, directions) {
  let bestIndex = -1;
  let bestDot = Number.NEGATIVE_INFINITY;

  for (let index = 0; index < directions.length; index += 1) {
    const psi = directions[index];
    let dot = 0;
    for (let axis = 0; axis < psi.length; axis += 1) dot += psi[axis] * normal[axis];
    if (dot > bestDot) {
      bestDot = dot;
      bestIndex = index;
    }
  }

  const clamped = Math.max(-1, Math.min(1, bestDot));
  return {
    index: bestIndex,
    psi: bestIndex >= 0 ? directions[bestIndex] : [],
    degrees: (Math.acos(clamped) * 180) / Math.PI,
  };
}

/**
 * Measures every hull triangle against the direction table.
 *
 * Normals are flipped to point away from `center` when it is given, so a mesh with
 * inconsistent winding still reports the outward face. Degenerate triangles keep their slot
 * in `facets` as `null` and are left out of the report, so `facets[i]` always describes
 * `triangles[i]`.
 *
 * @param {number[][][]} triangles triples of points
 * @param {number[][]} directions
 * @param {number[]|null} [center] a point inside the solid
 * @returns {{
 *   facets: ({normal: number[], nearestIndex: number, degrees: number}|null)[],
 *   groups: {index: number, psi: number[], count: number, minDegrees: number, maxDegrees: number}[],
 *   counted: number,
 *   worstDegrees: number,
 * }}
 */
export function facetReport(triangles, directions, center = null) {
  const facets = [];
  /** @type {Map<number, {index: number, psi: number[], count: number, minDegrees: number, maxDegrees: number}>} */
  const groups = new Map();
  let counted = 0;
  let worstDegrees = 0;

  for (const triangle of triangles) {
    let normal = triangleNormal(triangle[0], triangle[1], triangle[2]);
    if (normal === null) {
      facets.push(null);
      continue;
    }

    if (center) {
      const face = [0, 1, 2].map(
        (axis) => (triangle[0][axis] + triangle[1][axis] + triangle[2][axis]) / 3 - center[axis],
      );
      const outward = normal[0] * face[0] + normal[1] * face[1] + normal[2] * face[2];
      if (outward < 0) normal = normal.map((value) => -value);
    }

    const nearest = nearestDirection(normal, directions);
    facets.push({ normal, nearestIndex: nearest.index, degrees: nearest.degrees });
    counted += 1;
    if (nearest.degrees > worstDegrees) worstDegrees = nearest.degrees;

    const group = groups.get(nearest.index);
    if (group) {
      group.count += 1;
      if (nearest.degrees < group.minDegrees) group.minDegrees = nearest.degrees;
      if (nearest.degrees > group.maxDegrees) group.maxDegrees = nearest.degrees;
    } else {
      groups.set(nearest.index, {
        index: nearest.index,
        psi: nearest.psi,
        count: 1,
        minDegrees: nearest.degrees,
        maxDegrees: nearest.degrees,
      });
    }
  }

  return {
    facets,
    groups: [...groups.values()].sort(
      (a, b) => b.count - a.count || a.minDegrees - b.minDegrees,
    ),
    counted,
    worstDegrees,
  };
}
