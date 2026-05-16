
## CiNnaMoN (Cognitive-Narrative Musical Notation)

CiNnaMoN is a structured, human-readable temporal annotation layer for expressive or performative text segments.

Music elements may appear as semantic content inside ANY CNML container.


### CiNnaMoN Semantic Duality Principle

CiNnaMoN defines duration, beat, and measure-length attributes under a **dual semantic regime**:

---

#### 1. Authorial (Normative) Semantics

All temporal attributes (`time`, `beat`, `duration`) primarily serve a **normative compositional function**:

- to guide authors in producing musically coherent notation
- to constrain structural plausibility of rhythmic expression
- to establish local consistency of temporal intent

In this mode:
- values are treated as **intent descriptors**
- authors are NOT expected to perform arithmetic reasoning
- symbolic forms (including fractions) are expressive, not computational

---

#### 2. Operational (Renderer) Semantics

Temporal attributes are secondarily interpreted by renderers as **operational constraints**:

- to validate structural consistency of the score
- to resolve timing relationships for playback, layout, or simulation
- to detect under- or over-specified temporal structures

In this mode:
- values MAY be normalized or evaluated
- interpretation is implementation-defined
- results MUST NOT be re-encoded back into author-facing syntax

Renderers MAY reject temporal systems that cannot be operationally resolved within their target notation or playback model.

---

#### Non-Reification Constraint

Operational interpretation MUST NOT be treated as part of CiNnaMoN source semantics.

That is:

- computed timing is **derivative**, not intrinsic
- renderer resolution does not modify authorial meaning
- CiNnaMoN text remains invariant under evaluation

---

#### Structural Validation Hierarchy

Renderers MAY apply validation at two levels:

1. **Structural plausibility checks**
   - consistency of measure grouping
   - completeness of required fields
   - continuity constraints for tied or sustained events

2. **Temporal resolution**
   - mapping durations into internal time units
   - scheduling of events for output systems

---

#### Key Principle

CiNnaMoN source notation expresses **temporal intent**, not explicit computation.

Authors describe:
- rhythmic structure
- temporal continuity
- expressive grouping
- performative timing relationships

Renderers MAY:
- validate structural consistency
- resolve durations operationally
- map symbolic durations into playback or layout systems

However:

- operational resolution is derivative
- computed timing is not authorial syntax
- renderer interpretation MUST NOT alter source semantics

CiNnaMoN therefore functions simultaneously as:

- a symbolic notation system for human authors
- a temporal constraint system for evaluators and renderers

without collapsing either role into the other.

---

### CiNnaMoN Grammar

The content model is line-oriented and token-based.

#### Core Element

```xml
<music time="" beat="" key="" mode="" tempo="" mood="">
```

##### Attributes
* Together, `time` and `beat` define the enclosing measure duration for operational interpretation purposes:
  * `time`: the number of beats in a measure (numerator of time signature). Example values: 1, 2, 3, 4, 5, 6.5, 3.1415926535, pi, √2.
  * `beat`: the note-value unit representing the beat (inverse duration of the beat). Defines the denominator of the time signature. Example values: 2 = half note, 4 = quarter note, 5 = fifth note, 8 = eighth note, 16 = sixteenth note.
  * Renderers MAY approximate irrational or symbolic temporal values for time and beat.
* `key`: tonal center (A–G with optional accidentals). Examples: `C` for the key of C natural, `Gsharp` for G sharp, `Aflat` for A flat.
* `mode`: modal context (major, minor, lydian, phrygian, dorian, etc.)
* `tempo`: may be descriptive (largo, adagio, moderato, allegretto, etc.) or numeric (real number) in beats per minute (120)
* `mood`: descriptive or desired emotional outcome

---

#### Top-level structure

```
<music> ::= "<music" attributes ">" content "</music>"

attributes ::= (time | beat | key | mode | tempo | mood)*
```

#### Content model

```
content ::= bar ("|" bar)*

bar ::= sequence

sequence ::= element (whitespace element)*
```

#### Element types

```
element ::= note | rest | tie | caesura | speech
```

#### Duration

```
duration ::= ([1-9][0-9]*) "/" ([1-9][0-9]*)   // e.g. 1/4, 3/16, 99/167
```

Examples:
- `7/2`
- `2/3`
- `7/5`
- `3/11`

Durations MAY span multiple measures when interpreted by a renderer as exceeding the enclosing measure duration.

Examples:

```
D:1/4{Two} C:1/4{beats.}

X:2/1{Here are two measures of speech.}

C:7/2{Here are three and a half measures of freely sung vocals.}

R:9/5
```

* Renderers MAY approximate irrational or symbolic temporal values for duration.

---

#### Note syntax

```
note ::= pitch ":" duration "{" lyric "}" 
       | pitch ":" duration
       | pitch "{" lyric "}"
       | pitch

pitch ::= [A-G] accidental? octave?
accidental ::= "#" | "b"
octave ::= -? ( [0] | [1-9][0-9]* )
```

#### Vocalization Semantics

CiNnaMoN note events represent performative pitched articulations.

Lyrics are optional semantic attachments to note events rather than prerequisites for note existence.

An empty lyric body:

```text
{}
```

represents intentionally non-lexical vocalization or unspecified articulation.

Examples include:
- humming
- whistling
- vowel-based singing
- nonverbal melodic performance

Renderers MAY additionally interpret lyricless note events as instrumental or synthesized realizations.

#### Rest syntax

```
rest ::= "R" (":" duration)?
```

#### Tie / sustain syntax

```
tie ::= "~"
```

* Ties connect preceding and following notes or sustain state across elements.

#### Caesura / breath mark syntax

```
caesura ::= "//"
```

* `//` is a **caesura or breath mark**, not a comment.
* CNML does NOT define comments in CiNnaMoN.
* No comment syntax exists.


#### Speech syntax

Speech is a timed vocal articulation primitive. It does not encode pitch; pitch is resolved at evaluation time via renderer configuration, inference, or default policy.

```
speech ::= "X" (":" duration)? "{" lyric "}"
```

* NOTE: Common renderer profiles MAY include syllabic, rhythmic-spoken, or other amelodic vocal realizations.

---

##### Composer-interpreted speech rhythm (structured timing)

Speech MAY be subdivided into multiple articulated (`X`) events for rhythmic articulation.

Example:

```
X:3/8{Here} X:1/8{is} X:1/8{a} X:1/8{phrase.} |
```

Semantics:
- each `X` is a discrete articulation unit
- rhythm is explicitly composed
- timing is fully specified by durations

---

##### Performer-interpreted rhythm (free speech)

Speech MAY be encoded as a single event with internal timing left to interpretation.

Example:

```
X:1/1{Here is a measure of speech.}
```

Semantics:
- `X` defines total temporal window
- internal pacing is performer-determined
- no internal segmentation is specified

---

#### Inheritance Rules (Measure Scope)

Within a given measure (bar):

* The first explicit note MUST define:

  * pitch
  * octave
  * duration
  
* The first explicit rest or explicit speech MUST define:
  
  * duration

* Subsequent `note`s, `rests`, and `speech`es within the same measure MAY omit any of these fields.

  * Missing pitch, octave, or duration values inherit from the most recent explicitly defined values in that measure scope.

* Inheritance resets at each new measure boundary (`|`).

---