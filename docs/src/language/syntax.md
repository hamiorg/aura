# AURA (Adaptive Universal Relational Annotations)

## The Human-Readable Authoring Layer for ATOM/HAMI

> AURA is not JSON. Not YAML. Not TOML. Not XML.
> It is a language built for people who think in nodes, not files.
> Write it like prose. Compile it like a machine.

---

## Part I — Philosophy

### Why a New Language?

Every existing serialization format was designed for machines first.
JSON forces quotes around every key. YAML collapses on indentation ambiguity.
TOML becomes unreadable past two levels. XML is ceremony without substance.

AURA starts from the opposite direction: design for the author, compile for
the machine.

A music producer, a documentary editor, a podcast archivist — none of them
should think in angle brackets. They should think in tracks, nodes,
contributors, scenes, chapters, moments. AURA should feel like writing about
their work, not configuring it.

The compiled output — .atom and .hami — is where the machine takes over.
AURA is the bridge.

---

### Three Governing Rules

1. One sigil, one job.
Every symbol in AURA has one purpose and one purpose only. No symbol does
double duty. If you see ::, it is always a namespace jump. If you see @, it
is always a reference. If you see ##, it is always an annotation.
No context-dependent overloading.

2. Nodes are the atomic unit.
Everything in AURA is a node. A track is a node. A scene is a node.
An author is a node. A syllable is a node. A chord change is a node.
A rights boundary is a node. Nodes nest inside nodes. Nodes reference
other nodes. The hierarchy is not organizational — it is relational.
Depth is meaning.

3. People are first-class.
Contributors are not strings inside a field. They are defined entities with
their own namespace. They are referenced by identity, not by name. In any
text, at any depth, you can point to a person. The engine remembers who
they are.

---

## Part II — The Sigil System

AURA uses a small, unambiguous set of sigils. Each is chosen to be visually
distinct, keyboard-accessible, and semantically memorable.

| Sigil       | Name           | Role                                                    |
| ----------- | -------------- | ------------------------------------------------------- |
| ::          | Scope Opener   | Opens a block or declares a namespace                   |
| ->          | Value Arrow    | Assigns a literal value to a key                        |
| @           | Reference      | References a named entity anywhere                      |
| ##          | Annotation     | Queryable comment compiled into HAMI                    |
| --          | Divider        | Visual separator (no compile output)                    |
| \|          | Union Pipe     | One or more values, across different domains            |
| ?           | Optional Mark  | This field may be absent                                |
| !           | Required Mark  | This field must be present                              |
| ~           | Duration Mark  | Separates start and duration in a time triple           |
| [, ]        | List Bracket   | Time triple [start, end, duration] or multi-ID list     |
| >>          | Inherits From  | Extend a template or base schema                        |
| *           | Wildcard       | Match all in a namespace query                          |
| %           | Custom Mark    | Explicitly bypass strict key linting for non-standard keys |

This is the complete sigil vocabulary.

---

### Multi-ID Reference Syntax

When a field references multiple entities within the same domain, AURA uses
the list bracket directly after the domain path.

    ## Single reference — scalar, no brackets
    annotator  -> @annotator/p9xb3mn
    creator    -> @person/p4xt9k2

    ## Multi-ID reference — all IDs within the same domain
    annotators -> @annotator/[p9xb3mn, p3xr7kn]
    speakers   -> @person/[cc9xpq, lp2wnr]
    cast       -> @person/[mt4qbz, vr8kfw, xp3bnr]

The single form compiles to a scalar :: arc. The bracket form compiles
directly to a HAMI array of :: arcs. A single ID in brackets
@domain/[id] is valid and is normalized to a scalar at compile time.

The Union Pipe | is used when values span different domains or are
non-reference values (genres, tags, roles, territories).

    ## Pipe for non-ID and cross-domain unions
    genre    -> Electronic | Afro-Soul | Experimental
    roles    -> main-artist | vocalist | composer
    territory -> worldwide | KE | NG

---

## Part III — Document Structure

### The Namespace Block

The top-level structure of an AURA document is a sequence of namespace blocks.
A namespace block opens with a name followed by :: and a newline, and contains
indented key-value pairs or sub-nodes.

    manifest::
      name       -> "Project ATOM EP"
      version    -> 1.0.0
      language   -> en-US

Namespace blocks are order-independent. The AURA engine resolves all
dependencies before writing to ATOM/HAMI. You can declare persons after
tracks. It works.

---

### Key-Value Assignment

The -> arrow assigns a literal value to a key.

    name       -> "Song Title"
    released   -> 2024-11-01
    rating     -> explicit

Keys are unquoted. Values are quoted when they contain spaces or special
characters, and bare otherwise.

---

### Sub-Nodes

Nesting is done with indentation. There is no closing bracket.
Every indented block under a key is a sub-node of that key.

    credits::
      main       -> @person/p4xt9k2
      vocals     -> @person/p4xt9k2 | @author/j8mn2rk
      producer   -> @author/j8mn2rk
      writers::
        primary  -> @person/k7wrt2
        co       -> @person/h8dv5x

Each level of indentation becomes a HAMI sub-namespace.

---

### Lists

Multiple values on a single key use the union pipe |.
Alternatively, a list block spreads values across lines under the key.

    ## Inline list (union)
    genres -> Electronic | Afro-Soul | Experimental

    ## Block list
    formats::
      - flac
      - aac
      - opus
      - mp3

Both compile identically to a HAMI GS-delimited list.

---

### Optional and Required Markers

    ## Field must be present — engine halts if missing
    title !       -> "Untitled"

    ## Field may be absent — engine skips gracefully
    isrc  ?       -> "UNKNOWN"

    ## Optional with no default
    artwork ?

---

### The Custom Mark (%)

The AURA compiler enforces a strict dictionary of standard keys to catch typos and
enforce ecosystem consistency (the W006 Unknown Key lint). When a project requires
a domain-specific or non-standard key that is not in the global dictionary, it must
be explicitly marked with the `%` sigil.

    ## This will trigger a W006 error if 'engine-version' is not a standard key
    engine-version -> "4.2"

    ## This bypasses the lint and compiles successfully
    engine-version % -> "4.2"

The `%` mark signals intent: "I know this key is custom, let it through." The mark is
separated by spaces (`key % -> value`) and is stripped before compiling to ATOM/HAMI.

---

## Part IV — Time Notation

### The [start, end, duration] Triple

Every temporal object in AURA compiles to a three-value time triple
stored in ATOM. All three values — start, end, and duration — are
always stored. The engine validates that start + duration = end.

If any two values are provided and the third is absent, the engine derives
and writes the third before compiling. If all three violate the invariant,
the engine raises a compile-time error.

AURA provides three authoring syntaxes for the time triple. All three
compile to the same [start, end, duration] triple in ATOM.

---

### Syntax 1 — Range (start~end)

The author provides start and end. Duration is derived.

    ## Short form — seconds and minutes
    time -> 0s~30s
    time -> 22s~1m10s
    time -> 1m50s~3m00s

    ## Long form — HH:MM:SS for film and long-form video
    time -> 00:04:32~00:07:18
    time -> 01:22:05~01:38:47

This syntax is unchanged from the original specification and remains the
most common form for authored content.

---

### Syntax 2 — Start and Duration (start+duration)

The author provides start and a forward duration. End is derived.

    ## Verse begins at 22 seconds and runs for 48 seconds
    time -> 22s+48s

    ## Scene begins at 4 minutes 32 seconds and runs for 2 minutes 46 seconds
    time -> 00:04:32+2m46s

This syntax is preferred when an editor knows section length before
final timeline alignment is complete. The engine stores end as unresolved
until the master timeline is finalized.

---

### Syntax 3 — Explicit Triple ([start, end, duration])

The author provides all three values explicitly. The engine validates
the invariant.

    time -> [22s, 1m10s, 48s]
    time -> [00:04:32, 00:07:18, 2m46s]

This syntax is used when importing from external systems that provide
all three values, or when duration must be explicitly verified in the
source record for rights and billing purposes.

---

### Point Anchors

A point anchor is a single instant in time with no duration.
It compiles to a triple where start = end and duration = 0.

    sync-point -> @time/1m32s
    sample     -> @track/02 @time/2m44s

---

### Mixed Notation Within a Document

All three time syntaxes may be used within the same AURA document.
The engine normalizes all of them to [start, end, duration] triples
before writing ATOM/HAMI. Authors should choose the syntax that
most naturally matches their editorial workflow for each section.

---

## Part IV-B — Scalar Values and the Boolean System

### Why Not `true` and `false`

AURA deliberately avoids `true` and `false`. They are machine concepts pasted into
a domain that deserves its own vocabulary. A lyric line is not `enabled: true`. A
rights clearance is not `cleared: false`.

Instead, AURA uses two media-native boolean literals:

| Literal | Means       | Rationale                                           |
| ------- | ----------- | --------------------------------------------------- |
| `live`  | true / on   | Broadcast: "going live" = active, enabled, present  |
| `dark`  | false / off | Stage: "going dark" = inactive, disabled, absent    |

These are the only two boolean literals in AURA. Every boolean field accepts
exactly one of them.

---

### Boolean in Practice

    ## A rights node
    rights/doc::
      cleared -> live            ## rights confirmed
      blocked -> dark            ## not blocked in any territory

    ## A manifest field
    manifest::
      explicit  -> dark          ## not explicit
      featured  -> live          ## is currently featured
      published -> live          ## is published

    ## A track annotation
    tracks/t7xab3c.aura::
      karaoke   -> live          ## karaoke data present
      richsync  -> dark          ## word-level timing absent

    ## A sample node
    sample/one::
      cleared   -> live

    ## An ad slot
    slot/mid::
      required  -> live

The engine compiles `live` to 1 and `dark` to 0 in HAMI. Both are reachable
via ATOM node queries filtered on their field value. `true` and `false` are
accepted as synonyms for toolchain interoperability.

---

### The @access Domain

`@access` is the singular reference domain for content visibility and
permission levels. Unlike boolean fields (which express binary flag states),
`@access` expresses a content governance decision with multiple named levels.

    ## Declaring access on a node
    schema::
      root   -> https://hami.aduki.org/aura/1.0
      kind   -> audio::music
      lang   -> en-US
      access -> @access/open

    ## Overriding access on a specific track
    tracks/t7xab3c.aura::
      access -> @access/gated

    ## A pre-release track under embargo
    tracks/t4mn2rp.aura::
      access  -> @access/embargoed
      embargo -> 2025-06-01        ## date when it transitions to @access/open

#### Access Levels

| Level                | Meaning                                                         |
| -------------------- | --------------------------------------------------------------- |
| `@access/open`       | Public — unrestricted, no authentication required               |
| `@access/locked`     | Private — authentication required, owner-only                   |
| `@access/gated`      | Conditional — requires subscription, payment, or role           |
| `@access/embargoed`  | Time-locked — transitions to open after embargo date            |
| `@access/archived`   | Accessible but retired — marked for historical access           |
| `@access/restricted` | Geo- or rights-restricted — available in named territories only |

Access levels form an ordered hierarchy for cascade resolution:

    open < archived < restricted < gated < embargoed < locked

A parent collection's access level applies to all members unless explicitly
overridden on the member. A member may only restrict further, never relax,
without an explicit override.

---

### Access in a Collection

    ## FILE: collections/c8xab3d.aura

    manifest::
      name   ! -> "Signal Loss"
      access -> @access/open        ## whole album: public

    collection::
      members::

        track/one::
          aura-ref -> tracks/t7xab3c.aura
          access   -> @access/open

        track/two::
          aura-ref -> tracks/t4mn2rp.aura
          access   -> @access/gated     ## subscriber-only
          note     -> "Bonus track for subscribers"

        track/three::
          aura-ref -> tracks/t9vkx7q.aura
          access   -> @access/embargoed
          embargo  -> 2025-09-01        ## future release

---

### @access in the Engine

Access nodes compile to ATOM AccessNode objects (node class 0x13). The engine
evaluates the access bitmask at query time before returning any node payload.
Gated and embargoed statuses are re-evaluated on every request — they are
never baked into the compiled artifact.

---

## Part V — The People System

### @people Is the Base

In AURA every human entity — artist, director, narrator, transcriber, editor,
translator — is a person. The @people namespace is the single base from which
all human references resolve. Authors, annotators, speakers, cast members: all
are people first.

The plural/singular distinction in reference syntax is intentional and applies
across **all** reference domains in AURA, not just people:

| Form                          | Meaning                                        |
| ----------------------------- | ---------------------------------------------- |
| @entity/id                    | singular — one entity, resolves to a node      |
| @entities/[id1, id2, ...]     | plural — multiple entities, compiles to array  |

This applies to @person/@people, @genre/@genres, @role/@roles, @mood/@moods,
@track/@tracks, @annotator/@annotators, @event/@events, and every other domain.

---

### Singular vs Plural References

    ## One person — singular domain
    speaker    -> @person/p4xt9k2

    ## Multiple people — plural domain with bracket list
    cast       -> @people/[mt4qbz, vr8kfw, p4xt9k2]
    speakers   -> @people/[cc9xpq, lp2wnr]
    authors    -> @people/[p4xt9k2, j8mn2rk]

    ## One author (author is an alias for person)
    author     -> @person/j8mn2rk
    ## Multiple authors
    authors    -> @people/[p4xt9k2, j8mn2rk]

    ## One annotator
    annotator  -> @annotator/p9xb3mn
    ## Multiple annotators
    annotators -> @annotators/[p9xb3mn, p3xr7kn]

The `@author/` and `@authors/` domains are fully interchangeable with
`@person/` and `@people/`. All four resolve against `info/people.aura`
and the same global cloud path.

---

### Defining People

People are defined in `info/people.aura`. The namespace may be declared as
`people::`, `persons::`, or `authors::` — all are valid and compile identically.

    ## FILE: info/people.aura

    people::

      p4xt9k2::
        name    -> "Mario Aleka"
        legal   -> "Mario A. Mwangi"
        born    -> 1993-04-11
        country -> KE
        city    -> "Nairobi"
        roles   -> @roles/[main-artist, vocalist, composer, producer]
        genre   -> @genres/[electronic, afro-soul, experimental]
        links::
          spotify -> spotify::artist/mario-aleka
          website -> https://marioaleka.com
        bio     -> "Nairobi-based producer and vocalist. Known for blending
                    electronic architecture with Afro-Soul textures."

      j8mn2rk::
        name    -> "Jay Femar"
        legal   -> "James Femar Ogutu"
        country -> KE
        roles   -> @roles/[producer, mixer, engineer]

      k7wrt2::
        name    -> "Janet Doe"
        country -> KE
        roles   -> @roles/[writer, lyricist]

IDs are generated by the toolchain — never hand-authored:

    aura generate person    -> p4xt9k2

---

### Referencing People

    ## Credits block
    credits::
      vocals   -> @person/p4xt9k2
      producer -> @person/j8mn2rk       ## @author/ also valid
      writers  -> @people/[k7wrt2, p4xt9k2]

    ## Inside text
    description -> "Mixed by @person/j8mn2rk, written by @people/[k7wrt2]."

    ## Inside a lyric line
    line/four::
      text    -> "She said my name @person/p4xt9k2 in a voice like static"
      speaker -> @person/p4xt9k2

    ## Cast and speakers
    scene/cold-open::
      cast     -> @people/[mt4qbz, vr8kfw]

    chapter/interview::
      speakers -> @people/[cc9xpq, lp2wnr]

---

### Global Forms

All of the following resolve to the same node for p4xt9k2:

    @person/p4xt9k2
    @author/p4xt9k2
    @aduki.org/person/p4xt9k2
    @aduki.org/author/p4xt9k2
    @aduki.org/people/p4xt9k2

---

### Reference Domain Table

Complete singular/plural reference domain listing for all entity types.

#### People, Authors, Annotators

| Domain                  | Form     | Resolves via                                |
| ----------------------- | -------- | ------------------------------------------- |
| @person/id              | singular | info/people.aura or global                  |
| @people/[a, b]          | plural   | info/people.aura or global                  |
| @author/id              | singular | alias for @person/id                        |
| @authors/[a, b]         | plural   | alias for @people/[a, b]                    |
| @annotator/id           | singular | info/annotators.aura or global              |
| @annotators/[a, b]      | plural   | info/annotators.aura or global              |

#### Vocabulary

| Domain                  | Form     | Resolves via                                |
| ----------------------- | -------- | ------------------------------------------- |
| @genre/slug             | singular | meta/genres.aura or global vocab            |
| @genres/[a, b]          | plural   | meta/genres.aura or global vocab            |
| @role/slug              | singular | meta/roles.aura or global vocab             |
| @roles/[a, b]           | plural   | meta/roles.aura or global vocab             |
| @mood/slug              | singular | meta/moods.aura or global vocab             |
| @moods/[a, b]           | plural   | meta/moods.aura or global vocab             |

#### Content Files

| Domain                  | Form     | Resolves via                                |
| ----------------------- | -------- | ------------------------------------------- |
| @track/id               | singular | tracks/ folder by generated ID              |
| @tracks/[a, b]          | plural   | tracks/ folder by generated IDs             |
| @episode/id             | singular | episodes/ folder by generated ID            |
| @episodes/[a, b]        | plural   | episodes/ folder by generated IDs           |
| @scene/id               | singular | scenes/ folder by generated ID              |
| @scenes/[a, b]          | plural   | scenes/ folder by generated IDs             |
| @variant/id             | singular | variants/ folder by generated ID            |
| @chapter/id             | singular | in-file chapter node                        |
| @segment/id             | singular | in-file segment node                        |
| @collection/id          | singular | collection manifest by generated ID         |
| @season/id              | singular | season manifest by generated ID             |
| @member/id              | singular | another member in the same collection       |
| @member/id::node/path   | singular | specific node within another member         |

#### Time and Sync

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @time/value             | singular | a temporal point anchor in the current file |
| @tempo/id               | singular | a tempo node in the current file            |
| @anchor/id              | singular | a sync anchor node                          |

#### Music-Specific

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @sample/id              | singular | a sample reference node                     |
| @samples/[a, b]         | plural   | multiple sample reference nodes             |
| @interpolation/id       | singular | a musical interpolation node                |
| @interpolations/[a, b]  | plural   | multiple interpolation nodes                |

#### Annotation and Context

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @explainer/id           | singular | an explanation node for any content node    |
| @explainers/[a, b]      | plural   | multiple explanation nodes                  |
| @instruction/id         | singular | a processing instruction node               |
| @instructions/[a, b]    | plural   | multiple processing instruction nodes       |

#### Events

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @event/id               | singular | a signal-emitting event node                |
| @events/[a, b]          | plural   | multiple event nodes                        |

#### Info and Meta

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @info/people            | singular | the info/people.aura file                   |
| @info/annotators        | singular | the info/annotators.aura file               |
| @info/metadata          | singular | the info/metadata.aura file                 |
| @meta/genres            | singular | the meta/genres.aura file                   |
| @meta/roles             | singular | the meta/roles.aura file                    |
| @meta/moods             | singular | the meta/moods.aura file                    |

#### In-file Nodes

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @verse/label            | singular | an in-file verse node                       |
| @chorus/label           | singular | an in-file chorus node                      |
| @line/label             | singular | an in-file line node                        |

#### Cloud

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @aduki.org/             | any      | global cloud URI — all domains via path     |

#### Reserved (Future)

| Domain                  | Form     | Meaning                                     |
| ----------------------- | -------- | ------------------------------------------- |
| @thread/id              | singular | reserved — future parallel thread support   |
| @parallel/id            | singular | reserved — future parallel execution node   |

---

## Part VI — The Annotator System

### What an Annotator Is

An annotator is the actual human who writes, transcribes, and maintains a
AURA file. Annotators are distinct from persons. A person is someone whose
work appears in the media — an artist, a director, a narrator. An annotator
is someone who documents and encodes that work in AURA.

Annotators include lyric transcribers, subtitle writers, metadata editors,
translators working in AURA, and any contributor who authors or maintains a
.aura file. They are accountable for the accuracy and completeness of the data.

---

### Defining an Annotator

Annotators are defined in info/annotators.aura, separate from the persons
file. They use the same generated ID format and the same p prefix, because
they are also people. Keeping them in a separate file ensures the persons
list remains clean as a list of content contributors only.

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

---

### Declaring the Annotator in a File

Every AURA content file declares its annotator in the schema block.

    schema::
      root       -> https://hami.aduki.org/aura/1.0
      kind       -> audio::music
      lang       -> en-US
      annotator  -> @annotator/p9xb3mn

When more than one annotator contributed to a single file:

    schema::
      root        -> https://hami.aduki.org/aura/1.0
      kind        -> audio::music
      lang        -> en-US
      annotators  -> @annotator/[p9xb3mn, p3xr7kn]

Annotation attribution may also appear at the node level when different
annotators wrote different sections of the same file:

    verse/two::
      annotator -> @annotator/p3xr7kn
      lines::
        line/one::
          text -> "She said my name like static"

---

### Global Annotator References

Annotator IDs follow the same global URI convention as all other IDs.

    @annotator/p9xb3mn              <- local shorthand
    @aduki.org/annotators/p9xb3mn   <- global cloud reference

An annotator who contributes to multiple catalogs has one global ID that
appears in the annotators file of each project. Their global record is
maintained at @aduki.org/annotators/{id} and is shared across all catalogs.

---

## Part VII — The Meta Vocabulary System

### Why Vocabulary Nodes?

Genre, role, and mood values in AURA have always been written as free strings:

    genre -> Electronic | Afro-Soul | Experimental
    roles -> main-artist | vocalist | composer

Strings work but they carry no semantic weight. They cannot be queried by
type. They cannot carry additional metadata (parent genre, region, color
for UI). They cannot be shared and resolved across catalogs. And they are
fragile — a typo in one file silently diverges from the canonical term.

Vocabulary nodes make genre, role, and mood terms first-class typed
entities, defined once and referenced everywhere — exactly like persons.

---

### The meta/ Folder

Vocabulary files live in a folder called meta/ alongside info/. Unlike
info/, which holds project-specific data, meta/ holds vocabulary definitions
that may be local overrides of global platform vocabulary or entirely custom
terms for the project.

    meta/
      genres.aura     <- genre nodes
      roles.aura      <- role nodes
      moods.aura      <- mood vocabulary nodes

The meta/ folder is optional. If absent, the project uses only the global
vocabulary available at @aduki.org/genre/, @aduki.org/role/, @aduki.org/mood/.

---

### Vocabulary Node IDs

Vocabulary nodes use slug IDs — lowercase words with hyphens for compounds.
This is the one exception to the generated hex ID rule. Vocabulary slugs
are stable, human-readable, and platform-canonical.

    electronic        <- genre slug
    afro-soul         <- genre slug (hyphen for compound)
    main-artist       <- role slug
    lead-vocal        <- role slug
    ethereal          <- mood slug

Slugs must be unique within their namespace. The platform registry prevents
collisions across all catalogs.

---

### genres.aura

    ## FILE: meta/genres.aura

    genres::

      electronic::
        name    -> "Electronic"
        parent  -> @genre/instrumental
        tags    -> synthesizer | beat-driven | digital

      afro-soul::
        name    -> "Afro-Soul"
        parent  -> @genre/soul
        region  -> Africa
        note    -> "Soul music with African rhythmic and melodic elements"

      experimental::
        name    -> "Experimental"
        note    -> "Non-genre marker; denotes boundary-pushing or avant-garde work"

      instrumental::
        name    -> "Instrumental"

      soul::
        name    -> "Soul"
        parent  -> @genre/rnb

Each genre can declare a parent genre, building a simple genre hierarchy.
The platform maintains the canonical genre tree at @aduki.org/genre/.

---

### roles.aura

    ## FILE: meta/roles.aura

    roles::

      main-artist::
        name  -> "Main Artist"
        kind  -> performer

      vocalist::
        name  -> "Vocalist"
        kind  -> performer
        note  -> "Any singing role; use lead-vocal or backing-vocal for specificity"

      lead-vocal::
        name  -> "Lead Vocalist"
        kind  -> performer
        parent -> @role/vocalist

      backing-vocal::
        name  -> "Backing Vocalist"
        kind  -> performer
        parent -> @role/vocalist

      composer::
        name  -> "Composer"
        kind  -> creator

      producer::
        name  -> "Producer"
        kind  -> creator

      mixer::
        name  -> "Mixing Engineer"
        kind  -> technical

      master::
        name  -> "Mastering Engineer"
        kind  -> technical

      director::
        name  -> "Director"
        kind  -> creator

      narrator::
        name  -> "Narrator"
        kind  -> performer

---

### moods.aura

    ## FILE: meta/moods.aura

    moods::

      ethereal::
        name      -> "Ethereal"
        energy    -> 0.2
        valence   -> positive

      reflective::
        name      -> "Reflective"
        energy    -> 0.3
        valence   -> neutral

      euphoric::
        name      -> "Euphoric"
        energy    -> 0.9
        valence   -> positive

      melancholic::
        name      -> "Melancholic"
        energy    -> 0.3
        valence   -> negative

      tense::
        name      -> "Tense"
        energy    -> 0.7
        valence   -> negative

      ominous::
        name      -> "Ominous"
        energy    -> 0.5
        valence   -> negative
        context   -> film | video | drama

The mood-vocabulary directive in schema:: declares which mood vocab file
applies to the document. The engine validates mood descriptor values against
this vocabulary at compile time.

    schema::
      root            -> https://hami.aduki.org/aura/1.0
      kind            -> audio::music
      mood-vocabulary -> @meta/moods

---

### Referencing Vocabulary Nodes

Vocabulary nodes are referenced via their domain prefix exactly like persons.

    ## Single genre reference
    genre -> @genre/electronic

    ## Multi-genre reference — bracket form
    genre -> @genre/[electronic, afro-soul, experimental]

    ## Single role reference
    roles -> @role/main-artist

    ## Multi-role reference
    roles -> @role/[main-artist, vocalist, composer]

    ## Mood reference in support node
    descriptors -> @mood/[ethereal, reflective]

---

### Reference Domains for Vocabulary

| Domain       | Resolves to                                                     |
| ------------ | --------------------------------------------------------------- |
| @genre/slug  | Genre node by slug in meta/genres.aura or global vocab          |
| @role/slug   | Role node by slug in meta/roles.aura or global vocab            |
| @mood/slug   | Mood node by slug in meta/moods.aura or global vocab            |
| @meta/genres | The meta/genres.aura file for this project                      |
| @meta/roles  | The meta/roles.aura file for this project                       |
| @meta/moods  | The meta/moods.aura file for this project                       |

Global cloud forms:

    @aduki.org/genre/electronic
    @aduki.org/genre/[electronic, afro-soul]
    @aduki.org/role/main-artist
    @aduki.org/role/[main-artist, vocalist]
    @aduki.org/mood/ethereal

---

### Resolution Cascade

When the engine encounters a vocabulary reference, it resolves in this order:

1. The local meta/ folder for this project
2. The parent catalog's meta/ folder (if inside a catalog)
3. The global platform vocabulary at @aduki.org/genre/, @aduki.org/role/, @aduki.org/mood/
4. If not found anywhere: compile warning, stored as unresolved string

---

### Backward Compatibility — String Literals

The bare string form is still valid. The compiler tries to resolve each
string against the vocabulary automatically.

    ## These two produce identical ATOM output when "electronic" exists in vocab
    genre -> Electronic | Afro-Soul | Experimental
    genre -> @genre/[electronic, afro-soul, experimental]

The compiler lowercases and slugifies the string, looks it up in the vocab,
and if found, replaces it with a typed reference arc. If not found, it stores
the string literal and issues a compile warning in strict mode. This means
existing AURA files continue to compile without modification.

---

## Part VIII — Media Kinds

### Kind Declaration

Every AURA document declares its media kind upfront. The schema block also
declares the annotator responsible for this file.

    schema::
      root       -> https://hami.aduki.org/aura/1.0
      kind       -> audio::music
      lang       -> en-US
      annotator  -> @annotator/p9xb3mn

### Supported Kinds

    ## Audio
    kind -> audio::music          ## album, EP, single
    kind -> audio::podcast        ## podcast show or episode
    kind -> audio::audiobook      ## spoken word with chapters
    kind -> audio::live           ## live recording

    ## Video
    kind -> video::movie          ## feature or short film
    kind -> video::series         ## episodic series
    kind -> video::podcast        ## video podcast episode
    kind -> video::documentary    ## documentary
    kind -> video::music          ## music video
    kind -> video::live           ## live performance or concert
    kind -> video::short          ## short-form under 10 minutes

    ## Mixed
    kind -> mixed::album          ## visual album — audio and video tied
    kind -> mixed::interactive    ## interactive or branching media

The kind is not cosmetic. It tells the AURA engine which namespaces are
required, which are optional, and how to map the structure to ATOM.

---

## Part IX — Content Node Architecture

### The Granularity Stack

AURA maps to ATOM's content node hierarchy. The engine walks this stack:

    [Macro-Node]  Act / Scene / Shot / Verse       MANDATORY container
        [Meso-Node]  Line / Dialogue               MANDATORY temporal anchor
            [Micro-Node]  Word / Token             optional richsync
                [Nano-Node]  Syllable / Phoneme    optional karaoke
                    [Pico-Node]  Letter / Character   optional 60fps animation

You do not have to go all the way down. Define as much granularity as your
use case needs. The engine degrades gracefully. If syllables are absent, it
falls back to words. If words are absent, it falls back to lines.

Macro and Meso are mandatory in every AURA document that contains content
nodes. Micro, Nano, and Pico are optional.

### Node Identifier Convention

Every named node uses the slash identifier convention. The type comes before
the slash. The label after the slash is an ordinal in English words or a
unique meaningful name in lowercase with hyphens if compound.

    verse/one::        chorus/one::      bridge/one::
    verse/two::        chorus/two::      scene/cold-open::
    line/one::         word/one::        syllable/one::
    chapter/intro::    act/one::         shot/one::

---

### Content Nodes for Music

    ## FILE: tracks/t7xab3c.aura
    >> @info/metadata

    schema::
      root       -> https://hami.aduki.org/aura/1.0
      kind       -> audio::music
      lang       -> en-US
      annotator  -> @annotator/p9xb3mn

    manifest::
      name   ! -> "Signal Loss"
      creator  -> @person/p4xt9k2

    ## Content node hierarchy (Macro > Meso > Micro > Nano)
    verse/one::
      time -> 22s~1m10s
      lines::
        line/one::
          text    -> "The signal drops at the edge of the grid"
          time    -> 22s~25.4s
          speaker -> @person/p4xt9k2
          words::
            word/one::
              text -> "The"
              time -> 22s~22.3s
              syllables::
                syllable/one::
                  text -> "The"
                  time -> 22s~22.3s
            word/two::
              text -> "signal"
              time -> 22.3s~22.9s
        line/two::
          text    -> "She said my name like static"
          time    -> 25.4s~28.8s
          speaker -> @person/p4xt9k2

    chorus/one::
      time -> 1m10s~1m50s
      lines::
        line/one::
          text -> "Find me where the frequency breaks"
          time -> 1m10s~1m16s
        line/two::
          text -> "Find me at the edge of everything"
          time -> 1m16s~1m22s

    credits::
      vocals   -> @person/p4xt9k2
      producer -> @author/j8mn2rk
      writer   -> @person/k7wrt2

---

### Content Nodes for Podcasts

    ## FILE: episodes/ep7xb3n.aura
    >> @info/metadata

    schema::
      root      -> https://hami.aduki.org/aura/1.0
      kind      -> audio::podcast
      lang      -> en-US
      annotator -> @annotator/p9xb3mn

    manifest::
      name    ! -> "Recursive"
      show      -> "The Mesh"
      season    -> one
      episode   -> three
      host      -> @person/cc9xpq

    chapter/cold-open::
      name     -> "Cold Open"
      time     -> 0s~3m15s
      speakers -> @person/[cc9xpq]
      lines::
        line/one::
          text    -> "Welcome back to The Mesh."
          speaker -> @person/cc9xpq
          time    -> 0s~2s
        line/two::
          text    -> "Today we go deep into something that terrifies
                      engineers and excites theorists in equal measure."
          speaker -> @person/cc9xpq
          time    -> 2s~9s

    chapter/interview::
      name     -> "Interview Begins"
      time     -> 3m15s~18m40s
      speakers -> @person/[cc9xpq, lp2wnr]

---

### Content Nodes for Films

    ## FILE: f6np2qr.aura   (or split into per-scene files under scenes/)
    >> @info/metadata

    schema::
      root      -> https://hami.aduki.org/aura/1.0
      kind      -> video::movie
      lang      -> en-US
      annotator -> @annotator/p9xb3mn

    scene/cold-open::
      name     -> "Cold Open: Sector 7"
      time     -> 00:00:00~00:03:15
      location -> "Sector 7 Grid Hub"
      cast     -> @person/[mt4qbz]

    scene/the-diagnostic::
      name     -> "The Diagnostic"
      time     -> 00:03:15~00:11:40
      cast     -> @person/[mt4qbz, vr8kfw]
      dialogue::
        line/one::
          speaker -> @person/mt4qbz
          text    -> "The mesh isn't failing — it's been redirected."
          time    -> 00:04:12~00:04:18

    scene/root-node::
      name -> "The Root Node"
      time -> 00:38:20~00:44:10
      cast -> @person/[mt4qbz]
      note -> "Callback to @scene/cold-open visual motif"

---

## Part X — Support Node Architecture

### Why Support Nodes Exist

Content nodes carry text payloads — the lyrics, dialogue, words, syllables,
and characters that are rendered to the audience. Support nodes carry
structured metadata that supports, contextualizes, and governs those content
nodes without carrying renderable text.

Support nodes are time-anchored relational objects compiled to the ATOM mesh
as first-class place nodes in the OCPN graph. They are registered in the
interval tree alongside content nodes. A stabbing query at any time t returns
both the active content nodes and all support nodes whose intervals overlap t.

Support nodes are authored in the support:: namespace within any content file.
They can be declared at document level for nodes that span the entire work.

---

### Segment Support Nodes

A segment node marks a named musical or structural section. It has temporal
extent but no lyric payload. Segment support nodes use the slash identifier
convention matching their content node counterparts.

    support::
      segments::

        intro/one::
          kind    -> intro
          time    -> 0s~22s
          energy  -> 0.3

        verse/one::
          kind    -> verse
          time    -> 22s~1m10s
          energy  -> 0.6
          instruments -> @instrument/[keys, bass]

        chorus/one::
          kind    -> chorus
          time    -> 1m10s~1m50s
          energy  -> 0.9

        bridge/one::
          kind    -> bridge
          time    -> 2m30s+28s
          energy  -> 0.5
          note    -> "Key modulation — Ab major to F# minor"

The kind field accepts: intro, verse, pre-chorus, chorus, post-chorus,
bridge, drop, breakdown, outro, interlude, instrumental, transition, ad-lib,
hook, or any custom value.

---

### Instrument Support Nodes

An instrument node tracks the presence and activity of a specific instrument
across one or more time windows within a track.

    support::
      instruments::

        keys::
          name      -> "Rhodes Piano"
          family    -> keys
          performer -> @author/j8mn2rk
          active::
            - 0s~22s
            - 22s~1m10s
            - 2m30s~3m47s

        bass::
          name    -> "Electric Bass"
          family  -> bass
          active::
            - 22s~3m47s

        drums::
          name   -> "Electronic Drum Kit"
          family -> percussion
          active::
            - 22s~2m30s
            - 2m58s~3m47s
          stem   -> @track/t7xab3c::stems/drums

---

### Chapter Support Nodes

A chapter node marks a navigable division. It is the primary navigation unit
for podcast chapters, audiobook chapters, film act breaks, and album sides.

    support::
      chapters::

        side/one::
          kind   -> side
          title  -> "Side A"
          index  -> one
          time   -> 0s~22m00s
          cover  -> @art/cover-a

        side/two::
          kind   -> side
          title  -> "Side B"
          index  -> two
          time   -> 22m00s~44m30s
          cover  -> @art/cover-b

---

### Credit Window Support Nodes

A credit window node anchors a contributor credit to a specific time window.
It answers when in the playback timeline a given person's contribution occurs.

    support::
      credits::

        credit/one::
          person  -> @person/p4xt9k2
          role    -> lead-vocal
          time    -> 22s~1m10s
          via     -> @verse/one

        credit/two::
          person  -> @author/j8mn2rk
          role    -> keys
          time    -> 0s~22s

---

### Translation Support Nodes

A translation node shadows a content node in a different language or script.
It carries a text payload that is always derivative — a translation or
transliteration of the canonical content node it shadows.

Translation nodes are structured within the support:: namespace and grouped
by locale. A single content node can have translations in multiple locales
simultaneously.

    support::
      translations::

        fr-FR::
          line/one::
            source  -> @verse/one/line/one
            locale  -> fr-FR
            text    -> "Le signal tombe au bord de la grille"
            time    -> 22s~25.4s
            via     -> @annotator/p3xr7kn

          line/two::
            source  -> @verse/one/line/two
            locale  -> fr-FR
            text    -> "Elle a dit mon nom comme de la statique"
            time    -> 25.4s~28.8s

        sw-KE::
          line/one::
            source  -> @verse/one/line/one
            locale  -> sw-KE
            script  -> Latn
            text    -> "Ishara inashuka ukingoni mwa gridi"
            time    -> 22s~26.1s

---

### Mood Support Nodes

A mood node attaches emotional, tonal, or affective metadata to a time window.

    support::
      moods::

        mood/one::
          time        -> 0s~22s
          descriptors -> ethereal | reflective
          confidence  -> 0.82
          source      -> authored

        mood/two::
          time        -> 1m10s~1m50s
          descriptors -> euphoric
          confidence  -> 0.91
          source      -> hybrid
          note        -> "High-energy chorus — confirmed by analysis engine"

The mood vocabulary is declared in the directives:: namespace for this file.
The mood-vocabulary directive specifies which descriptor values are valid.

---

### Rights Support Nodes

A rights node marks a segment with licensing or rights boundary metadata.

    support::
      rights::

        rights/doc::
          scope   -> document
          territory -> worldwide
          license -> "ISRC:KE-A00-24-00001"
          holder  -> @person/p4xt9k2
          expires -> 2034-11-01

        rights/sample::
          scope     -> window
          time      -> 2m44s~2m58s
          territory -> worldwide
          license   -> proprietary
          holder    -> "Unnamed Session Records"
          blocked   -> US | CA
          note      -> "Sample clearance pending in North American territories"

---

### Ad Slot Support Nodes

An ad slot node defines a pre-declared advertising insertion point.

    support::
      slots::

        slot/pre::
          kind     -> pre-roll
          time     -> 0s~0s
          max      -> 30s
          fill-policy -> optional

        slot/mid::
          kind     -> mid-roll
          time     -> 1m50s~1m50s
          max      -> 60s
          fill-policy -> required

        slot/post::
          kind     -> post-roll
          time     -> 3m47s~3m47s
          max      -> 30s
          fill-policy -> house

---

### Sync Anchor Support Nodes

A sync anchor node declares an explicit, authored hard recovery point.

    support::
      anchors::

        anchor/verse-one::
          kind -> hard
          time -> @time/22s
          note -> "Start of verse one — verified spectral anchor"

        anchor/chorus-one::
          kind -> verified
          time -> @time/1m10s

        anchor/chorus-two::
          kind -> hard
          time -> @time/2m00s

---

### Tempo Support Nodes

A tempo node declares the rhythmic tempo at a time window. Tempo changes affect
lyric sync directly — the engine uses tempo nodes to validate and adjust word-
and syllable-level timing when the beat grid shifts. This is critical for tracks
with tempo automation, live recordings, or freestyle sections.

    support::
      tempo::

        tempo/one::
          time    -> 0s~1m10s
          bpm     -> 112
          type    -> steady
          grid    -> 4/4

        tempo/two::
          time    -> 1m10s~1m50s
          bpm     -> 116
          type    -> increasing
          note    -> "Slight tempo push into chorus"

        tempo/three::
          time    -> 2m30s~2m58s
          bpm     -> 92
          type    -> variable
          note    -> "Bridge — rubato feel, no strict grid"

Fields:

- time — the interval during which this tempo applies
- bpm — beats per minute at the start of the interval
- type — one of: steady, increasing, decreasing, variable, free
- grid — time signature (4/4, 3/4, 6/8, etc.)
- note — optional annotation

References:

    ## A lyric line may declare the tempo it falls under
    line/one::
      text  -> "The signal drops at the edge of the grid"
      time  -> 22s~25.4s
      tempo -> @tempo/one

---

### Sample Support Nodes

A sample node attributes a sampled element to its original source: which track,
which segment, what kind of sample, and clearance status. Samples differ from
interpolations — a sample uses the actual audio recording; an interpolation
re-records the same melody or progression.

    support::
      samples::

        sample/one::
          source  -> @track/t9vkx7q        ## original track sampled
          time    -> [1m20s, 1m24s, 4s]    ## portion of source used
          used-at -> 2m44s                  ## where this appears in the work
          kind    -> loop                   ## loop | stab | chop | vocal | melodic | rhythmic
          cleared -> live
          note    -> "Four-bar drum loop from bridge section"

        sample/two::
          source  -> @aduki.org/track/mx4nq7b
          time    -> [0s, 3s, 3s]
          used-at -> 0s
          kind    -> vocal
          cleared -> dark
          note    -> "Vocal stab — clearance pending"

Fields:

- source — @track/id or global @aduki.org/track/id of the sampled work
- time — [start, end, duration] of the portion taken from the source
- used-at — time point in this work where the sample appears
- kind — loop | stab | chop | vocal | melodic | rhythmic | atmospheric | custom
- cleared — boolean: true if rights are confirmed, false if pending
- note — optional annotation

---

### Explainer Support Nodes

An explainer node attaches a detailed explanation to any content node — a line,
a word, a scene, an act. Explainers may be cultural glosses, historical context,
lyrical interpretation, technical notes, or detailed per-node documentation.

    support::
      explainers::

        explainer/line-four::
          target  -> @line/four
          kind    -> cultural
          lang    -> en-US
          text    -> "\"She said my name like static\" — references the protagonist's
                       experience of dissociation during radio interference outages
                       documented in urban Nairobi circa 2019."

        explainer/chorus-concept::
          target  -> @chorus/one
          kind    -> lyrical
          lang    -> en-US
          text    -> "The chorus collapses the metaphor: \"frequency\" = emotional bandwidth.
                       The grid is both the power grid and the social one."

        explainer/bridge-chord::
          target  -> @bridge/one
          kind    -> technical
          lang    -> en-US
          text    -> "Key modulation from Ab major to F# minor via enharmonic pivot
                       (G# = Ab). Creates tension before final chorus resolution."

Fields:

- target — @node/id of the content or support node being explained
- kind — cultural | lyrical | historical | technical | translation | annotation | custom
- lang — IETF BCP 47 locale of this explanation
- text — the explanation content
- via — optional @person/id of the author of this explanation
- note — optional further annotation

---

### Interpolation Support Nodes

An interpolation node attributes a melodic, harmonic, or lyrical element
that is re-recorded from an existing composition — distinct from a sample,
which uses the original audio. Interpolations require a separate mechanical
license, not a master license.

    support::
      interpolations::

        interpolation/one::
          source   -> @aduki.org/track/mx4nq7b
          element  -> melody
          time     -> [32s, 1m10s, 38s]    ## interval in this work
          cleared  -> true
          writers  -> @people/[p4xt9k2, k7wrt2]
          note     -> "Chorus melody re-records main hook from source. Separate
                       mechanical clearance filed 2024-09."

Fields:

- source — @track/id or global URI of the original composition
- element — melody | chord-progression | lyric | rhythm | hook | bassline | custom
- time — [start, end, duration] interval in this work where it appears
- cleared — whether mechanical license is confirmed
- writers — @people/[...] who wrote the interpolated element in this work
- note — optional annotation

---

### Instruction Support Nodes

An instruction node carries a processing instruction — a directive to the engine,
player, or downstream system. Instructions are not rendered to the audience.
They control playback behavior, encoding choices, looping logic, or editorial flags.

    support::
      instructions::

        instruction/intro-loop::
          kind      -> loop
          target    -> @segment/intro-one
          condition -> @event/first-play
          count     -> 2
          note      -> "Loop intro twice before verse on first play"

        instruction/fade-out::
          kind      -> fade
          target    -> @segment/outro-one
          duration  -> 8s
          type      -> linear
          note      -> "Fade to silence over last 8 seconds"

        instruction/skip-ad::
          kind      -> skip
          target    -> @slot/mid
          condition -> @event/subscriber
          note      -> "Skip mid-roll for authenticated subscribers"

Fields:

- kind — loop | skip | jump | repeat | fade | crossfade | trim | mute | custom
- target — @segment/id, @slot/id, or @chapter/id this instruction applies to
- condition — optional @event/id that triggers this instruction
- count — for loop: number of repetitions
- duration — for fade: fade length
- type — for fade: linear | exponential | logarithmic
- note — optional annotation

---

### Event Support Nodes

An event node defines a condition-triggered signal. When the engine detects the
triggering condition at playback time, it fires the configured signal to all
registered listeners. Events bridge AURA media data with real-world reactive
systems — smart lighting, AR effects, haptic feedback, IoT stage control,
accessibility adapters, and interactive overlays.

    support::
      events::

        event/lights-dim::
          trigger   -> @moods/[ominous, tense]
          at        -> @time/1m32s
          signal    -> lights::dim(0.1)
          kind      -> ambient
          note      -> "Dim all stage lights to 10% when tense mood begins"

        event/strobe-drop::
          trigger   -> @segment/drop-one
          at        -> onset                   ## fires at the start of the segment
          signal    -> lights::strobe | haptic::pulse(80ms)
          kind      -> reactive
          note      -> "Strobe + haptic pulse on bass drop"

        event/scene-colour::
          trigger   -> @scene/cold-open
          at        -> onset
          signal    -> lights::colour(#0a1628) | ar::overlay(frost)
          kind      -> ambient
          note      -> "Cold-open: deep navy, AR frost overlay on screens"

        event/credits-roll::
          trigger   -> @segment/outro-one
          at        -> onset
          signal    -> display::credits | lights::fade(white, 30s)
          kind      -> editorial
          note      -> "Begin credits display and fade to white on outro"

        event/subscriber-only::
          trigger   -> condition::subscriber == false
          at        -> @slot/mid
          signal    -> player::insert-ad
          kind      -> interactive

Fields:

- trigger — what fires this event:
  a mood reference (@mood/slug or @moods/[...]),
  a segment reference (@segment/id),
  a scene (@scene/id), a time point (@time/value),
  or a boolean condition string
- at — when within the trigger's interval to fire: onset | offset | peak | @time/value
- signal — what to emit. Free-form dotted path: system::action(params).
  Multiple signals separated by |
- kind — ambient | reactive | interactive | editorial | broadcast | custom
- note — optional

Signal path convention:

    lights::dim(0.1)              <- lights subsystem, dim to 10%
    lights::colour(#0a1628)      <- set colour
    lights::strobe                <- strobe
    haptic::pulse(80ms)          <- haptic motor, 80ms burst
    ar::overlay(frost)            <- AR layer
    display::credits              <- show credits overlay
    player::insert-ad             <- trigger ad insertion
    player::pause                 <- pause playback
    iot::gpio(17, HIGH)          <- raw GPIO signal for custom hardware

Events compile to ATOM EventNode objects (node class 0x0D) registered in the
interval tree. The engine evaluates their trigger conditions during the stabbing
query loop and emits signals via the registered EventBus.

---

## Part XI — The History System

### Why History Is First-Class

Every change to an AURA document is permanent by design. No take is ever
overwritten or deleted. The entire history of a file — every edit, every
annotator's contribution, every version of every line — is preserved in the
ledger and accessible via the @history reference domain.

This is not backup. This is provenance. In music, film, and spoken word, the
origin of every lyric, every subtitle edit, every credit change is a legal and
creative matter. AURA treats history as data, not metadata.

---

### Media-Native Vocabulary

AURA uses its own versioning vocabulary derived from the recording studio,
the film production floor, and the broadcast booth. Git verbs are not used.

| AURA term   | Meaning                                              | Origin                                         |
| ----------- | ---------------------------------------------------- | ---------------------------------------------- |
| **take**    | A recorded snapshot of the current state             | Studio takes: "take one", "take two"           |
| **mark**    | A named reference point on the ledger                | Cue marks and chapter marks in production      |
| **stream**  | A named line of parallel development                 | Audio streams; parallel recording sessions     |
| **delta**   | The set of changes between two takes                 | Signal processing: a differential change       |
| **rewind**  | Restore the working state to a previous take         | Tape rewind — the universal undo of recording  |
| **mix**     | Combine two streams into one                         | Audio mixing; merging two production branches  |
| **ledger**  | The full ordered history of all takes                | Production ledger; a cue sheet of all events   |
| **hold**    | Temporarily set aside uncommitted changes            | "Put on hold" — park work without taking       |
| **recall**  | Load a specific take as the working state            | Session recall; restoring a saved session      |
| **release** | Publish the current take to public distribution      | Releasing a record or episode to the world     |
| **sync**    | Pull the latest state from the cloud distribution    | Syncing a session from the master archive      |
| **dub**     | Create an independent copy of the full history       | Dubbing a tape — a full copy                   |
| **draft**   | The current uncommitted working state                | Working draft before a take is recorded        |

---

### Taking a Snapshot

A take captures the current state of one or more AURA files. It receives a
generated ID and an optional message. Takes are immutable once recorded.

    ## Toolchain commands
    aura take                                -> cx3ab7k
    aura take "Finalized verse two timing"   -> cx3ab8m
    aura take "Released to Spotify"          -> cx3ab9n

The take ID follows the standard format with the `tx` prefix reserved for
take objects:

    tx3ab7k    <- a take

Takes are ordered, append-only, and permanent. Every node state at every
take is resolvable via @history.

---

### Marking a Take

A mark is a human-readable name attached to a specific take. Marks are how
you identify releases, versions, significant milestones, and named checkpoints.

    ## Toolchain commands
    aura mark v1.0        -> marks current take as "v1.0"
    aura mark premiere    -> marks current take as "premiere"
    aura mark final-mix   -> named checkpoint

Marks resolve via @history just like take IDs:

    @history/v1.0
    @history/premiere
    @history/final-mix

---

### The @history Reference Domain

`@history` is the AURA reference domain for versioned access. It accepts
take IDs, mark names, ordinal positions, and node paths.

#### Referencing a Take

    ## A specific take by generated ID
    @history/tx3ab7k

    ## A named mark
    @history/v1.0
    @history/premiere

    ## Relative position (ordinal from current, past-tense)
    @history/~1       <- one take before current
    @history/~5       <- five takes before current
    @history/~0       <- current take (same as @history/current)

    ## The first take ever recorded
    @history/origin

    ## The current take (latest)
    @history/current

#### Referencing a Node at a Specific Take

Any node path follows the take reference via ::

    ## What did verse/one look like at take v1.0?
    @history/v1.0::verse/one

    ## What was line/four at five takes ago?
    @history/~5::verse/one/line/four

    ## The original first take's chorus
    @history/origin::chorus/one

    ## Compare node at two specific marks
    @history/v1.0::verse/one/line/two
    @history/premiere::verse/one/line/two

This is how editors verify what a specific line said at a specific version,
what a credit said at release, or what a subtitle read at broadcast.

---

### In-Document History Declarations

An AURA document may declare history references inline as metadata, linking
back to the take at which a piece of content was first authored or last changed.

    ## In a line node
    line/four::
      text     -> "She said my name like static"
      time     -> 25.4s~28.8s
      speaker  -> @person/p4xt9k2
      authored -> @history/tx3ab7k      ## take when this line was first recorded
      revised  -> @history/v1.0         ## take when last revised

    ## In a translation node
    fr-FR/line/one::
      source   -> @verse/one/line/one
      text     -> "Le signal tombe au bord de la grille"
      via      -> @annotator/p3xr7kn
      authored -> @history/tx3ab8m

---

### Streams — Parallel Development

A stream is a named parallel line of development within a project. Streams
allow simultaneous work on translations, regional variants, alternative edits,
and pre-release drafts — all without affecting the main line.

    ## Toolchain commands
    aura stream open translation-fr      ## open a new stream
    aura stream open deluxe-edition      ## another parallel stream
    aura stream list                     ## show all active streams
    aura stream close translation-fr     ## close and archive a stream

Streams are referenced in @history paths:

    @history/stream/translation-fr
    @history/stream/translation-fr::verse/one

Referencing the main line:

    @history/main                        ## the primary line of development
    @history/main::chorus/one

---

### Rewinding

Rewind restores the working draft to the state of a previous take. The
rewind itself is non-destructive — a rewind creates a new take, pointing
the draft to the older state. The history ledger always grows forward.

    ## Toolchain commands
    aura rewind tx3ab7k                  ## rewind to a specific take
    aura rewind v1.0                     ## rewind to a named mark
    aura rewind ~3                       ## rewind three takes

After rewinding, a fresh take records the restored state:

    aura rewind v1.0
    aura take "Restored to premiere version"   -> cx3ac0p

The history now shows the chain:

    tx3ab7k  "original"
    tx3ab8m  "Finalized verse two"
    cx3ab9n  "Released to Spotify"       <- forward history preserved
    cx3ac0p  "Restored to premiere"      <- rewind take — ledger entry

---

### Viewing the Ledger

    ## Show full history of this file
    aura ledger

    ## Show history of a specific node
    aura ledger verse/one/line/four

    ## Show delta between two takes
    aura delta tx3ab7k tx3ab8m

    ## Show delta between a mark and current
    aura delta v1.0 current

The ledger output shows:

    take      mark       when                  message
    cx3ac0p              2025-04-13T18:00Z     "Restored to premiere version"
    cx3ab9n   release    2025-03-01T09:00Z     "Released to Spotify"
    cx3ab8m   v1.0       2025-02-14T12:30Z     "Finalized verse two timing"
    tx3ab7k   origin     2025-01-10T10:00Z     "First take"

---

### Mixing Streams

When work on a stream is complete, it is mixed back into the main line
(or another stream).

    aura mix translation-fr              ## mix translation-fr into current stream
    aura take "Mixed French translation" -> cx3ac1q

The mix creates a delta between the stream's head and the target, applies it,
and records a new take. Both streams' full histories remain intact and
independently resolvable.

---

### Sync and Release

`release` publishes the current take to the cloud distribution. `sync`
pulls the latest released state from the cloud into the local working state.

    aura release                         ## publish current take to @aduki.org store
    aura sync                            ## pull latest from @aduki.org store

After release, the take is also accessible from the global cloud path:

    @aduki.org/track/t7xab3c/history/current
    @aduki.org/track/t7xab3c/history/premiere
    @aduki.org/track/t7xab3c/history/~1::verse/one/line/four

---

### Holding Work

`hold` parkss the current working draft without taking a snapshot. Useful
when switching to a different stream mid-session.

    aura hold                            ## park current draft
    aura stream open urgent-fix          ## switch context
    ## ... make changes ...
    aura take "Emergency credit fix"
    aura stream close urgent-fix
    aura hold restore                    ## restore parked draft

---

### History in the Engine

The history system compiles to HAMI HistoryNode objects (node class 0x14).
The engine stores the full delta chain, not full copies. Each take records
only what changed from the previous take. The ATOM stabbing query can be
directed at any historical take state via the @history domain.

A node query at @history/v1.0::verse/one causes the engine to reconstruct
the verse/one node from the delta chain up to the v1.0 take and return it
as if it were the current state — without altering the active document.

History resolution is always read-only. The working state can only change
via take, rewind, or mix.

---

## Part XII — The Manifest and Meta Namespaces

### What Each Namespace Does

manifest:: — The identity of this work. What is it? Who made it? What is
it called? These fields are permanent and do not change across releases.

meta:: — The context of this work. When was it released? What genre?
What are its relationships to other works? What credits apply globally?
These fields may be updated.

For projects with multiple files, both manifest:: and meta:: live in
info/metadata.aura and are inherited by all member files via >>.

---

### Manifest

    manifest::
      name      ! -> "Signal Loss"
      creator   ! -> @person/p4xt9k2
      version     -> 1.0.0
      language    -> en-US
      released    -> 2024-11-01
      label       -> "Self-Released"
      territory   -> worldwide

For a series or podcast with episodes, manifest:: describes the show,
while each track or episode file carries its own identity.

---

### Meta

    meta::
      genre      -> Electronic | Afro-Soul | Experimental
      tags       -> Nairobi | Instrumental | Ambient
      isrc       -> "KE-A00-24-00001"
      bpm        -> 112
      key        -> "F# minor"

      credits::
        main       -> @person/ao9txa
        vocals     -> @person/ao9txa
        producer   -> @person/j3fmr9
        writer     -> @person/k7wrt2 | @person/h8dv5x
        mixer      -> @person/j3fmr9
        mastering  -> @person/j3fmr9

      description -> "Five tracks built from field recordings across Nairobi.
                      Composed by @person/ao9txa over eighteen months.
                      Mixed and mastered by @person/j3fmr9."

      related::
        previous   -> @collection/c4xvp2k          ## prior release
        next       ?
        sampled    -> @track/t9vkx7q @time/2m44s  ## sampled segment

---

## Part XII — Collections and Variations

### Why Collections Are First-Class in AURA

A single AURA document describes one work: one track, one episode, one film.
But media is almost never singular. An album is a collection of tracks.
A series is a collection of episodes. A label's catalog is a collection
of albums.

AURA handles this through the collection:: namespace, which authors the
CollectionManifest that the engine compiles to a HAMI index document.
A collection file is authored separately from its member files.

---

### Collection File Structure

A collection file carries the schema, manifest, meta, and collection::
namespaces. It does not carry tracks::, scenes::, or chapters:: — those
belong to the member files.

    ## FILE: collections/c8xab3d.aura

    schema::
      root    -> https://hami.aduki.org/aura/1.0
      kind    -> audio::music
      lang    -> en-US

    --

    manifest::
      name       ! -> "Signal Loss"
      creator    ! -> @person/ao9txa
      version      -> 1.0.0
      released     -> 2024-11-01
      territory    -> worldwide
      label        -> "Self-Released"

    --

    meta::
      genre      -> Electronic | Afro-Soul | Experimental
      total-duration -> 22m41s
      track-count    -> 5

    --

    collection::
      kind        -> album
      persons-ref -> @info/people

      members::

        track.01::
          index    -> 1
          name     -> "Signal Loss"
          aura-ref -> tracks/t7xab3c.aura
          duration -> 3m47s

        track.02::
          index    -> 2
          name     -> "Fold"
          aura-ref -> tracks/t4mn2rp.aura
          duration -> 4m12s

        track.03::
          index    -> 3
          name     -> "Recursive"
          aura-ref -> tracks/t9vkx7q.aura
          duration -> 5m08s

        track.04::
          index    -> 4
          name     -> "Meridian"
          aura-ref -> tracks/t2nq5wb.aura
          duration -> 4m55s

        track.05::
          index    -> 5
          name     -> "Origin Point"
          aura-ref -> tracks/t6rj8vc.aura
          duration -> 4m39s

---

### Declaring Variations

Variations are declared within the collection:: namespace. Each variation
references the canonical member it diverges from.

    collection::
      ## ... members as above

      variations::

        track.01.acoustic::
          canonical-ref   -> track.01
          variant-kind    -> acoustic
          name            -> "Signal Loss (Acoustic)"
          aura-ref        -> variants/t7xab3c-acoustic.aura
          duration-delta  -> +0m34s

        track.01.radio-edit::
          canonical-ref   -> track.01
          variant-kind    -> radio-edit
          name            -> "Signal Loss (Radio Edit)"
          aura-ref        -> variants/t7xab3c-radio.aura
          duration-delta  -> -0m47s

        track.01.sw-KE::
          canonical-ref   -> track.01
          variant-kind    -> dubbed
          locale          -> sw-KE
          name            -> "Signal Loss (Kiswahili)"
          aura-ref        -> variants/t7xab3c-swahili.aura
          duration-delta  -> +0m04s

---

### Series and Season Nesting

A series is authored as a collection of seasons. A season is authored as
a collection of episodes. The structure mirrors the content hierarchy.

    ## FILE: collections/c3xn7rp.aura

    schema::
      root -> https://hami.aduki.org/aura/1.0
      kind -> audio::podcast
      lang -> en-US

    manifest::
      name       ! -> "The Mesh"
      creator    ! -> @person/cc9xpq
      version      -> 1.0.0

    collection::
      kind        -> series
      persons-ref -> @info/people

      members::

        season.01::
          kind     -> season
          index    -> 1
          name     -> "Season One"
          aura-ref -> seasons/s1xp4fm.aura

        season.02::
          kind     -> season
          index    -> 2
          name     -> "Season Two"
          aura-ref -> seasons/s2xq8nt.aura

A season collection file follows the same pattern, listing episode member
files in its members:: block.

---

### Shared Person Index

When a collection:: block declares a persons-ref, the engine uses that
file as the primary person index for all members of the collection.
Individual member files may still declare a local persons:: block for
members not present in the shared index. The engine resolves @person/slug
references against the shared index first, then the local block.

    ## FILE: info/people.aura

    people::

      ao9txa::
        name    -> "Mario Aleka"
        roles   -> main-artist | vocalist | composer
        country -> KE

      j3fmr9::
        name    -> "Jay Femar"
        roles   -> producer | mixer
        country -> KE

      k7wrt2::
        name    -> "Janet Doe"
        roles   -> writer

---

### Cross-Member References

Within any member file, you can reference other members in the same
collection using the @member/ domain.

    ## In track 03, reference a moment in track 01
    note -> "Melodic theme introduced at @member/track.01 @time/1m10s"

    ## Reference a specific line in another episode
    ## (from an episode AURA file within a series)
    related-to -> @member/ep7xb3n::scene/cold-open

---

## Part XII — Source Data Store Directives

### Declaring the Store

The directives:: namespace is extended to support source data store
declaration. This tells the engine where to find and write compiled
artifacts for this file and its collection.

    directives::
      store           -> aura://store.hami.aduki.org/catalogs/cx0ab3de
      collection-ref  -> @collection/c8xab3d
      variation-default -> canonical
      rights-verify   -> true
      granularity     -> word
      dtw-enabled     -> true
      mood-vocabulary -> joyful | melancholic | tense | euphoric |
                         reflective | aggressive | ethereal | romantic

---

### Store URI Formats

    ## Cloud object store (production)
    store -> aura://store.hami.aduki.org/catalogs/{catalog-id}

    ## Cloudflare R2 or equivalent
    store -> r2://bucket-name/catalogs/{catalog-id}

    ## Self-hosted
    store -> https://media.example.com/atom/{catalog-id}

    ## Local filesystem (development)
    store -> file:///home/user/projects/signals/dist

---

### Directives Quick Reference

| Directive          | Value Type     | Description                                             |
| ------------------ | -------------- | ------------------------------------------------------- |
| store              | URI            | Source data store base URI                              |
| collection-ref     | file path      | Path or URI to the collection manifest for this file    |
| variation-default  | variant-id     | Default variant when canonical is unavailable           |
| rights-verify      | bool           | Validate RightsNodes against license store at startup   |
| granularity        | level name     | Minimum granularity floor (word, syllable, line)        |
| index-depth        | integer        | B-tree height ceiling for HAMI indexing                 |
| sync-anchors       | list of ids    | Named nodes to treat as hard sync anchors               |
| dtw-enabled        | bool           | Enable temporal alignment engine for variations         |
| mood-vocabulary    | union list     | Valid mood descriptor values for MoodNodes in this file |

---

## Part XIII — Media Assets, Industry Entities, and Availability

### @art — Cover Art Assets

An art node declares a static image asset for a work: album covers, single art, movie posters,
episode art, and show banners. Art assets are **uploaded separately** to the Triverse store to
obtain their cloud URL. The local project stores the URL as literal text — no image files live
in the project. Art nodes live in `info/arts.aura` and compile to HAMI manifests. They are not
interval-tree indexed.

    ## FILE: info/arts.aura

    arts::

      cover-main::
        kind  -> square
        url   -> https://store.hami.aduki.org/art/ar4xab3c?ratio=square
        note  -> "Primary album cover — 3000x3000px"

      cover-wide::
        kind  -> 16:9
        url   -> https://store.hami.aduki.org/art/ar4xab3c?ratio=16x9
        note  -> "Streaming platform header"

      poster-tall::
        kind  -> 2:3
        url   -> https://store.hami.aduki.org/art/ar7mn9rk?ratio=2x3
        note  -> "Tall movie poster"

Aspect ratio kinds: `square`, `landscape`, `portrait`, `16:9`, `4:3`, `9:16`, `21:9`, `2:3`,
`custom`. For custom, declare `width` and `height` in pixels as additional fields.

Reference syntax:

    cover  -> @art/cover-main
    covers -> @arts/[cover-main, cover-wide, poster-tall]

    ## In a manifest
    manifest::
      name  -> "Signal Loss"
      cover -> @art/cover-main

    ## Global cloud reference
    @aduki.org/art/{id}

Art nodes compile to ATOM ArtNode objects (node class `0x15`).

---

### @motion — Motion Cover Assets

A motion node declares a short looping video or animated art asset: Apple Music-style animated
album covers, ambient show backgrounds. Motion assets are **uploaded separately** to the
Triverse store. The local project stores the cloud URL as literal text — no video files live
in the project. Motions are authored in the `motions::` block of `info/arts.aura`.

    motions::

      motion-main::
        kind      -> album-motion
        url       -> https://store.hami.aduki.org/motion/mo7xk9p2
        duration  -> 8s
        loop      -> live
        ratio     -> square
        note      -> "Apple Music-style looping cover — 1080x1080px"

      motion-wide::
        kind      -> album-motion
        url       -> https://store.hami.aduki.org/motion/mo3xb5qr
        duration  -> 6s
        loop      -> live
        ratio     -> 16:9

Motion kinds: `album-motion`, `episode-motion`, `movie-motion`, `show-motion`, `background`.

Reference syntax:

    motion  -> @motion/motion-main
    motions -> @motions/[motion-main, motion-wide]

Motion nodes compile to ATOM MotionNode objects (node class `0x16`).

---

### @trailer — Trailer and Preview Assets

A trailer inherits from `@motion` — it is a purposeful motion clip with deliberate editorial
structure. Trailer assets are **uploaded separately** to the Triverse store. The local project
stores the cloud URL as literal text. Trailers are declared in the `trailers::` block of
`info/arts.aura`.

    trailers::

      main-trailer::
        kind      -> movie-trailer
        url       -> https://store.hami.aduki.org/trailer/tr6xp3lm
        duration  -> 2m30s
        loop      -> dark
        ratio     -> 16:9
        released  -> 2024-09-01
        note      -> "Official theatrical trailer"

      ep-preview::
        kind      -> episode-trailer
        url       -> https://store.hami.aduki.org/trailer/tr2mn8xk
        duration  -> 45s
        ratio     -> 16:9
        loop      -> dark

Trailer kinds: `movie-trailer`, `episode-trailer`, `podcast-trailer`, `series-trailer`,
`teaser`, `announcement`, `behind-the-scenes`.

Reference syntax:

    trailer  -> @trailer/main-trailer
    trailers -> @trailers/[main-trailer, ep-preview]

Trailer nodes compile to ATOM TrailerNode objects (node class `0x17`).

---

### @studio — Production Studio Entities

A studio node declares a production studio, production company, or broadcast network. Studios
support ownership hierarchy via the `parent` field. They are defined in `info/studios.aura`.

    ## FILE: info/studios.aura

    studios::

      st4xab3c::
        name    -> "Warner Bros. Pictures"
        kind    -> film
        country -> US
        founded -> 1923-04-04
        logo    -> @art/wb-logo

      st8mn2rk::
        name    -> "DC Films"
        kind    -> film
        parent  -> @studio/st4xab3c     ## owned by Warner Bros.
        country -> US
        note    -> "DC Comics adaptation studio"

Studio kinds: `film`, `television`, `animation`, `documentary`, `music`, `game`, `custom`.

The engine traverses the `parent` arc chain to resolve rights and credit inheritance.

Reference syntax:

    studio  -> @studio/st4xab3c
    studios -> @studios/[st4xab3c, st8mn2rk]

    ## In a manifest
    manifest::
      studio  -> @studio/st4xab3c

Studio nodes compile to ATOM StudioNode objects (node class `0x18`).

---

### @label — Record Label Entities

A label node declares a music record label or publishing imprint. Labels support ownership
hierarchy via the `parent` field. They are defined in `info/labels.aura`.

    ## FILE: info/labels.aura

    labels::

      lb3xab7c::
        name    -> "Universal Music Group"
        kind    -> major
        country -> US
        founded -> 1934-01-01

      lb7mn4rp::
        name    -> "Def Jam Recordings"
        kind    -> imprint
        parent  -> @label/lb3xab7c    ## owned by UMG
        country -> US
        founded -> 1983-01-01
        note    -> "Hip-hop imprint under UMG"

      lb2xq9bk::
        name    -> "Self-Released"
        kind    -> independent
        country -> KE

Label kinds: `major`, `independent`, `imprint`, `publisher`, `distributor`, `custom`.

Reference syntax:

    label  -> @label/lb7mn4rp
    labels -> @labels/[lb7mn4rp, lb2xq9bk]

Label nodes compile to ATOM LabelNode objects (node class `0x19`).

---

### @watch, @buy, @rent, @download — Content Availability

Availability nodes declare where and how content can be accessed externally. They are
defined in `info/availability.aura` and compiled to HAMI manifests.

    ## FILE: info/availability.aura

    watch::

      netflix::
        platform  -> "Netflix"
        url       -> https://netflix.com/title/12345678
        territory -> worldwide
        quality   -> 4k | hd | sd
        access    -> @access/gated

      youtube::
        platform  -> "YouTube"
        url       -> https://youtube.com/watch?v=abc123
        territory -> worldwide
        quality   -> hd
        access    -> @access/open

    buy::

      itunes::
        platform  -> "Apple TV / iTunes"
        url       -> https://tv.apple.com/movie/buy/12345678
        price     -> "14.99 USD"
        currency  -> USD
        territory -> US
        quality   -> 4k

    rent::

      itunes-rent::
        platform  -> "Apple TV / iTunes"
        url       -> https://tv.apple.com/movie/rent/12345678
        price     -> "3.99 USD"
        currency  -> USD
        territory -> US
        window    -> 30d

    download::

      bandcamp-dl::
        platform  -> "Bandcamp"
        url       -> https://artist.bandcamp.com/album/signal-loss
        territory -> worldwide
        quality   -> lossless
        format    -> flac | mp3 | aac
        drm       -> dark
        access    -> @access/gated

Declared in `manifest::` via the `availability::` block:

    manifest::
      name   -> "Signal Loss"
      cover  -> @art/cover-main
      studio -> @studio/st4xab3c
      label  -> @label/lb7mn4rp

      availability::
        watch    -> @watch/[netflix, youtube]
        buy      -> @buy/itunes
        rent     -> @rent/itunes-rent
        download -> @download/bandcamp-dl

Availability node classes: watch `0x1A`, buy `0x1B`, rent `0x1C`, download `0x1D`.

---

### Person Name Fields and Screen Attribution

Person nodes carry structured name fields so the SDK can show the right name in the right
context — a short on-screen label for dialogue captions, or a full name in credits.

  people::

    p4xt9k2::
      first   -> "Mario"
      last    -> "Aleka"
      screen  -> "Mario"
      legal   -> "Mario A. Mwangi"
      kind    -> artist            ## what kind of creative person
      born    -> 1993-04-11
      country -> KE
      roles   -> @roles/[main-artist, vocalist, composer, producer]
      bio     -> "Nairobi-based producer and vocalist."

    mt4qbz::
      first   -> "Amara"
      middle  -> "Chukwuemeka"
      last    -> "Okafor"
      screen  -> "Amara"          ## short name for on-screen captions
      kind    -> actor
      country -> NG
      roles   -> @roles/[lead-actor, voice-actor]

Name field rules:

| Field    | Required | Description                                                    |
| -------- | -------- | -------------------------------------------------------------- |
| `first`  | yes      | Given name                                                     |
| `middle` | no       | Middle name(s) — may be omitted                                |
| `last`   | no       | Family name — may be omitted for mononyms                      |
| `screen` | no       | Short on-screen identifier for dialogue, captions, mini-player |
| `legal`  | no       | Full legal name when different from first + middle + last      |

Valid `kind` values for a person node: `actor`, `artist`, `musician`, `director`, `producer`,
`writer`, `narrator`, `host`, `journalist`, `comedian`, `athlete`, `presenter`,
`voice-artist`, `dj`, `character`, `custom`.

---

### Comments That Survive Compilation

In AURA, ## annotations are not discarded. They compile to HAMI metadata
under a reserved aura::annotations namespace. They are queryable.

    ## This is an annotation — it compiles into the HAMI file.
    ## Use it to document intent, decisions, or editorial notes.

    track.01::
      ## Intro was recorded in one take — do not re-sequence
      name -> "Signal Loss"

---

## Part XIV — Template Inheritance

### The >> Operator

AURA documents can extend base templates using >>. This allows you to define
shared schema structure once and reuse it across many files.

    ## FILE: templates/music_album.aura
    schema::
      root  -> https://hami.aduki.org/aura/1.0
      kind  -> audio::music

    ## FILE: my_album.aura
    >> templates/music_album

    manifest::
      name -> "My New Album"

The child file inherits all namespace structure from the parent and can
override or extend any field. Inheritance compiles to an ATOM inheritance arc.

---

## Part XV — AURA to ATOM/HAMI Compilation Map

| AURA Construct                   | ATOM/HAMI Target                                      |
| -------------------------------- | ----------------------------------------------------- |
| schema::                         | HAMI top-level namespace root                         |
| kind -> audio::music             | ATOM Macro-layer kind node                            |
| persons:: / people:: / authors:: | ATOM person-index namespace (all aliases)             |
| ao9txa:: under persons           | ATOM person node with ID ao9txa                       |
| manifest::                       | HAMI manifest namespace                               |
| meta::                           | HAMI metadata namespace                               |
| collection::                     | HAMI CollectionManifest document                      |
| members::                        | HAMI ordered member index                             |
| variations::                     | HAMI variation descriptor index                       |
| tracks::                         | ATOM Meso-layer track nodes                           |
| track/one::                      | ATOM Meso-node t7xab3c                                |
| segments:: or chapters::         | ATOM Micro-layer sub-nodes                            |
| lines::                          | ATOM Meso line-level nodes                            |
| words::                          | ATOM Micro word-level nodes                           |
| syllables::                      | ATOM Nano syllable-level nodes                        |
| letters::                        | ATOM Pico character-level nodes                       |
| support::                        | ATOM support namespace                                |
| support.segments::               | ATOM SegmentNode place nodes                          |
| support.instruments::            | ATOM InstrumentNode place nodes                       |
| support.chapters::               | ATOM ChapterNode place nodes                          |
| support.credit-windows::         | ATOM CreditWindowNode place nodes                     |
| support.translations::           | ATOM TranslationNode place nodes                      |
| support.moods::                  | ATOM MoodNode place nodes                             |
| support.rights::                 | ATOM RightsNode place nodes                           |
| support.ad-slots::               | ATOM AdSlotNode place nodes                           |
| support.sync-anchors::           | ATOM SyncAnchorNode place nodes                       |
| arts:: (in info/arts.aura)       | HAMI ArtNode manifest entries (class 0x15)            |
| motions:: (in info/arts.aura)    | HAMI MotionNode manifest entries (class 0x16)         |
| trailers:: (in info/arts.aura)   | HAMI TrailerNode manifest entries (class 0x17)        |
| studios:: (info/studios.aura)    | HAMI StudioNode manifest entries (class 0x18)         |
| labels:: (info/labels.aura)      | HAMI LabelNode manifest entries (class 0x19)          |
| watch:: (availability.aura)      | HAMI WatchNode manifest entries (class 0x1A)          |
| buy:: (availability.aura)        | HAMI BuyNode manifest entries (class 0x1B)            |
| rent:: (availability.aura)       | HAMI RentNode manifest entries (class 0x1C)           |
| download:: (availability.aura)   | HAMI DownloadNode manifest entries (class 0x1D)       |
| @art/id in manifest              | HAMI :: relational arc to ArtNode                     |
| @motion/id in manifest           | HAMI :: relational arc to MotionNode                  |
| @trailer/id in manifest          | HAMI :: relational arc to TrailerNode                 |
| @studio/id in manifest           | HAMI :: relational arc to StudioNode                  |
| @label/id in manifest            | HAMI :: relational arc to LabelNode                   |
| @watch/id in manifest            | HAMI :: relational arc to WatchNode                   |
| @buy/id in manifest              | HAMI :: relational arc to BuyNode                     |
| @rent/id in manifest             | HAMI :: relational arc to RentNode                    |
| @download/id in manifest         | HAMI :: relational arc to DownloadNode                |
| namespace:: (namespace.aura)     | HAMI namespace descriptor node                        |
| exports:: (namespace.aura)       | HAMI exports index for a project namespace            |
| 0s~30s (range)                   | Allen interval triple [0, 30, 30]                     |
| 22s+48s (start+duration)         | Allen interval triple [22, 70, 48]                    |
| [22s, 70s, 48s] (explicit)       | Allen interval triple [22, 70, 48]                    |
| @time/1m32s (point)              | Allen interval triple [92, 92, 0]                     |
| @person/ao9txa in value          | ATOM :: inline relational arc                         |
| @person/ao9txa in field          | ATOM :: relational arc to person node                 |
| @track/02                        | ATOM :: relational arc to track node                  |
| @member/track.01                 | ATOM :: cross-file relational arc                     |
| @collection/id                   | ATOM :: catalog-level relational arc                  |
| >> (inherits)                    | ATOM inheritance arc                                  |
| ? (optional)                     | HAMI optional flag                                    |
| ! (required)                     | HAMI required constraint                              |
| ## annotation                    | HAMI aura::annotations metadata node                  |
| \| (union)                       | HAMI US-delimited field set                           |
| - list item                      | HAMI GS-delimited list                                |

---

## Part XVI — File Naming and Organization

    ## Track and episode source files — named by generated ID
    namespace.aura                     <- project entry point
    tracks/namespace.aura              <- folder namespace manifest
    tracks/t7xab3c.aura
    tracks/t4mn2rp.aura
    episodes/namespace.aura
    episodes/ep7xb3n.aura

    ## Collection manifests — named by generated ID
    collections/c8xab3d.aura   <- album
    collections/c3xn7rp.aura   <- series
    seasons/s1xp4fm.aura       <- season within a series

    ## Info folder — shared project metadata (descriptive names only here)
    info/namespace.aura
    info/people.aura
    info/annotators.aura
    info/metadata.aura
    info/credits.aura
    info/rights.aura
    info/labels.aura           <- record label and imprint entities
    info/studios.aura          <- production studio entities
    info/arts.aura             <- cover art, motion covers, trailers
    info/availability.aura     <- watch/buy/rent/download entries

    ## Meta folder — vocabulary (descriptive names only here)
    meta/namespace.aura
    meta/genres.aura
    meta/roles.aura
    meta/moods.aura

    ## Configs folder — NEVER compiled, NEVER tracked by history
    configs/llm.aura
    configs/stores.aura
    configs/account.aura
    configs/ignore.aura

    ## Reusable templates
    templates/music_album.aura
    templates/podcast_episode.aura
    templates/film.aura
    templates/series.aura

    ## Compiled output — single file
    dist/t7xab3c.atom
    dist/t7xab3c.hami

    ## Compiled output — collection (folder named by collection ID)
    dist/c8xab3d/
      manifest.hami
      people.hami
      annotators.hami
      t7xab3c.atom
      t7xab3c.hami
      t7xab3c-acoustic.atom
      t7xab3c-acoustic.hami
      t7xab3c-acoustic.atlas
      ## ...

---

## Part XVII — Sigil Quick Reference

| Sigil          | Name           | Compiles To                        | Use For                                |
| -------------- | -------------- | ---------------------------------- | -------------------------------------- |
| ::             | Scope Opener   | HAMI namespace / ATOM node         | Opening a block or namespace           |
| ->             | Value Arrow    | HAMI key-value pair                | Assigning a value                      |
| @domain/id     | Reference      | ATOM :: relational arc             | Referencing a named entity             |
| ##             | Annotation     | HAMI aura::annotations             | Queryable documentation                |
| --             | Divider        | none                               | Visual separation only                 |
| \|             | Union Pipe     | HAMI US-delimited set              | One or more values                     |
| ?              | Optional Mark  | HAMI optional flag                 | Field may be absent                    |
| !              | Required Mark  | HAMI required constraint           | Field must be present                  |
| ~              | Range Mark     | ATOM Allen interval start~end      | Time range syntax 1                    |
| +              | Duration Mark  | ATOM Allen interval start+duration | Time range syntax 2                    |
| [, ]           | List Bracket   | Time triple or multi-ID list       | [start, end, dur] / @domain/[id1, id2] |
| >>             | Inherits From  | ATOM inheritance arc               | Template extension                     |
| *              | Wildcard       | ATOM namespace sweep               | Query all in namespace                 |

---

*AURA Language Reference — v0.4*
*Human-readable authoring layer for the Triverse Protocol (ATOM + HAMI)*
*Incorporates: support node layer, [start, end, duration] time triples,*
*collection and variation authoring, cloud source data store directives,*
*@art / @motion / @trailer, @studio / @label, @watch / @buy / @rent / @download,*
*person kind+role, project namespace.aura convention, configs/ folder.*
*Write it like prose. Compile it like a machine.*
