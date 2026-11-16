import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { ConvexGeometry } from "three/examples/jsm/geometries/ConvexGeometry.js";
import { bounds, parseOutput } from "./parse.js";
import {
  DIRECTION_COUNT,
  axisGaps,
  convexHull2D,
  facetReport,
  selectPlanes,
  supportPlanes,
  unitDirections,
} from "./directions.js";

const LINE_COLOR = 0x5ee0ff;
const POINT_COLOR = 0xffffff;
const SURFACE_COLOR = 0x3fb6d8;
const FACET_COLOR = 0xf2c14e;
const OTHER_PLANE_COLOR = 0x4a5b6c;
const DEGENERATE_FACET_COLOR = 0x6b7684;
const GHOST_COLOR = 0x4c7dff;
/// How long a previous outline stays on screen while fading, in milliseconds.
const GHOST_LIFE_MS = 1100;
/// Containers tried in order when recording: MP4 where the browser can mux it, WebM elsewhere.
const RECORD_TYPES = [
  "video/mp4;codecs=avc1.42E01E",
  "video/mp4",
  "video/webm;codecs=vp9",
  "video/webm;codecs=vp8",
  "video/webm",
];

/// What the page shows when it is served and no `?src=` is given.
const DEFAULT_SOURCE = "../examples/out/example-1.out.txt";
/// Where the engine server listens unless `?server=` names another address.
const DEFAULT_SERVER = "http://127.0.0.1:8787";

const ui = {
  host: document.getElementById("canvas-host"),
  file: document.getElementById("file"),
  example: document.getElementById("example"),
  drop: document.getElementById("drop"),
  slider: document.getElementById("block"),
  blockCount: document.getElementById("block-count"),
  ticks: document.getElementById("ticks"),
  prev: document.getElementById("prev"),
  next: document.getElementById("next"),
  play: document.getElementById("play"),
  reset: document.getElementById("reset"),
  line: document.getElementById("show-line"),
  points: document.getElementById("show-points"),
  surface: document.getElementById("show-surface"),
  planes: document.getElementById("show-planes"),
  facetOnly: document.getElementById("facet-only"),
  facetColors: document.getElementById("facet-colors"),
  info: document.getElementById("info"),
  status: document.getElementById("status"),
  hint: document.getElementById("hint"),
  record: document.getElementById("record"),
  invalidate: document.getElementById("invalidate"),
  server: document.getElementById("server"),
  progress: document.getElementById("progress"),
  progressFill: document.getElementById("progress-fill"),
  progressText: document.getElementById("progress-text"),
};

const state = {
  blocks: [],
  index: 0,
  playing: false,
  timer: null,
  show: { line: true, points: true, surface: true, planes: true },
  facetOnly: true,
  facetColors: true,
  facets: null,
  ghosts: [],
  recorder: null,
  recordStarted: 0,
  recordBlock: 1,
  recordName: "",
  recordTimer: null,
  shownIndex: null,
  stepSize: null,
  terminalTime: null,
  radius: 1,
  name: "",
};

let renderer;
try {
  renderer = new THREE.WebGLRenderer({ antialias: true });
} catch (error) {
  setStatus(`WebGL is unavailable in this browser: ${error.message}`, "error");
  throw error;
}
renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
ui.host.appendChild(renderer.domElement);

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x0b0f14);

const camera = new THREE.PerspectiveCamera(45, 1, 0.01, 1000);
camera.position.set(0, 0, 3);

const controls = new OrbitControls(camera, renderer.domElement);
controls.enableDamping = true;
controls.dampingFactor = 0.08;

const hemi = new THREE.HemisphereLight(0xffffff, 0x223344, 1.1);
const key = new THREE.DirectionalLight(0xffffff, 1.4);
scene.add(hemi, key);

const axes = new THREE.AxesHelper(1);
scene.add(axes);

let blockGroup = null;

function resize() {
  const width = ui.host.clientWidth;
  const height = ui.host.clientHeight;
  if (!width || !height) return;
  renderer.setSize(width, height, false);
  renderer.domElement.style.width = "100%";
  renderer.domElement.style.height = "100%";
  camera.aspect = width / height;
  camera.updateProjectionMatrix();
}
new ResizeObserver(resize).observe(ui.host);

function animate(now = performance.now()) {
  requestAnimationFrame(animate);
  controls.update();
  fadeGhosts(now);
  renderer.render(scene, camera);
}

function disposeGroup(group) {
  group.traverse((object) => {
    if (object.geometry) object.geometry.dispose();
    if (object.material) {
      const materials = Array.isArray(object.material) ? object.material : [object.material];
      materials.forEach((material) => material.dispose());
    }
  });
}

/**
 * A fading copy of the shape that was just replaced, so playback reads as motion.
 *
 * The ghost owns its own geometry and materials, so disposing it cannot touch the live block.
 */
function spawnGhost(block) {
  if (!block) return;
  if (block.dim === 3) {
    spawnHullGhost(block);
    return;
  }
  const hull = convexHull2D(block.points);
  if (hull.length < 2) return;

  const group = new THREE.Group();
  const materials = [];

  const outline = new THREE.LineBasicMaterial({
    color: GHOST_COLOR,
    transparent: true,
    opacity: 0.55,
    depthWrite: false,
  });
  outline.userData.baseOpacity = outline.opacity;
  materials.push(outline);
  // Outline only: stacked translucent fills merge into a solid blob, while rings read as a
  // chain of ripples.
  const points = hull.map(([x, y]) => new THREE.Vector3(x, y, 0));
  group.add(new THREE.LineLoop(new THREE.BufferGeometry().setFromPoints(points), outline));

  group.userData.born = performance.now();
  group.userData.materials = materials;
  state.ghosts.push(group);
  scene.add(group);
}

/**
 * A fading cage of the previous block's hull.
 *
 * A 3D block's vertex order is an enumeration rather than an outline, so there is no loop to
 * trace; the hull's edges are the equivalent shape. They are drawn through the live surface
 * (`depthTest: false`) because the bridge usually grows, and the previous, smaller shell would
 * otherwise sit buried inside the current one.
 */
function spawnHullGhost(block) {
  const vectors = toVectors(block.points);
  if (vectors.length < 4) return;

  let edges;
  try {
    // A generous crease angle keeps the cage to facet seams rather than every triangle.
    const hull = new ConvexGeometry(vectors);
    edges = new THREE.EdgesGeometry(hull, 20);
    hull.dispose();
  } catch {
    return;
  }

  const material = new THREE.LineBasicMaterial({
    color: GHOST_COLOR,
    transparent: true,
    opacity: 0.4,
    depthWrite: false,
    depthTest: false,
  });
  material.userData.baseOpacity = material.opacity;

  const group = new THREE.Group();
  group.add(new THREE.LineSegments(edges, material));
  group.userData.born = performance.now();
  group.userData.materials = [material];
  state.ghosts.push(group);
  scene.add(group);
}

/** Ages every ghost, removing the ones that have finished fading. */
function fadeGhosts(now) {
  for (let index = state.ghosts.length - 1; index >= 0; index -= 1) {
    const ghost = state.ghosts[index];
    const age = (now - ghost.userData.born) / GHOST_LIFE_MS;
    if (age >= 1) {
      scene.remove(ghost);
      disposeGroup(ghost);
      state.ghosts.splice(index, 1);
      continue;
    }
    const fade = (1 - age) ** 1.5;
    for (const material of ghost.userData.materials) {
      material.opacity = material.userData.baseOpacity * fade;
    }
  }
}

function clearGhosts() {
  for (const ghost of state.ghosts) {
    scene.remove(ghost);
    disposeGroup(ghost);
  }
  state.ghosts.length = 0;
}

/** The first video container this browser can mux, or `null` when recording is unavailable. */
function recordMimeType() {
  if (typeof MediaRecorder === "undefined") return null;
  return RECORD_TYPES.find((type) => MediaRecorder.isTypeSupported(type)) ?? null;
}

function formatDuration(milliseconds) {
  const seconds = Math.max(0, Math.floor(milliseconds / 1000));
  return `${Math.floor(seconds / 60)}:${String(seconds % 60).padStart(2, "0")}`;
}

function downloadRecording(blob, mimeType, extension) {
  const stem = (state.name || "stable-bridge").replace(/[^\w.-]+/g, "-");
  const url = URL.createObjectURL(blob);
  const link = document.createElement("a");
  link.href = url;
  link.download = `${stem}-block-${state.recordBlock}.${extension}`;
  link.click();
  setTimeout(() => URL.revokeObjectURL(url), 10_000);
}

function tickRecordLabel() {
  const elapsed = performance.now() - state.recordStarted;
  ui.record.textContent = `stop · ${formatDuration(elapsed)}`;
}

/**
 * Records exactly what the canvas draws: the stream comes from the renderer's own canvas, so
 * the sidebar stays out and orbiting or playing during a take is captured as seen.
 */
function startRecording() {
  const mimeType = recordMimeType();
  if (!mimeType || typeof renderer.domElement.captureStream !== "function") {
    setStatus("This browser cannot record the canvas; try Chrome, Edge or Safari.", "error");
    return;
  }

  const stream = renderer.domElement.captureStream(60);
  const chunks = [];
  let recorder;
  try {
    recorder = new MediaRecorder(stream, { mimeType, videoBitsPerSecond: 12_000_000 });
  } catch (error) {
    stream.getTracks().forEach((track) => track.stop());
    setStatus(`The recorder refused ${mimeType}: ${error.message}`, "error");
    return;
  }

  recorder.ondataavailable = (event) => {
    if (event.data && event.data.size > 0) chunks.push(event.data);
  };
  recorder.onstop = () => {
    stream.getTracks().forEach((track) => track.stop());
    window.clearInterval(state.recordTimer);
    state.recordTimer = null;
    state.recorder = null;
    ui.record.textContent = "record";
    delete ui.record.dataset.active;
    ui.record.title = "record the canvas to a video file";

    const extension = mimeType.startsWith("video/mp4") ? "mp4" : "webm";
    downloadRecording(new Blob(chunks, { type: mimeType }), mimeType, extension);
    // Loading another document also stops the take; the new document's status is the one to
    // keep, so the saved-clip note is only written for the document the take belongs to.
    if (state.recordName === state.name) {
      setStatus(
        `recorded ${formatDuration(performance.now() - state.recordStarted)} from block ${state.recordBlock} as .${extension}`,
        "ok",
      );
    }
  };

  state.recorder = recorder;
  state.recordStarted = performance.now();
  state.recordBlock = state.index + 1;
  state.recordName = state.name;
  ui.record.dataset.active = "recording";
  ui.record.title = "stop and save the recording";
  tickRecordLabel();
  state.recordTimer = window.setInterval(tickRecordLabel, 250);
  recorder.start(250);
  setStatus(`recording ${mimeType.split(";")[0]} — orbit and play while it runs`, "ok");
}

function stopRecording() {
  if (!state.recorder) return;
  state.recorder.stop();
}

ui.record.addEventListener("click", () => {
  if (state.recorder) stopRecording();
  else startRecording();
});

if (!recordMimeType() || typeof renderer.domElement.captureStream !== "function") {
  ui.record.disabled = true;
  ui.record.title = "this browser cannot record canvas video";
}

function toVectors(points) {
  return points.map((point) => new THREE.Vector3(point[0], point[1], point[2] ?? 0));
}

function blockDim(blocks) {
  return blocks.length > 0 ? blocks[0].dim : 0;
}

/** Vertices needed before a plane counts as carrying a facet rather than a corner. */
function facetThreshold(dim) {
  return dim === 2 ? 2 : 3;
}

/**
 * Measures the 110 directions' bounds against one block. The direction table is a constant
 * of the algorithm, so this needs nothing but the block's own vertices.
 */
function planesFor(block) {
  const directions = unitDirections(block.dim);
  if (directions.length === 0) return null;

  const planes = supportPlanes(block.points, directions);
  const threshold = facetThreshold(block.dim);
  const facets = planes.filter((plane) => plane.touching >= threshold).length;
  const { selected, fellBack } = selectPlanes(planes, threshold);
  return { directions, planes, threshold, facets, selected, fellBack };
}

function buildPlanes(info, dim) {
  const group = new THREE.Group();
  const half = state.radius * 1.6;

  const drawn = state.facetOnly ? info.selected : info.planes;
  for (const plane of drawn) {
    const isFacet = plane.touching >= info.threshold;

    const normal = new THREE.Vector3(
      plane.psi[0],
      plane.psi[1],
      dim === 3 ? plane.psi[2] : 0,
    );
    const center = normal.clone().multiplyScalar(plane.offset);
    const material = new THREE.LineBasicMaterial({
      color: isFacet ? FACET_COLOR : OTHER_PLANE_COLOR,
      transparent: true,
      opacity: isFacet ? 0.9 : 0.25,
    });

    if (dim === 2) {
      // A 2D half-space bounds along a line, perpendicular to the normal in the xy-plane.
      const along = new THREE.Vector3(-normal.y, normal.x, 0);
      const geometry = new THREE.BufferGeometry().setFromPoints([
        center.clone().addScaledVector(along, half),
        center.clone().addScaledVector(along, -half),
      ]);
      group.add(new THREE.Line(geometry, material));
      continue;
    }

    const reference =
      Math.abs(normal.x) < 0.9 ? new THREE.Vector3(1, 0, 0) : new THREE.Vector3(0, 1, 0);
    const tangent = new THREE.Vector3().crossVectors(normal, reference).normalize();
    const binormal = new THREE.Vector3().crossVectors(normal, tangent).normalize();
    const corners = [
      center.clone().addScaledVector(tangent, half).addScaledVector(binormal, half),
      center.clone().addScaledVector(tangent, -half).addScaledVector(binormal, half),
      center.clone().addScaledVector(tangent, -half).addScaledVector(binormal, -half),
      center.clone().addScaledVector(tangent, half).addScaledVector(binormal, -half),
    ];
    const geometry = new THREE.BufferGeometry().setFromPoints(corners);
    group.add(new THREE.LineLoop(geometry, material));
  }

  return group;
}

/** Hue of a sampled direction, shared by the facet colors and the sidebar swatches. */
function facetHue(index) {
  return (index * 360) / DIRECTION_COUNT;
}

function facetColor(index) {
  return new THREE.Color().setHSL(facetHue(index) / 360, 0.62, 0.56);
}

/**
 * Colors every hull triangle by the sampled direction closest to its outward normal.
 *
 * This is what makes the face normals legible: faces bounded by the same direction share a
 * hue, and the sidebar reports how far each one leans away from it. Normals are oriented
 * outwards from the block's center so the mesh winding does not matter.
 */
function colorHull(geometry, directions, center) {
  const position = geometry.getAttribute("position");
  if (!position || position.count % 3 !== 0) return null;

  const triangles = [];
  for (let vertex = 0; vertex < position.count; vertex += 3) {
    const triangle = [];
    for (let corner = 0; corner < 3; corner += 1) {
      const i = vertex + corner;
      triangle.push([position.getX(i), position.getY(i), position.getZ(i)]);
    }
    triangles.push(triangle);
  }

  const report = facetReport(triangles, directions, center);
  const colors = new Float32Array(position.count * 3);
  const degenerate = new THREE.Color(DEGENERATE_FACET_COLOR);

  for (let vertex = 0; vertex < position.count; vertex += 1) {
    const facet = report.facets[Math.floor(vertex / 3)];
    const color = facet ? facetColor(facet.nearestIndex) : degenerate;
    colors[vertex * 3] = color.r;
    colors[vertex * 3 + 1] = color.g;
    colors[vertex * 3 + 2] = color.b;
  }

  geometry.setAttribute("color", new THREE.BufferAttribute(colors, 3));
  return report;
}

function surfaceFor(block, vectors) {
  if (block.dim === 2) {
    // The vertex order is an enumeration, so the outline is the hull, not file order.
    const hull = convexHull2D(block.points);
    if (hull.length < 3) return null;
    const outline = new THREE.Shape(hull.map(([x, y]) => new THREE.Vector2(x, y)));
    const geometry = new THREE.ShapeGeometry(outline);
    return new THREE.Mesh(
      geometry,
      new THREE.MeshBasicMaterial({
        color: SURFACE_COLOR,
        transparent: true,
        opacity: 0.14,
        side: THREE.DoubleSide,
        depthWrite: false,
      }),
    );
  }

  if (vectors.length < 4) return null;
  try {
    const geometry = new ConvexGeometry(vectors);

    let report = null;
    if (state.facetColors) {
      const directions = unitDirections(block.dim);
      if (directions.length > 0) {
        const box = bounds(block.points);
        const center = [0, 1, 2].map((axis) => (box.min[axis] + box.max[axis]) / 2);
        report = colorHull(geometry, directions, center);
      }
    }

    const mesh = new THREE.Mesh(
      geometry,
      new THREE.MeshStandardMaterial({
        color: state.facetColors ? 0xffffff : SURFACE_COLOR,
        vertexColors: report !== null,
        transparent: true,
        opacity: state.facetColors ? 0.55 : 0.34,
        side: THREE.DoubleSide,
        roughness: 0.65,
        metalness: 0.05,
        flatShading: true,
      }),
    );

    if (report) mesh.userData.facetReport = report;
    return mesh;
  } catch {
    // Coplanar or otherwise degenerate blocks have no hull to draw; the outline still shows.
    return null;
  }
}

function buildBlock(block, info) {
  if (blockGroup) {
    scene.remove(blockGroup);
    disposeGroup(blockGroup);
    blockGroup = null;
  }

  const vectors = toVectors(block.points);
  blockGroup = new THREE.Group();
  state.facets = null;

  if (state.show.planes && info) {
    blockGroup.add(buildPlanes(info, block.dim));
  }

  if (state.show.surface) {
    const surface = surfaceFor(block, vectors);
    if (surface) {
      state.facets = surface.userData.facetReport ?? null;
      blockGroup.add(surface);
    }
  }

  if (state.show.line && vectors.length >= 2) {
    // In 2D the hull is the shell; in 3D the raw enumeration is kept as a diagnostic.
    const hull = block.dim === 2 ? convexHull2D(block.points) : [];
    const loop =
      hull.length >= 2 ? hull.map(([x, y]) => new THREE.Vector3(x, y, 0)) : vectors;
    const geometry = new THREE.BufferGeometry().setFromPoints(loop);
    blockGroup.add(new THREE.LineLoop(geometry, new THREE.LineBasicMaterial({ color: LINE_COLOR })));
  }

  if (state.show.points) {
    const geometry = new THREE.BufferGeometry().setFromPoints(vectors);
    blockGroup.add(
      new THREE.Points(
        geometry,
        new THREE.PointsMaterial({
          color: POINT_COLOR,
          size: Math.max(state.radius * 0.016, 1e-6),
          sizeAttenuation: true,
        }),
      ),
    );
  }

  scene.add(blockGroup);
}

function fitView(blocks) {
  const all = blocks.flatMap((block) => block.points);
  const dim = blockDim(blocks);
  const box = bounds(all);
  const center = new THREE.Vector3(
    (box.min[0] + box.max[0]) / 2,
    (box.min[1] + box.max[1]) / 2,
    dim === 3 ? (box.min[2] + box.max[2]) / 2 : 0,
  );
  const size = new THREE.Vector3(
    box.max[0] - box.min[0],
    box.max[1] - box.min[1],
    dim === 3 ? box.max[2] - box.min[2] : 0,
  );
  const radius = Math.max(size.length() / 2, 1e-3);
  state.radius = radius;

  const direction =
    dim === 2 ? new THREE.Vector3(0, 0, 1) : new THREE.Vector3(0.8, 0.5, 0.95).normalize();

  camera.position.copy(center).addScaledVector(direction, radius * 2.7);
  camera.near = radius * 0.01;
  camera.far = radius * 200;
  camera.updateProjectionMatrix();
  camera.lookAt(center);

  controls.target.copy(center);
  controls.update();

  key.position.copy(center).add(new THREE.Vector3(1, 1.4, 0.8).multiplyScalar(radius * 3));
  axes.scale.setScalar(radius * 1.1);
}

/** Time of a block: the block line states it, with `t = k · Step` as the fallback. */
function blockTime(block, index) {
  if (block && block.meta && Number.isFinite(block.meta.time)) return block.meta.time;
  return Number.isFinite(state.stepSize) ? (index + 1) * state.stepSize : index + 1;
}

function formatNumber(value) {
  if (!Number.isFinite(value)) return "?";
  return Number(value.toFixed(4)).toString();
}

function renderTicks(blocks) {
  ui.ticks.replaceChildren();
  const count = blocks.length;
  if (count < 1) return;

  const marks = [...new Set([1, Math.max(1, Math.ceil(count / 2)), count])].sort((a, b) => a - b);
  for (const k of marks) {
    const span = document.createElement("span");
    span.style.left = `${count === 1 ? 50 : ((k - 1) / (count - 1)) * 100}%`;
    span.textContent = formatNumber(blockTime(blocks[k - 1], k - 1));
    ui.ticks.appendChild(span);
  }
}

function describe(block, index, total, info) {
  const box = bounds(block.points);
  const time = blockTime(block, index);
  const rows = [
    `<div><span class="key">block</span> ${index + 1} / ${total}</div>`,
    `<div><span class="key">t</span> ${formatNumber(time)} <span class="dim">from the block line</span></div>`,
  ];

  if (Number.isFinite(state.terminalTime)) {
    rows.push(`<div><span class="key">of T</span> ${formatNumber(state.terminalTime)}</div>`);
  }

  rows.push(
    `<div><span class="key">dimension</span> ${block.dim}D</div>`,
    `<div><span class="key">vertices</span> ${block.points.length}</div>`,
    `<div><span class="key">path</span> ${block.points.length >= 2 ? "closed, last → first" : "none"}</div>`,
  );

  const axisText = "xyz"
    .slice(0, block.dim)
    .split("")
    .map((axis, i) => `${axis} [${box.min[i].toFixed(3)}, ${box.max[i].toFixed(3)}]`)
    .join("<br>");
  rows.push(`<div class="axes">${axisText}</div>`);

  if (info) {
    rows.push(
      `<div class="axes"><span class="key">planes</span> ${info.directions.length} directions, ${info.facets} bound a facet${info.fellBack ? " — none, so all are drawn" : ""}</div>`,
    );
    const gaps = axisGaps(info.directions)
      .map((gap) => `${gap.axis} ${gap.degrees.toFixed(1)}°`)
      .join(", ");
    rows.push(`<div class="dim">nearest direction to an axis: ${gaps}</div>`);
  }

  if (state.facets) {
    rows.push(
      `<div class="axes"><span class="key">hull facets</span> ${state.facets.counted} triangles, ${state.facets.groups.length} directions</div>`,
    );
    for (const group of state.facets.groups.slice(0, 6)) {
      const spread = group.maxDegrees - group.minDegrees;
      const range =
        spread > 0.05
          ? `${group.minDegrees.toFixed(1)}–${group.maxDegrees.toFixed(1)}°`
          : `${group.minDegrees.toFixed(1)}°`;
      rows.push(
        `<div class="facet"><span class="swatch" style="background:hsl(${facetHue(group.index).toFixed(0)} 62% 56%)"></span>ψ#${group.index} · ${group.count} face${group.count === 1 ? "" : "s"} · ${range}</div>`,
      );
    }
    const hidden = state.facets.groups.length - 6;
    if (hidden > 0) {
      rows.push(`<div class="dim">+ ${hidden} more direction${hidden === 1 ? "" : "s"}</div>`);
    }
    rows.push(
      `<div class="dim">worst normal-to-direction gap ${state.facets.worstDegrees.toFixed(1)}°</div>`,
    );
  }

  ui.info.innerHTML = rows.join("");
}

function setStatus(message, level = "ok") {
  ui.status.textContent = message;
  ui.status.dataset.level = level;
}

function renderBlock() {
  if (state.blocks.length === 0) {
    if (blockGroup) {
      scene.remove(blockGroup);
      disposeGroup(blockGroup);
      blockGroup = null;
    }
    clearGhosts();
    state.shownIndex = null;
    return;
  }

  const index = Math.min(state.index, state.blocks.length - 1);
  const previous = state.shownIndex;
  state.index = index;
  if (state.playing && previous !== null && previous !== index) {
    spawnGhost(state.blocks[previous]);
  }
  state.shownIndex = index;
  const block = state.blocks[index];
  const info = state.show.planes ? planesFor(block) : null;

  ui.slider.value = String(index);
  ui.blockCount.textContent = `${index + 1} / ${state.blocks.length}`;
  // The facet report comes from the rebuilt surface, so the sidebar reads it afterwards.
  buildBlock(block, info);
  describe(block, index, state.blocks.length, info);
  updateHint(block);
}

function updateHint(block) {
  ui.hint.textContent =
    block.dim === 3
      ? "3D: the file order is not a surface — the surface view shows the convex hull of the block."
      : "";
}

function load(text, name) {
  // A take belongs to the document it started on, so switching files saves it rather than
  // silently splicing two runs into one clip.
  stopRecording();
  state.name = name || "";
  const { blocks, issues, meta } = parseOutput(text);
  const errors = issues.filter((issue) => issue.level === "error");
  const warnings = issues.filter((issue) => issue.level === "warning");

  if (errors.length > 0) {
    state.blocks = [];
    renderTicks([]);
    renderBlock();
    const first = errors[0];
    setStatus(`${state.name}: line ${first.line}: ${first.message}`, "error");
    return;
  }

  if (blocks.length === 0) {
    state.blocks = [];
    renderTicks([]);
    renderBlock();
    setStatus(`${state.name}: no shapes found`, "error");
    return;
  }

  state.blocks = blocks;
  state.index = 0;
  state.shownIndex = null;
  clearGhosts();

  // The header states T and Step, so the timeline needs no input file at all.
  state.terminalTime = Number.isFinite(meta.T) ? meta.T : null;
  state.stepSize = Number.isFinite(meta.Step) && meta.Step > 0 ? meta.Step : null;

  fitView(blocks);
  renderTicks(blocks);
  renderBlock();

  const timing = Number.isFinite(meta.Step) ? `, Step ${formatNumber(meta.Step)}` : "";
  const summary = `${state.name}: ${blocks.length} block${blocks.length === 1 ? "" : "s"}, ${blockDim(blocks)}D${timing}`;
  if (warnings.length > 0) {
    setStatus(`${summary} — ${warnings[0].message} (line ${warnings[0].line})`, "warning");
  } else {
    setStatus(summary, "ok");
  }
}

function setPlaying(playing) {
  state.playing = playing;
  ui.play.textContent = playing ? "pause" : "play";
  if (state.timer) {
    clearInterval(state.timer);
    state.timer = null;
  }
  if (playing && state.blocks.length > 1) {
    state.timer = setInterval(() => {
      state.index = (state.index + 1) % state.blocks.length;
      renderBlock();
    }, 180);
  }
}

function step(delta) {
  if (state.blocks.length === 0) return;
  const count = state.blocks.length;
  state.index = (state.index + delta + count) % count;
  renderBlock();
}

async function readOutputFile(file) {
  // A dropped file is not one of the examples, so the quick list goes back to custom.
  ui.example.value = "";
  try {
    const text = await file.text();
    // The same drop target takes both kinds of document: an output is drawn, a problem is sent
    // to the engine server.
    if (isProblemText(text)) computeProblem(text, file.name);
    else load(text, file.name);
  } catch (error) {
    setStatus(`could not read ${file.name}: ${error.message}`, "error");
  }
}

/** Marks the example matching `source` in the quick list, or clears it for a custom path. */
function syncExample(source) {
  const known = [...ui.example.options].some((option) => option.value === source);
  ui.example.value = known ? source : "";
}

/** Loads a document by URL: a bundled example, or whatever `?src=` names. */
function loadSource(source) {
  const name = source.split("/").pop() || source;
  setStatus(`loading ${name}…`, "idle");
  fetch(source)
    .then((response) => {
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      return response.text();
    })
    .then((text) => {
      load(text, name);
      syncExample(source);
    })
    .catch(() => {
      // From file:// a fetch is refused outright, and the examples sit outside the page's
      // directory unless the repository root is what is being served.
      ui.example.value = "";
      setStatus(`could not load ${name} — serve the repository root or drop the file`, "warning");
    });
}

/* ---- compute: the viewer asks the engine server for a run ------------------------------- */

const SERVER = (() => {
  const given = new URLSearchParams(location.search).get("server");
  return (given || DEFAULT_SERVER).replace(/\/$/, "");
})();

function setProgress(stateName, done = 0, total = 0) {
  // Before the first block lands there is no ratio to draw, but the run is alive: the fill spans
  // the track and a gradient band travels across it.
  const sweeping = stateName === "running" && (total <= 0 || done <= 0);
  ui.progress.dataset.state = stateName;
  ui.progress.dataset.indeterminate = sweeping ? "1" : "0";

  // Width is inline, so it outranks the stylesheet: the state has to pick it here, not in CSS.
  let percent = 0;
  if (sweeping || stateName === "done" || stateName === "error") percent = 100;
  else if (total > 0) percent = Math.min(100, (done / total) * 100);
  ui.progressFill.style.width = `${percent.toFixed(1)}%`;
}

function progressNote(text) {
  ui.progressText.textContent = text;
}

function serverState(ready, detail) {
  ui.server.dataset.state = ready ? "ok" : "off";
  ui.server.textContent = ready ? "server ready" : "server offline";
  ui.server.title = detail || `${SERVER} · start it with: cd engine && cargo run --release --bin bridge_server`;
}

async function checkServer() {
  try {
    const response = await fetch(`${SERVER}/api/health`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const body = await response.json();
    serverState(true, `${body.service || "bridge_server"} · ${SERVER} · cache ${body.cache_dir}`);
  } catch {
    serverState(false);
  }
}

/** An input problem, as opposed to a computed output document. */
function isProblemText(text) {
  const head = text.trimStart();
  return !head.startsWith("stable_bridge output") && /^dimension\s+\d+/m.test(head);
}

/** The example list holds both outputs (`*.out.txt`) and problems (`*.txt`). */
function isProblemSource(source) {
  return !/\.out\.[^/]*$/.test(source);
}

function failCompute(name, message) {
  setProgress("error");
  progressNote(`${name}: ${message}`);
  setStatus(`compute failed: ${message}`, "error");
}

async function loadResult(id, name) {
  const response = await fetch(`${SERVER}/api/result?id=${encodeURIComponent(id)}`);
  const text = await response.text();
  if (!response.ok) throw new Error(text.trim() || `HTTP ${response.status}`);
  setProgress("done", 1, 1);
  load(text, `${name} → out`);
}

/**
 * Follows a run to completion, repainting the bar from the server's own block count.
 *
 * Elapsed time is printed on every poll, so the panel keeps moving even while a single block
 * is still being advanced.
 */
async function followJob(id, name) {
  for (;;) {
    const response = await fetch(`${SERVER}/api/progress?id=${encodeURIComponent(id)}`);
    if (!response.ok) throw new Error(`lost the run (HTTP ${response.status})`);
    const job = await response.json();
    setProgress(job.state === "running" ? "running" : job.state, job.blocks, job.total);
    const seconds = (job.elapsed_ms / 1000).toFixed(1);

    if (job.state === "running") {
      const where = job.total > 0 ? `block ${job.blocks} / ${job.total}` : "starting";
      progressNote(`${where} · ${seconds} s · ${job.cache}`);
    }
    if (job.state === "done") {
      await loadResult(id, name);
      progressNote(`computed in ${seconds} s · ${job.cache}`);
      return;
    }
    if (job.state === "error") throw new Error(job.message || "the server failed");
    await new Promise((resolve) => setTimeout(resolve, 200));
  }
}

async function computeProblem(text, name) {
  stopRecording();
  const invalidate = ui.invalidate.checked;
  setProgress("running");
  progressNote(`${name}: ${invalidate ? "invalidating cache, " : ""}sending…`);
  setStatus(`${invalidate ? "invalidating the cache and computing" : "asking the server for"} ${name}…`, "idle");

  try {
    const response = await fetch(`${SERVER}/api/run${invalidate ? "?invalidate=1" : ""}`, {
      method: "POST",
      headers: { "content-type": "text/plain" },
      body: text,
    });
    if (!response.ok) throw new Error(`the server answered HTTP ${response.status}`);
    const job = await response.json();
    progressNote(`${name}: ${job.cache === "hit" ? "cache hit" : "computing"}…`);
    await followJob(job.id, name);
  } catch (error) {
    const offline = error instanceof TypeError;
    failCompute(
      name,
      offline
        ? `no server at ${SERVER} — start it with: cd engine && cargo run --release --bin bridge_server`
        : error.message,
    );
  }
}

/** Fetches one of the tracked problem files, then computes it. */
function loadProblemSource(source) {
  const name = source.split("/").pop() || source;
  setProgress("running");
  progressNote(`fetching ${name}…`);
  fetch(source)
    .then((response) => {
      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      return response.text();
    })
    .then((text) => {
      // The loaded document is the computed output, not the selected input, so the list goes
      // back to custom; that also lets the same problem be picked again — for instance after
      // ticking invalidate — instead of the second selection firing nothing.
      ui.example.value = "";
      computeProblem(text, name);
    })
    .catch((error) => failCompute(name, error.message));
}

ui.file.addEventListener("change", () => {
  const file = ui.file.files && ui.file.files[0];
  if (file) readOutputFile(file);
});

ui.example.addEventListener("change", () => {
  const source = ui.example.value;
  if (!source) return;
  if (isProblemSource(source)) loadProblemSource(source);
  else loadSource(source);
});

["dragenter", "dragover"].forEach((type) => {
  ui.drop.addEventListener(type, (event) => {
    event.preventDefault();
    ui.drop.classList.add("over");
  });
});
["dragleave", "drop"].forEach((type) => {
  ui.drop.addEventListener(type, (event) => {
    event.preventDefault();
    ui.drop.classList.remove("over");
  });
});
ui.drop.addEventListener("drop", (event) => {
  const file = event.dataTransfer && event.dataTransfer.files && event.dataTransfer.files[0];
  if (file) readOutputFile(file);
});

ui.slider.addEventListener("input", () => {
  state.index = Number(ui.slider.value);
  setPlaying(false);
  renderBlock();
});
ui.prev.addEventListener("click", () => step(-1));
ui.next.addEventListener("click", () => step(1));
ui.play.addEventListener("click", () => setPlaying(!state.playing));
ui.reset.addEventListener("click", () => {
  if (state.blocks.length > 0) fitView(state.blocks);
});

[
  ["line", ui.line],
  ["points", ui.points],
  ["surface", ui.surface],
  ["planes", ui.planes],
].forEach(([flag, element]) => {
  element.checked = state.show[flag];
  element.addEventListener("change", () => {
    state.show[flag] = element.checked;
    renderBlock();
  });
});

ui.facetOnly.checked = state.facetOnly;
ui.facetOnly.addEventListener("change", () => {
  state.facetOnly = ui.facetOnly.checked;
  renderBlock();
});

ui.facetColors.checked = state.facetColors;
ui.facetColors.addEventListener("change", () => {
  state.facetColors = ui.facetColors.checked;
  renderBlock();
});

window.addEventListener("keydown", (event) => {
  if (event.target instanceof HTMLInputElement) return;
  if (event.key === "ArrowLeft") step(-1);
  if (event.key === "ArrowRight") step(1);
  if (event.key === " ") {
    event.preventDefault();
    setPlaying(!state.playing);
  }
});

resize();
animate();
checkServer();

// Served over http the page can fetch by itself; from file:// the picker is the way in.
if (location.protocol === "http:" || location.protocol === "https:") {
  loadSource(new URLSearchParams(location.search).get("src") || DEFAULT_SOURCE);
} else {
  setStatus("pick an example or drop a file to begin", "idle");
}
