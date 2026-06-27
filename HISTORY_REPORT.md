# OpenAlbion — Archaeology Report

*A survey of everything built across the repo's history (2019‑06 → 2026‑06, 284 commits), what survives at `HEAD`, and what's worth resurrecting as spiritual successors.*

The spine of this report follows your stated direction — **sky → landscape → models → world** — and flags, for each subsystem, the high‑water‑mark commit, how complete it got, and whether `HEAD` still has it.

---

## 0. The shape of the project

OpenAlbion is a ~7‑year "kitchen sink" of Fable (2004) / Fable: The Lost Chapters (2005) reverse‑engineering. It has cycled through at least four overlapping ambitions:

1. **Format library** — parse (and sometimes re‑encode) Lionhead's file formats.
2. **Renderer/engine** — load real game assets and draw them (sky, terrain, models).
3. **Live RE / modding tooling** — inject into the running game, read its memory, CLI asset tools, an asset editor GUI.
4. **Documentation** — a book of the formats.

It has also churned through a remarkable number of tech stacks: **Svelte/Electron → Neon → iced → wgpu (raw) → rend3 → egui → tauri → slint → wgpu (raw, again) → GPUI**, and parser styles **hand‑rolled → nom → hand‑rolled (typed kv lexer)**.

### What `HEAD` (main, 205c23c) actually is

A focused, clean **wgpu sky renderer**. Three packages:

- `packages/fable-data` — `big` (744 LOC, the strongest it's ever been), `wad`, `def` (text 981 + binary 278), `environment` (sky themes, 313), `texture` (165, BC decode), `tga` (274, lighting LUT), `crc32`, `bytes`.
- `packages/lzo` — minilzo C bindings (the decompressor used by `.big`/`.wad`).
- `packages/open-albion` — winit + wgpu app that renders Fable's **skybox**: loads `textures.big`, parses `environment.def` themes, drives a day/night cycle (`time_of_day`), uploads the lighting‑colours LUT and sky textures, animates a camera, and runs 5 sky WGSL shaders (`inner_sky`, `outer_sky`, `sky_base_band`, `sky_screen_space_sprite`, `sky_sprite`, `sky_star_field`).

`fable-data/src/lib.rs` carries a tombstone comment listing everything cut: `bba, bbm, bncfg, bwd, gtg, ini, lug, lut, lev, met, qst, save, stb, tng, wld`. That list is essentially the gap.

> **Note on branches:** `origin/big_editor` (diverged 2025‑11‑30 at 137006d, 4 commits ahead) is a *parallel present*, not ancient history. It carries things main lacks today — see §6.

---

## 1. The descent: sky → landscape → models

This is the part most relevant to your current plan, in the order you want to build it.

### 1a. Sky — ✅ present (this is HEAD)
The most polished skybox the project has ever had is the one you have now. Nothing to resurrect here; this is the new foundation.

### 1b. Landscape / terrain — ❌ gone, but built twice before

**Lev (level/heightmap) parser** — the data source for terrain.
- Peak: `cdf52b1` (2024‑09) `lev.rs` **654 LOC**; also strong at `84ca195` (2022, 632) and the original `274d0cf` (2019).
- Decodes: header, **heightmap cells** (height, ground theme triplet + strength, walkable/passover/shore flags, sound theme), **soundmap cells**, and a full **navigation graph** (interactive nodes + a `LevNavigationNode` enum: Regular / Navigation / Exit / Blank / several Unknowns, with the 7‑layer subset structure documented from fabletlcmod.com).
- `HEAD`: **absent** (`// pub mod lev;`).

**Terrain actually on screen — two prior implementations:**
1. **glTF export** (`fable_gltf`, 2019‑11, `274d0cf` "mesh 98% works, edges fucked"). `encode_lev_to_mesh()` turned the Lev heightmap into a triangulated terrain mesh (2 tris/cell, `cell_height_modifier = 2048`) and wrote a full glTF (`gltf_json`: accessors, buffer views, meshes, nodes, scene) for viewing in an external viewer. Worked except edge stitching.
2. **In‑engine landscape pass** (`open_albion/src/renderer/landscape.rs`, 2021‑01, `c52dfa0`, 109 LOC). Built a wgpu heightmap mesh from Lev with a GLSL `shader.vert`/`shader.frag` pipeline, index buffer, back‑face culling. Wiring between `update()` (buffer build) and `draw()` was incomplete (buffers weren't stored into `draw_data`), so it was close but not finished. Same commit also had an in‑engine **console** (`renderer/console.rs`).

> **Resurrection target #1 (your next step):** port the `lev.rs` parser back, then redo the heightmap‑mesh build in your current clean wgpu renderer. You have two reference implementations (2019 glTF math, 2021 in‑engine) to crib from. The hard part historically wasn't drawing it — it was cell adjacency/edge stitching and texturing (ground themes → texture splatting were never done).

### 1c. Models — ❌ gone; this is the peak you remember

**This is "previously I had (some) models rendered."** The high‑water mark is **`2515e14` (2021‑08)**, `src/renderer/scene.rs` (**600 LOC**) + `scene.wgsl`:

- Loaded a model entry from `graphics.big`, decoded it with `fable_data::Model::decode`, decoded its **BC1/BC2 (DXT) textures**, uploaded GPU textures, built per‑primitive vertex/index buffers, and bound a per‑material texture+sampler bind group.
- **Two pipelines**: textured model (`vs_main`/`fs_main`) and **wireframe** (`vs_wire`/`fs_wire`, line‑list, generated wire indices).
- **Arcball camera**, perspective projection, depth buffer, hot model‑swapping via a `selected_model_name` + vector‑clock reload.
- Logged the full decoded model: bounding sphere/box, materials, bones, cloth, helper points, per‑primitive vertex/triangle/index counts, pos bias/scale, vertex size.

**The mesh parser behind it** — `model.rs` (**1093 LOC** at `e3266c7`, 2021‑12) / `bbm.rs` (568–571 LOC at 2022 & 2024). Bbm is Lionhead's "3D Mesh File" (chunked: `3DMF/MTLS/MTRL/SUBM/TRFM/PRIM/TRIS/VERT/BONE/CLTH/HLPR/HDMY/HPNT/HCVL`). The parser understood materials, primitives, bones, cloth, helper points/dummies, bounding volumes — enough to feed the renderer above.

- `HEAD`: both `bbm`/`model` **absent**.

> **Resurrection target #2:** the 2021‑08 model viewer is the richest renderer in the repo and a near drop‑in template for a successor — it already solved DXT decode → material bind group → textured + wireframe draw. Pair it with the `model.rs`/`bbm.rs` decoder.

### 1d. World assembly — ❌ gone
**Tng** (the "Thing" format: every entity/object/light/region placed in a level) and **Wld** (world) are what turn parsed terrain + models into an actual populated map. See §2 — Tng is the single most developed parser in the whole history and is gone.

---

## 2. Format parser census

Lionhead formats, with the best historical implementation and current status. "Script‑like" formats (def/tng/qst/wld/gtg) share a key‑value tree text syntax.

| Format | What it is | Best historical (LOC, commit) | `HEAD` status |
|---|---|---|---|
| **big** | Main asset archive | 744 — **HEAD** | ✅ strongest ever |
| **wad** | Archive (+pack/unpack 2025) | 359 (`84ca195`, 2022) | ✅ present (279) |
| **def** (text) | Definitions (script syntax) | **981+278 — HEAD** | ✅ strongest ever |
| **def_xml** | XML‑schema‑driven def parse | 78 (`e3266c7`, 2021) | ❌ gone (niche experiment) |
| **texture / tga** | BC/DXT textures, TGA LUT | 165+274 — HEAD | ✅ present |
| **environment** | Sky/lighting themes | 313 — HEAD | ✅ new this era |
| **tng** | "Things" placed in world | **4208** (`cdf52b1`, 2024‑09) | ❌ **gone — biggest loss** |
| **lev** | Level heightmap + nav | 654 (`cdf52b1`, 2024) | ❌ gone |
| **bbm / model** | 3D meshes | 1093 model (2021) / 568 bbm | ❌ gone |
| **stb** | String/level table | 444 (`84ca195`, 2022) | ❌ gone |
| **wld** | World | 375 (`84ca195`, 2022) | ❌ gone |
| **save** | Save games | 171 (multiple) | ❌ gone |
| **gtg** | Game tag/graph (script) | 89 (2022/2024) | ❌ gone |
| **met** | Audio metadata | 77 | ❌ gone |
| **bwd** | (level‑adjacent) | 68 | ❌ gone |
| **qst** | Quest scripts | 62 | ❌ gone |
| **bba** | Animation/bones | 43 | ❌ gone |
| **bncfg** | Bone config | 35–37 | ❌ gone |
| **lut** | Audio lookup | 41 | ❌ gone (lighting LUT now via tga) |
| **lug** | Language/text | 12 | ❌ gone |
| **anim / ini / dat / fmp** | misc | small | ❌ gone |

**The crown jewel: the 2024 Tng parser (`cdf52b1`, 4208 LOC).** Built on a reusable typed key‑value lexer `util/kv.rs` (632 LOC) with rich, line‑numbered error types (`InvalidPath`, `InvalidValue`, `UnexpectedField`, `KvPathItem`, `KvValueKind`…). You had previously iterated the Tng parser many times (2019 nom‑ish, 2024‑08 first stage / FSM, then "contemplate PDA instead of FSM", several rewrites) before landing this. It parses sections → things → typed fields rigorously. **This is the most labor that exists anywhere in the repo, and it's commented out at HEAD.**

> **Resurrection target #3:** the `util/kv.rs` typed lexer is the right shared foundation for *all* the script‑syntax formats (def, tng, qst, wld, gtg). Bring `kv.rs` back first, then the parsers ride on it. Your current `def/text.rs` (981 LOC) is a separate, newer hand‑rolled implementation of the same idea — worth reconciling the two rather than maintaining both.

**Round‑trip encoding (modding):** the **2023 slint era (`0801f6f`)** is unique in giving *every* format a matched **`parser.rs` + `compiler.rs`** pair (decode *and* re‑encode), e.g. `format/lev/parse.rs` + `format/lev/compile.rs`. No other era committed to bidirectional round‑tripping. If a successor aims at *modding* (not just viewing), that compiler half is the thing to revive.

---

## 3. Renderer / engine stacks (chronological)

| Era | Stack | What it drew | Notes |
|---|---|---|---|
| 2019‑11 | `gltf_json` export | terrain heightmap → glTF | external viewer; "mesh 98%" |
| 2020‑05 | wgpu + **ember** shader compiler + **iced**; 3D **PGA** experiment | swapchain only | `dad27bc` "try 3D PGA (not continuing)" |
| 2021‑01 (`c52dfa0`) | raw wgpu, GLSL | **landscape** (partial) + console | real draw calls |
| **2021‑08 (`2515e14`)** | raw wgpu, WGSL | **textured models + wireframe** | ⭐ richest renderer ever |
| 2021‑10 (`f5567c0`) | **rend3** | (refactor, RenderRoutine stub) | high‑level engine attempt |
| 2021‑11/12 | rend3 → **egui** + wgpu | egui GUI works; 3D scene pass empty | depth buffer, regressed 3D |
| 2022‑05/07 | dedicated **`renderer` crate** | clears green; pipeline bound, no draw | renderdoc/Vulkan; scaffolding |
| 2023‑05/06 | **slint** `explorer` + `client` + abstracted renderer crate (bind_group/buffer/gpu_buffer/pipeline/texture **atlas**) | abstractions, color.wgsl | most "engine‑architected" |
| 2025‑11→ (HEAD) | raw wgpu + winit | **sky** | current, clean |
| 2025‑11 (big_editor) | **GPUI** | editor UI | parallel branch |

Takeaway: the project has **never had terrain + models + sky on screen at the same time**. The closest individual peaks were terrain (2021‑01, partial) and models (2021‑08, solid). Every "engine architecture" rewrite (rend3, 2022 renderer crate, 2023 abstracted renderer) actually *drew less* than the scrappy 2021‑08 direct‑wgpu version. **Lesson for the successor: resist the abstraction rewrite; extend the direct renderer.**

---

## 4. Live RE / modding tooling — ❌ all gone, and genuinely valuable

### The "goldmine": `tlse_sys` + `tlse_hack` (2020, peak `10a65af` "Hit the goldmine")
A reverse‑engineering of Fable: TLC's **actual C++ engine ABI**:

- **~140 engine classes** mapped as `#[repr(C)]` Rust structs with field‑accurate layouts: `CMainGameComponent`, `CThing`/`CThingManager`/`CThingSearchTools`, `CWorld`/`CWorldMap`, `CGame`, `CPlayer`/`CPlayerManager`, `CDefinitionManager`/`CGameDefinitionManager`, `CGraphicDataBank`/`CMeshDataBank`, `CNavigationManager`, `CLUA`, `CScriptThing`, `CCamera`, `CCombatManager`, `CFactionManager`, etc.
- `CMainGameComponent` alone has its full field layout **and its complete ~130‑entry vtable** transcribed (update/render/init_graphics/init_world/init_lua/...).
- **Hand‑built C++/STL/boost ABI shims** (`cxx/`): `std_vector`, `std_map`, `std_list`, `std_set`, `std_pair`, `std_basic_string`, `std_allocator`, `boost_scoped_ptr` — so the Rust structs bit‑match the real objects.
- **`tlse_hack/src/loc.rs`**: **119 concrete absolute offsets** into the retail binary — entry point (`0x401067`), `CMainGameComponent` vtable (`0x122f180`) and global instance ptr (`0x13b86a0`), `Direct3DCreate9` call site, zlib CRC32, the script‑info manager (flagged "Good candidates for modding"), etc.
- Method bodies were `unimplemented!()` stubs (the layouts/offsets are the asset, not the logic).

This is brittle (tied to one binary build) but it's the foundation for a **trainer / live memory inspector / engine‑function hooking**, and represents enormous reversing effort. Abandoned at `3d524d9` ("Abandon tlse for now").

### DLL injection / cheat (2019‑11 → 2020‑01)
`fable_cheat` + `defable_launcher` + the injection iterations (`614b2c6` "more injection methods", `bb977d7` "Working injection!", `88cc0b0` obtain window handle, a DirectX9 hook attempt). A 32‑bit launcher that injected a cheat DLL into the running game. Worked at least to the injection stage.

### CLI tool `fool` — ❌ gone from main, ✅ alive on `big_editor`
Asset command‑line tool. Peak on main `f0b3bd9` (2025‑08): `extract_big`, `extract_wad`, `pack_wad`, `insert_texture_big`. Earlier (2024) it had `wad`/`lev`/`tng` inspect subcommands. Removed from main around `2b33a5d` (2026‑01); the `big_editor` branch still has it (big dump, texture import/export, wad pack/unpack). **This is the lowest‑effort high‑value thing to bring back to main.**

---

## 5. GUI / editor experiments — ❌ none finished
A long graveyard, none completed:
- **2019**: Svelte + Electron app (`big.svelte`, `wad-extract.svelte`, `selector.svelte`) over JS parsers; then `defable_editor` (Svelte + **Neon** Rust‑native node bindings).
- **2020**: **iced** attempt.
- **2021**: **egui** (the 2021‑12 app had a working egui overlay, empty 3D scene).
- **2023**: **tauri** → switched to **slint** (`explorer/ui/App.slint`).
- **2025**: **GPUI** (`big_editor` branch, `big_editor/src/ui.rs`, a counter demo + start of a .big browser).

No asset editor ever reached usable state. If you want one, GPUI (big_editor) is the most recent bet.

---

## 6. The `big_editor` branch (parallel present, not history)
Diverged 2025‑11‑30 (`137006d`), 4 commits, never merged. Relative to main HEAD it **adds**: a GPUI `big_editor` app, the `fool` CLI (big dump / texture import‑export / wad pack‑unpack), and a cleaner `fable_data` layout (`format/` + `io/` split). Main HEAD has 5 commits it lacks (the sky shader work). **These two branches have not been reconciled** — main got the sky, big_editor got the tooling. Worth a deliberate merge decision.

---

## 7. Documentation — ❌ scaffolding only
An **mdBook** ("book", later "manual", 2023‑24, `972c4e8`) with `SUMMARY.md` and stubs for `lev.md`/`tng.md`/`wad.md`/`usage.md`. The pages were essentially empty placeholders — the *intent* to document the formats, never filled. (The real format knowledge lives as comments inside the parsers, copied from fabletlcmod.com / obscuregamers.com.)

---

## 8. Recommendations — what to resurrect, in order

Ordered to match your sky→landscape→models descent and to maximize reuse:

1. **`fool` CLI back onto main** (cheap, immediately useful). Pull from `big_editor`. Gives you `unbig`/`unwad`/texture export to inspect data while you build the renderer.
2. **Lev parser → terrain mesh** (your literal next step). Resurrect `lev.rs` (2024 `cdf52b1` is the best version), then build the heightmap mesh in the current wgpu renderer using the 2019 glTF math + 2021 `landscape.rs` as references. Budget time for edge stitching and ground‑theme texture splatting (never solved before).
3. **Model viewer** (the part you remember fondly). Resurrect `model.rs`/`bbm.rs` (2021 `model.rs`, 1093 LOC) and port the 2021‑08 `scene.rs` textured+wireframe pipeline into the current renderer. It already solved DXT→material→draw.
4. **Shared `kv.rs` lexer + Tng** (world population). Bring back `util/kv.rs` (2024, 632 LOC) as the foundation for all script formats, then the 4208‑LOC Tng parser. Reconcile with your newer `def/text.rs` so you have one kv implementation, not two.
5. **Optional, bigger bets:**
   - *Modding round‑trip*: revive the 2023 `parser.rs`+`compiler.rs` pattern if you want to write assets back, not just read.
   - *Live RE*: the `tlse_sys`/`loc.rs` goldmine for a trainer/inspector — high value but brittle and a different track from the renderer.

### Cross‑cutting lesson from the history
Every time the project chased a *new engine abstraction* (rend3, the 2022 renderer crate, the 2023 abstracted renderer/client/explorer split, each new GUI toolkit) it ended up drawing **less** than the previous scrappy direct‑wgpu version, and momentum reset. The two times real game data hit the screen (2021‑01 terrain, 2021‑08 models) were both **direct wgpu, minimal abstraction**. Your current HEAD is in exactly that healthy mode. The highest‑leverage move is to keep extending it downward (terrain, then models) and *resist* the next architectural rewrite until something is actually on screen.

---

## Appendix: key commits

- `bbb7c7c` 2019‑06 — initial commit (Svelte/Electron + JS parsers)
- `28e678d` 2019‑09 — "Refactor into Rust"
- `274d0cf` 2019‑11 — Lev→glTF terrain, "mesh 98% works"
- `88cc0b0` 2020‑01 — working DLL injection / cheat era
- `10a65af` 2020‑05 — **"Hit the goldmine"** (tlse_sys RE peak)
- `3d524d9` 2020‑12 — "Abandon tlse for now"
- `c52dfa0` 2021‑01 — in‑engine landscape pass + console
- `2515e14` 2021‑08 — **textured model + wireframe renderer** (richest ever)
- `f5567c0` 2021‑10 — rend3 refactor
- `e3266c7` 2021‑12 — egui app; `model.rs` 1093 LOC; widest format set
- `84ca195` 2022‑07 — dedicated `renderer` crate (scaffold)
- `0801f6f` 2023‑06 — slint `explorer`/`client`; parser+compiler round‑trip per format
- `cdf52b1` 2024‑09 — **Tng parser 4208 LOC** on typed `kv.rs` lexer
- `f0b3bd9` 2025‑08 — `fool` CLI peak (extract/pack/insert)
- `b4ac4d1` 2025‑11 — "reorganizing and basic pipeline" (start of current sky era)
- `137006d` 2025‑11 — fork point of `big_editor` (GPUI)
- `205c23c` 2026‑06 — current HEAD (sky renderer)
