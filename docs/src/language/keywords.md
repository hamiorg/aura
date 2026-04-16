# AURA — Keyword Reference

> Single-page lookup for every keyword, sigil, domain, enumerated value,
> standard key, and toolchain command in the AURA language.
> For full explanations, see `flux.md`, `structure.md`, and `chages.md`.

---

## Sigils

Every sigil in AURA has exactly one meaning. No context-dependent overloading.

| Sigil  | Name           | Purpose                                                    |
| ------ | -------------- | ---------------------------------------------------------- |
| `::`   | Scope Opener   | Opens a namespace block or declares a node block           |
| `->`   | Value Arrow    | Assigns a literal value to a key                           |
| `@`    | Reference      | References a named entity anywhere in a document           |
| `##`   | Annotation     | Queryable comment — compiled into HAMI, not a code comment |
| `--`   | Divider        | Visual separator — no compile output                       |
| `\|`   | Union Pipe     | One or more values across different domains or literals    |
| `?`    | Optional Mark  | This field may be absent                                   |
| `!`    | Required Mark  | This field must be present or compile fails                |
| `~`    | Duration Mark  | Separates start and duration in a time triple              |
| `[` `]`| List Bracket   | Time triple `[start, end, duration]` or multi-ID list      |
| `>>`   | Inherits From  | Extend a template or base schema                           |
| `*`    | Wildcard       | Match all nodes in a namespace query                       |
| `::`   | Leap Operator  | Cross-boundary reference when combined with a file ID      |

---

## Boolean Literals

AURA uses media-native boolean values instead of `true` and `false`.
`true` and `false` are accepted as synonyms for toolchain interoperability.

| Literal | Value | Reads as      | Analogy                              |
| ------- | ----- | ------------- | ------------------------------------ |
| `live`  | 1     | true / on     | Broadcast: "going live" = active     |
| `dark`  | 0     | false / off   | Stage: "going dark" = inactive       |

```text
explicit  -> dark      ## not explicit
cleared   -> live      ## rights confirmed
featured  -> live      ## currently featured
published -> dark      ## still a draft
```

---

## Reference Domains

All domains use the `@domain/id` or `@domain/[id1, id2]` pattern.
Singular domain = one entity. Plural domain = multiple entities (compiles to array).

### People

| Domain              | Form     | Resolves via                                  |
| ------------------- | -------- | --------------------------------------------- |
| `@person/id`        | singular | `info/people.aura` or global cloud            |
| `@people/[a, b]`    | plural   | `info/people.aura` or global cloud            |
| `@author/id`        | singular | alias for `@person/id`                        |
| `@authors/[a, b]`   | plural   | alias for `@people/[a, b]`                    |
| `@annotator/id`     | singular | `info/annotators.aura` or global cloud        |
| `@annotators/[a,b]` | plural   | `info/annotators.aura` or global cloud        |

### Vocabulary

| Domain              | Form     | Resolves via                                  |
| ------------------- | -------- | --------------------------------------------- |
| `@genre/slug`       | singular | `meta/genres.aura` or global vocab            |
| `@genres/[a, b]`    | plural   | `meta/genres.aura` or global vocab            |
| `@role/slug`        | singular | `meta/roles.aura` or global vocab             |
| `@roles/[a, b]`     | plural   | `meta/roles.aura` or global vocab             |
| `@mood/slug`        | singular | `meta/moods.aura` or global vocab             |
| `@moods/[a, b]`     | plural   | `meta/moods.aura` or global vocab             |

### Content Files

| Domain                   | Form     | Resolves via                            |
| ------------------------ | -------- | --------------------------------------- |
| `@track/id`              | singular | `tracks/` folder by generated ID        |
| `@tracks/[a, b]`         | plural   | `tracks/` folder by generated IDs       |
| `@episode/id`            | singular | `episodes/` folder by generated ID      |
| `@episodes/[a, b]`       | plural   | `episodes/` folder                      |
| `@scene/id`              | singular | `scenes/` folder by generated ID        |
| `@scenes/[a, b]`         | plural   | `scenes/` folder                        |
| `@variant/id`            | singular | `variants/` folder by generated ID      |
| `@collection/id`         | singular | collection manifest file by ID          |
| `@season/id`             | singular | season manifest by ID                   |
| `@member/id`             | singular | another member in the same collection   |
| `@member/id::node/path`  | singular | specific node within another member     |

### Time, Sync, and Tempo

| Domain           | Form     | Meaning                                         |
| ---------------- | -------- | ----------------------------------------------- |
| `@time/value`    | singular | a temporal point anchor in the current file     |
| `@tempo/id`      | singular | a tempo node in the current file                |
| `@anchor/id`     | singular | a hard sync anchor node                         |

### Music Attribution

| Domain                      | Form     | Meaning                                    |
| --------------------------- | -------- | ------------------------------------------ |
| `@sample/id`                | singular | a sample attribution node                  |
| `@samples/[a, b]`           | plural   | multiple sample nodes                      |
| `@interpolation/id`         | singular | a musical interpolation node               |
| `@interpolations/[a, b]`    | plural   | multiple interpolation nodes               |

### Annotation and Context

| Domain                      | Form     | Meaning                                    |
| --------------------------- | -------- | ------------------------------------------ |
| `@explainer/id`             | singular | an explanation node for any content node   |
| `@explainers/[a, b]`        | plural   | multiple explainer nodes                   |
| `@instruction/id`           | singular | a processing instruction node              |
| `@instructions/[a, b]`      | plural   | multiple instruction nodes                 |

### Events

| Domain              | Form     | Meaning                                         |
| ------------------- | -------- | ----------------------------------------------- |
| `@event/id`         | singular | a condition-triggered signal node               |
| `@events/[a, b]`    | plural   | multiple event nodes                            |

### Access

| Domain                 | Meaning                                                    |
| ---------------------- | ---------------------------------------------------------- |
| `@access/open`         | Public — unrestricted, no authentication required          |
| `@access/locked`       | Private — authentication required, owner-only              |
| `@access/gated`        | Conditional — subscription, payment, or role required      |
| `@access/embargoed`    | Time-locked — transitions to open after embargo date       |
| `@access/archived`     | Accessible but retired — marked for historical access      |
| `@access/restricted`   | Geo- or rights-restricted — named territories only         |

Access cascade (least → most restrictive):

```text
open < archived < restricted < gated < embargoed < locked
```

### History

| Domain                      | Meaning                                               |
| --------------------------- | ----------------------------------------------------- |
| `@history/current`          | The latest take in this file's ledger                 |
| `@history/origin`           | The first take ever recorded for this file            |
| `@history/tx3ab7k`          | A specific take by generated ID (tx prefix)           |
| `@history/v1.0`             | A named mark on the ledger                            |
| `@history/~1`               | One take before current (relative)                    |
| `@history/~n`               | n takes before current                                |
| `@history/stream/name`      | The head of a named development stream                |
| `@history/main`             | The head of the main development stream               |
| `@history/take-id::path`    | A specific node's state at a given take               |

### Info and Meta Files

| Domain                | Meaning                                                    |
| --------------------- | ---------------------------------------------------------- |
| `@info/people`        | The `info/people.aura` file for this project               |
| `@info/annotators`    | The `info/annotators.aura` file for this project           |
| `@info/metadata`      | The `info/metadata.aura` file for this project             |
| `@meta/genres`        | The `meta/genres.aura` file for this project               |
| `@meta/roles`         | The `meta/roles.aura` file for this project                |
| `@meta/moods`         | The `meta/moods.aura` file for this project                |

### Media Assets

| Domain              | Form     | Meaning                                               |
| ------------------- | -------- | ----------------------------------------------------- |
| `@art/id`           | singular | A static image art asset (cover, poster, show banner)  |
| `@arts/[a, b]`      | plural   | Multiple art assets                                    |
| `@motion/id`        | singular | An animated motion cover or looping video asset       |
| `@motions/[a, b]`   | plural   | Multiple motion assets                                |
| `@trailer/id`       | singular | A promotional trailer or preview clip                 |
| `@trailers/[a, b]`  | plural   | Multiple trailers                                     |

### Industry Entities

| Domain              | Form     | Resolves via                                          |
| ------------------- | -------- | ----------------------------------------------------- |
| `@studio/id`        | singular | `info/studios.aura` or global cloud studio registry  |
| `@studios/[a, b]`   | plural   | `info/studios.aura` or global cloud                  |
| `@label/id`         | singular | `info/labels.aura` or global cloud label registry    |
| `@labels/[a, b]`    | plural   | `info/labels.aura` or global cloud                   |

### Content Availability

| Domain              | Form     | Meaning                                               |
| ------------------- | -------- | ----------------------------------------------------- |
| `@watch/id`         | singular | Streaming platform availability entry                 |
| `@watch/[a, b]`     | plural   | Multiple streaming platforms                          |
| `@buy/id`           | singular | Purchase availability entry with pricing              |
| `@buy/[a, b]`       | plural   | Multiple purchase options                             |
| `@rent/id`          | singular | Rental availability entry with pricing and window     |
| `@rent/[a, b]`      | plural   | Multiple rental options                               |
| `@download/id`      | singular | Download availability entry                           |
| `@download/[a, b]`  | plural   | Multiple download options                             |

### Cloud

| Domain          | Meaning                                                    |
| --------------- | ---------------------------------------------------------- |
| `@aduki.org/`   | Global cloud URI — all domains available via path          |

### Reserved (Future)

| Domain         | Meaning                                                    |
| -------------- | ---------------------------------------------------------- |
| `@thread/id`   | Future — parallel thread support (not yet implemented)     |
| `@parallel/id` | Future — parallel execution node (not yet implemented)     |

---

## Namespace Blocks

Namespace blocks open with `::` and contain key-value pairs or sub-nodes.

### Core Namespaces

| Keyword           | Purpose                                                        |
| ----------------- | -------------------------------------------------------------- |
| `schema::`        | Document-level metadata: root, kind, lang, annotator           |
| `manifest::`      | Content identity: name, creator, released, access, etc.        |
| `directives::`    | Compiler / engine directives for this file                     |
| `meta::`          | Descriptive metadata block (genre, tags, etc.)                 |
| `collection::`    | Collection root block inside a manifest file                   |
| `members::`       | Member list inside a collection block                          |
| `seasons::`       | Season list inside a series manifest                           |
| `related::`       | Relational links to other works                                |
| `links::`         | External URL block inside a person or manifest node            |
| `info::`          | Inline info reference block                                    |
| `namespace::`     | Project or folder namespace descriptor (in namespace.aura)     |
| `exports::`       | Re-exports of sub-namespaces in root namespace.aura            |
| `availability::`  | Content platform availability block in a manifest              |

### People and Annotators

| Keyword          | Purpose                                                     |
| ---------------- | ----------------------------------------------------------- |
| `people::`       | Person definitions block (canonical namespace)              |
| `persons::`      | Alias for `people::` — identical compile output             |
| `authors::`      | Alias for `people::` — identical compile output             |
| `annotators::`   | Annotator definitions block in `info/annotators.aura`       |

### Vocabulary - meta/ and info/ files

| Keyword       | Purpose                                                        |
| ------------- | -------------------------------------------------------------- |
| `genres::`    | Genre vocabulary block in `meta/genres.aura`                   |
| `roles::`     | Role vocabulary block in `meta/roles.aura`                     |
| `moods::`     | Mood vocabulary block in `meta/moods.aura`                     |

### Media Assets and Industry Entities

| Keyword         | Purpose                                                      |
| --------------- | ------------------------------------------------------------ |
| `arts::`        | Art asset block in `info/arts.aura`                          |
| `motions::`     | Motion cover block in `info/arts.aura`                       |
| `trailers::`    | Trailer and preview block in `info/arts.aura`                |
| `studios::`     | Studio entity block in `info/studios.aura`                   |
| `labels::`      | Label entity block in `info/labels.aura`                     |
| `watch::`       | Streaming availability block in `info/availability.aura`     |
| `buy::`         | Purchase availability block in `info/availability.aura`      |
| `rent::`        | Rental availability block in `info/availability.aura`        |
| `download::`    | Download availability block in `info/availability.aura`      |

### Configs (Non-Compiled)

| Keyword       | Purpose                                                        |
| ------------- | -------------------------------------------------------------- |
| `llm::`       | LLM provider definitions in `configs/llm.aura`                 |
| `stores::`    | Remote store origins in `configs/stores.aura`                  |
| `accounts::`  | Cloud identity credentials in `configs/account.aura`           |
| `ignore::`    | History exclusion list in `configs/ignore.aura`                 |

### Support Node Namespaces

Declared inside `support::` blocks within content files.

| Keyword           | Purpose                                                     |
| ----------------- | ----------------------------------------------------------- |
| `support::`       | Container for all support nodes in a file                   |
| `segments::`      | Musical/structural section markers                          |
| `instruments::`   | Instrument activity windows                                 |
| `chapters::`      | Navigable chapter divisions                                 |
| `credits::`       | Time-windowed contributor credit windows                    |
| `translations::`  | Language overlay nodes for content nodes                    |
| `moods::`         | Mood and emotional annotation windows                       |
| `rights::`        | Licensing and territorial boundary nodes                    |
| `slots::`         | Advertising insertion point nodes                           |
| `anchors::`       | Hard synchronization recovery point nodes                   |
| `tempo::`         | Tempo windows affecting lyric sync                          |
| `samples::`       | Audio sample attribution nodes                              |
| `explainers::`    | Explanation and gloss nodes                                 |
| `interpolations::`| Re-recorded composition element attribution nodes           |
| `instructions::`  | Processing directive nodes                                  |
| `events::`        | Condition-triggered signal nodes                            |

---

## Content Node Types

Content nodes carry renderable text payloads.

| Type          | Granularity Level | Used In                                         |
| ------------- | ----------------- | ----------------------------------------------- |
| `act`         | Macro             | Film, stage, long-form video                    |
| `scene`       | Macro             | Film, music video, animation, documentary       |
| `shot`        | Macro             | Film, music video — camera unit                 |
| `verse`       | Macro             | Song lyrics                                     |
| `chorus`      | Macro             | Song lyrics                                     |
| `bridge`      | Macro             | Song lyrics                                     |
| `intro`       | Macro             | Song, podcast, speech                           |
| `outro`       | Macro             | Song, podcast, speech                           |
| `hook`        | Macro             | Song lyrics                                     |
| `drop`        | Macro             | Electronic music                                |
| `interlude`   | Macro             | Song, album                                     |
| `breakdown`   | Macro             | Song                                            |
| `pre-chorus`  | Macro             | Song lyrics                                     |
| `post-chorus` | Macro             | Song lyrics                                     |
| `chapter`     | Macro             | Audiobook, podcast, documentary                 |
| `segment`     | Macro             | Speech, lecture, panel                          |
| `section`     | Macro             | Any long-form                                   |
| `line`        | Meso              | All content types                               |
| `dialogue`    | Meso              | Film, series, podcast — speaker-attributed      |
| `word`        | Micro             | All content types                               |
| `token`       | Micro             | Transcription systems                           |
| `syllable`    | Nano              | Song, speech                                    |
| `phoneme`     | Nano              | Speech, accessibility                           |
| `letter`      | Pico              | Animation, 60fps rendering                      |
| `character`   | Pico              | Non-Latin scripts                               |

---

## Support Node Types

Support nodes carry metadata without a renderable text payload.

| Type             | ATOM Class | Purpose                                                  |
| ---------------- | ---------- | -------------------------------------------------------- |
| `segment`        | 0x02       | Musical/structural section marker                        |
| `instrument`     | 0x03       | Instrument activity window                               |
| `chapter`        | 0x04       | Navigable division (audiobook, podcast, film)            |
| `credit`         | 0x05       | Time-windowed contributor credit                         |
| `translation`    | 0x06       | Language overlay for a content node                      |
| `mood`           | 0x07       | Emotional or tonal annotation window                     |
| `rights`         | 0x08       | Licensing or territorial boundary                        |
| `slot`           | 0x09       | Advertising insertion point                              |
| `anchor`         | 0x0A       | Hard synchronization recovery point                      |
| `annotator`      | 0x0B       | Annotator attribution node                               |
| `vocab`          | 0x0C       | Genre, role, or mood vocabulary node                     |
| `event`          | 0x0D       | Condition-triggered signal node                          |
| `tempo`          | 0x0E       | BPM and time-signature window; affects lyric sync        |
| `sample`         | 0x0F       | Audio sample attribution (source, kind, clearance)       |
| `explainer`      | 0x10       | Explanation or gloss for any node                        |
| `interpolation`  | 0x11       | Re-recorded composition element attribution              |
| `instruction`    | 0x12       | Processing directive to engine or player                 |
| `access`         | 0x13       | Content visibility and permission level                  |
| `history`        | 0x14       | Versioned take and delta chain provenance                |
| `art`            | 0x15       | Static cover art or image asset (manifest-level)         |
| `motion`         | 0x16       | Animated motion cover or looping video (manifest-level)  |
| `trailer`        | 0x17       | Promotional trailer or preview clip (manifest-level)     |
| `studio`         | 0x18       | Production studio entity with ownership hierarchy        |
| `label`          | 0x19       | Record label or publishing imprint with hierarchy        |
| `watch`          | 0x1A       | Streaming platform availability entry                    |
| `buy`            | 0x1B       | Purchase availability entry with pricing                 |
| `rent`           | 0x1C       | Rental availability entry with pricing and window        |
| `download`       | 0x1D       | Download availability entry                              |

---

## Standard Keys

| Key          | Meaning                                                        |
| ------------ | -------------------------------------------------------------- |
| `name`       | Human-readable title of this node (legacy person field; prefer first+last) |
| `first`      | Given name of a person node                                    |
| `middle`     | Middle name(s) of a person node — optional                     |
| `last`       | Family name of a person node — optional for mononyms           |
| `screen`     | Short on-screen identifier for captions, dialogue, mini-player |
| `kind`       | Type or category within this node's class (also person kind)   |
| `time`       | Temporal interval of this node                                 |
| `duration`   | Total length as a standalone declared value                    |
| `text`       | Text payload of a content node                                 |
| `locale`     | IETF BCP 47 language tag                                       |
| `script`     | Explicit script code (Latn, Arab, Cyrl, etc.)                  |
| `label`      | Short human-readable tag or marker                             |
| `genre`      | Genre descriptor — may be a union list                         |
| `released`   | ISO 8601 release date                                          |
| `territory`  | Geographic scope                                               |
| `version`    | Semantic version string                                        |
| `creator`    | Primary creator reference                                      |
| `speaker`    | Active speaker reference at this node                          |
| `speakers`   | Multiple speaker references at this node                       |
| `cast`       | Cast list for a scene or act                                   |
| `host`       | Podcast or show host reference                                 |
| `guest`      | Guest speaker reference                                        |
| `language`   | Primary language of this document                              |
| `country`    | Country of origin                                              |
| `city`       | City of origin                                                 |
| `born`       | Date of birth for a person node                                |
| `bio`        | Biography or description text                                  |
| `note`       | Annotation or editorial note                                   |
| `source`     | Origin indicator                                               |
| `store`      | Source data store URI                                          |
| `hash`       | Content hash for integrity verification                        |
| `index`      | Ordinal position within a collection                           |
| `count`      | Quantity field                                                 |
| `main`       | Primary entry in a credits block                               |
| `vocals`     | Vocalist reference in a credits block                          |
| `producer`   | Producer reference in a credits block                          |
| `writer`     | Writer reference in a credits block                            |
| `mixer`      | Mixing engineer reference                                      |
| `master`     | Mastering engineer reference                                   |
| `director`   | Director reference                                             |
| `editor`     | Editor reference                                               |
| `narrator`   | Narrator reference                                             |
| `energy`     | Normalized intensity float 0.0 — 1.0                           |
| `bpm`        | Beats per minute                                               |
| `grid`       | Time signature (4/4, 3/4, 6/8, etc.)                           |
| `key`        | Musical key                                                    |
| `isrc`       | International Standard Recording Code                          |
| `iswc`       | International Standard Musical Work Code                       |
| `license`    | License identifier                                             |
| `expires`    | Expiry date for a rights or license field                      |
| `show`       | Parent show name for episodic content                          |
| `season`     | Season index or identifier                                     |
| `episode`    | Episode index or identifier                                    |
| `synopsis`   | Long-form description                                          |
| `tags`       | Free-form tag list                                             |
| `links`      | External link block                                            |
| `roles`      | Role list for a person node                                    |
| `family`     | Instrument family classification                               |
| `active`     | List of active time windows (instrument nodes)                 |
| `stem`       | Reference to a discrete audio stem file                        |
| `thumbnail`  | Removed — use `cover -> @art/id` for chapter and episode art   |
| `artwork`    | Removed — use `cover -> @art/id` in manifest                   |
| `confidence` | Float confidence value for inferred annotations                |
| `format`     | File format or encoding                                        |
| `codec`      | Audio or video codec identifier                                |
| `rating`     | Content rating (explicit, clean, etc.)                         |
| `legal`      | Legal name of a person                                         |
| `marks`      | Serialized OCPN marking vector snapshot                        |
| `aura`       | AURA source file reference for a collection member             |
| `atom`       | Compiled `.atom` file reference for a member                   |
| `hami`       | Compiled `.hami` file reference for a member                   |
| `atlas`      | Compiled `.atlas` file reference for a variant mapping         |
| `access`     | Content access level — `@access/` domain value                 |
| `embargo`    | Date when `@access/embargoed` transitions to `@access/open`    |
| `live`       | Boolean true literal — "going live"                            |
| `dark`       | Boolean false literal — "going dark"                           |
| `published`  | Boolean publish flag (live = published, dark = draft)          |
| `featured`   | Boolean editorial featuring flag                               |
| `explicit`   | Boolean explicit content flag                                  |
| `cleared`    | Boolean rights clearance flag (sample, interpolation)          |
| `authored`   | `@history/take-id` — the take when this node was first recorded|
| `revised`    | `@history/take-id` or mark — the take when last changed        |
| `annotator`  | Single annotator reference for this file or node               |
| `annotators` | Multiple annotator references for this file                    |
| `contact`    | Contact address for a person or annotator node                 |
| `used-at`    | Time point in this work where a sampled element appears        |
| `trigger`    | Condition expression that fires an event node                  |
| `signal`     | Signal path emitted when an event fires                        |
| `target`     | Reference to the node this instruction or explainer applies to |
| `condition`  | Optional `@event/id` that activates an instruction             |
| `element`    | Musical element type for an interpolation node                 |
| `writers`    | People who wrote the interpolated element                      |
| `lang`       | Language of an explainer or translation node                   |
| `via`        | Annotator or person responsible for an annotation              |
| `blocked`    | Territory or boolean blocking indicator for a rights node      |
| `holder`     | Rights holder — person or legal entity                         |
| `scope`      | Rights coverage scope (document, window, etc.)                 |
| `max`        | Maximum duration for an ad slot                                |
| `performer`  | Performer reference for an instrument node                                        |
| `parent`     | Parent entity reference for studio/label inheritance hierarchy                     |
| `founded`    | ISO 8601 founding date for a studio or label entity                               |
| `logo`       | @art/id reference to a studio or label logo                                       |
| `website`    | External URL for a studio, label, or entity                                       |
| `url`        | Cloud or platform URL — art/motion/trailer asset URLs and platform availability   |
| `ratio`      | Aspect ratio of an art or motion asset (square, 16:9, etc.)                       |
| `loop`       | Boolean — live = loops, dark = plays once (motion/trailer)                        |
| `platform`   | Platform name for availability nodes (Netflix, Spotify, etc.)                     |
| `price`      | Price string with currency (e.g., "9.99 USD") for buy/rent                        |
| `currency`   | ISO 4217 currency code for buy/rent pricing                    |
| `window`     | Rental access period (e.g., "30d", "48h") for rent nodes       |
| `drm`        | DRM status — live = DRM protected, dark = DRM free             |
| `quality`    | Playback quality (4k, hd, sd, lossless, audio-only)            |
| `provider`   | LLM provider name in configs/llm.aura                          |
| `model`      | LLM model identifier in configs/llm.aura                       |
| `endpoint`   | Local LLM endpoint URL for ollama or self-hosted providers     |
| `auth`       | @account/id reference for credential lookup                    |
| `env`        | Environment variable name(s) for credential resolution         |
| `cover`      | @art/id reference to primary cover art                         |
| `motion`     | @motion/id reference to motion cover asset                     |
| `trailer`    | @trailer/id reference to primary trailer                       |
| `studio`     | @studio/id reference to production studio                      |
| `label`      | @label/id reference to record label                            |

---

## Approved Hyphenated Keys

Hyphens are permitted only when no single word cleanly carries the full meaning.

| Key              | Reason                                                       |
| ---------------- | ------------------------------------------------------------ |
| `pre-chorus`     | Recognized song section with no single-word equivalent       |
| `post-chorus`    | Same as above                                                |
| `lead-vocal`     | Distinguishes from backing, harmony, and ad-lib roles        |
| `co-writer`      | The co- prefix is the only way to express co-authorship      |
| `voice-over`     | An established compound industry term                        |
| `rights-holder`  | Holder alone is ambiguous; rights context is required        |
| `fill-policy`    | Policy alone is ambiguous; fill-policy is the ad term        |
| `mood-vocabulary`| Directive key for a vocabulary declaration block             |
| `aura-ref`       | AURA source file reference within a collection member block  |
| `persons-ref`    | People index reference within a collection manifest          |

---

## Media Kinds

Every AURA document declares its `kind` in the `schema::` block.

### Audio

| Value              | Meaning                                     |
| ------------------ | ------------------------------------------- |
| `audio::music`     | Album, EP, single, or musical work          |
| `audio::podcast`   | Podcast episode or show                     |
| `audio::audiobook` | Spoken word with chapters                   |
| `audio::live`      | Live recording                              |

### Video

| Value                  | Meaning                                     |
| ---------------------- | ------------------------------------------- |
| `video::movie`         | Feature or short film                       |
| `video::series`        | Episodic series                             |
| `video::podcast`       | Video podcast episode                       |
| `video::documentary`   | Documentary work                            |
| `video::music`         | Music video                                 |
| `video::live`          | Live performance or concert                 |
| `video::short`         | Short-form content under 10 minutes         |

### Mixed

| Value               | Meaning                                     |
| ------------------- | ------------------------------------------- |
| `mixed::album`      | Visual album — audio and video tied         |
| `mixed::interactive`| Interactive or branching media              |

---

## Enumerated Values

### Segment Kinds

Used in `segments::` support nodes under the `kind` key.

```text
intro | verse | pre-chorus | chorus | post-chorus | bridge |
drop | breakdown | outro | interlude | instrumental | transition |
ad-lib | hook | custom
```

### Sample Kinds

Used in `samples::` support nodes under the `kind` key.

```text
loop | stab | chop | vocal | melodic | rhythmic | atmospheric | custom
```

### Interpolation Elements

Used in `interpolations::` support nodes under the `element` key.

```text
melody | chord-progression | lyric | rhythm | hook | bassline | custom
```

### Explainer Kinds

Used in `explainers::` support nodes under the `kind` key.

```text
cultural | lyrical | historical | technical | translation | annotation | custom
```

### Tempo Types

Used in `tempo::` support nodes under the `type` key.

```text
steady | increasing | decreasing | variable | free
```

### Instruction Kinds

Used in `instructions::` support nodes under the `kind` key.

```text
loop | skip | jump | repeat | fade | crossfade | trim | mute | custom
```

### Instruction Fade Types

Used under `type` in fade instructions.

```text
linear | exponential | logarithmic
```

### Event Kinds

Used in `events::` support nodes under the `kind` key.

```text
ambient | reactive | interactive | editorial | broadcast | custom
```

### Event Trigger `at` Values

When within a trigger's interval the event fires.

```text
onset | offset | peak | @time/value
```

### Anchor Kinds

Used in `anchors::` support nodes under the `kind` key.

```text
hard | verified | soft
```

### Rights Scope

Used in `rights::` support nodes under the `scope` key.

```text
document | window
```

### Ad Slot Fill Policy

Used in `slots::` support nodes under the `fill-policy` key.

```text
optional | required | house
```

### Ad Slot Kinds

Used in `slots::` support nodes under the `kind` key.

```text
pre-roll | mid-roll | post-roll
```

### Inference Source

Used in `moods::` support nodes under the `source` key.

```text
authored | hybrid | inferred
```

---

## History Vocabulary

AURA-native versioning terms. Git verbs (`commit`, `branch`, `tag`, `push`, etc.) are not used.

| AURA term    | Meaning                                                   | Analogy                          |
| ------------ | --------------------------------------------------------- | -------------------------------- |
| `take`       | An immutable snapshot of the current document state       | Studio take ("take one")         |
| `mark`       | A human-readable name attached to a specific take         | Cue mark, chapter mark           |
| `stream`     | A named parallel line of development                      | Audio recording stream           |
| `main`       | The primary line of development within a project          | Main recording session           |
| `delta`      | The set of changes between any two takes                  | Signal differential              |
| `rewind`     | Restore the draft to a previous take (non-destructive)    | Tape rewind                      |
| `mix`        | Combine two streams into one                              | Audio mixing                     |
| `ledger`     | The full ordered and permanent history of all takes       | Production ledger                |
| `hold`       | Park the current draft without recording a take           | "Put on hold"                    |
| `recall`     | Load a specific take as the working session               | Session recall                   |
| `release`    | Publish the current take to cloud distribution            | Releasing a record               |
| `sync`       | Pull the latest released state from cloud                 | Syncing from master archive      |
| `dub`        | Create an independent full-history copy of the project    | Dubbing a tape                   |
| `draft`      | The current uncommitted working state                     | Working draft before a take      |
| `origin`     | The first take ever recorded for a file                   | The first session                |
| `current`    | The latest take in the ledger                             | Now playing                      |

---

## ID Prefix Reference

Every AURA ID begins with a type prefix. Prefixes are one or two characters.

| Prefix | Class        | Notes                                                |
| ------ | ------------ | ---------------------------------------------------- |
| `t`    | track        | An audio music track                                 |
| `c`    | collection   | Album, EP, single, or compilation manifest           |
| `p`    | person       | A contributor, creator, or any named individual      |
| `v`    | variant      | An alternate version of any content file             |
| `ep`   | episode      | A single episode in a series or podcast              |
| `sn`   | season       | A season within a series or podcast                  |
| `s`    | season-item  | A season file within a series folder                 |
| `tv`   | series       | A TV, podcast, or episodic series root manifest      |
| `f`    | film         | A feature or short film                              |
| `dc`   | documentary  | A documentary work                                   |
| `pc`   | podcast      | A podcast series root manifest                       |
| `an`   | animation    | An animated or anime series root manifest            |
| `sp`   | speech       | A speech, lecture, talk, or commencement address     |
| `b`    | book         | An audiobook                                         |
| `mv`   | music video  | A music video                                        |
| `sg`   | single       | A single release                                     |
| `cy`   | interview    | A discrete interview file                            |
| `r`    | rights       | A rights or licensing declaration file               |
| `i`    | info         | An info document (metadata, credits, labels)         |
| `tx`   | take         | A history take (immutable version snapshot)          |
| `st`   | studio       | A production studio or broadcast network entity      |
| `lb`   | label        | A record label or publishing imprint                 |
| `ar`   | art          | A static image art asset (cover art, poster)         |
| `mo`   | motion       | An animated motion cover or looping video asset      |
| `tr`   | trailer      | A promotional trailer or preview clip                |

ID format: `{prefix}{6 alphanumeric chars}` using charset `a-z0-9`.

```text
t7xab3c    <- track
p4xt9k2    <- person
ep7xb3n    <- episode
c8xab3d    <- collection
tx3ab7k    <- take
```

---

## ATOM Node Class Reference

The `node_class` byte in ATOM's interval tree entry allows stabbing queries
to filter by node type at SIMD evaluation time.

| Class  | Type           | Description                                           |
| ------ | -------------- | ----------------------------------------------------- |
| `0x01` | content        | Macro, Meso, Micro, Nano, Pico content nodes          |
| `0x02` | segment        | Musical/structural section marker                     |
| `0x03` | instrument     | Instrument activity window                            |
| `0x04` | chapter        | Navigable chapter division                            |
| `0x05` | credit         | Time-windowed contributor credit window               |
| `0x06` | translation    | Language overlay for a content node                   |
| `0x07` | mood           | Emotional or tonal annotation window                  |
| `0x08` | rights         | Licensing or territorial boundary                     |
| `0x09` | slot           | Advertising insertion point                           |
| `0x0A` | anchor         | Hard synchronization recovery point                   |
| `0x0B` | annotator      | First-class annotator attribution node                |
| `0x0C` | vocab          | Genre, role, or mood vocabulary node                  |
| `0x0D` | event          | Condition-triggered signal node                       |
| `0x0E` | tempo          | BPM and time-signature window                         |
| `0x0F` | sample         | Audio sample attribution node                         |
| `0x10` | explainer      | Explanation or gloss node                             |
| `0x11` | interpolation  | Re-recorded composition element node                  |
| `0x12` | instruction    | Processing directive node                             |
| `0x13` | access         | Content visibility and permission node                |
| `0x14` | history        | Versioned take and delta chain node                   |
| `0x15` | art            | Static cover art or image asset node                  |
| `0x16` | motion         | Animated motion cover or looping video node           |
| `0x17` | trailer        | Promotional trailer or preview clip node              |
| `0x18` | studio         | Production studio entity node                         |
| `0x19` | label          | Record label or publishing imprint entity node        |
| `0x1A` | watch          | Streaming platform availability node                  |
| `0x1B` | buy            | Purchase availability node                            |
| `0x1C` | rent           | Rental availability node                              |
| `0x1D` | download       | Download availability node                            |

---

## Toolchain Commands

All commands are issued via the `aura` CLI.

### ID Generation

| Command                       | Output           | Description                          |
| ----------------------------- | ---------------- | ------------------------------------ |
| `aura generate track`         | `t7xab3c`        | Generate a new track ID              |
| `aura generate person`        | `p4xt9k2`        | Generate a new person ID             |
| `aura generate episode`       | `ep7xb3n`        | Generate a new episode ID            |
| `aura generate collection`    | `c8xab3d`        | Generate a new collection ID         |
| `aura generate annotator`     | `p9xb3mn`        | Generate a new annotator ID          |
| `aura generate season`        | `sn2kr9l`        | Generate a new season ID             |
| `aura generate variant`       | `v3qr7st`        | Generate a new variant ID            |

### History(.history)

| Command                          | Description                                               |
| -------------------------------- | --------------------------------------------------------- |
| `aura take`                      | Record current draft as a new immutable take              |
| `aura take "message"`            | Record a take with a descriptive message                  |
| `aura mark name`                 | Attach a human-readable name to the current take          |
| `aura rewind take-id`            | Restore draft to a specific take by ID                    |
| `aura rewind mark-name`          | Restore draft to a named mark                             |
| `aura rewind ~n`                 | Restore draft n takes before current                      |
| `aura stream open name`          | Open a new named development stream                       |
| `aura stream close name`         | Close and archive a named stream                          |
| `aura stream list`               | List all open streams                                     |
| `aura mix stream-name`           | Mix a stream into the current stream                      |
| `aura delta take-a take-b`       | Show all changed nodes between two takes                  |
| `aura delta mark-name current`   | Show changes between a mark and the current state         |
| `aura ledger`                    | Show the full take history for this file                  |
| `aura ledger node/path`          | Show the take history for a specific node                 |
| `aura hold`                      | Park the current working draft without taking             |
| `aura hold restore`              | Restore a previously parked draft                         |
| `aura release`                   | Publish the current take to `@aduki.org` distribution     |
| `aura sync`                      | Pull the latest released state from `@aduki.org`          |
| `aura dub`                       | Create an independent full-history copy of the project    |

### Compilation

| Command            | Description                                                       |
| ------------------ | ----------------------------------------------------------------- |
| `aura compile`     | Compile `.aura` source files to `.atom` and `.hami` artifacts     |
| `aura validate`    | Validate syntax and reference resolution without compiling        |
| `aura lint`        | Check for style violations and best practice warnings             |

---

## Info and Meta Files : (Base)

### `info/` Folder — Project-Specific Data

| File                       | Namespace                 | Required                         |
| -------------------------- | ------------------------- | -------------------------------- |
| `info/people.aura`         | `people::`                | Always                           |
| `info/annotators.aura`     | `annotators::`            | Always                           |
| `info/metadata.aura`       | `schema::` + `manifest::` | Always                           |
| `info/credits.aura`        | `credits::`               | Albums, films                    |
| `info/rights.aura`         | `rights::`                | When applicable                  |
| `info/labels.aura`         | `labels::`                | Music projects with label info   |
| `info/studios.aura`        | `studios::`               | Film, TV, animation              |
| `info/arts.aura`           | `arts::` + `motions::` + `trailers::` | When media assets declared |
| `info/availability.aura`   | `watch::` + `buy::` + `rent::` + `download::` | When availability declared |
| `info/releases.aura`       | varies                    | When applicable                  |

### `namespace.aura` Files — Project and Folder Manifests

| File                       | Purpose                                                         |
| -------------------------- | --------------------------------------------------------------- |
| `namespace.aura` (root)    | Project entry point: declares namespace, exports sub-namespaces |
| `info/namespace.aura`      | Lists all files contained in the `info/` folder                 |
| `meta/namespace.aura`      | Lists all files contained in the `meta/` folder                 |
| `tracks/namespace.aura`    | Lists all tracks with their names                               |
| `episodes/namespace.aura`  | Lists all episodes                                              |
| `scenes/namespace.aura`    | Lists all scene files                                           |

### `configs/` Folder — Non-Compiled Toolchain Config

Never compiled to `.atom` / `.hami`. Never tracked by `.history/`.

| File                       | Namespace     | Purpose                                              |
| -------------------------- | ------------- | ---------------------------------------------------- |
| `configs/llm.aura`         | `llm::`       | LLM provider definitions for editor integration      |
| `configs/stores.aura`      | `stores::`    | Remote store origins and authentication references   |
| `configs/account.aura`     | `accounts::`  | Cloud identity — reads from `.env` or env variables  |
| `configs/ignore.aura`      | `ignore::`    | Paths excluded from `.history/` tracking             |

### `meta/` Folder — Vocabulary Definitions

Vocabulary nodes use slug IDs (e.g. `electronic`, `main-artist`), not generated hex IDs.

| File               | Namespace   | Required        |
| ------------------ | ----------- | --------------- |
| `meta/genres.aura` | `genres::`  | When vocab used |
| `meta/roles.aura`  | `roles::`   | When vocab used |
| `meta/moods.aura`  | `moods::`   | When vocab used |

### Vocabulary Resolution Cascade

When the engine encounters a `@genre/`, `@role/`, or `@mood/` reference:

1. Local `meta/` folder for this project
2. Parent catalog's `meta/` folder (if inside a catalog)
3. Global platform vocabulary at `@aduki.org/genre/`, `@aduki.org/role/`, `@aduki.org/mood/`
4. Not found → stored as string literal, compile warning in strict mode

---

## Signal Path Convention

Used in `event::` nodes under the `signal` key. Multiple signals separated by `|`.

```text
lights::dim(0.1)               <- dim lights to 10%
lights::colour(#0a1628)        <- set light colour
lights::strobe                 <- strobe
lights::fade(white, 30s)       <- fade to white over 30 seconds
haptic::pulse(80ms)            <- haptic motor burst
ar::overlay(frost)             <- AR frost overlay
display::credits               <- show credits display
player::insert-ad              <- trigger ad insertion
player::pause                  <- pause playback
player::skip                   <- skip current segment
iot::gpio(17, HIGH)            <- raw GPIO for custom hardware
```

Format: `subsystem::action(params)` where params are optional.

---

## Time Notation Quick Reference

```text
## Full triple — explicit
time -> [22s, 1m10s, 48s]

## Range — start ~ end (engine derives duration)
time -> 22s~1m10s

## Duration offset — start + duration (engine derives end)
time -> 2m30s+28s

## Point anchor (no duration)
sync-point -> @time/1m32s

## Cross-file time reference
sampled -> @track/t9vkx7q @time/2m44s
```

Time units: `s` (seconds), `m` (minutes), `h` (hours), `ms` (milliseconds).
Full timestamps for film/video: `HH:MM:SS`, `HH:MM:SS.mmm`.

---

## @history Reference Quick Reference

```text
@history/current                  <- latest take
@history/origin                   <- first take ever
@history/premiere                 <- named mark
@history/v1.0                     <- another named mark
@history/~1                       <- one take back
@history/~3                       <- three takes back
@history/tx3ab7k                  <- specific take by ID
@history/stream/translation-fr    <- head of a stream
@history/main                     <- head of main stream

## Node state at a specific take
@history/v1.0::verse/one
@history/~3::chorus/one/line/two
@history/premiere::credits

## Global cloud form
@aduki.org/track/t7xab3c/history/premiere
@aduki.org/track/t7xab3c/history/~1::verse/one
```

---

*AURA Keyword Reference — Engine Documentation*
*See `flux.md` for syntax, `structure.md` for structure, `changes.md` for engine internals.*
