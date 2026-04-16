# aura

The command-line compiler and project tool for the AURA language.

AURA is a text format for describing the content and metadata of media works —
music, film, television, podcasts, audiobooks, and speech. Authors write `.aura`
files by hand. This tool reads them and compiles them into binary formats the
Hami engine can load at runtime.

---

## Output formats

| File     | Contents                                                                                                                                                                                                   |
| -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `.atom`  | A flat-array augmented interval tree. Stores every timed node (verse, line, scene, chapter, credit window, etc.) ordered by start time. The engine queries this with a stabbing-query algorithm at 60 fps. |
| `.hami`  | A B-Tree manifest. Stores all non-timed data: credits, vocabulary, rights, platform availability, art references, and the lexical data region the `.atom` file points into.                                |
| `.atlas` | A DTW warp path. Maps timestamps from a canonical recording to a variant stream (a dub, a live take, a radio edit).                                                                                        |

The three files for a single work are always published together. The engine
memory-maps all three and holds them in RAM for the duration of a session.

---

## Status

This is an alpha release. The project scaffold, type system, and distribution
pipeline are in place. The full compilation pipeline (lexer output wired through
the parser into the emitters) is under active development. Do not use this in
production.

---

## Install

### Homebrew (macOS and Linux)

```sh
brew install hamiorg/aura/aura
```

Works on both macOS and Linux (via Linuxbrew). Updates arrive automatically
with `brew upgrade`.

### Linux — one-line installer

```sh
curl -fsSL https://hami.aduki.org/install.sh | bash
```

Detects your architecture (`x86_64` or `arm64`) and installs the static binary to
`/usr/local/bin/aura`. No system libraries required.

### Linux — package managers

Download the package for your architecture from the
[releases page](https://github.com/hamiorg/aura/releases) and install it:

**Debian, Ubuntu:**
```sh
sudo dpkg -i aura-<version>-linux-x86_64.deb
```

**Fedora, RHEL, AlmaLinux:**
```sh
sudo rpm -i aura-<version>-linux-x86_64.rpm
```

### Direct download

Pre-built tarballs for all supported targets are attached to every release:

| Platform | Architecture |
| -------- | ------------ |
| Linux    | x86\_64      |
| Linux    | arm64        |
| macOS    | arm64        |
| macOS    | x86\_64      |

---

## Usage

### Start a new project

```sh
aura init audio::album --name "Signal Loss" --lang en-US
```

Creates the standard folder structure for an album project, generates the root
collection manifest with a typed hex ID, and scaffolds the `info/`, `meta/`,
`tracks/`, and `configs/` directories.

Supported kinds include `audio::music`, `audio::podcast`, `audio::audiobook`,
`video::movie`, `video::series`, `video::documentary`, and others. See the
[Initialization](https://hamiorg.github.io/aura/project/init.html) docs for
the full list.

### Generate an ID

Every content object in AURA has a typed hex ID. To generate one:

```sh
aura generate track      # t7xab3c
aura generate person     # p4xt9k2
aura generate episode    # ep7xb3n
aura generate collection # c8xab3d
```

### Compile a project

```sh
aura compile
```

Reads the project in the current directory, resolves all references, normalizes
time expressions, and writes `.atom`, `.hami`, and `.atlas` files to `dist/`.

```sh
aura compile --take tx3ab7k   # compile a historical version
aura validate                 # check without writing output
aura lint                     # style and convention warnings
```

### Project history

AURA has a built-in version system that uses its own vocabulary rather than git
terminology. The unit of history is a take.

```sh
aura take "first complete draft"   # record a take
aura mark premiere                  # name this take
aura rewind premiere                # restore draft to that take
aura ledger                         # show all takes in order
aura delta tx3ab7k tx9zz1p          # compare two takes
```

History is stored in `.history/` at the project root. It tracks only `.aura`
source files — compiled binaries and media assets are excluded automatically.

---

## Build from source

Requires Rust 1.75 or later.

```sh
git clone https://github.com/hamiorg/aura.git
cd aura
cargo build --release -p compiler
```

The binary is written to `target/release/aura`.

The workspace has two crates:

| Crate      | Purpose                                                               |
| ---------- | --------------------------------------------------------------------- |
| `core`     | Shared data types and ID generation. Zero external dependencies.      |
| `compiler` | The compiler pipeline, CLI, history subsystem, and packaging tooling. |

The engine crate is a separate repository and is not included here.

---

## Documentation

Full documentation is at **[hamiorg.github.io/aura](https://hamiorg.github.io/aura/)**.

| Section                                                                                   | Contents                                     |
| ----------------------------------------------------------------------------------------- | -------------------------------------------- |
| [Syntax and Sigils](https://hamiorg.github.io/aura/language/syntax.html)                  | AURA language syntax and sigil reference     |
| [Keyword Reference](https://hamiorg.github.io/aura/language/keywords.html)                | Full key vocabulary table                    |
| [Conventions](https://hamiorg.github.io/aura/language/conventions.html)                   | ID system, reference grammar, folder layouts |
| [Compiler Architecture](https://hamiorg.github.io/aura/compiler/overview.html)            | Lexer, parser, and emitter pipeline design   |
| [Crate Structure](https://hamiorg.github.io/aura/compiler/structure.html)                 | Module layout and data type definitions      |
| [Initialization](https://hamiorg.github.io/aura/project/init.html)                        | `aura init` and `aura add` commands          |
| [History System](https://hamiorg.github.io/aura/project/history.html)                     | Takes, marks, streams, rewind                |

---

## Changelog

See [CHANGELOG.md](CHANGELOG.md).

---

## License

MIT. See [LICENSE](LICENSE) for the full text.
