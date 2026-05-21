## PÂTE (Poetical Algebraic Typography Engine DSL)

### Overview

PÂTE is a spatial projection layer for CNML.

It defines typographic and geometric relationships between textual spans using deterministic constraint-based layout.

PÂTE is independent from CNML core structural frames.

---

### Core Model

PÂTE operates on:

- spans
- anchors
- geometric relations
- affine offset expressions

Layout resolves through iterative fixed-point evaluation until convergence.

---

### Span Syntax

```text
{{#span_id :: content}}
```

Example:

```text
{{#line1 :: Here is some text}}
```

---
#### Anchors

Anchors may be defined inside span text.

```text
{{#anchorName}}
```

Example:

```text
{{#line1 :: Here is an {{#pivot}}anchor.}}
```

Anchors:

- are geometric reference points
- do not render independently
- are immutable after layout construction

#### Spans

A span is the fundamental layout unit.

Spans contain:

- textual content
- geometric extent
- implicit anchors

All spans possess:

| Anchor | Meaning       |
|--------|---------------|
| left   | leading edge  |
| center | midpoint      |
| right  | trailing edge |

---

#### References

Anchors are referenced using:

```text
@span.anchor
```

Examples:

```text
@line1.center
@line1.pivot
```

#### Width References

Span widths are first-class geometric values used in offset arithmetic.
Access of span width is via `.width`:

```text
2/3 * span1.width
```

#### Measure Spans

Measure spans participate in layout/geometry calculation but are omitted from rendering.

Syntax:

```text
{{#id:measure::content}}
```

Example:

```text
{{#tripleM:measure::MMM}}
```

The span width evaluates from rendered content geometry exactly as with normal spans.

Measure spans may therefore act as reusable geometric constants.

---

### Alignment Syntax

General form:

```text
{{#id; alignment-expression :: content}}
```

Example:

```text
{{#span2; @span1.center |> tripleM :: text with its leading edge displaced from the centerline of span1 by the geometric width of tripleM}}
```

---

#### Alignment Operators

| Operator | Meaning                                          |
|----------|--------------------------------------------------|
| `<`      | trailing edge aligned to anchor line             |
| `>`      | leading edge aligned to anchor line              |
| `|`      | center aligned to anchor line                    |
| `<| d`   | center aligned, offset left by displacement `d`  |
| `|> d`   | center aligned, offset right by displacement `d` |

---

#### Alignment Semantics

##### Edge Pinning

###### `<`

The span hangs to the left of the anchor line.

Its trailing edge touches the anchor line.

###### `>`

The span hangs to the right of the anchor line.

Its leading edge touches the anchor line.

---

##### Center Alignment

###### `|`

The span center lies on the anchor line.

---

##### Offset Center Alignment

###### `<| d`

The span center is displaced left from the anchor line by `d`.

###### `|> d`

The span center is displaced right from the anchor line by `d`.

---

##### Arithmetic Expressions

Offsets support affine arithmetic over rational values and span widths.

Allowed operations:

- addition
- subtraction
- multiplication by rational constants

Examples:

```text
2 * measureSpan
1/2 * title - 3/4 * subtitle
```

---

### Constraint Model

PÂTE constraints are affine-linear.

Layout variables are resolved using deterministic iterative fixed-point evaluation.

Constraint graphs may contain cyclic dependencies.

---

#### Geometry

PÂTE geometry is renderer-derived.

Widths and anchor positions are determined by the underlying text layout engine.

Examples include:

- TeX engines
- PDF layout engines
- browser typography engines
- PostScript renderers

PÂTE itself does not define glyph metrics.

---

#### SCC Evaluation

Constraint graphs are decomposed into strongly connected components (SCCs).

- acyclic SCCs resolve topologically
- cyclic affine SCCs resolve iteratively until convergence

Evaluation order within an iteration is order-free and snapshot-isolated.

---

#### Topological Equivalence

Layout evaluation induces a constraint solution graph over geometric entities, including spans, anchors, and derived positional relationships produced by PÂTE evaluation.

Each resolved layout state may be represented as a directed, edge-labeled graph G where:

- nodes correspond to layout entities (spans, anchors, measure spans, or equivalent geometric primitives)
- edges correspond to spatial constraints or evaluated relational bindings (alignment, offset, containment, or anchoring relationships)
- edge labels encode constraint type and resolved geometric relationship

Two layout states are considered topologically equivalent if their corresponding graphs are isomorphic under relabeling of non-structural identifiers, preserving:

- adjacency structure
- constraint edge types
- ordering constraints where semantically significant (e.g., sequential or temporal ordering in narrative or musical domains)

Geometric values (absolute positions, pixel coordinates, and renderer-specific metrics) are not part of the topological representation and MUST be ignored in equivalence comparison.

---

#### Meaningful Difference

A change in layout is considered meaningful if and only if it induces a non-isomorphic transformation in the layout constraint graph, or if it produces a change in spatial relationships that alters constraint satisfaction structure (including but not limited to alignment shifts, containment changes, or broken anchoring relationships).

Renderers MAY additionally treat a change as meaningful if it produces a visually distinguishable difference in the output medium; however, such perceptual differentiation is considered a secondary interpretation layer and does not override graph-based equivalence.

---

#### Convergence

Layout evaluation operates by iterative fixed-point resolution of the PÂTE constraint system, producing successive graph states G₀, G₁, G₂, …

Let E be a scalar residual energy functional defined over geometric displacement of constraint-satisfying entities.

Epsilon is a dimensionless convergence threshold over this residual functional, normalized relative to the EM-square scale of the active typeface and renderer context.

Evaluation terminates when successive graph states are topologically equivalent under the above definition, and the residual energy satisfies:

E < ε

Epsilon is a convergence threshold only and is not a coordinate unit system.

Renderers MAY reject cyclic constraint systems exhibiting divergence, persistent oscillation, or failure to reach a stable topological equivalence class within implementation-defined iteration bounds.

### Rendering

PÂTE MAY be rendered into:

- SVG
- HTML/CSS
- PDF
- canvas systems
- other typographic projection systems

Rendered appearance depends on the target layout engine.

---
