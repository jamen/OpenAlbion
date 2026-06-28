# OpenAlbion — Project Guide & Roadmap

*The single source of truth for OpenAlbion: where we are, what the original engine does, and the
work ahead. Read top to bottom and you have the lay of the land. Keep this current — fold new
context in here rather than spawning parallel notes.*

---

## 1. What this project is

OpenAlbion is a **Rust + wgpu** re-creation of the renderer/engine of **Fable: The Lost Chapters**
(Lionhead, 2005), built to load and render the **real retail game assets**. The near-term goal is a
program that loads a level and draws it faithfully — terrain, sky, models, eventually a populated
world — not a faithful binary reproduction of the original `.exe`.

Two external references anchor the work:

- **The decompilation** at `/home/jamen/git/fable-decomp` — a clean C++ reimplementation of the
  original Win32/D3D9 engine, with PDB-derived class layouts (`src/types/*.hpp`, the *spec*) and
  Ghidra-decompiled bodies (`src/engine/**/*.cpp`, the *logic reference*). See its `GUIDE.md`. This
  is our **authoritative reference** for formats, render order, math conventions, and scale.
- **The assets**: retail at `/home/jamen/Fable`, and the **Anniversary debug build** at
  `/home/jamen/doc/Fable_Anniversary-2013-02-25/Fable` — which ships **loose, uncompiled** assets
  (text `.def`/`.tng`/`.wld`, loose `meshes/`, `shaders/`) plus full debug symbols. The debug build
  is our preferred development fixture (see §5).

**How we work** (see also project memory): resurrect/build subsystems incrementally, **validate on
real game data**, and cross-check structure and constants against the decomp rather than guessing.
Direct wgpu, minimal abstraction — the history shows every "engine abstraction" rewrite drew *less*
than the scrappy direct renderer. Extend the renderer; resist the framework rewrite.

---

## 2. Where we are now

**Workspace** (`packages/`):
- `fable-data` — format library: `big` (asset archive), `wad`, `lev` (heightmap + nav graph),
  `mesh` (the "Bbm" 3D model format), `texture` (BC/DXT), `tga` (lighting LUT), `environment` (sky
  themes), `def` (text + binary/compiled-def parsing), `crc32`, `bytes`.
- `fable-def-compiler` — text `.def` → structured defs.
- `fool` — CLI for inspecting assets (`big`, `wad`, `lev`, `mesh`, `texture` subcommands).
- `lzo` — minilzo bindings (the `.big`/`.wad`/mesh decompressor).
- `open-albion` — the winit + wgpu app. Renderer passes: `sky`, `terrain`, `model`, plus `depth`,
  `texture` (shared BCn upload), `camera`, `files`.

**What renders today** (all deliberately rough / experimental):
- **Sky** — outer-sky dome only, with time-of-day texture selection driven per-frame from the
  environment theme. The lighting-colours **LUT is uploaded and bound but not yet sampled** in the
  shader (so time-of-day *colour* is inert). Missing: base band, inner sky (clouds), star field,
  sun, moon, sun/lens flares, screen-space sprites.
- **Landscape** — one flat **untextured** mesh built from the `.lev` heightmap, single directional
  light, **true world scale** from the decomp (file float × 2048.0). The ground-theme + cliff
  texturing data exists in the `.lev` but isn't decoded or rendered.
- **Models** — full mesh: all primitives, per-material textures, opaque / alpha-test / alpha-blend
  modes. Packed-vertex decode (signed-normalized 11/11/10) is fixed and validated across all 3,295
  meshes. Placed at a hardcoded test transform; backface culling off (strip winding unverified);
  skinning/bones not applied.
- **Camera** — free-fly camera (WASD + mouse-look, scroll-wheel speed, Esc to release cursor,
  Enter to re-lock). Near/far planes derived from world extents. Default FOV 70°.

**Known foundational gaps** that cut across everything: a single **world scale** shared by terrain,
models, and sky (the dome is ~7000 units; terrain cells are currently 1 unit); a real **camera**;
**lighting** beyond a hardcoded directional; and **world assembly** (placing many things).

---

## 3. The original engine, scoped

### 3a. Asset & format pipeline

| Format | Role | OpenAlbion status |
|---|---|---|
| `.big` | Main asset archive (graphics, textures, meshes) | ✅ parsed |
| `.wad` | Level archive (`FinalAlbion.wad` bundles all levels) | ✅ parsed |
| `.lev` | Per-level heightmap, ground/sound themes, **nav graph** | ✅ parsed (themes/cliff palette unused) |
| `.tng` | "Things": entities placed in a level (def + transform + components) | ❌ not parsed |
| `.wld` | World: maps/regions, places `.lev`s at `MapX/MapY`, quests | ❌ not parsed |
| `.def` (text) | Definitions — the data model for everything (entities, buildings, atmos, camera, …) | ◻ partial (env themes) |
| CompiledDefs `game.bin` | Retail compiled/binary form of the defs | ◻ partial (env themes) |
| `.bncfg` / `.bba` | Bone config / bone animation | ❌ not parsed |
| textures (BC1/2/3) | DXT-compressed; `dxt_compression` tag → BCn | ✅ |
| shaders `.bbb` | Compiled shader banks (per subsystem) | n/a — we author WGSL |

The `.tng`/`.wld`/`.def` family is one **text key-value tree syntax** (`Field value;`,
`StartX … EndX;`, `NewThing … EndThing;`). Our `def/text` parser is the seed of a shared parser for
all of them.

### 3b. The world model (how a populated map is assembled)

```
.wld  →  Map/Region entries (MapX, MapY, LevelName "X.lev", RegionDef, minimap…)
            └─ .lev   terrain heightmap + themes + nav graph   (the ground)
            └─ .tng   list of Things:
                         DefinitionType "MARKER_…" / "OBJECT_…" / creature / light / …
                         PositionX/Y/Z, RHSetForward/Up (orientation)
                         StartCTC… EndCTC…  (components: physics, editor, atmos, light, …)
            └─ each Thing's DefinitionType  →  a def  →  graphic/mesh appearance
```

So "render a real area" = parse `.wld` → for each map, parse its `.lev` (terrain) and `.tng`
(things), resolve each thing's def to a mesh, and draw it at its transform. This is the user's
"maps (multiple models)" milestone.

### 3c. The render pipeline (authoritative order)

From `EEngineRenderLayer` in the decomp, the engine renders in this order (trimmed to what we'd
reproduce):

1. **Setup lights** → **shadow buffer**
2. **Landscape base** (terrain)
3. **Meshes** (occlusion + no-occlusion) — static props
4. Non-sorted primitives → landscape post-solid
5. **Alpha meshes** (transparent)
6. **Background = sky** (drawn *after* the solid scene, depth-tested at the far plane — matches our
   `clip.z = w*0.9999` trick)
7. **Decals**
8. **Water** (displacement → water → post-water)
9. **Effects / flares**
10. Colour filter → **screen effects** (glow, radial blur, displacement)
11. **2D / UI**

Renderer components in the decomp: `CEngineSkyRenderer`, `CEngineLandscapeRenderer`,
`CEngineWaterRenderer`, `CEngineWeatherRenderer`, `CEngineShadowRenderer`, `CEngineClothRenderer`,
particle simulation, `CEngineScreenEffect*`, `CEngineLightingManager` + `CEngineLocalLightArray`,
and the mesh render-info types below.

### 3d. The three mesh categories

The decomp splits mesh rendering into three `CEngineRenderInfo*` kinds — these define our model work:
- **StaticMesh** — props/buildings. ✅ basically working.
- **RepeatedStaticMesh** — *scatter*: one mesh instanced many times = **flora** (grass/trees). This
  is the `repeating_mesh_reps` field already in our `mesh::Primitive` (`vs_repeated_mesh` shader).
- **AnimatedMesh** — **skinned characters**: bone palette + per-vertex bone weights/indices
  (`vs_pal_skin` shader), driven by `.bncfg` + `.bba` animation. We parse bones but don't skin.

### 3e. Coordinate system & scale (settled — WP-SCALE)

Fable stores level data in a **Z-up** world (`.tng` `RHSetUp ≈ (0,0,1)`, `PositionZ` is vertical).
The renderer converts to **Y-up** at asset-upload time to match wgpu/glam conventions:
- **Terrain**: height is placed in the Y component: `position = [col * CELL_SIZE, height, row * CELL_SIZE]`.
- **Sky dome**: built with Y=zenith (not Fable's Z=zenith); the `sky_view_projection_matrix`
  corrects the horizon via yaw-only rotation.
- **Models**: will follow the same Y-up convention when placed.

**Horizontal scale:** 1 heightmap cell = 1 world unit (`CELL_SIZE = 1.0`). Evidence: `.tng`
`PositionX/Y` live in `[0, width]` cell units (Blank.lev is 128×128; Things at X≈74.8, Y≈68.8).

**Height scale:** file `.lev` cell Height is a normalised float multiplied by **2048.0** to produce
world-space height. Derived from `CMap::LoadFromFile` in the decomp: the constant
`___real_40a0000000000000 = 2048.0` is the scale factor and also the clipping ceiling in
`CHeightMap::SetSizeZAt`. Validated empirically: LookoutPoint heights 0.013–0.027 → world Z
26.6–55.3, matching Thing Z values (27–42).

---

## 4. Assets on disk

- **Retail** `/home/jamen/Fable/data`: `graphics/graphics.big`, `graphics/pc/textures.big`,
  `Levels/FinalAlbion.wad` (all levels, packed), `CompiledDefs/` (`game.bin` + `names.bin`),
  `Defs/` (some text), `Bones/` (`.bncfg`), `LightingTable/lighting_colours.tga`, `shaders/`,
  `EngineCache/`, `Misc/`, `Sound/`, `Tattoos/`.
- **Debug build** `/home/jamen/doc/Fable_Anniversary-2013-02-25/Fable/Data`: the same tree **plus
  loose, uncompiled assets** — `Levels/**/*.{lev,tng,wld}` (per-level, not in a WAD),
  `Defs/*.def` + `*.h`/`*.tpl` (human-readable definition source), loose `meshes/`, `shaders/`. Plus
  `Ego_d.exe`/`Ego_d.pdb`, `ego_r.exe`/`Ego_r.pdb`, and `main.cpp` at the root.

**Dev recommendation:** prefer the **debug build's loose assets** while building each subsystem —
text defs and unpacked `.lev`/`.tng`/`.wld` are far easier to parse and diff than the retail WAD +
compiled defs. Validate against **retail** before calling something done.

---

## 5. Reference map (where to look for each subsystem)

| Subsystem | Decomp reference | Asset / data |
|---|---|---|
| Sky | `engine/core/Engine_Sky_Renderer.reference.cpp`, `CSkyDef.hpp` | `Defs/atmos*.def`, `LightingTable/`, `GBANK_MAIN_PC` sky textures |
| Landscape | `engine/core/Engine_Landscape_*.cpp`, `world/height_map.cpp`, `CHeightMapCell.hpp`, `CTVertexLandscapeForegroundBase.hpp` | `.lev` themes + the skipped palette block; ground/cliff textures |
| Models | `bbblibrary/lib_mesh_data_bank.*`, `lib_3d_mesh*.cpp`, `C3DMeshMaterial.hpp` | `graphics.big` meshes; debug `meshes/` |
| World / Things | `world/world_map.cpp`, `world/map.cpp`, `CWorld*`, `CThing*` | `.wld` + `.tng` + defs |
| Defs | `bbblibrary/lib_definition*.cpp`, `CDefinitionManager.hpp` | `Defs/*.def`, `CompiledDefs/` |
| Lighting | `CEngineLightingManager`, `CShaderGenericLight`, `CTCLight` | `lighting_colours.tga`, per-cell + `.tng` lights |
| Camera | `engine/camera/*`, `CCamera` | `Defs/camera_mode*.def` |
| Animation | `engine/animation/*`, `CEngineRenderInfoAnimatedMesh` | `Bones/*.bncfg`, `.bba` |
| Water / Weather | `CEngineWaterRenderer`, `CEngineWeatherRenderer`, `CWaterSeaGenerator` | level `IsSea`, atmos |

`fool` is the fast path for poking at data (`fool mesh info`, `fool lev info`, …); extend it with new
subcommands as formats come online.

---

## 6. Roadmap — work packages

Each package lists scope, inputs, and dependencies. Packages marked **‖ parallel** are largely
independent and suit splitting across people/agents; **→ after X** means it needs X first.

### Tier 0 — foundations ✅ DONE
- **WP-SCALE — World coordinate system & scale.** ✅ Adopted Y-up at render time; derived real
  height scale = 2048.0 from decomp `CMap::LoadFromFile` (`___real_40a0000000000000`). Replaced the
  `HEIGHT_SCALE=500` placeholder. Documented convention in §3e. *Unblocks framing, camera, world assembly.*
- **WP-CAM — Real camera.** ✅ Free-fly camera (WASD + mouse-look, scroll speed, Esc/Enter for
  cursor lock). Near/far from world extents (not radius hack). Default FOV 70° (matches game
  template). Sky view-projection stays horizon-locked. → done.

### Tier 1 — the visible world (the core descent)
- **WP-TERRAIN-TEX — Landscape texturing. ‖** Decode the `.lev` heightmap theme palette (the skipped
  33 KB block), resolve ground + **cliff/slope** textures, and build the 3-way blend + cliff shader
  (`CTVertexLandscapeForegroundBase`: theme indices, blend, `CliffLookupU/V`). Biggest visual
  payoff. → after WP-SCALE.
- **WP-MODEL-POLISH — Model correctness. ‖** Fix triangle-strip winding in `mesh::expand_block`,
  then enable `two_sided` culling; depth-sort transparent sub-meshes; honor real placement once
  WP-SCALE lands. Small, self-contained.
- **WP-WORLD — World/map assembly.** Parse `.wld` + `.tng` (extend the `def/text` parser into a
  shared key-value parser), resolve thing defs → meshes, place many models in a level. → after
  WP-MODEL-POLISH, WP-SCALE.

### Tier 2 — populating and lighting
- **WP-FLORA — Repeated/scatter meshes. ‖** Render `repeating_mesh_reps` instanced flora
  (grass/trees) via instancing (`vs_repeated_mesh`). → after WP-MODEL-POLISH.
- **WP-LIGHT — Lighting. ‖** Sample the lighting LUT (finally use it in the sky + as ambient/diffuse
  palette), per-cell terrain lighting, and `.tng`/`CTCLight` local lights. Touches every shader.
- **WP-SKY2 — Sky build-out. ‖** Base band → star field → sun/moon → clouds (inner sky) → flares,
  per `RenderSky` order in the decomp. Make the LUT actually drive sky colour. Incremental.

### Tier 3 — characters and depth
- **WP-ANIM — Skinned characters + animation.** Per-vertex bone weights/indices (`vs_pal_skin`),
  bone palette upload, `.bncfg` + `.bba` parsing and playback. Largest single subsystem. → after
  WP-WORLD.
- **WP-WATER — Water/sea. ‖** `CEngineWaterRenderer` / `CWaterSeaGenerator`; `.lev` `IsSea`.
- **WP-SHADOW — Shadows. ‖** `CEngineShadowRenderer`. → after WP-LIGHT.

### Tier 4 — interface & polish
- **WP-UI — 2D/UI. ‖** Fonts (`.xpr`), frontend defs, the `RENDER_LAYER_2D` pass.
- **WP-FX — Screen effects. ‖** Glow / radial blur / displacement / colour filter / weather.

**Parallelization:** Tier-1 `WP-TERRAIN-TEX`, `WP-MODEL-POLISH`, and Tier-2 `WP-SKY2`/`WP-LIGHT` are
mostly independent and can run concurrently. `WP-WORLD` and `WP-ANIM` are the serialization points
that pull the others together.

---

## 7. Milestones

- **M1 — Coherent single map.** Real scale + camera; textured terrain; correctly placed, correctly
  scaled models. (WP-SCALE, WP-CAM, WP-TERRAIN-TEX, WP-MODEL-POLISH)
- **M2 — A populated level.** `.tng` things placed and drawn over the terrain, with flora and
  baseline lighting. (WP-WORLD, WP-FLORA, WP-LIGHT)
- **M3 — A believable area.** Upgraded sky, water, shadows; the area reads as "Albion." (WP-SKY2,
  WP-WATER, WP-SHADOW)
- **M4 — Living world.** Animated characters; UI/HUD. (WP-ANIM, WP-UI)

---

## 8. Working conventions

- **Validate on real game data.** Every parser/renderer is checked against actual assets (the
  `fable-data` integration tests scan all of `graphics.big`; keep that discipline). Cross-check
  layouts/constants against the decomp before trusting them.
- **Decomp is the spec for structure, not for code.** Use `src/types` for layouts/offsets and
  `src/engine` bodies for logic/order; reimplement idiomatically in Rust/WGSL.
- **Keep `fool` useful.** New format → new `fool` subcommand for inspection/diffing.
- **Direct wgpu.** Add passes to the existing renderer; don't introduce an engine-abstraction layer.
- **Keep this doc current** — update §2 (state), §6 (packages), §7 (milestones) as work lands.

---

## 9. Open questions / unknowns

- ~~**Heightmap tile world-size & height encoding**~~ — RESOLVED (WP-SCALE). File float × 2048.0 =
  world Z. See §3e.
- **`.lev` theme palette layout** — the two 33,792-byte palette blocks we skip; how theme indices
  map to ground/cliff **texture names/ids** (WP-TERRAIN-TEX).
- **Def → mesh resolution** — how a `.tng` `DefinitionType` resolves through the def system to a
  graphic/mesh asset id (WP-WORLD).
- **Vertex layout details** — the trailing bytes on larger `vertex_size` meshes, the `init_flags`
  bit-1 extra-attribute and animated bone-weight block (relevant to WP-ANIM).
- **Whether to share one key-value parser** across `def`/`tng`/`wld` (recommended) vs. per-format.
