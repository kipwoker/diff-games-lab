import test from "node:test";
import assert from "node:assert/strict";
import { bounds, parseOutput } from "../src/parse.js";

const V2 = `stable_bridge output v2
dimension 2
T 2
Step 0.2
blocks 2
directions 110

block 1 time 0.2 vertices 2
0.0 1.0
2.0 3.0
separator
block 2 time 0.4 vertices 1
4.0 5.0
separator
`;

test("splits blocks on the separator and keeps vertex order", () => {
  const text = "0.0 1.0\r\n2.0 3.0\r\nseparator\r\n4.0 5.0\r\nseparator\r\n";
  const { blocks, issues } = parseOutput(text);

  assert.deepEqual(issues, []);
  assert.equal(blocks.length, 2);
  assert.deepEqual(blocks[0].points, [
    [0, 1],
    [2, 3],
  ]);
  assert.deepEqual(blocks[1].points, [[4, 5]]);
  assert.equal(blocks[0].dim, 2);
});

test("detects three dimensions from the number of fields", () => {
  const { blocks } = parseOutput("0.5 -0.5 0.25\n1 2 3\nseparator\n");
  assert.equal(blocks.length, 1);
  assert.equal(blocks[0].dim, 3);
  assert.deepEqual(blocks[0].points[1], [1, 2, 3]);
});

test("ignores blank lines and surrounding whitespace", () => {
  const { blocks, issues } = parseOutput("\n  1.0 2.0  \n\nseparator\n\n");
  assert.deepEqual(issues, []);
  assert.deepEqual(blocks[0].points, [[1, 2]]);
});

test("rejects a non-numeric field and reports its line", () => {
  const { blocks, issues } = parseOutput("1.0 2.0\n3.0 four\nseparator\n");
  assert.equal(blocks.length, 0);
  assert.equal(issues.length, 1);
  assert.equal(issues[0].level, "error");
  assert.equal(issues[0].line, 2);
});

test("rejects a vertex with the wrong number of coordinates", () => {
  const { issues } = parseOutput("1.0 2.0\n3.0 4.0 5.0\nseparator\n");
  assert.equal(issues.length, 1);
  assert.equal(issues[0].level, "error");
  assert.match(issues[0].message, /expected 2 coordinates/);
});

test("warns but keeps a block missing its trailing separator", () => {
  const { blocks, issues } = parseOutput("1.0 2.0\n3.0 4.0");
  assert.equal(blocks.length, 1);
  assert.equal(issues.length, 1);
  assert.equal(issues[0].level, "warning");
});

test("warns on a separator with no vertices", () => {
  const { blocks, issues } = parseOutput("separator\n1.0 2.0\nseparator\n");
  assert.equal(blocks.length, 1);
  assert.equal(issues.length, 1);
  assert.equal(issues[0].message, "separator closes a block that has no vertices");
});

test("reads the v2 header and each block's time", () => {
  const { blocks, issues, meta } = parseOutput(V2);

  assert.deepEqual(issues, []);
  assert.equal(meta.dimension, 2);
  assert.equal(meta.T, 2);
  assert.equal(meta.Step, 0.2);
  assert.equal(meta.blocks, 2);
  assert.equal(meta.directions, 110);
  assert.equal(meta.generator, "output v2");

  assert.equal(blocks.length, 2);
  assert.equal(blocks[0].meta.index, 1);
  assert.equal(blocks[0].meta.time, 0.2);
  assert.equal(blocks[0].meta.vertices, 2);
  assert.equal(blocks[1].meta.time, 0.4);
  assert.deepEqual(blocks[1].points, [[4, 5]]);
});

test("a file without a header still parses", () => {
  const { blocks, issues, meta } = parseOutput("1 2\nseparator\n");
  assert.deepEqual(issues, []);
  assert.equal(meta.T, undefined);
  assert.equal(blocks[0].meta, null);
});

test("a header dimension is enforced on the vertex lines", () => {
  const text = "stable_bridge output v2\ndimension 3\nblock 1 time 0.1 vertices 1\n1 2\nseparator\n";
  const { issues } = parseOutput(text);
  assert.equal(issues.length, 1);
  assert.equal(issues[0].level, "error");
  assert.match(issues[0].message, /from the header/);
});

test("warns when a block declares a vertex count it does not have", () => {
  const text = "stable_bridge output v2\nblock 1 time 0.1 vertices 5\n1 2\nseparator\n";
  const { issues } = parseOutput(text);
  assert.equal(issues.length, 1);
  assert.match(issues[0].message, /declares 5 vertices but has 1/);
});

test("warns when the header block count disagrees with the file", () => {
  const text = "stable_bridge output v2\nblocks 3\n1 2\nseparator\n";
  const { issues } = parseOutput(text);
  assert.ok(issues.some((issue) => /declares 3 blocks/.test(issue.message)));
});

test("a block line closes a block that was left open", () => {
  const text = "block 1 time 0.1\n1 2\nblock 2 time 0.2\n3 4\nseparator\n";
  const { blocks, issues } = parseOutput(text);
  assert.equal(blocks.length, 2);
  assert.equal(issues[0].level, "warning");
});

test("bounds cover every axis present", () => {
  const { min, max } = bounds([
    [0, 5, -1],
    [-2, 1, 3],
  ]);
  assert.deepEqual(min, [-2, 1, -1]);
  assert.deepEqual(max, [0, 5, 3]);
});
