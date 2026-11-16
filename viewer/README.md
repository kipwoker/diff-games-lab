# Output viewer

The lab's viewer: a small three.js page that draws what an output document describes, so the
geometry can be checked by eye instead of only through assertions.

## Reading the file

A document opens with `stable_bridge output v2` and states `dimension`, `T`, `Step`, `blocks`
and `directions`. Each block is introduced by `block <k> time <t> vertices <n>`, holds one
vertex per line, and is closed by `separator`.

Every numeric line is one vertex: `n` coordinates, two or three of them. The vertices are
printed in an enumeration order rather than around the boundary, and each block is **closed**:
the last vertex joins back to the first. Each block is therefore the shell at one moment in
time, and the slider walks the sequence.

The dimension is not assumed: it comes from the header when the file declares one, otherwise
from the number of fields on the first vertex. So the page draws 2D and 3D runs alike.

## Running it

Serve the repository root and open the page:

```sh
python -m http.server 8000
# then open http://localhost:8000/viewer/
# or name a document: http://localhost:8000/viewer/?src=../examples/out/example-2a.out.txt
```

`npm run dev` serves the same root; the repository's root `index.html` forwards `/` to
`/viewer/`, so the URL it prints opens the viewer rather than a directory listing.

The **example** list loads one of the five committed runs for a quick check; the picker and
the drop zone below it stay for your own output files. Loading an example uses `fetch`, so it
needs a server — opened straight from disk, the page can only read what you drop on it,
because the examples live outside `viewer/`. `dist/app.js` is a prebuilt bundle, so there is
no build step: three.js is compiled in.

## Time

The document states every block's time itself, so the sidebar and the slider ticks need
nothing else: the per-block `time` is used as written, and `T`/`Step` come from the header.
There is no input file to load and no step to type.

## Controls

- drag to orbit, wheel to zoom, right-drag to pan;
- the **example** list loads a committed run; dropping a file switches the list back to
  custom;
- the slider, `◀`/`▶`, and the arrow keys step through blocks; `space` plays;
- while playing, every replaced block stays behind as a blue ghost that fades out, so the
  sequence reads like ripples: the outline in 2D, the hull's edges in 3D, drawn through the
  surface because the previous, smaller shell sits inside the current one;
- **surface** draws the filled shape, **path** the outline, **vertices** the points;
- **half-space planes** draws the sampled half-spaces, and **facet planes only** keeps the
  ones that carry a facet;
- **facet colors** colors each hull triangle by the sampled direction closest to its normal;
- the sidebar reports the block index, time, dimension, vertex count, per-axis extents and
  the facet groups;
- **record** captures the canvas to a video file; press it again to save.

## Computing

The page is also the lab's entry point to the engine. With `bridge_server` running, dropping a
**problem** file — anything that is not an output document — posts its text to the server and
loads the returned run; the `compute from a problem` group in the example list does the same for
the five tracked inputs. The same drop target still takes output documents and draws them.

A run answers from the cache when that exact input has been computed before, so a second load is
instant. Tick **invalidate cache** first to delete that case and force a full recomputation.
While it runs, the bar under the selector shows blocks finished out of the total the server
reports, with elapsed seconds beside it; until the first block is known the bar sweeps, so a slow
start reads as alive rather than stuck.

The server address is `http://127.0.0.1:8787`; `?server=` names another one:

```
http://localhost:8000/viewer/?server=http://127.0.0.1:9000
```

If the probe at load time fails the panel says *server offline* and dropping an input file
reports the same, while output documents keep working without a server.

## Recording

**record** streams the renderer's own canvas, so the panel stays out and the take is the
picture rather than the UI. Orbiting and playback are captured as they happen, which is what a
promo clip wants: pick a block, press record, move the camera while the timeline plays, press
stop. The file downloads as `example-1.out.txt-block-7.mp4`.

The container is MP4 when the browser's `MediaRecorder` can mux it — Chrome, Edge and Safari
can — and WebM otherwise; the status line names the format in use, and the button disables
itself where canvas capture is unavailable. The canvas is recorded at its backing size, up to
twice the CSS size on a high-density display, at 12 Mbit/s.

The page does not write GIF: that needs a palette-quantising encoder the viewer does not ship.
Convert a take instead:

```sh
ffmpeg -i example-1.out.txt-block-7.mp4 \
  -vf "fps=24,scale=960:-1:flags=lanczos,split[a][b];[a]palettegen[p];[b][p]paletteuse" promo.gif
```

## Facet normals

With **surface** and **facet colors** on, every hull triangle is colored by the one of the 110
sampled directions closest to its outward normal (hue = direction index), and the sidebar lists
the groups: `ψ#50 · 2 faces · 0.0°`. A face whose normal is itself sampled gets an exact bound;
the gap shown is how far the face leans away from the nearest direction.

That readout is the quantitative version of the z-axis gap. For a 3D run the x and y faces land
on directions 0.0° away and stay at `±0.5`, while the faces around the poles are 18° off and
the shell has to bulge there. Faces bounded by the same direction share a hue, so a face split
into several triangles reads as one color.

Normals are oriented outwards from the block's bounding-box center, so the mesh winding does not
matter. Degenerate triangles (no area) are drawn in grey and left out of the counts. This is a
3D view: in 2D the filled shape is a single polygon, and its edges are the subject of the
planes view instead.

## What the planes show

The 110 directions are a constant of the algorithm, not data from a file, so for each block the
viewer recomputes each direction's bound as `c_i = max_j ⟨ψ_i, v_j⟩` over that block's own
vertices. That is the tight support of the polytope that was actually printed, so no
reconstruction is re-run.

For a 3D run, around 30 of the 110 planes carry a facet. The interesting readout is the last
line: the nearest sampled direction to an axis. The table reaches the x and y axes exactly but
never comes within 18° of the z poles, which is why the printed shell sits at `±0.5` in x and y
but bulges along z while no plane is available to cap it.

In 2D no sampled plane carries an edge — the printed points are pairwise intersections and the
shell is their hull, so its edges do not lie on sampled lines. The facet filter falls back to
drawing all 110 at low opacity there and the sidebar says so.

## Vertex order

The printed vertices are an enumeration, not a traversal. The page therefore derives each
block's outline from the **convex hull** of its points: in 2D both the fill and the path follow
it, and in 3D the surface is the hull. The raw enumeration is still drawn as the path in 3D,
where it crosses itself like a knot, so turn the path off to look at the surface.

## Development

```sh
npm install
npm test          # parser + direction tests, node's built-in runner
npm run build     # regenerate dist/app.js after editing src/

npm run dev       # or: bun run dev
# rebuilds dist/app.js on every edit and serves the repository root, so the
# page is at http://localhost:8000/viewer/ and its examples are reachable;
# http://localhost:8000/ forwards there
```

`npm run dev` is a plain esbuild command, so it works the same under bun (esbuild is a local
binary; bun only supplies the shell that runs it). It uses `--watch=forever` rather than
`--watch`, because plain `--watch` stops as soon as stdin closes, which happens immediately in
a non-interactive shell.
