# CNML Music DSL --- v0.1.3

## `<music>` Element

\<music time-signature="a/b" key="..." \[tempo="..."\]\> ...content...
`</music>`{=html}

------------------------------------------------------------------------

## Measures

Delimited by: - `|` - newline (`\n`)

Rules: - delimiters are equivalent\
- consecutive delimiters collapse\
- leading/trailing delimiters ignored\
- empty measures invalid

------------------------------------------------------------------------

## Measure Duration Constraint

Let: M = a/b\
Σ = sum of durations in measure

Single measure: 0 \< Σ ≤ M

Multiple measures: - first: 0 \< Σ ≤ M\
- last: 0 \< Σ ≤ M\
- interior: Σ = M

------------------------------------------------------------------------

## Event Model

Linear sequence of: - notes\
- rests\
- delimiters

------------------------------------------------------------------------

## Note Syntax

note ::= \[pitch\] \[":" duration\]? \[lyric\]\
pitch ::= letter \[accidental\] \[octave\]

------------------------------------------------------------------------

## Pitch System

-   letters: A--G\
-   accidentals: #, b, N\
-   octave: integer

------------------------------------------------------------------------

## Pitch & Octave Inheritance

Scoped to measure.

Valid: - full pitch: C4\
- letter only: C (requires prior octave)\
- omitted pitch: :1/8 (requires prior note)

Invalid: - omission at measure start\
- inheritance across measures

------------------------------------------------------------------------

## Duration

`<int>`{=html}/`<int>`{=html}\[.\]

-   dotted allowed\
-   normalized by parser

------------------------------------------------------------------------

## Duration Inheritance

-   may be omitted\
-   inherits from prior note with explicit duration\
-   resets at measure boundary

Invalid: - omitted at measure start\
- omitted with no prior explicit duration

------------------------------------------------------------------------

## Rest

R:`<duration>`{=html}

-   contributes to measure\
-   no lyric\
-   terminates lyric continuation\
-   does NOT establish duration for inheritance

------------------------------------------------------------------------

## Lyric Binding

{...}

-   attaches to note\
-   omission continues prior lyric\
-   rest terminates

------------------------------------------------------------------------

## Tie

\~

-   connects identical pitches\
-   second note has no `{}`\
-   pitch may be omitted if resolvable

------------------------------------------------------------------------

## Slur (Optional)

( ... )

-   no semantic effect\
-   grouping only

------------------------------------------------------------------------

## Accidental Resolution

explicit \> key \> default

------------------------------------------------------------------------

## Normalization

Parser MUST: - resolve pitch\
- resolve duration\
- validate measure totals

------------------------------------------------------------------------

## Measure Initialization Requirement

The first note of a measure MUST explicitly specify both pitch
(including octave) and duration.

Effects: - no inheritance allowed at measure start

------------------------------------------------------------------------

## Guarantees

-   deterministic parsing\
-   bounded state\
-   strong error visibility\
-   minimal authoring overhead
