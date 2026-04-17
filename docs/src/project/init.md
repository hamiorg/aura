# Project Initialization (`aura init`)

> **Bootstrapping new AURA projects with correct structures and identifiers.**

The `aura init` command scaffolds a new AURA project. It generates the folder structure, creates the base `.aura` manifests, sets up toolchain configuration, and generates the initial root identities.

Because AURA relies heavily on generated hex IDs, initializing a project manually is tedious. `aura init` automates this process while enforcing the layout rules defined in `conventions.md`.

---

## Basic Usage

```bash
aura init <kind> [options]
```

Where `<kind>` is the media kind of the project, corresponding to the `kind` field in the root schema block.

### Common Kinds

- **Audio:** `audio::music`, `audio::album`, `audio::ep`, `audio::podcast`, `audio::audiobook`, `audio::speech`
- **Video:** `video::movie`, `video::series`, `video::music`, `video::animation`
- **Text:**  `text::book`, `text::article`

**Example:**

```bash
aura init audio::album --name "Signal Loss" --lang en-US
```

---

## Generated Identifiers

When you run `aura init audio::album`:

1. It generates a collection ID (e.g., `c8xab3d`).
2. It generates a generic project directory based on the kind (`album/`, `track/`, `film/`, etc.). Detail identities like names, metadata, and the ID stay inside the `.aura` node manifest files, ensuring root project folder names remain clean and standardized.
3. It generates an annotator ID for the current user and populates `info/annotators.aura`.

---

## Output Structures by Kind

The folder structure generated depends strictly on the `kind` argument.

### 1. `audio::album` (or `audio::ep`)

Creates a hierarchical collection for multiple tracks.

```text
album/                           <- Project root folder
  namespace.aura                 <- Project entry point
  c8xab3d.aura                   <- Collection manifest
  info/
    namespace.aura
    metadata.aura                <- Initialized with the album name
    people.aura                  <- Empty people registry
    annotators.aura              <- Initialized with the current user
    credits.aura                 <- Empty global credits
    arts.aura                    <- Ready for artwork URLs
  meta/
    namespace.aura               <- Empty local vocabulary registry
  tracks/
    namespace.aura               <- Export block for tracks
  variants/
    namespace.aura
  artwork/                       <- Default local asset folders
  motion/
  trailers/
  stems/
  configs/
    llm.aura                     <- Toolchain integrations
    stores.aura                  <- Cloud origin definition
    account.aura                 <- Local deployment credentials
    ignore.aura                  <- History exclusion list
```

### 2. `audio::music` (Single Track)

A much flatter structure when releasing a single track.

```text
track/
  namespace.aura
  t7xab3c.aura                   <- Track manifest (serves as root document)
  info/
    namespace.aura
    metadata.aura
    people.aura
    annotators.aura
  artwork/
  stems/
  configs/
    llm.aura
    stores.aura
    account.aura
    ignore.aura
```

### 3. `audio::podcast`

Bootstraps a podcast series with a season-oriented layout.

```text
podcast/
  namespace.aura
  pc5xk4m.aura                   <- Series manifest
  info/
    namespace.aura
    metadata.aura
    people.aura
    annotators.aura
  seasons/
    namespace.aura               <- Add seasons here using `aura add season`
  artwork/
  configs/
```

### 4. `video::music`

Creates a companion video project, prioritizing scenes and shots.

```text
music-video/
  namespace.aura
  mv6xp3l.aura                   <- Video manifest
  info/
    namespace.aura
    metadata.aura
    people.aura
    annotators.aura
    credits.aura
  scenes/
    namespace.aura               <- Initialized empty, ready for scenes
  shots/
  artwork/
  configs/
```

### 5. `video::movie` (Film)

Bootstraps a long-form video output with acts and scenes.

```text
film/
  namespace.aura
  f6np2qr.aura                   <- Film manifest
  info/
    namespace.aura
    metadata.aura
    people.aura
    annotators.aura
    credits.aura
    rights.aura
  acts/
    namespace.aura
  scenes/
    namespace.aura
  configs/
```

### 6. `audio::speech`

Bootstraps a short or long-form address, lecture, or panel.

```text
speech/
  namespace.aura
  sp2xr7n.aura                   <- Speech manifest
  info/
    namespace.aura
    metadata.aura
    people.aura
    annotators.aura
  segments/
    namespace.aura               <- Standard division unit for speeches
  artwork/
  configs/
```

---

## Default File Contents (Example: `audio::album`)

### `namespace.aura`

The compiler entry point automatically routes to the generated collection manifest.

```aura
schema::
  root       -> https://hami.aduki.org/aura/1.0
  kind       -> audio::album
  namespace  -> signal-loss
  lang       -> en-US

exports::
  info       -> @info/metadata
  people     -> @info/people
  tracks     -> @tracks/*
  collection -> c8xab3d.aura
```

### `info/metadata.aura`

The central identity metadata for the release.

```aura
schema::
  root    -> https://hami.aduki.org/aura/1.0
  kind    -> metadata
  lang    -> en-US

manifest::
  name      ! -> "Signal Loss"
  creator   ! -> @person/PLACEHOLDER
  version     -> 1.0.0
  released    -> 0000-00-00
```

### `info/annotators.aura`

AURA automatically populates the first annotator using the local system's toolchain configuration. This ID tracks who made the file changes.

```aura
annotators::

  p9xb3mn::
    name     -> "Local System User"
    roles    -> annotator | editor
```

---

## Modifiers and Commands Post-Init

Once `aura init` establishes the base directory, authors use the toolchain to scaffold files within the active project.

```bash
# Generate a new track inside an album project
aura add track "Fold"
> Created tracks/t4mn2rp.aura

# Create a scene inside a film project
aura add scene "Cold Open"
> Created scenes/f2xb7np.aura

# Generate a new person ID in the annotators or people file
aura generate person
> p5xz2kr
```

---

*AURA Compiler Reference — v0.3.2-beta.2*  
*`init.md` maps directly to the constraints laid out in `conventions.md` and `flux.md`.*
