# Introduction

AURA is a plain-text language for describing media content. Authors write `.aura`
files that capture the timing, credits, rights, vocabulary, and availability of
a work — whether that work is a music track, a film, a podcast episode, or an
audiobook chapter.

The `aura` tool reads those files and compiles them into binary formats the
Hami engine loads at runtime.

## What gets compiled

| Output  | Contains                                                                 |
| ------- | ------------------------------------------------------------------------ |
| `.atom` | A flat-array augmented interval tree. Every timed node — verse, line, scene, credit window, chapter, mood annotation — is stored as an entry with `[start, end, duration]` fields and queried at 60 fps by the engine. |
| `.hami` | A B-Tree manifest. Stores everything non-temporal: credits, vocabulary slugs, rights declarations, platform availability, art asset references. The `.atom` file points into the `.hami` lexical data region for string data. |
| `.atlas` | A DTW warp path. Maps timestamps from a canonical recording to a variant stream — a dub, a live take, a radio edit — without duplicating the content nodes. |

These three files are always published together for a given work. The engine
memory-maps all three and holds them in RAM for the duration of a playback session.

## How to use this documentation

- **Language Reference** — start here if you are authoring `.aura` files.
  [Syntax and Sigils](language/syntax.md) covers the AURA language in full.
  [Keyword Reference](language/keywords.md) is the complete key vocabulary table.
  [Conventions](language/conventions.md) covers the ID system, reference grammar,
  and folder layout rules for every supported media type.

- **Compiler** — read this if you are working on the `aura` tool itself.
  [Architecture](compiler/overview.md) explains the lexer, parser, and emitter
  pipeline. [Crate Structure](compiler/structure.md) documents the module layout
  and data type definitions.

- **Project Management** — covers the `aura init` scaffolding command and the
  built-in history system (takes, marks, streams, rewind).

## Current status

This is the 0.3.2-beta.2 release. The toolchain now features a standardized, 
high-contrast logging system and strict grammatical enforcement for multi-ID 
domain references. End-to-end compilation is stable.
