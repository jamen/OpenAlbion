# Def compiler and def library

This directory is the archive home of the Rust def compiler and format library merged from
[fable-defs](https://github.com/jamen/fable-defs) v0.5.0. The crates themselves live alongside
it in [`packages/`](..):

| Crate | Role |
|---|---|
| [`defs`](../defs) | Format library: text parsing, the typed def model, binary I/O, the `SemVal` differ |
| [`defs-derive`](../defs-derive) | The proc-macro derives for the def model |
| [`def-compiler`](../def-compiler) | Lowering (text → typed structs) and the build pipeline |
| [`defc`](../defc) | The `defc` command-line compiler |
| [`def-compiler-sys`](../def-compiler-sys) | C ABI retained for archival; EgoCore now uses its C++ port |

The architecture and handoff guide is [`AGENTS.md`](AGENTS.md). It covers the text grammar,
the typed-struct IR, the specialized lowering arms, the binary container, the diagnostic model,
the verification loop, and the manifest.

> [!NOTE]
> The C++ port and its CMake build were moved to
> [`EgoCore/FableDefs`](https://github.com/eeeeeAeoN/EgoCore/tree/master/FableDefs). Paths and
> commands in `AGENTS.md` that refer to the C++ tree are historical.

## Building the corpus

```bash
cargo run -q -p defc -- <Defs-dir> <out-dir>

# Golden byte-determinism gate (needs the Anniversary text corpus)
OA_TEXT_DIR=<Defs-dir> cargo test -p defc --test golden
```

The defs describe Fable (2004) game data — item values, behaviours, and so on — written as
`.def` text and compiled to the game's `.bin` files. See
[`AGENTS.md`](AGENTS.md) for the pipeline and the split between `game.bin`, `frontend.bin`,
and `script.bin`.
