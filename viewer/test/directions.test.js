import test from "node:test";
import assert from "node:assert/strict";
import {
  DIRECTION_COUNT,
  axisGaps,
  convexHull2D,
  facetReport,
  nearestDirection,
  selectPlanes,
  supportPlanes,
  triangleNormal,
  unitDirections,
} from "../src/directions.js";

test("110 unit directions in both supported dimensions", () => {
  for (const dim of [2, 3]) {
    const directions = unitDirections(dim);
    assert.equal(directions.length, DIRECTION_COUNT);
    for (const direction of directions) {
      assert.equal(direction.length, dim);
      const norm = Math.hypot(...direction);
      assert.ok(Math.abs(norm - 1) < 1e-12, `norm ${norm} for ${direction}`);
    }
  }
});

test("other dimensions have no table, matching the original's missing branch", () => {
  assert.deepEqual(unitDirections(1), []);
  assert.deepEqual(unitDirections(4), []);
});

test("2D starts at (1, 0) and steps by 2pi/110", () => {
  const directions = unitDirections(2);
  assert.ok(Math.abs(directions[0][0] - 1) < 1e-15);
  assert.ok(Math.abs(directions[0][1]) < 1e-15);
  assert.ok(Math.abs(directions[1][0] - Math.cos((2 * Math.PI) / 110)) < 1e-15);
  assert.ok(Math.abs(directions[1][1] - Math.sin((2 * Math.PI) / 110)) < 1e-15);
});

test("3D samples x and y exactly but leaves the z poles empty", () => {
  const directions = unitDirections(3);
  const gaps = Object.fromEntries(axisGaps(directions).map((gap) => [gap.axis, gap.degrees]));
  assert.ok(gaps.x < 1e-9, `x gap ${gaps.x}`);
  assert.ok(gaps.y < 1e-9, `y gap ${gaps.y}`);
  // sin(72°) = cos(18°), so the closest ring to the pole is 18° away.
  assert.ok(Math.abs(gaps.z - 18) < 1e-9, `z gap ${gaps.z}`);
});

test("2D leaves a small gap on y between sampled angles", () => {
  const gaps = Object.fromEntries(axisGaps(unitDirections(2)).map((gap) => [gap.axis, gap.degrees]));
  assert.ok(gaps.x < 1e-9);
  assert.ok(gaps.y > 1.5 && gaps.y < 1.8, `y gap ${gaps.y}`);
});

test("the hull orders a scrambled point set and drops interior points", () => {
  const hull = convexHull2D([
    [0, 0],
    [1, 0],
    [1, 1],
    [0.5, 0.5],
    [0, 1],
    [0.2, 0.8],
  ]);

  assert.equal(hull.length, 4);
  const area =
    Math.abs(
      hull.reduce((sum, point, index) => {
        const next = hull[(index + 1) % hull.length];
        return sum + point[0] * next[1] - next[0] * point[1];
      }, 0),
    ) / 2;
  assert.ok(Math.abs(area - 1) < 1e-12, `hull area ${area}`);
});

test("a point set with no area has no polygon", () => {
  assert.equal(convexHull2D([[0, 0], [1, 0], [2, 0]]).length, 2);
  assert.equal(convexHull2D([[0, 0], [0, 0], [0, 0]]).length, 1);
});

test("support planes bound a diamond at its vertices and edges", () => {
  const diamond = [
    [1, 0],
    [0, 1],
    [-1, 0],
    [0, -1],
  ];
  const planes = supportPlanes(diamond, [
    [1, 0],
    [Math.SQRT1_2, Math.SQRT1_2],
  ]);

  assert.equal(planes[0].offset, 1);
  assert.equal(planes[0].touching, 1);

  assert.ok(Math.abs(planes[1].offset - Math.SQRT1_2) < 1e-15);
  assert.equal(planes[1].touching, 2);
});

test("facet selection keeps the planes that carry a facet", () => {
  const planes = [
    { psi: [1, 0], offset: 1, touching: 1 },
    { psi: [0, 1], offset: 1, touching: 2 },
  ];
  const { selected, fellBack } = selectPlanes(planes, 2);
  assert.equal(fellBack, false);
  assert.equal(selected.length, 1);
  assert.equal(selected[0].touching, 2);
});

test("facet selection falls back to every plane instead of drawing nothing", () => {
  const planes = [
    { psi: [1, 0], offset: 1, touching: 1 },
    { psi: [0, 1], offset: 1, touching: 1 },
  ];
  const { selected, fellBack } = selectPlanes(planes, 2);
  assert.equal(fellBack, true);
  assert.equal(selected.length, 2);
});

test("support planes count a cube face", () => {
  const cube = [
    [0.5, 0.5, 0.5],
    [-0.5, 0.5, 0.5],
    [0.5, -0.5, 0.5],
    [-0.5, -0.5, 0.5],
    [0.5, 0.5, -0.5],
    [-0.5, 0.5, -0.5],
    [0.5, -0.5, -0.5],
    [-0.5, -0.5, -0.5],
  ];

  const [plane] = supportPlanes(cube, [[0, 0, 1]]);
  assert.equal(plane.offset, 0.5);
  assert.equal(plane.touching, 4);
});

test("triangle normals are unit length and degenerate triangles have none", () => {
  const normal = triangleNormal([0, 0, 0], [1, 0, 0], [0, 1, 0]);
  assert.deepEqual(normal.map((value) => Math.round(value)), [0, 0, 1]);
  assert.ok(Math.abs(Math.hypot(...normal) - 1) < 1e-15);

  assert.equal(triangleNormal([0, 0, 0], [1, 0, 0], [2, 0, 0]), null);
});

test("the nearest direction to an axis is measured in degrees", () => {
  const directions = unitDirections(3);

  const x = nearestDirection([1, 0, 0], directions);
  assert.ok(x.degrees < 1e-9, `x gap ${x.degrees}`);
  assert.equal(x.index, 50, "azimuth index 5, elevation index 0");

  // No ring reaches the z pole, so even the best direction leans 18°.
  const z = nearestDirection([0, 0, 1], directions);
  assert.ok(Math.abs(z.degrees - 18) < 1e-9, `z gap ${z.degrees}`);
});

test("the facet report groups triangles by the direction nearest their normal", () => {
  const directions = unitDirections(3);
  const top = [
    [[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5]],
    [[-0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]],
  ];
  const side = [
    [[0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [0.5, 0.5, 0.5]],
    [[0.5, -0.5, -0.5], [0.5, 0.5, 0.5], [0.5, -0.5, 0.5]],
  ];

  const report = facetReport([...top, ...side], directions, [0, 0, 0]);
  assert.equal(report.counted, 4);
  assert.equal(report.groups.length, 2);
  assert.ok(report.facets.every((facet) => facet !== null));

  // Both top triangles share the nearest direction and the 18° gap.
  assert.equal(report.facets[0].nearestIndex, report.facets[1].nearestIndex);
  assert.ok(Math.abs(report.facets[0].degrees - 18) < 1e-9);

  // The +x face is bounded exactly by direction 50.
  assert.equal(report.facets[2].nearestIndex, 50);
  assert.ok(report.facets[2].degrees < 1e-9);
  assert.ok(Math.abs(report.worstDegrees - 18) < 1e-9);

  const [largest] = report.groups;
  assert.equal(largest.count, 2);
});

test("normals are flipped outwards when a center is given", () => {
  const directions = unitDirections(3);
  const inward = [[[-0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [0.5, -0.5, 0.5]]];
  const outward = [[[-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5]]];

  const flipped = facetReport(inward, directions, [0, 0, 0]);
  const natural = facetReport(outward, directions, [0, 0, 0]);

  assert.equal(flipped.facets[0].nearestIndex, natural.facets[0].nearestIndex);
  assert.ok(flipped.facets[0].normal[2] > 0);
});

test("a degenerate triangle keeps its slot but is left out of the report", () => {
  const report = facetReport(
    [
      [[0, 0, 0], [0, 0, 0], [1, 0, 0]],
      [[0, 0, 0], [1, 0, 0], [0, 1, 0]],
    ],
    unitDirections(3),
  );

  assert.equal(report.facets.length, 2);
  assert.equal(report.facets[0], null);
  assert.equal(report.counted, 1);
  assert.equal(report.groups.length, 1);
});
