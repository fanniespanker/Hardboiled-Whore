# CNML Core Specification v0.1.4

## Foundations

CNML is a constrained XML vocabulary with semantic rules.

## Discourse

- Default mode: implicit `<narrative>`
- `<irreal>`: non-actualized discourse; non-nesting; may span units

## Discourse Units

- Derived from blank-line separation
- Computed per container

## Expression Modality

- `<say>`: spoken
- `<feel>`: internal cognition
- `<gesture>`: nonverbal

## Prosody

Prosody is compositional across inline elements and may stack.

- `<soft>`: attenuation
- `<strong>`: intensity
- `<emph>`: emphasis

## Integrity

- `<redacted>`, `<censored>`, `<glitched>`

## Music DSL

Music DSL is a structured, human-readable temporal annotation layer for expressive or performative text segments.

### Core Element

```xml
<music time="" beat="" key="" mode="" tempo="" feel="">
```

### Attributes

- `time`: bar or measure index (qualitative or numeric)
- `beat`: beat grouping or meter unit
- `key`: tonal center (optional, may be approximate)
- `mode`: modal context (e.g., major, minor, lydian)
- `tempo`: expressive tempo descriptor (may be non-metronomic, e.g. "andante rubato")
- `feel`: qualitative affective descriptor (non-quantized, subjective)

### Semantics

- Music DSL is *non-binding*: it does not enforce timing or execution
- It annotates interpretive performance of textual voice
- Attributes may be omitted or underspecified
- Values are allowed to be qualitative rather than numeric

### Composition Rules

- Inline with narrative flow
- May overlap with `<say>` and `<gesture>` contexts
- Does not override discourse or prosody semantics
- Multiple `<music>` tags may exist per discourse unit

## Rules

- Inline elements compose
- No cross-boundary inheritance
- Prosody is compositional across nested inline spans
- Music DSL is interpretive and does not alter discourse structure

