CNML Specification (Core and Extended Profiles)

1. Overview

CNML (Cognitive Narrative Markup Language) is a semantic markup language for representing narrative structure, modality, and expressive delivery (speech, thought, gesture, and music) in a renderer-agnostic way.

The language is divided into two profiles:

Core Profile: Required for interoperability. Defines meaning.

Extended Profile: Optional. Enhances expressive rendering.



---

2. Design Principles

1. Semantic over presentational – markup encodes meaning, not formatting.


2. Renderer-agnostic – interpretation is deferred.


3. Compositional prosody – expressive qualities combine orthogonally.


4. Modality clarity – explicit distinction between reality and nonactual experience.


5. Minimal core – smallest useful interoperable set.




---

3. Core Profile

3.1 Structural and Modal Elements

<narrative> (implicit root)
<nonactual type="memory|dream|imagination|hypothetical">
<say>
<think>
<gesture>
<expression>

Semantics

narrative: default diegetic layer.

nonactual: explicitly non-real or alternate experiential layer.

say: spoken utterance.

think: internal cognition.

gesture: non-facial physical communication.

expression: facial or affective nonverbal communication.



---

3.2 Nesting Constraints

<say> MUST NOT nest within <say>.

<think> MUST NOT nest within <say>.

<nonactual> MAY nest within <think>.

<think> MAY nest within <nonactual>.

<say> MAY nest within <nonactual>.

Authors MAY relax constraints in loose interpretation mode.



---

3.3 Core Prosody Axes

Amplitude

<quiet>
<loud>

Arousal

<calm>
<agitated>

Tone / Attack

<gentle>
<harsh>

Salience

<emph>
<deemph>

Pitch Contour

<raise>
<lower>
<jump/>

Timing Bias

<drag>
<rush>

Semantics

Tags are compositional and orthogonal.

Tags modify delivery, not structure.

Tags may nest arbitrarily.



---

3.4 Referential Abstraction

<work medium="...">
<quote>

Semantics

<work>: identifies creative works; formatting deferred.

<quote>: quoted content; rendering style deferred.



---

3.5 Voice (Minimal Core Support)

<voice id="...">

Presence of voice definitions MUST be recognized.

Interpretation is OPTIONAL in Core.



---

4. Extended Profile (Expressive)

4.1 Voice System (Full)

<voice id="...">
  <dialect>
  <accent>
  <range>
  <feature>
  <!-- optional embedded prosody tendencies -->
</voice>

Semantics

Descriptive, not parametric.

Acts as baseline tendencies.

Does not override structural meaning.



---

4.2 Voice Tendencies

Voice elements MAY include prosody tags:

<voice id="Diane">
  <quiet/>
  <drag/>
</voice>

Semantics

Represent default tendencies.

Local markup overrides.



---

4.3 Music DSL

<music>

Features

Pitch notation (e.g., C4)

Duration (e.g., :1/4)

Chords: [C4 E G]

Rests: R:1/4

Inheritance of pitch/duration

Inline CNML inside {}


Semantics

Defines structural musical layer.

Prosody applies within note content.



---

4.4 Chat / Messaging Layer

<chat>
  <in>
  <out>
  <from>
  <msg>
</chat>

Semantics

Represents structured conversational exchange.



---

4.5 Optional Expressive Structure

<phrase>
<breath/>

Semantics

<phrase>: grouping and timing boundary hint.

<breath>: explicit physiological pause.



---

4.6 Advanced Interpretation

Includes:

prosody stacking

voice-prosody interaction

timing nuance beyond structure


Renderer-dependent.


---

5. Conformance

5.1 Core Compliance

An implementation MUST:

Parse all Core elements

Preserve modality and structure

Preserve prosodic intent (even if not rendered acoustically)


5.2 Extended Compliance

An implementation MAY:

Interpret voice characteristics

Render music DSL

Apply advanced prosody



---

6. Interpretation Model

Processing Order

1. Parse structure


2. Resolve modality


3. Apply voice baseline (if supported)


4. Apply prosody modifiers


5. Render output




---

7. Non-Goals

CNML does NOT:

prescribe exact acoustic output

require numeric parameterization

enforce a single rendering model



---

8. Future Extensions (Non-Normative)

phonetic alignment

overlapping dialogue

context-sensitive voice states

orchestration/multi-voice timing



---

9. Summary

CNML separates:

Structure (what happens)

Modality (how it is experienced)

Prosody (how it is delivered)


Core ensures meaning. Extended enables expression.