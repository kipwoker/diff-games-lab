// Parser for stable_bridge output documents.
//
// A document opens with a header and gives one meta line per block; each block is a list of
// vertices closed by a `separator` line:
//
//   stable_bridge output v2
//   dimension 2
//   T 2
//   Step 0.2
//   blocks 10
//   directions 110
//
//   block 1 time 0.2 vertices 31
//   0.390574797 0.450000000
//   ...
//   separator
//
// Nothing here assumes `n`: the dimension comes from the header when it is declared,
// otherwise from the first vertex.

const NUMBER = /^[+-]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][+-]?\d+)?$/;
const SEPARATOR = "separator";

/** Header fields of the v2 format: first token -> key in `meta`. */
const HEADER_KEYS = new Map([
  ["dimension", "dimension"],
  ["t", "T"],
  ["step", "Step"],
  ["blocks", "blocks"],
  ["directions", "directions"],
  ["version", "version"],
]);

/**
 * @typedef {{level: 'error'|'warning', line: number, message: string}} Issue
 * @typedef {{index: number|null, time: number|null, vertices: number|null}} BlockMeta
 * @typedef {{dim: number, points: number[][], meta: BlockMeta|null}} Block
 * @typedef {{dimension?: number, T?: number, Step?: number, blocks?: number, directions?: number, version?: number, generator?: string}} OutputMeta
 */

/**
 * @param {string} text
 * @returns {{blocks: Block[], issues: Issue[], meta: OutputMeta}}
 */
export function parseOutput(text) {
  if (typeof text !== "string") {
    throw new TypeError("parseOutput expects a string");
  }

  /** @type {Block[]} */
  const blocks = [];
  /** @type {Issue[]} */
  const issues = [];
  /** @type {OutputMeta} */
  const meta = {};

  /** @type {{points: number[][], meta: BlockMeta|null}|null} */
  let current = null;
  /** @type {number|null} */
  let dim = null;
  let dimFromHeader = false;

  const lines = text.split(/\r\n|\r|\n/);

  for (let index = 0; index < lines.length; index += 1) {
    const raw = lines[index].trim();
    const lineNumber = index + 1;

    if (raw === "") {
      continue;
    }

    if (raw.toLowerCase() === SEPARATOR) {
      if (current === null || current.points.length === 0) {
        issues.push({
          level: "warning",
          line: lineNumber,
          message: "separator closes a block that has no vertices",
        });
      } else {
        const declared = current.meta ? current.meta.vertices : null;
        if (Number.isFinite(declared) && declared !== current.points.length) {
          issues.push({
            level: "warning",
            line: lineNumber,
            message: `block ${current.meta.index ?? "?"} declares ${declared} vertices but has ${current.points.length}`,
          });
        }
        blocks.push({ dim: dim ?? 0, points: current.points, meta: current.meta });
      }
      current = null;
      continue;
    }

    const tokens = raw.split(/\s+/);
    const head = tokens[0].toLowerCase();

    // --- v2 header, before the first block ------------------------------------
    if (
      blocks.length === 0 &&
      current === null &&
      (head === "stable_bridge" || HEADER_KEYS.has(head))
    ) {
      if (head === "stable_bridge") {
        meta.generator = tokens.slice(1).join(" ");
      } else {
        const field = HEADER_KEYS.get(head);
        if (tokens.length !== 2) {
          issues.push({ level: "warning", line: lineNumber, message: `${head}: expected one value` });
        } else {
          const value = Number(tokens[1]);
          if (!Number.isFinite(value)) {
            issues.push({
              level: "error",
              line: lineNumber,
              message: `${head}: expected a number, found ${JSON.stringify(tokens[1])}`,
            });
            return { blocks, issues, meta };
          }
          meta[field] = value;
        }
      }

      if (head === "dimension" && Number.isInteger(meta.dimension) && meta.dimension >= 2) {
        dim = meta.dimension;
        dimFromHeader = true;
      }
      continue;
    }

    // --- v2 block meta line ---------------------------------------------------
    if (head === "block") {
      if (current !== null) {
        issues.push({
          level: "warning",
          line: lineNumber,
          message: `block ${current.meta?.index ?? "?"} was still open and is closed here`,
        });
        blocks.push({ dim: dim ?? 0, points: current.points, meta: current.meta });
        current = null;
      }

      /** @type {BlockMeta} */
      const blockMeta = { index: null, time: null, vertices: null };
      if (tokens.length < 2 || !/^\d+$/.test(tokens[1])) {
        issues.push({
          level: "error",
          line: lineNumber,
          message: "a block line needs a one-based block number",
        });
        return { blocks, issues, meta };
      }
      blockMeta.index = Number(tokens[1]);

      if ((tokens.length - 2) % 2 !== 0) {
        issues.push({
          level: "error",
          line: lineNumber,
          message: "block metadata must come as key value pairs",
        });
        return { blocks, issues, meta };
      }

      for (let field = 2; field < tokens.length; field += 2) {
        const key = tokens[field].toLowerCase();
        const value = Number(tokens[field + 1]);
        if (!Number.isFinite(value)) {
          issues.push({
            level: "error",
            line: lineNumber,
            message: `${key}: expected a number, found ${JSON.stringify(tokens[field + 1])}`,
          });
          return { blocks, issues, meta };
        }
        if (key === "time") blockMeta.time = value;
        else if (key === "vertices") blockMeta.vertices = value;
        else
          issues.push({
            level: "warning",
            line: lineNumber,
            message: `ignored unknown block field ${JSON.stringify(key)}`,
          });
      }

      current = { points: [], meta: blockMeta };
      continue;
    }

    // --- a vertex -------------------------------------------------------------
    /** @type {number[]} */
    const point = [];
    let failed = false;

    for (const token of tokens) {
      if (!NUMBER.test(token)) {
        issues.push({
          level: "error",
          line: lineNumber,
          message: `not a number: ${JSON.stringify(token)}`,
        });
        failed = true;
        break;
      }
      const value = Number(token);
      if (!Number.isFinite(value)) {
        issues.push({
          level: "error",
          line: lineNumber,
          message: `value out of range: ${JSON.stringify(token)}`,
        });
        failed = true;
        break;
      }
      point.push(value);
    }
    if (failed) {
      return { blocks, issues, meta };
    }

    if (dim === null) {
      dim = point.length;
      if (dim < 2) {
        issues.push({
          level: "error",
          line: lineNumber,
          message: `a vertex needs at least two coordinates, found ${dim}`,
        });
        return { blocks, issues, meta };
      }
    }

    if (point.length !== dim) {
      issues.push({
        level: "error",
        line: lineNumber,
        message: dimFromHeader
          ? `expected ${dim} coordinates from the header, found ${point.length}`
          : `expected ${dim} coordinates like the first vertex, found ${point.length}`,
      });
      return { blocks, issues, meta };
    }

    if (current === null) {
      current = { points: [], meta: null };
    }
    current.points.push(point);
  }

  if (current !== null && current.points.length > 0) {
    issues.push({
      level: "warning",
      line: lines.length,
      message: "the last block is not closed by a separator line",
    });
    blocks.push({ dim: dim ?? 0, points: current.points, meta: current.meta });
  }

  if (Number.isFinite(meta.blocks) && blocks.length !== meta.blocks) {
    issues.push({
      level: "warning",
      line: lines.length,
      message: `the header declares ${meta.blocks} blocks, found ${blocks.length}`,
    });
  }

  return { blocks, issues, meta };
}

/**
 * Axis-aligned bounds over one or more point lists.
 * @param {number[][]} points
 * @returns {{min: number[], max: number[]}}
 */
export function bounds(points) {
  const dim = points.length > 0 ? points[0].length : 0;
  const min = new Array(dim).fill(Number.POSITIVE_INFINITY);
  const max = new Array(dim).fill(Number.NEGATIVE_INFINITY);
  for (const point of points) {
    for (let axis = 0; axis < dim; axis += 1) {
      if (point[axis] < min[axis]) min[axis] = point[axis];
      if (point[axis] > max[axis]) max[axis] = point[axis];
    }
  }
  return { min, max };
}
