# Changelog

All notable changes to `aura` are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
Versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

Nothing yet.

---

## [0.3.3-beta.1] — 2026-04-17

### Added

- **AURA Sigils in terminal logs.** The toolchain now uses language-native sigils 
  to denote phases and actions:
  - `::` for top-level phases (e.g. `:: COMPILE`, `:: SUCCESS`)
  - `->` for processing paths (e.g. `PARSE    -> album/name.aura`)
  - `!!` for emphasis in success summary and diagnostic markers

- **Relative path reporting.** All compiler, validator, and linter commands now 
  strip the absolute project root from file paths in the output, providing a 
  cleaner, more context-aware experience.

- **High-contrast log formatting.** Improved alignment and color-coded labels 
  for better scannability in dense terminal output.

### Fixed

- **Syntax Error in `compile.rs`.** Resolved a mismatched delimiter bug in the 
  `validate` and `lint` command match arms that caused compilation failures.

---

## [0.3.1-beta.1] — 2026-04-17

### Fixed

- **`@domain/[id1, id2]` parse errors.** The parser now correctly handles
  inline bracket-list references like `producers -> @people/[p9gregk, p8paule]`.
  Previously the scanner stopped at `[`, leaving it on the stream and causing
  `expected \`->\`, got Comma` errors in `credits.aura` and `people.aura`.
  Fix is in `parse.rs` — the `RefAt` handler detects a trailing `/` path and
  consumes the `[id, id]` list directly, building a `RefBody::List`.

### Added

- **`%` Custom Key marker.** Fields marked `key % -> value` (space before `%`
  is conventional) suppress W006 vocabulary warnings for that key only.
  This follows the same pattern as `!` (required) and `?` (optional).
  - Token: `Kind::Custom` in `lex/token.rs`
  - Scanner: `%` byte → `Kind::Custom` in `lex/scan.rs`
  - AST: `FieldMarker::Custom` in `parse/ast.rs`
  - Parser: detected in `parse_field` in `parse.rs`
  - Lint: W006 skips `FieldMarker::Custom` fields in `lint/rules.rs`
  - W006 error message now mentions the `%` escape hatch

- **Plural keys added to standard vocabulary.** AURA now enforces a
  plural/singular convention: singular key for `@domain/id`, plural for
  `@domain/[id1, id2]`. All pairs added to `lint/keys.rs`:
  `producers/producer`, `writers/writer`, `labels/label`,
  `episodes/episode`, `seasons/season`, `tracks/track`, `scenes/scene`,
  `acts/act`, `chapters/chapter`, `segments/segment`, `sections/section`,
  `variants/variant`, `directors/director`, `editors/editor`,
  `narrators/narrator`, `hosts/host`, `guests/guest`,
  `performers/performer`, `instruments/instrument`, `samples/sample`,
  `arts/art`, `motions/motion`, `trailers/trailer`, `studios/studio`.

- **Rust logging module (`compiler::logs`).** Zero-dependency, AURA-native
  coloured terminal logger for the compiler CLI.
  - `logs/colors.rs` — ANSI colour constants for all 11 log kinds
  - `logs/formatter.rs` — message + diagnostic formatters with timestamp
  - `logs/logger.rs` — `Logger` struct with phase methods
    (`compile`, `lex`, `parse`, `lint`, `emit`) and diagnostic methods
    (`warn`, `error`, `info`, `success`, `note`, `debug`)
  - `logs/mod.rs` — public API and log-kind docs table
  - All output goes to **stderr** to keep stdout clean

### Changed

- Version bumped from `0.3.0-alpha.2` to `0.3.1-beta.1`.

---

## [0.3.0-alpha.2] — 2026-04-17

### Added

- **`name.aura` as project entry point.** `aura init` now generates `name.aura`
  instead of the old `namespace.aura` + `{id}.aura` pair. The root `name.aura`
  contains:
  - `name::` block with `id`, `root`, `kind`, `slug`, `lang`, `annotator`
  - `manifest::` block with `name`, `creator`, `version`, `released`, `access`
  - `tracks::members::` (for album/music kinds)
  - `exports::` block

- **Sub-folder `name.aura`.** Each content subfolder gets a `name.aura` with
  `name::folder ->` and `contains::` blocks. `aura add` appends to `contains::`.

- **`ref` key** replaces `aura-ref` in collection member blocks. W002 deprecation
  warning fires on any file still using `aura-ref`.

- **`$` vocab slug escape sigil.** `$identifier::` is now a valid namespace block
  opener. The `$` prefix signals a raw vocabulary slug that may match an AURA
  keyword (e.g. `$live::`, `$dark::` in `meta/moods.aura`). W006 is suppressed
  inside `$`-prefixed blocks. `Namespace.raw_slug = true`.

- **New `NodeType` variants:** `Name`, `Tracks`, `Episodes`, `Scenes`, `Variants`,
  `VocabSlug` — covering collection sub-containers and the project entry block.

- **`project_stem()` reads `id` from `name.aura`.** The output filename in `dist/`
  is derived from `name::id ->` in the root `name.aura`. Falls back to scanning
  root `.aura` files and then the folder name for backward compatibility.

### Changed

- `ns/load.rs` now discovers `name.aura` files (not `namespace.aura`). The minimal
  parser handles the new `name::` block format with `id ->` and `folder ->` fields.

- `aura init` output message shows `Entry point: name.aura` and `Project ID: {id}`.

- `aura add` registers new files in `{folder}/name.aura`'s `contains::` block.

### Fixed

- Doctest code-block examples in module doc comments are now marked `ignore`
  to prevent false doctest failures.

---

## [0.3.0-alpha.1] — 2026-04-17

### Added

- **Full recursive-descent AURA parser** (`parse/parse.rs`). Consumes the
  `Scanner` token stream and produces a `Document` AST using two-token
  lookahead and column-based indentation tracking.

- **Lint system** (`lint/`). Seven rules run on every parsed document:
  W001 `true`/`false` instead of `live`/`dark`, W002 deprecated keys
  (`thumbnail`, `artwork`), W003 interval-indexed node missing `time`,
  W004 manifest missing `name`, W005 manifest missing `creator`, W006
  unknown key (strict mode), E001 required field absent.

- **Compile pipeline fully wired**. `aura compile` now parses all `.aura`
  files, merges them into a single document, and emits:
  - `dist/{root-id}.hami` — B-Tree manifest for the entire project
  - `dist/{root-id}.atom` — interval tree (only if interval nodes exist)
  Output is named after the root manifest file ID (e.g. `c3yt8vi.hami`),
  not a hyphenated slug.

- **Per-subfolder `namespace.aura`** on `aura init`. Each content
  subfolder (`tracks/`, `scenes/`, `acts/`, `variants/`, etc.) gets a
  `namespace.aura` with an empty `contains::` block. The root
  `namespace.aura` exports block references each subfolder.

- **History serialization with TOML** (`hist/serial.rs`). Replaces the
  broken hand-rolled JSON serializer and stub deserializer. Take objects
  are stored as `.toml` files in `.history/objects/`. Full round-trip
  read/write using `serde` + `toml`.

### Fixed

- **Scanner: `::` at end-of-word** now terminates key scanning, so
  `schema::` correctly tokenizes as `Key("schema")` + `ScopeOpen` instead
  of `Key("schema::")`. Fixes all namespace parsing.

- **Scanner: slash in namespace paths.** `/` is now included in key
  scanning so `verse/one` tokenizes as a single `Key("verse/one")`.

- **Scanner: digits in bare values.** `is_key_start` now allows digits, so
  `1.0.0`, `0000-00-00`, and integer timestamps parse as bare values
  instead of causing "unexpected byte" errors.

- **Scanner: `looks_like_time` precision.** Version strings like `1.0.0`
  no longer mis-classify as time literals. A time literal must end with
  `s`, `m`, or `h`, or contain `:` for `HH:MM:SS` format.

## [0.2.0-alpha.1] — 2026-04-16

### Fixed

- **`core::id` — index out of bounds panic in ID generator.** `aura init`
  and `aura generate` panicked immediately with:
  `index out of bounds: the len is 36 but the index is 58`.
  The PRNG used `v >> 58` to index into `CHARSET` (36 entries), which
  produces values 0–63 on a 64-bit system — out of range for any value
  above 35. Fixed to `v % CHARSET.len()` so the index is always in 0..35.

- **Homebrew formula missing `on_linux` block.** The initial formula only
  declared URLs for `on_macos`, causing `brew install hamiorg/aura/aura`
  on Linux to fail with `formula requires at least a URL`. Added `on_linux`
  blocks with x86\_64 and arm64 URLs and correct SHA256 checksums.

- **`dist/package/brew.sh` only handled macOS.** Updated to accept all
  four platform checksums and write both `on_macos` and `on_linux` blocks.
  Future releases update the formula correctly from CI.

### Changed

- Release versioning policy: each fix or change now gets its own version
  tag. No more retagging. `v0.1.0-alpha.1` is superseded by this release.

---

## [0.1.0-alpha.1] — 2026-04-16 — superseded by 0.2.0-alpha.1

> This release was retagged twice after publication, which caused
> Homebrew checksum mismatches. Use 0.2.0-alpha.1 instead.

## [0.1.0-alpha.1] — 2026-04-16

This is the first tagged release. It establishes the project structure,
type system, CLI surface, and distribution pipeline. The compilation
pipeline is scaffolded but not yet connected end-to-end — the lexer,
parser, and emitters exist as separate modules but are not yet wired
together into a working `aura compile` command. That work is the focus
of the next development cycle.

### Added

**Core crate (`core/`)**

- `id` — typed hex ID generator. Every AURA object has a prefixed ID
  (e.g. `t7xab3c` for a track, `p4xt9k2` for a person). 25 prefixes
  defined covering all content and entity types.
- `node` — `AtomNode` and `HamiNode` structs with `#[repr(C)]` layout.
  `AtomNode` is 24 bytes: `low`, `high`, `duration`, `max`, `data_ptr`,
  `node_class`. The SIMD stabbing-query loop in the engine is designed
  around this layout.
- `interval` — `Interval` triple `[low, high, duration]` with invariant
  enforcement: `low + duration == high`.
- `delta` — `TakeObject`, `SourceDelta`, `StreamPointer`, `MarkEntry`.
  History chain types shared between the compiler and future engine.
- `vocab` — `VocabNode` for genre, role, and mood slug vocabulary.
- `person` — `PersonNode` and `AnnotatorNode`. Annotators are the humans
  who author `.aura` files; they are distinct from content contributors.
- `asset` — `ArtNode`, `MotionNode`, `TrailerNode`. Media asset
  references (CDN URLs, not embedded binary data).
- `entity` — `StudioNode` and `LabelNode` for industry entities.
- `access` — `AccessLevel` enum: Open, Archived, Restricted, Gated,
  Embargoed, Locked.
- `history` — `HistoryNode` for embedded versioned provenance in `.atom`.

**Compiler crate (`compiler/`)**

- `lex` — zero-copy byte scanner. Emits `Token` values holding `&'src str`
  slices into the source buffer. No heap allocation in the hot path.
- `parse::ast` — `Document`, `Namespace`, `Field`, `Value`, `NodeType`.
  Mirrors the AURA document hierarchy.
- `parse::time` — Time expression normalizer. Converts `22s~1m10s`,
  `22s+48s`, `[22s, 1m10s, 48s]`, and `@time/value` into `Interval`.
- `parse::resolve` — Two-phase `@domain/id` reference resolver with
  forward arc collection and configurable strict mode.
- `parse::inherit` — `>>` arc expander. Merges fields from referenced
  documents into the current AST node.
- `emit::hami` — HAMI B-Tree emitter. Replaces AURA sigils with ASCII
  control codes (US/RS/GS/FS) and builds a fixed-width positional index.
- `emit::atom` — ATOM flat-array interval tree emitter. Two-pass
  algorithm: first pass builds the sorted node array; second pass fills
  the augmented `max` values bottom-up.
- `emit::atlas` — ATLAS DTW warp path emitter. Writes `WarpPoint` arrays
  mapping canonical timestamps to variant timestamps.
- `ns` — Namespace loader and `exports::` block resolver. Builds the
  project symbol table from `namespace.aura` files.
- `hist` — History subsystem. Object store reader/writer for `.history/`,
  node-level diff engine, and delta chain replayer.
- `cfg` — Toolchain config reader for the `configs/` folder. Parses
  `stores.aura`, `llm.aura`, and `ignore.aura`.
- `directives` — `FileDirectives` parser for `schema::` and
  `directives::` blocks. Provides per-file compilation settings.
- `error` — `CompileError`, `Diagnostic`, `Level`, `Span`. All pipeline
  stages return `Result<_, CompileError>`.
- CLI with all subcommands: `compile`, `validate`, `lint`, `generate`,
  `init`, `add`, `take`, `mark`, `rewind`, `ledger`, `delta`, `stream`,
  `mix`, `hold`, `release`, `sync`, `dub`.

**Distribution (`dist/`)**

- `dist/build/linux.sh` — builds Linux musl static binaries via `cross`
  and Docker. Targets: `x86_64-unknown-linux-musl`,
  `aarch64-unknown-linux-musl`.
- `dist/build/mac.sh` — builds macOS binaries on macOS. Targets:
  `aarch64-apple-darwin`, `x86_64-apple-darwin`.
- `dist/build/check.sh` — prerequisite checker for local release builds.
- `dist/package/tar.sh` — creates `.tar.gz` tarballs for all targets.
- `dist/package/deb.sh` — creates `.deb` packages via `cargo-deb`.
- `dist/package/rpm.sh` — creates `.rpm` packages via `cargo-generate-rpm`.
  Handles the RPM convention of splitting a SemVer pre-release identifier
  into separate `Version` and `Release` RPM fields.
- `dist/package/brew.sh` — writes `Formula/aura.rb` for the Homebrew tap
  with correct version and SHA256 checksums.
- `dist/install/linux.sh` and `dist/install/mac.sh` — end-user
  `curl | bash` installers. Auto-detect architecture, download from
  GitHub Releases, install to `/usr/local/bin`.
- `.github/workflows/release.yml` — four-job CI pipeline triggered on
  `v*` tags: Linux builds (via `cross`), macOS builds (arm64 runner,
  cross-compiles x86\_64), GitHub Release publish, Homebrew tap update.
  The Homebrew formula is updated automatically with computed SHA256
  checksums after every release.

**Homebrew tap**

- `hamiorg/homebrew-aura` created. Formula updated automatically on each
  release via the CI `update-brew` job.

### Notes

- `Cargo.lock` is committed intentionally. For a binary crate the lock
  file ensures reproducible release builds.
- The `HOMEBREW_TAP_TOKEN` Actions secret holds a GitHub PAT with `repo`
  write access to `hamiorg/homebrew-aura`. It is required for the
  automated formula update step to work.
- Node.js 20 deprecation warnings appear in the CI log. These come from
  the GitHub Actions runner infrastructure, not from this project's code.
  They will clear when GitHub updates its default action runners to
  Node.js 24 (scheduled for June 2026).

### Known limitations

- `aura compile` logs file paths but does not yet produce output. The
  pipeline stages are implemented individually; the wiring between them
  is the next milestone.
- `aura take` records a `TakeObject` with an empty `deltas` field. The
  diff engine (`hist::delta::DeltaEngine`) is implemented but not yet
  called from the take command.
- Time expressions in the emitter fall back to a debug representation.
  The `TimeNorm` pass runs correctly in isolation but its output is not
  yet passed through to `emit::atom`.

---

[Unreleased]: https://github.com/hamiorg/aura/compare/v0.3.1-beta.1...HEAD
[0.3.1-beta.1]: https://github.com/hamiorg/aura/compare/v0.3.0-alpha.2...v0.3.1-beta.1
[0.3.0-alpha.2]: https://github.com/hamiorg/aura/compare/v0.3.0-alpha.1...v0.3.0-alpha.2
[0.3.0-alpha.1]: https://github.com/hamiorg/aura/releases/tag/v0.3.0-alpha.1
[0.2.0-alpha.1]: https://github.com/hamiorg/aura/releases/tag/v0.2.0-alpha.1
[0.1.0-alpha.1]: https://github.com/hamiorg/aura/releases/tag/v0.1.0-alpha.1
