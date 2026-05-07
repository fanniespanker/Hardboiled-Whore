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



--- CNML_SPEC_PREVIOUS
+++ CNML_SPEC_PATCH_PHENOMENOLOGICAL_RUNTIME

@@ STRUCTURAL MODEL

+ Introduce explicit chapter-local narrative containment.
+
+ Canonical chapter structure:
+
+ <chapter>
+     <header>
+         <title>...</title>
+         <subheading>...</subheading>
+     </header>
+
+     <narrative>
+         ...
+     </narrative>
+ </chapter>
+
+ Notes:
+ - <header> is metadata-only.
+ - <narrative> contains all experiential/prose content.
+ - Nested phenomenological operators occur only inside <narrative>.
+ - Multiple <narrative> blocks MAY be permitted in future revisions.
+
+ Rationale:
+ - Separates metadata from experiential payload.
+ - Preserves explicit baseline narrative scope.
+ - Simplifies parser traversal and stack restoration.
+ - Prevents global boilerplate <narrative> wrapping.


@@ MODALITY / NONACTUAL MODEL

- Remove binary actual/nonactual partition semantics.
-
- Deprecate:
-     <nonactual>
-
- Replace broad nonactual categorization with local phenomenological operators.
-
- Phenomenological state is now modeled through nested cognitive-event elements.
-
- Baseline narration is represented implicitly by enclosing <narrative>.
-
- Example:
-
- <narrative>
-     I opened the door.
-
-     <dream>
-         ...
-     </dream>
-
-     I lit a cigarette.
- </narrative>
-
- Interpretation:
- - <dream> temporarily overlays baseline narrative phenomenology.
- - Exiting the tag restores enclosing narrative state.


@@ PHENOMENOLOGICAL OPERATOR MODEL

+ Introduce verb-oriented cognitive/phenomenological operators.
+
+ Operators represent:
+ - cognitive acts,
+ - perceptual transformations,
+ - epistemic instability,
+ - or phenomenological overlays.
+
+ Operators are compositional and recursively nestable.
+
+ Canonical operators currently include:
+
+     <think>
+     <feel>
+     <intuit>
+     <say>
+     <dream>
+     <fantasize>
+     <hallucinate>
+     <flashback>
+     <intrude>
+     <deceive>
+     <distort>
+     <misremember>
+     <project>
+     <recontextualize>
+
+ Future operators MAY include:
+
+     <suppress>
+     <avoid>
+     <dissociate>
+     <fixate>
+     <obsess>
+
+ Notes:
+ - Operators model active cognitive/phenomenological processes.
+ - Operators are NOT diagnostic or psychiatric labels.
+ - Operators are semantically local unless explicitly specified otherwise.
+ - Inner operators modify local cognition.
+ - Outer operators establish phenomenological environment.


@@ NESTING SEMANTICS

+ Define phenomenological stack semantics.
+
+ Parsing rule:
+
+ - Outer operators define active phenomenological substrate.
+ - Inner operators modify or interrupt local cognitive state.
+ - Closing an operator restores the previous enclosing state.
+
+ Example:
+
+ <dream>
+     <intrude>...</intrude>
+     <flashback>
+         <intuit>...</intuit>
+     </flashback>
+ </dream>
+
+ Interpretation:
+ - Flashback occurs within dream-state cognition.
+ - Intuition occurs within recalled memory state.
+ - Intrusion locally interrupts dream cognition.
+
+ This model is intentionally runtime-like and stack-based.


@@ SEMANTIC DESIGN PRINCIPLES

+ CNML phenomenological operators SHOULD:
+
+ - represent distinct cognitive acts,
+ - remain psychologically intuitive,
+ - avoid excessive taxonomic granularity,
+ - preserve ambiguity where appropriate,
+ - and support recursive compositionality.
+
+ CNML SHOULD model:
+ - consciousness-shaped narration,
+ - cognitive instability,
+ - subjective phenomenology,
+ - and layered epistemic states.
+
+ CNML SHOULD NOT prematurely collapse:
+ - ambiguity,
+ - identity instability,
+ - projection semantics,
+ - or phenomenological uncertainty.
+
+ Preference is given to:
+ - process semantics,
+ over:
+ - static metadata classification.


@@ PARSING MODEL

+ CNML parsers SHOULD treat phenomenological operators as stack transitions.
+
+ Recommended runtime behavior:
+
+ - Enter tag:
+     push phenomenological state
+
+ - Exit tag:
+     restore previous state
+
+ This behavior applies recursively.
+
+ Operators MAY be nested arbitrarily unless otherwise restricted by future revisions.
+
+ CNML parsers SHOULD preserve:
+ - nesting order,
+ - interruption locality,
+ - and phenomenological containment hierarchy.


@@ BACKWARD COMPATIBILITY

+ Existing documents using:
+
+     <nonactual>
+
+ MAY be migrated incrementally.
+
+ Recommended migration path:
+
+ - Replace broad nonactual wrappers with specific local operators:
+
+     <dream>
+     <fantasize>
+     <hallucinate>
+     <flashback>
+     etc.
+
+ - Preserve baseline prose inside enclosing <narrative>.
+
+ Migration MAY occur incrementally and does NOT require single-pass full-document conversion.
+ 
--- CNML_SPEC_PREVIOUS
+++ CNML_SPEC_PATCH_DOCUMENT_COMPOSITION_AND_RUNTIME

@@ DOCUMENT TOPOLOGY

+ Introduce hierarchical publication/document structure.
+
+ Canonical top-level structure:
+
+ <cnml>
+     <book>
+         ...
+     </book>
+ </cnml>
+
+ Higher-order aggregation structures MAY include:
+
+     <series>
+     <universe>
+
+ Publication partitioning elements:
+
+     <frontmatter>
+     <mainmatter>
+     <backmatter>
+
+ Organizational narrative structures:
+
+     <act>
+     <chapter>
+     <scene>
+     <section>
+
+ Notes:
+ - <section> is a generic subdivision element.
+ - <section> is especially intended for frontmatter/backmatter organization.
+ - Structural hierarchy MAY vary by work.
+ - Standalone documents are permitted and need not participate in higher-order containers.


@@ EXPLICIT NARRATIVE RUNTIME

- narrative previously treated as implicit baseline layer
+ narrative is now explicitly represented.

+ Canonical narrative structure:
+
+ <chapter>
+     <header>
+         <title>...</title>
+         <subheading>...</subheading>
+     </header>
+
+     <scene>
+         <narrative>
+             ...
+         </narrative>
+     </scene>
+ </chapter>
+
+ Semantics:
+ - <narrative> establishes phenomenological runtime context.
+ - Cognitive and phenomenological operators are interpreted relative to enclosing narrative scope.
+ - Non-narrative structural elements MAY appear between narrative blocks.
+
+ Example:
+
+ <narrative>
+     I heard a sound.
+ </narrative>
+
+ <note>
+     ...
+ </note>
+
+ <narrative>
+     <say>Boo.</say>
+ </narrative>
+
+ Rationale:
+ - Separates experiential payload from metadata and metatext.
+ - Enables interruptible narrative continuity.
+ - Simplifies runtime traversal and phenomenological restoration.


@@ PHENOMENOLOGICAL / COGNITIVE OPERATOR MODEL

- Remove binary actual/nonactual partition semantics.
-
- Deprecate:
-     <nonactual>
-
- Replace broad nonactual categorization with local phenomenological operators.
+ Replace broad binary modality partitioning with local phenomenological operators.

+ Retain:
+
+     <nonactual>
+
+ as a generalized fallback phenomenological operator.

+ <nonactual> semantics:
+ - intentionally underspecified,
+ - semantically weak relative to specialized operators,
+ - and intended for ambiguous or composite altered experiential states.
+
+ Specific operators SHOULD be preferred where applicable.

+ Canonical phenomenological substrate operators:
+
+     <dream>
+     <flashback>
+     <hallucinate>
+     <fantasize>
+     <nonactual>
+
+ Canonical cognitive process operators:
+
+     <think>
+     <feel>
+     <intuit>
+     <misremember>
+     <project>
+
+ Canonical expressive operators:
+
+     <say>
+     <gesture>
+     <express>


@@ MODIFIER OPERATOR MODEL

+ Introduce modifier-oriented interpretive operators.
+
+ Modifier operators alter:
+ - cognition flow,
+ - interpretive framing,
+ - epistemic relation,
+ - or expressive delivery.
+
+ Canonical modifier operators currently include:
+
+     <intrusive>
+     <distorted>
+     <deceptive>
+     <recontextualized>
+
+ Semantics:
+ - Modifier operators inherit semantic domain from enclosing operators.
+ - Modifier operators are compositional and recursively nestable.
+
+ Example:
+
+ <think>
+     <intrusive>
+         RUN THEM OVER!
+     </intrusive>
+ </think>
+
+ Interpretation:
+ - intrusive cognition,
+ - rather than standalone intrusive state.


@@ SEMANTIC COMPONENT GUIDANCE

+ Operators MAY participate in one or more semantic components.
+
+ Semantic components are descriptive interpretive dimensions intended to:
+ - aid authorship,
+ - guide implementation,
+ - preserve conceptual coherence,
+ - and support ontology extensibility.
+
+ Components are non-exclusive and non-restrictive.
+
+ Canonical semantic components currently include:
+
+     experiential
+     cognitive
+     expressive
+     epistemic
+
+ Operators MAY participate in multiple semantic components simultaneously.
+
+ Interpretation WILL be author-specific in some contexts.
+
+ Implementations MUST preserve operator structure even where interpretation is not standardized.


@@ NESTING / RUNTIME SEMANTICS

+ Define phenomenological stack semantics.
+
+ Runtime traversal model:
+
+ - Enter operator:
+     push phenomenological/cognitive state
+
+ - Exit operator:
+     restore enclosing state
+
+ Semantics:
+ - Outer operators establish active phenomenological substrate.
+ - Inner operators locally modify cognition or interpretation.
+ - Closing operators restores enclosing runtime state.
+
+ Operators MAY nest recursively unless otherwise restricted.


@@ STRUCTURAL DOCUMENT COMPOSITION

+ Structural elements MAY either:
+ - contain inline content,
+ - OR reference external content via src,
+ - but MAY NOT do both simultaneously.
+
+ Example:
+
+ <chapter src="chapter03.cnml"/>
+
+ Semantics:
+ - Structural tags preserve logical narrative role.
+ - src specifies external physical document location.
+
+ This enables:
+ - modular document composition,
+ - incremental parsing,
+ - hierarchical publication assembly,
+ - and partial runtime traversal.
+
+ Example:
+
+ <series>
+     <book src="book1.cnml"/>
+     <book src="book2.cnml"/>
+ </series>
+
+ Notes:
+ - Structural inclusion is recursive.
+ - Documents form compositional narrative trees.
+ - Implementations MAY lazy-load referenced structures.


@@ SEMANTIC ADVISORIES

- hard nesting prohibitions
+ semantic advisories

+ Certain nested structures MAY produce semantics that are:
+ - author-specific,
+ - context-dependent,
+ - or interpretation-variable.
+
+ Implementations SHOULD preserve:
+ - nesting order,
+ - interruption locality,
+ - containment hierarchy,
+ - and operator relationships,
+ even where semantic interpretation differs between ontologies or authors.