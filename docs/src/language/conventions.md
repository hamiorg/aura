# AURA (Adaptive Universal Relational Annotations) — Structure Reference

## ID System, Key Conventions, File References, and Folder Layouts

> This document defines the structural conventions of AURA before any content
> is authored. It covers how identifiers are generated and used, how keys are
> written, how files reference each other locally and globally, how the info
> folder system works, and how the folder tree is organized for every supported
> media type. File names carry no human-readable meaning. Names live inside
> the files. File names are IDs.

---

## Part I — The Identifier System

### Why IDs, Not Names

File names in an AURA project carry no human-readable information. The name
of a track, the title of an episode, the full name of a contributor — all
of that lives inside the .aura file, inside the manifest or persons block.
The file system sees only IDs.

This design serves three purposes.

First, it eliminates naming collisions globally. Two different artists may
both have a track called "Fade Out" or a scene called "Cold Open." When files
are named by generated IDs, those titles coexist without conflict in any
shared store.

Second, it makes renaming a non-event. Changing a title is editing one line
inside a .aura file. No file is renamed. No reference breaks. No downstream
link is invalidated.

Third, it makes cloud references unambiguous and permanent. Once an ID is
issued, it is the stable address of that object forever. Any reference to
it — local or global — resolves to exactly one thing.

---

### ID Format

An AURA ID is a short alphanumeric string composed of a one or two character
type prefix followed by six lowercase alphanumeric characters.

    Format:   {prefix}{6 chars}
    Charset:  a-z, 0-9 (36 possible values per character position)
    Length:   7 to 8 characters total depending on prefix length

The 6-character body gives 36^6 = 2,176,782,336 unique values per prefix.
Collisions within a prefix space are statistically negligible and are
additionally prevented by the ID generator which registers each issued ID
against the active store before returning it.

Examples of well-formed IDs:

    t7xab3c     track
    t4mn2rp     track
    c8xab3d     collection (album, ep, single, compilation)
    p4xt9k2     person
    v3qr7st     variant
    ep7xb3n     episode
    sn2kr9l     season
    tv4x7ab     TV series manifest
    f6np2qr     film
    dc3wr8x     documentary
    pc5xk4m     podcast series
    an9vl3b     animation or anime series
    sp2xr7n     speech
    b8mt4kx     audiobook
    mv6xp3l     music video
    sg4xr9t     single release
    cy3wp8n     interview
    r1xb7kp     rights declaration
    i0xmt3q     info document (metadata, credits, labels)

---

### Prefix Reference

Every ID begins with its type prefix. The prefix encodes what the object is.
A system reading any ID can determine the object class from the prefix alone,
without any additional context.

| Prefix | Class              | Notes                                            |
| ------ | ------------------ | ------------------------------------------------ |
| t      | track              | An audio music track                             |
| c      | collection         | Album, EP, single, or compilation manifest       |
| p      | person             | A contributor, creator, or any named individual  |
| v      | variant            | An alternate version of any content file         |
| ep     | episode            | A single episode in a series or podcast          |
| sn     | season             | A season within a series or podcast              |
| s      | season-item        | A season file within a series folder             |
| tv     | series             | A TV, podcast, or episodic series root manifest  |
| f      | film               | A feature or short film                          |
| dc     | documentary        | A documentary work                               |
| pc     | podcast            | A podcast series root manifest                   |
| an     | animation          | An animated or anime series root manifest        |
| sp     | speech             | A speech, lecture, talk, or commencement address |
| b      | book               | An audiobook                                     |
| mv     | music video        | A music video                                    |
| sg     | single             | A single release (when not using collection)     |
| cy     | interview          | A discrete interview file                        |
| r      | rights             | A rights or licensing declaration file           |
| i      | info               | An info document (metadata, credits, labels)     |
| tx     | take               | A history take (immutable version snapshot)      |
| st     | studio             | A production studio or broadcast network entity  |
| lb     | label              | A record label or publishing imprint             |
| ar     | art                | A static image art asset (cover art, poster)     |
| mo     | motion             | An animated motion cover or looping video asset  |
| tr     | trailer            | A promotional trailer or preview clip            |

---

### ID Generation

IDs are generated by the AURA toolchain. No ID is hand-authored. The
generator produces IDs on demand, checks each against the active store's
ID registry, and retries on the rare collision before returning the ID
to the caller.

In local development, the generator uses a local registry file at the
project root. In cloud deployments, the store's ID registry is the
authoritative source. The generator is always invoked — IDs are never
typed manually.

The generation command follows this pattern (exact toolchain syntax is
defined in the Engine Reference):

    aura generate track         -> t7xab3c
    aura generate person        -> p4xt9k2
    aura generate episode       -> ep7xb3n
    aura generate collection    -> c8xab3d

The generator returns the ID. The ID becomes the file name and the
canonical reference for that object everywhere.

---

### A Note on Scene and Act Files

Scene and act files inside film, music-video, and documentary projects use
generated IDs whose prefix is drawn from the parent content kind. For example,
a scene file in a film project may carry an `f`-prefixed ID because it belongs
to that film's namespace. There is no separate `sc` or `ac` prefix — scene and
act files are sub-objects of their parent manifest and share its prefix space.

The folder they reside in (`scenes/`, `acts/`) defines their content role;
the ID prefix encodes their parent kind. This is intentional: it keeps the
prefix table small and makes parent ownership legible from the ID alone.

---

## Part II — The Reference System

### Local References

Within a project, any AURA file references another file or node using the
@ sigil followed by the type domain and the ID.

    ## Reference a track file from a collection manifest
    aura -> @track/t7xab3c

    ## Reference an episode file
    aura -> @episode/ep7xb3n

    ## Reference a variant file
    aura -> @variant/v3qr7st

    ## Reference a person defined in info/people.aura
    producer -> @person/p4xt9k2

    ## Multiple people — plural domain
    cast     -> @people/[mt4qbz, vr8kfw]

    ## Reference the info folder's people file
    >> @info/people

    ## Reference the info folder's metadata file
    >> @info/metadata

The engine resolves local references relative to the project root. The type
domain — track, episode, variant, person — matches the subfolder where files
of that type reside. The engine knows where to look from the domain alone.

---

### Global Cloud References

When a reference must be globally unambiguous — crossing catalog boundaries,
cited from an external system, embedded in a published record — it uses the
full global URI form with the aduki.org domain.

    ## Global reference to a specific track
    @aduki.org/track/t7xab3c

    ## Global reference to a specific person
    @aduki.org/person/p4xt9k2

    ## Global reference to a person via the info path
    @aduki.org/people/p4xt9k2

    ## Global reference to an episode
    @aduki.org/episode/ep7xb3n

    ## Global reference to a collection manifest
    @aduki.org/collection/c8xab3d

    ## Global reference to a season within a series
    @aduki.org/series/tv4x7ab/season/sn2kr9l

    ## Global reference to an episode within a season
    @aduki.org/series/tv4x7ab/season/sn2kr9l/episode/ep7xb3n

    ## Global reference to an info document
    @aduki.org/people/p4xt9k2
    @aduki.org/info/metadata/c8xab3d

The global URI is the same ID — only the prefix domain changes. A track
referenced locally as @track/t7xab3c is referenced globally as
@aduki.org/track/t7xab3c. The ID body never changes.

---

### Short Global Form

Because the type prefix is embedded in the ID itself, a short global form
is also valid when the calling system can resolve the prefix to the type:

    @aduki.org/t7xab3c       -> resolves to track t7xab3c
    @aduki.org/p4xt9k2       -> resolves to person p4xt9k2
    @aduki.org/ep7xb3n       -> resolves to episode ep7xb3n
    @aduki.org/c8xab3d       -> resolves to collection c8xab3d

The short form is used in tight contexts like credit fields, relation
declarations, and sampling references. The full path form is used in
collection manifests, rights instruments, and any context where an auditor
or external system must resolve the reference without engine assistance.

---

### Node Path References

Within a file, specific nodes are addressed by following the slash path
from the node type down to the target node. These are in-file references:
they do not cross file boundaries.

    @verse/one
    @chorus/two
    @scene/cold-open
    @act/one
    @chapter/interview-begins
    @line/one
    @word/three
    @syllable/one
    @anchor/chorus-one

Nested nodes are addressed by extending the path:

    @verse/one/line/three
    @verse/two/line/one/word/four
    @act/two/scene/cold-open

Cross-file node references combine the file ID with the in-file node path:

    @track/t7xab3c::verse/two/line/one
    @episode/ep7xb3n::scene/cold-open
    @aduki.org/track/t7xab3c::verse/two/line/one

The :: leap signals a cross-boundary reference — either into a different
file or into a different memory region in the compiled ATOM mesh.

---

### The Full Reference Grammar

    ## Local: type domain + file ID
    @track/t7xab3c
    @episode/ep7xb3n
    @person/p4xt9k2
    @variant/v3qr7st

    ## Local: info folder document
    @info/people
    @info/metadata
    @info/credits
    @info/rights

    ## Local: in-file node
    @verse/one
    @chorus/two/line/three

    ## Local: cross-file node
    @track/t7xab3c::verse/two
    @episode/ep7xb3n::scene/cold-open

    ## Global: full path form
    @aduki.org/track/t7xab3c
    @aduki.org/person/p4xt9k2
    @aduki.org/people/p4xt9k2
    @aduki.org/series/tv4x7ab/season/sn2kr9l/episode/ep7xb3n

    ## Global: short form (prefix-resolved)
    @aduki.org/t7xab3c
    @aduki.org/p4xt9k2

    ## Global: cross-file node
    @aduki.org/track/t7xab3c::verse/two/line/one

    ## Temporal anchor
    @time/1m32s

    ## Singular people forms
    @person/p4xt9k2
    @author/p4xt9k2

    ## Plural people forms
    @people/[p4xt9k2, j8mn2rk]
    @authors/[p4xt9k2, j8mn2rk]

    ## Sampling reference (file + time point)
    @track/t7xab3c @time/2m44s

---

### Reference Domain Index

All domains follow the singular/plural convention:
`@entity/id` for one, `@entities/[a, b]` for many.

#### People

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @person/             | singular | info/people.aura or global @aduki.org/person/            |
| @people/[a, b]       | plural   | info/people.aura or global                               |
| @author/             | singular | alias for @person/ — identical resolution                |
| @authors/[a, b]      | plural   | alias for @people/[a, b]                                 |
| @annotator/          | singular | info/annotators.aura or global @aduki.org/annotators/    |
| @annotators/[a, b]   | plural   | info/annotators.aura or global                           |

#### Vocabulary

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @genre/slug          | singular | meta/genres.aura or global vocab                         |
| @genres/[a, b]       | plural   | meta/genres.aura or global vocab                         |
| @role/slug           | singular | meta/roles.aura or global vocab                          |
| @roles/[a, b]        | plural   | meta/roles.aura or global vocab                          |
| @mood/slug           | singular | meta/moods.aura or global vocab                          |
| @moods/[a, b]        | plural   | meta/moods.aura or global vocab                          |

#### Content Files

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @track/              | singular | tracks/ folder by generated ID                           |
| @tracks/[a, b]       | plural   | tracks/ folder by generated IDs                          |
| @episode/            | singular | episodes/ folder by generated ID                         |
| @episodes/[a, b]     | plural   | episodes/ folder by generated IDs                        |
| @scene/              | singular | scenes/ folder by generated ID                           |
| @scenes/[a, b]       | plural   | scenes/ folder                                           |
| @variant/            | singular | variants/ folder by generated ID                         |
| @collection/         | singular | collection manifest file by ID                           |
| @season/             | singular | season subfolder manifest by ID                          |
| @member/             | singular | another member in the same collection                    |

#### Time, Sync, and Tempo

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @time/               | singular | a temporal point anchor in the current file              |
| @tempo/              | singular | a tempo node in the current file                         |
| @anchor/             | singular | a sync anchor node in the current file                   |

#### Music Attribution

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @sample/             | singular | a sample reference node                                  |
| @samples/[a, b]      | plural   | multiple sample reference nodes                          |
| @interpolation/      | singular | a musical interpolation node                             |
| @interpolations/[a]  | plural   | multiple interpolation nodes                             |

#### Annotation and Context

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @explainer/          | singular | an explanation node for any content or support node      |
| @explainers/[a, b]   | plural   | multiple explanation nodes                               |
| @instruction/        | singular | a processing instruction node                            |
| @instructions/[a, b] | plural   | multiple processing instruction nodes                    |

#### Events

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @event/              | singular | a condition-triggered signal node                        |
| @events/[a, b]       | plural   | multiple event nodes                                     |

#### Info and Meta Files

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @info/people         | singular | info/people.aura for this project                        |
| @info/annotators     | singular | info/annotators.aura for this project                    |
| @info/metadata       | singular | info/metadata.aura for this project                      |
| @meta/genres         | singular | meta/genres.aura for this project                        |
| @meta/roles          | singular | meta/roles.aura for this project                         |
| @meta/moods          | singular | meta/moods.aura for this project                         |

#### Media Assets

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @art/                | singular | info/arts.aura or global @aduki.org/art/                 |
| @arts/[a, b]         | plural   | info/arts.aura or global                                 |
| @motion/             | singular | info/arts.aura motions block or global @aduki.org/motion/|
| @motions/[a, b]      | plural   | info/arts.aura motions block or global                   |
| @trailer/            | singular | info/arts.aura trailers block or @aduki.org/trailer/     |
| @trailers/[a, b]     | plural   | info/arts.aura trailers block or global                  |

#### Industry Entities

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @studio/             | singular | info/studios.aura or global @aduki.org/studio/           |
| @studios/[a, b]      | plural   | info/studios.aura or global                              |
| @label/              | singular | info/labels.aura or global @aduki.org/label/             |
| @labels/[a, b]       | plural   | info/labels.aura or global                               |

#### Content Availability

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @watch/              | singular | info/availability.aura watch block                       |
| @watch/[a, b]        | plural   | multiple streaming platform entries                      |
| @buy/                | singular | info/availability.aura buy block                         |
| @buy/[a, b]          | plural   | multiple purchase options                                |
| @rent/               | singular | info/availability.aura rent block                        |
| @rent/[a, b]         | plural   | multiple rental options                                  |
| @download/           | singular | info/availability.aura download block                    |
| @download/[a, b]     | plural   | multiple download options                                |

#### Cloud

| Domain prefix        | Resolves to                                              |
| -------------------- | -------------------------------------------------------- |
| @aduki.org/          | Global cloud URI — all domains available via path        |

#### Access

| Domain prefix        | Form     | Resolves to                                              |
| -------------------- | -------- | -------------------------------------------------------- |
| @access/open         | singular | public — unrestricted access                             |
| @access/locked       | singular | private — owner/auth only                                |
| @access/gated        | singular | conditional — subscription, payment, or role required    |
| @access/embargoed    | singular | time-locked — transitions to open after embargo date     |
| @access/archived     | singular | accessible but retired — historical access               |
| @access/restricted   | singular | geo- or rights-restricted — named territories only       |

#### History

| Domain prefix             | Form     | Resolves to                                         |
| ------------------------- | -------- | --------------------------------------------------- |
| @history/current          | singular | the latest take in this file's ledger               |
| @history/origin           | singular | the first take ever recorded for this file          |
| @history/take-id          | singular | a specific take by generated ID                     |
| @history/mark-name        | singular | a named mark on the ledger                          |
| @history/~n               | singular | n takes before current (relative)                   |
| @history/stream/name      | singular | the head of a named development stream              |
| @history/main             | singular | the primary line of development                     |
| @history/take-id::path    | singular | a specific node's state at a given take             |

#### Reserved (Future)

| Domain prefix        | Meaning                                                  |
| -------------------- | -------------------------------------------------------- |
| @thread/             | reserved — future parallel thread support                |
| @parallel/           | reserved — future parallel execution node                |

When referencing in-file node types (verse, chorus, line, word, etc.) the
domain is the node type itself, not a folder. These resolve within the
current file unless combined with a file ID via the :: leap.

---

## Part III — Node Identifier Convention

### The Slash Identifier

Every named node in an AURA document uses the slash convention. The name
before the slash is the node type. The name after the slash is the ordinal
or a unique meaningful label.

Ordinals are written as full English words. No numerals, dots, or underscores
in node identifiers.

    verse/one::
    verse/two::
    chorus/one::
    chorus/two::
    bridge/one::
    act/one::
    act/two::
    scene/one::
    scene/cold-open::
    chapter/one::
    chapter/interview-begins::
    line/one::
    line/two::
    word/one::
    syllable/one::
    letter/one::

When a node has a unique meaningful name rather than an ordinal position,
the label after the slash is that name, lowercase with hyphens if compound.

    scene/cold-open::
    scene/the-diagnostic::
    scene/root-node::
    chapter/interview-begins::
    bridge/breakdown::
    anchor/chorus-one::

---

### Ordinal Word Reference

| Position | Word        | Position | Word          |
| -------- | ----------- | -------- | ------------- |
| 1        | one         | 11       | eleven        |
| 2        | two         | 12       | twelve        |
| 3        | three       | 13       | thirteen      |
| 4        | four        | 14       | fourteen      |
| 5        | five        | 15       | fifteen       |
| 6        | six         | 16       | sixteen       |
| 7        | seven       | 17       | seventeen     |
| 8        | eight       | 18       | eighteen      |
| 9        | nine        | 19       | nineteen      |
| 10       | ten         | 20       | twenty        |

Beyond twenty: twenty-one, twenty-two, and so on as full hyphenated words.
Season/one, episode/twenty-two, line/thirty-five — the pattern extends without
any change in convention.

---

### Node Type Vocabulary

Content node types and their levels:

| Type        | Level  | Used In                                      |
| ----------- | ------ | -------------------------------------------- |
| act         | Macro  | film, stage, long-form video                 |
| scene       | Macro  | film, music video, animation, documentary    |
| shot        | Macro  | film, music video — camera unit              |
| verse       | Macro  | song lyrics                                  |
| chorus      | Macro  | song lyrics                                  |
| bridge      | Macro  | song lyrics                                  |
| intro       | Macro  | song, podcast, speech                        |
| outro       | Macro  | song, podcast, speech                        |
| hook        | Macro  | song lyrics                                  |
| drop        | Macro  | electronic music                             |
| interlude   | Macro  | song, album                                  |
| breakdown   | Macro  | song                                         |
| pre-chorus  | Macro  | song lyrics                                  |
| post-chorus | Macro  | song lyrics                                  |
| chapter     | Macro  | audiobook, podcast, documentary              |
| segment     | Macro  | speech, lecture, panel                       |
| section     | Macro  | any long-form                                |
| line        | Meso   | all content types                            |
| dialogue    | Meso   | film, series, podcast — speaker-attributed   |
| word        | Micro  | all content types                            |
| token       | Micro  | transcription systems                        |
| syllable    | Nano   | song, speech                                 |
| phoneme     | Nano   | speech, accessibility                        |
| letter      | Pico   | animation, 60fps rendering                   |
| character   | Pico   | non-Latin scripts                            |

Support node types:

| Type           | Purpose                                                 |
| -------------- | ------------------------------------------------------- |
| segment        | Musical section marker (under support::)                |
| instrument     | Instrument activity window                              |
| mood           | Emotional or tonal annotation window                    |
| rights         | Licensing or territorial boundary                       |
| translation    | Language overlay for a content node                     |
| credit         | Time-windowed contributor credit                        |
| slot           | Advertising insertion point                             |
| anchor         | Hard synchronization recovery point                     |
| tempo          | BPM and time-signature window, affects lyric sync       |
| sample         | Audio sample attribution (source, kind, clearance)      |
| explainer      | Explanation or gloss attached to any node               |
| interpolation  | Re-recorded composition element attribution             |
| instruction    | Processing directive to engine or player                |
| event          | Condition-triggered signal for reactive systems         |
| access         | Content visibility and permission level node            |
| history        | Versioned take and delta chain provenance node          |
| art            | Static cover art or image asset (manifest-level)        |
| motion         | Animated motion cover or looping video (manifest-level) |
| trailer        | Promotional trailer or preview clip (manifest-level)    |
| studio         | Production studio entity with ownership hierarchy       |
| label          | Record label or publishing imprint with hierarchy       |
| watch          | Streaming platform availability entry                   |
| buy            | Purchase availability entry with pricing                |
| rent           | Rental availability entry with pricing and window       |
| download       | Download availability entry                             |

---

## Part IV — Key Naming Convention

### The Single-Word Rule

Keys in AURA are written as single lowercase words wherever the language
allows it. Hyphens are used only when there is genuinely no single word
that carries the same meaning without ambiguity.

The goal is readability. An AURA file should read as close to natural prose
as the structure permits.

---

### Standard Keys

| Key       | Meaning                                                                             |
| --------- | ----------------------------------------------------------------------------------- |
| name      | Human-readable title of this node (legacy person field)                             |
| first     | Given name of a person node                                                         |
| middle    | Middle name(s) of a person node — optional                                          |
| last      | Family name of a person node — optional for mononyms                                |
| screen    | Short on-screen identifier for captions, dialogue, mini-player                      |
| kind      | Type or category within this node's class                                           |
| time      | Temporal interval of this node                                                      |
| duration  | Total length as a standalone declared value                                         |
| text      | Text payload of a content node                                                      |
| locale    | IETF BCP 47 language tag                                                            |
| script    | Explicit script (Latn, Arab, Cyrl, etc.)                                            |
| label     | Short human-readable tag or marker                                                  |
| genre     | Genre descriptor, may be a union list                                               |
| released  | ISO 8601 release date                                                               |
| territory | Geographic scope                                                                    |
| version   | Semantic version string                                                             |
| creator   | Primary creator reference                                                           |
| speaker   | Active speaker reference at this node                                               |
| speakers  | Multiple speaker references at this node                                            |
| cast      | Cast list                                                                           |
| host      | Podcast or show host reference                                                      |
| guest     | Guest speaker reference                                                             |
| language  | Primary language of this document                                                   |
| country   | Country of origin                                                                   |
| city      | City of origin                                                                      |
| born      | Date of birth for a person node                                                     |
| bio       | Biography or description text                                                       |
| note      | Annotation or editorial note                                                        |
| source    | Origin indicator                                                                    |
| store     | Source data store URI                                                               |
| hash      | Content hash for integrity verification                                             |
| index     | Ordinal position within a collection                                                |
| count     | Quantity field                                                                      |
| main      | Primary entry in a credits block                                                    |
| vocals    | Vocalist reference in a credits block                                               |
| producer  | Producer reference in a credits block                                               |
| writer    | Writer reference in a credits block                                                 |
| mixer     | Mixing engineer reference                                                           |
| master    | Mastering engineer reference                                                        |
| director  | Director reference                                                                  |
| editor    | Editor reference                                                                    |
| narrator  | Narrator reference                                                                  |
| energy    | Normalized intensity float 0.0 to 1.0                                               |
| bpm       | Beats per minute                                                                    |
| key       | Musical key                                                                         |
| isrc      | International Standard Recording Code                                               |
| iswc      | International Standard Musical Work Code                                            |
| license   | License identifier                                                                  |
| expires   | Expiry date for a rights or license field                                           |
| show      | Parent show name for episodic content                                               |
| season    | Season index or identifier                                                          |
| episode   | Episode index or identifier                                                         |
| synopsis  | Long-form description                                                               |
| tags      | Free-form tag list                                                                  |
| links     | External link block                                                                 |
| roles     | Role list for a person node                                                         |
| family    | Instrument family classification                                                    |
| active    | List of active time windows (for instrument nodes)                                  |
| stem      | Reference to a discrete audio stem file                                             |
| thumbnail | Removed — use `cover -> @art/id` for chapter and episode art                        |
| artwork   | Removed — use `cover -> @art/id` in manifest                                        |
| confidence| Float confidence value for inferred annotations                                     |
| format    | File format or encoding                                                             |
| codec     | Audio or video codec identifier                                                     |
| rating    | Content rating (explicit, clean, etc.)                                              |
| legal     | Legal name — single word replacing legal-name                                       |
| marks     | Serialized OCPN marking vector snapshot                                             |
| aura      | AURA source file reference for a member in a collection                             |
| atom      | Compiled .atom file reference for a member                                          |
| hami      | Compiled .hami file reference for a member                                          |
| atlas     | Compiled .atlas file reference for a variant mapping                                |

| access    | Content access level — @access/open, locked, gated, embargoed, archived, restricted |
| embargo   | Date after which an embargoed item transitions to @access/open                      |
| live      | Boolean true literal (broadcast: "going live")                                      |
| dark      | Boolean false literal (stage: "going dark")                                         |
| published | Boolean publish flag (live = published, dark = draft)                               |
| featured  | Boolean flag for editorial featuring                                                |
| explicit  | Boolean explicit content flag                                                       |
| authored  | @history/take-id — the take when this node was first recorded                       |
| revised   | @history/take-id or mark — the take when this node was last changed                 |

---

### Approved Hyphenated Keys

Hyphens are permitted only when no single word cleanly carries the full
meaning. This list is closed.

| Key             | Reason                                                    |
| --------------- | --------------------------------------------------------- |
| pre-chorus      | Recognized song section with no single-word equivalent    |
| post-chorus     | Same as above                                             |
| lead-vocal      | Distinguishes from backing, harmony, and ad-lib roles     |
| co-writer       | The co- prefix is the only way to express co-authorship   |
| voice-over      | An established compound industry term                     |
| rights-holder   | Holder alone is ambiguous; rights context is required     |
| fill-policy     | Policy alone is ambiguous; fill-policy is the ad term     |
| mood-vocabulary | A directive key for a vocabulary declaration block        |

---

## Part V — The Info Folder System

### What Info Is

Every project has an info folder and an optional meta folder. Both live
at the project root alongside the content folders.

- **info/** holds project-specific data: persons, annotators, credits,
  rights, labels. All entries are unique to this project.
- **meta/** holds vocabulary definitions: genres, roles, moods. These may
  be local specializations of or additions to the global platform vocabulary
  at @aduki.org. The meta/ folder is optional — if absent, the project uses
  only the global vocabulary.

Person nodes in info/people.aura each carry their own generated hex ID.
Vocabulary nodes in meta/ use slug IDs instead.

---

### Info Folder Contents

| File                       | Purpose                                              | Required                       |
| -------------------------- | ---------------------------------------------------- | ------------------------------ |
| info/people.aura           | All contributor person nodes for this project        | always                         |
| info/annotators.aura       | All annotators who authored and edited the files     | always                         |
| info/metadata.aura         | Project-level identity and descriptive fields        | always                         |
| info/credits.aura          | Global credit declarations for the project           | albums, films                  |
| info/rights.aura           | Licensing and territorial rights declarations        | when needed                    |
| info/labels.aura           | Record label and publishing imprint entities         | music with label info          |
| info/studios.aura          | Production studio and broadcast network entities     | film, TV, animation            |
| info/arts.aura             | Art assets, motion covers, and trailer clips         | when media assets declared     |
| info/availability.aura     | Watch, buy, rent, and download availability entries  | when availability declared     |
| info/releases.aura         | Release variant declarations for the project         | when needed                    |

### Meta Folder Contents

| File              | Purpose                                           | Required      |
| ----------------- | ------------------------------------------------- | ------------- |
| meta/genres.aura  | Genre nodes for this project or catalog           | when used     |
| meta/roles.aura   | Role nodes for this project or catalog            | when used     |
| meta/moods.aura   | Mood vocabulary nodes for this project or catalog | when used     |

Vocabulary nodes in meta/ use slug IDs, not generated hex IDs. The slug
IS the canonical identifier — e.g. `electronic`, `main-artist`, `ethereal`.
Slugs must be unique within their namespace and are registered in the
platform registry to prevent collisions.

---

### People, Authors, and Annotators — The @people Base

All human entities in AURA — artists, directors, narrators, annotators,
transcribers — resolve from the @people base namespace. The @person/ and
@author/ domains are singular aliases; @people/ and @authors/ are their
plural counterparts. All four resolve against info/people.aura.

    ## Singular — one person
    creator  -> @person/p4xt9k2
    creator  -> @author/p4xt9k2    ## same thing

    ## Plural — multiple people
    cast     -> @people/[mt4qbz, vr8kfw]
    writers  -> @people/[p4xt9k2, k7wrt2]
    authors  -> @authors/[p4xt9k2, j8mn2rk]

    ## Both local and global forms
    @person/p4xt9k2
    @people/[p4xt9k2, j8mn2rk]
    @aduki.org/person/p4xt9k2
    @aduki.org/people/p4xt9k2

The namespace in info/people.aura may be declared as people::, persons::,
or authors:: — all compile identically.

    ## FILE: info/people.aura

    people::

      p4xt9k2::
        first   -> "Mario"
        last    -> "Aleka"
        screen  -> "Mario"
        legal   -> "Mario A. Mwangi"
        kind    -> artist
        born    -> 1993-04-11
        country -> KE
        city    -> "Nairobi"
        roles   -> @roles/[main-artist, vocalist, composer, producer]
        genre   -> @genres/[electronic, afro-soul, experimental]
        links::
          spotify -> spotify::artist/mario-aleka
          website -> https://marioaleka.com
        bio     -> "Nairobi-based producer and vocalist."

      j8mn2rk::
        first   -> "Jay"
        last    -> "Femar"
        screen  -> "Jay"
        legal   -> "James Femar Ogutu"
        kind    -> producer
        country -> KE
        roles   -> @roles/[producer, mixer, engineer]

All equivalent global forms for p4xt9k2:

    @aduki.org/people/p4xt9k2
    @aduki.org/person/p4xt9k2
    @aduki.org/author/p4xt9k2
    @person/p4xt9k2       <- local shorthand
    @author/p4xt9k2       <- local shorthand, identical resolution

---

### Annotators File Structure

Annotators are the actual humans who write, transcribe, and maintain AURA
files. They are distinct from persons (the content contributors — artists,
actors, directors, musicians). A person is someone whose work appears in
the media. An annotator is someone who documents and encodes that work in
AURA.

Annotators include lyric transcribers, subtitle writers, metadata editors,
translators working in AURA, and any editor who authors or maintains a .aura
file. They are accountable for the accuracy and completeness of the data.

Annotators are stored in info/annotators.aura. They use the same p prefix
for their IDs as persons because they are also people — but they are indexed
separately to keep the persons list clean (persons are content contributors,
not documentation contributors).

Every .aura file declares its annotator in the schema block.

    ## FILE: info/annotators.aura

    annotators::

      p9xb3mn::
        name     -> "Amina Weru"
        roles    -> transcriber | editor
        country  -> KE
        contact  -> amina@aduki.org

      p3xr7kn::
        name     -> "Diego Ferraz"
        roles    -> translator | annotator
        country  -> BR
        locale   -> pt-BR
        contact  -> diego@aduki.org

In every AURA content file, the annotator is declared in the schema block:

    schema::
      root       -> https://hami.aduki.org/aura/1.0
      kind       -> audio::music
      lang       -> en-US
      annotator  -> @annotator/p9xb3mn

When a file has been edited by more than one annotator:

    schema::
      root        -> https://hami.aduki.org/aura/1.0
      kind        -> audio::music
      lang        -> en-US
      annotators  -> @annotator/[p9xb3mn, p3xr7kn]

Annotation attribution may also appear at the node level for granular
accountability — useful when different annotators contributed different
sections of the same file:

    verse/two::
      annotator -> @annotator/p3xr7kn
      lines::
        line/one::
          text -> "She said my name like static"

The global cloud references for annotators follow the same pattern as
persons but use the annotators path:

    @aduki.org/annotators/p9xb3mn
    @aduki.org/annotators/p3xr7kn
    @annotator/p9xb3mn              <- local shorthand

Annotator IDs are generated by the same toolchain as all other IDs:

    aura generate annotator         -> p9xb3mn

The cloud store path for annotators is flat across all catalogs. An
annotator who contributes to multiple catalogs on the platform has a single
global ID that appears in the annotators file of each project they worked
on. Their global record is maintained at:

    @aduki.org/annotators/p9xb3mn

---

### Metadata File Structure

    ## FILE: info/metadata.aura

    schema::
      root    -> https://hami.aduki.org/aura/1.0
      kind    -> audio::music
      lang    -> en-US

    manifest::
      name      ! -> "Signal Loss"
      creator   ! -> @person/p4xt9k2
      version     -> 1.0.0
      released    -> 2024-11-01
      territory   -> worldwide
      label       -> "Self-Released"

    meta::
      genre    -> Electronic | Afro-Soul | Experimental
      tags     -> Nairobi | Instrumental | Ambient

---

## Part VI — Folder Structures by Media Type

### Universal Conventions

- Every project folder is named in lowercase with no spaces and no special characters other
  than hyphens. It is the human-readable name of the project, not a generated ID.
- Every project folder must have a `name.aura` at its root. This is the **index file** and
  project entry point — the compiler looks for this file first in every folder.
- Every sub-folder (`info/`, `meta/`, `tracks/`, `episodes/`, `scenes/`, etc.) has its own
  `name.aura` index file listing the files it contains.
- Content files keep their ID-based names (e.g., `t7xab3c.aura`, `c8xab3d.aura`).
- Every content file is named by its generated object ID plus `.aura`.
- The info folder uses descriptive file names (people, annotators, metadata, credits, rights,
  labels, studios, arts, availability) because these are structural roles.
- The meta folder uses descriptive file names (genres, roles, moods). Vocabulary nodes inside
  use slug IDs.
- The dist folder receives all compiled `.atom`, `.hami`, and `.atlas` files.
- No descriptive words appear in folder or file names outside of info, meta, and dist.
- Artwork, stems, and binary assets live in named folders: `artwork/`, `stems/`.
- Stems are organized under subdirectories named by the track ID they belong to.
- The `configs/` folder is **never compiled** and **never tracked** by `.history/`. It holds
  toolchain configuration: LLM providers, store origins, credentials, and ignore lists.
- The `.history/` folder is an append-only object store maintained by the toolchain.
  It is private to the local project and is never published to the cloud store.

---

### The `name.aura` Index File

Every project and every project sub-folder has a `name.aura`. This uniform convention means
the compiler always knows exactly one file to look for when entering any folder.

#### Root `name.aura`

This is the project entry point. It declares the project's namespace identity, kind, and
language, and re-exports its sub-namespaces. The compiler reads this file first.

    ## FILE: name.aura

    schema::
      root       -> https://hami.aduki.org/aura/1.0
      kind       -> audio::music
      namespace  -> signal-loss-album
      lang       -> en-US

    exports::
      info       -> @info/metadata
      people     -> @info/people
      tracks     -> @tracks/*
      collection -> c8xab3d.aura

#### Sub-folder `name.aura`

Every sub-folder's `name.aura` lists its contained files.

    ## FILE: info/name.aura

    namespace::
      folder    -> info
      contains::
        - people.aura
        - annotators.aura
        - metadata.aura
        - credits.aura
        - arts.aura
        - studios.aura
        - labels.aura
        - availability.aura

    ## FILE: tracks/name.aura

    namespace::
      folder    -> tracks
      contains::
        t7xab3c -> "Signal Loss"
        t4mn2rp -> "Fold"
        t9vkx7q -> "Recursive"
        t2nq5wb -> "Meridian"
        t6rj8vc -> "Origin Point"

---

### The configs/ Folder

The `configs/` folder is always at the project root. It is excluded from compilation and
from `.history/` tracking. It holds toolchain configuration.

    configs/
      llm.aura        <- LLM provider definitions (editor integration)
      stores.aura     <- Remote store origins and authentication refs
      account.aura    <- Cloud identity — reads from .env or env variables
      ignore.aura     <- Extra paths excluded from .history/ tracking

Credential values are never stored in `configs/account.aura`. That file declares environment
variable names. Actual secrets come from `.env` at project root or process environment.

---

### Album

    album/                          <- project directory
      name.aura                     <- project entry point (index file)
      info/
        name.aura                   <- info/ index file
        people.aura
        annotators.aura
        metadata.aura
        credits.aura
        rights.aura
        labels.aura                 <- record label entities
        studios.aura                <- production studio entities (if needed)
        arts.aura                   <- cover art, motion covers, trailers
        availability.aura           <- watch/buy/rent/download entries
      meta/
        name.aura                   <- meta/ index file
        genres.aura                 <- optional: local genre vocab
        roles.aura                  <- optional: local role vocab
      tracks/
        name.aura                   <- tracks/ index file
        t7xab3c.aura
        t4mn2rp.aura
        t9op5lw.aura
        t2kr8xn.aura
        t6bx4qm.aura
      variants/
        name.aura                   <- variants/ index file
        v3qr7st.aura                <- acoustic variant of t7xab3c
        v5nm9xb.aura                <- radio edit of t4mn2rp
        v8xp2kl.aura                <- dubbed track
      artwork/
        cover.jpg
        back.jpg
        booklet.pdf
      motion/
        cover-loop.mp4              <- Apple Music-style motion cover
      trailers/
        main-trailer.mp4
      stems/
        t7xab3c/
          drums.flac
          bass.flac
          keys.flac
          vocals.flac
      c8xab3d.aura                  <- collection manifest
      configs/                      <- Never compiled. Not tracked by history.
        llm.aura
        stores.aura
        account.aura
        ignore.aura
      .history/                     <- History object store (toolchain only)
        objects/
        marks/
        streams/
        HEAD
      dist/
        c8xab3d.hami                <- compiled collection manifest
        people.hami                 <- compiled people index
        t7xab3c.atom
        t7xab3c.hami
        v3qr7st.atom
        v3qr7st.atlas               <- DTW alignment atlas for this variant

Each variant file declares which canonical track it diverges from using
its canonical field pointing at the track ID.

---

### EP

    ep/                             <- project directory
      info/
        people.aura
        metadata.aura
        credits.aura
      tracks/
        t2ab8xk.aura
        t7xr3mn.aura
        t5nq4bp.aura
      artwork/
        cover.jpg
      c3xn7rp.aura
      dist/

---

### Single

    single/                         <- project directory
      info/
        people.aura
        metadata.aura
      tracks/
        t7xab3c.aura                <- A-side
        t3mn8rk.aura                <- B-side or instrumental
      artwork/
        cover.jpg
      sg4xr9t.aura
      dist/

If the single has no B-side, the collection manifest is optional and the
single track file may serve as the root document.

---

### Compilation

    compilation/                    <- project directory
      info/
        people.aura
        metadata.aura
        credits.aura
        rights.aura
        labels.aura
      tracks/
        t7xab3c.aura                <- licensed track, local copy
        t4mn2rp.aura
      links/
        links.aura                  <- external track references for licensed
      artwork/                       content that lives in other catalogs
        cover.jpg
      c5xr2nm.aura
      dist/

Licensed tracks that originate from other catalogs are declared as linked
members in the collection manifest, referencing them via their global ID.
They are not copied into the local tracks folder.

---

### Music Video

    music-video/                    <- project directory
      info/
        people.aura
        metadata.aura
        credits.aura
      scenes/
        f2xb7np.aura
        f8nm3kr.aura
        f4xp9rt.aura
        f6bq2xm.aura
      shots/
        f2xb7np/                    <- shots organized by parent scene ID
          sh1.aura
          sh2.aura
      artwork/
        thumbnail.jpg
        poster.jpg
      mv6xp3l.aura
      dist/

When a music video is companion content inside an album folder:

    album/
      ...
      video/
        mv6xp3l/
          info/
            credits.aura
          scenes/
            f2xb7np.aura
          mv6xp3l.aura
          dist/

---

### Podcast

    podcast/                        <- project directory
      info/
        people.aura
        metadata.aura
      seasons/
        sn2kr9l/                    <- season ID
          info/
            people.aura
            metadata.aura
          episodes/
            ep7xb3n.aura
            ep3mn8rk.aura
            ep9xp4lw.aura
          sn2kr9l.aura              <- season manifest
          dist/
        sn8pq3xv/
          info/
            people.aura
            metadata.aura
          episodes/
            ep2xb8mn.aura
          sn8pq3xv.aura
          dist/
      pc5xk4m.aura                  <- series manifest
      dist/

---

### TV Series

    tv-series/                      <- project directory
      info/
        people.aura
        metadata.aura
        credits.aura
      seasons/
        sn2kr9l/                    <- season ID
          info/
            people.aura
            metadata.aura
            credits.aura
          episodes/
            ep7xb3n.aura
            ep3mn8rk.aura
            ep4xp9rt.aura
            ep6bq2xm.aura
          variants/
            v5nm9xb.aura            <- dubbed version of ep7xb3n
          sn2kr9l.aura
          dist/
        sn8pq3xv/
          info/
            people.aura
            metadata.aura
          episodes/
            ep2xb8mn.aura
          sn8pq3xv.aura
          dist/
      tv4x7ab.aura                  <- series manifest
      dist/

---

### Anime and Animation

    animation/                      <- project directory
      info/
        people.aura
        metadata.aura
        credits.aura
      seasons/
        sn2kr9l/
          info/
            people.aura
            metadata.aura
          episodes/
            ep7xb3n.aura
            ep3mn8rk.aura
          variants/
            v2xb8kp.aura            <- English dub of ep7xb3n
            v7nm3xr.aura            <- French dub of ep7xb3n
          sn2kr9l.aura
          dist/
      an9vl3b.aura
      dist/

---

### Film

    film/                           <- project directory
      info/
        people.aura
        metadata.aura
        credits.aura
        rights.aura
      acts/
        fa2xb7n.aura                <- act file
        fa8mn3k.aura
        fa4xp9r.aura
      scenes/
        fs1xb4n.aura                <- scene files
        fs3mn9x.aura
        fs7xr2kp.aura
        fs5bq8nm.aura
      variants/
        v3qr7st.aura                <- director's cut
        v8xp2kl.aura                <- dubbed version
        v2nm4xb.aura                <- commentary track
      artwork/
        poster.jpg
        thumbnail.jpg
      f6np2qr.aura                  <- film manifest
      dist/

For short films where scenes are not broken into separate files, a single
f6np2qr.aura document carries all scenes inline and the acts and scenes
folders are absent.

---

### Documentary

    dc3wr8x/                        <- documentary ID
      info/
        people.aura
        metadata.aura
        credits.aura
      segments/
        sg8xb2mn.aura
        sg4xr9t.aura
        sg6np3kx.aura
        sg2xb7nm.aura
      interviews/
        cy9xb3mn.aura               <- interview ID files
        cy4xr7kp.aura
      artwork/
        poster.jpg
        thumbnail.jpg
      dc3wr8x.aura
      dist/

Interview files are linked from segment files via @interview/ references
using the interview file's ID.

---

### Audiobook

    b8mt4kx/                        <- audiobook ID
      info/
        people.aura
        metadata.aura
        rights.aura
      chapters/
        part-one/                   <- parts use descriptive folder names
          ch3xb7mn.aura
          ch8xr4np.aura
          ch2xb9kl.aura
        part-two/
          ch7xp3rn.aura
          ch5xb8mk.aura
      artwork/
        cover.jpg
      b8mt4kx.aura
      dist/

Part folders use descriptive names (part-one, part-two) because they are
grouping containers, not addressable content objects with IDs. Chapter files
inside them are addressed by their generated IDs.

---

### Speech

    speech/                         <- project directory
      info/
        people.aura
        metadata.aura
      segments/
        sg8xb2mn.aura
        sg4xr9t.aura
        sg6np3kx.aura
      artwork/
        thumbnail.jpg
      sp2xr7n.aura
      dist/

For a short speech that fits in a single file, the segments folder is absent
and sp2xr7n.aura is the complete document.

---

### Music (Standalone — No Release Context)

For standalone music files not part of a formal release — demos, session
recordings, library music, stem packages:

    track/                          <- project directory
      info/
        people.aura
        metadata.aura
      stems/
        drums.flac
        bass.flac
        keys.flac
      t7xab3c.aura
      dist/
        t7xab3c.atom
        t7xab3c.hami

---

## Part VII — Catalog Structure

### The Catalog Root

A catalog groups all collections belonging to one label, publisher, or
creator. The catalog root folder is named by the catalog's own ID.

    catalog/                        <- catalog root directory
      info/
        people.aura                 <- global people index for all projects
        annotators.aura             <- global annotators index
        metadata.aura               <- catalog-level identity
        labels.aura
      meta/
        genres.aura                 <- catalog-level genre vocabulary
        roles.aura                  <- catalog-level role vocabulary
        moods.aura                  <- catalog-level mood vocabulary
      collections/
        c8xab3d/                    <- album
        c3xn7rp/                    <- EP
        tv4x7ab/                    <- TV series
        f6np2qr/                    <- film
        pc5xk4m/                    <- podcast
      catalog.aura                  <- catalog root manifest
      dist/

The catalog-level people.aura is the global index. Individual collection-level
persons files carry only persons specific to that collection who are not
already in the catalog index. Resolution cascades: collection info first,
catalog info second.

---

### Multi-Catalog Global Store

At the global store level, catalogs are organized by their IDs under the
store root. The store has no folder hierarchy beyond the top-level catalog
folders. All nesting lives inside each catalog.

    store-root/
      cx0ab3de/                     <- catalog A
      cx9mn4rp/                     <- catalog B
      cx3xr7bk/                     <- catalog C

The global URI for any object in the store is:

    @aduki.org/{type}/{id}
    @aduki.org/people/{person-id}
    @aduki.org/series/{series-id}/season/{season-id}/episode/{episode-id}

The store ID registry maps every issued ID to its type and the catalog that
owns it. Cross-catalog references are resolved via this registry. Rights
validation runs before any cross-catalog :: arc is resolved.

---

*AURA Structure Reference — v0.2*
*Covers: ID system, prefix vocabulary, ID generation, reference grammar,*
*local and global cloud references, key naming, info folder, and folder*
*layouts for all supported media types.*
*File names are IDs. Names live inside files.*
