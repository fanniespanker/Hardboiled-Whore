# PÂTE (Positional Affine Text Encoding) (***FINALIZED***)

## 1. Overview

PÂTE is a deterministic **spatial constraint language** for CNML.

It compiles span declarations into a **directed affine constraint graph** over span anchor positions.

It is:
- declarative
- graph-based
- deterministic
- non-optimizing

PÂTE defines layout purely as deterministic affine relationships between span anchors.

---

## 2. Role in and Relationship to CNML (***FINALIZED***)

PÂTE is currently distributed and instantiated as part of the CNML toolchain, where it functions as a projection layer within the CNML rendering pipeline:

```
CNML → PÂTE projection → constraint graph → renderer
```

PÂTE is currently integrated with CNML for practical deployment and bootstrapping.

It is not defined in terms of CNML and is not semantically dependent on it. CNML is the implementation bootstrap substrate.

Future versions of PÂTE are intended to operate independently of CNML as a standalone representation and compilation target. CNML may continue to serve as one possible upstream serialization format.

---

## 3. Core Model

Formally, a PÂTE program compiles into a graph:

$ G = (V, E) $

Where:

\[
\begin{aligned}
V &= V_s \cup V_a \\
V_s &= \{ s \mid s \text{ is a span} \} \\
V_a &= \{ a \mid a \text{ is an anchor} \} \\
\text{Span} &:= V_s \\
\text{Anchor} &:= V_a \\
E &\subseteq \mathcal{S} \\
\mathcal{S} &= \left(Span \times Anchor\right)^2 \times \mathbb{Q} \\
\end{aligned}
\]

This can be interpreted as the tuple:

$ \left(A,a,B,b,\delta\right) \in E $

Whose components are defined as:

- $ A, B \in \text{spans} $
- $ a, b \in \{<, |, >\} $ denote anchor selectors (left, center, right respectively)
- $ \delta \in \mathbb{Q} $

---

## 4. Spans

A span is a contiguous textual unit with intrinsic geometry.

Each span defines exactly three anchors:

- leading edge
- center
- trailing edge

Anchors are immutable geometric projections.

---

## 5. Identifier System (PÂTE Scope Only) (***FINALIZED***)

All PÂTE identifiers are defined as non-empty sequences of Unicode scalar values whose Unicode General Category is in the permitted set defined below.

The Permitted Unicode General Categories for Identifiers are:
- Letter (L)
- Mark (M)
- Number (N)
- Punctuation, connector (Pc)
- Private Use (Co)
- Symbol, modifier (Sk)
- Symbol, other (So)
- Symbol, currency (Sc)
- Other, format (Cf)

### Identifier Grammar

```ebnf
ID := NON_NUMERIC_PERMITTED_CODEPOINT PERMITTED_CODEPOINT*
```
Where:
- `PERMITTED_CODEPOINT` is any Unicode scalar value whose Unicode General Category is in the permitted set defined above.
- `NON_NUMERIC_PERMITTED_CODEPOINT` is any `PERMITTED_CODEPOINT` that is not in the Number General Category.

---

### Non-Normative Notes on Identifier Validity and Equivalence

* Order is sensitive.
* Length is sensitive.
* No visual equivalence is implied.
* Visually indistinguishable glyphs MAY correspond to different identifiers.
* **Identifier equality is equality of sequences.**
* Authors should take care in selecting identifiers that are visually distinct from each other and which can be reproduced consistently.

Equality is strictly:
```
ID_A == ID_B  ⇔  identical Unicode scalar value sequence
```

---

### PÂTE Identifier Algebra Axiom Kernel (Self-Contained)

\[
\Sigma =
\{ c \in \text{Unicode scalar values} \mid GC(c) \in \{\text{L}, \text{M}, \text{N}, \text{Pc}, \text{Co}, \text{Sk}, \text{So}, \text{Sc}, \text{Cf}\} \}
\]

\[
\mathcal{I} =
\{ s \in \Sigma^+ \mid GC(s_1) \neq \text{N} \}
\]

\[
a = b \Longleftrightarrow \text{sequence}(a)=\text{sequence}(b)
\]

---

+++++++++++++++++++
---

## 6.2 Alignment Operators (Syntactic Selectors Only)

Alignment operators are **pure syntactic selectors over the anchor basis A**.

They are not anchors.
They are not nodes.
They are not graph entities.

They are compile-time projections that select elements of `A × A`.

---

### Operator Tokens

The operator system consists of two independent selectors:

```
SOURCE_ALIGN_OP ∈ A
TARGET_ALIGN_OP ∈ A
```

An alignment expression is therefore a pair:

```
(SOURCE_ALIGN_OP, TARGET_ALIGN_OP)
```

---

## 6.3 Cartesian Product Semantics

Alignment is defined over the Cartesian product:

```
A × A
```

Each alignment expression selects exactly one element of this set:

```
(source_anchor, target_anchor)
```

This pair is resolved into a constraint edge during compilation.

---

## 6.4 Edge Construction Rule

Given spans `A` and `B`:

```
A SOURCE_OP TARGET_OP B
```

compiles to:

```
A[SOURCE_OP] → B[TARGET_OP]
```

Where:

- `A[SOURCE_OP] ∈ {A.<, A.|, A.>}`
- `B[TARGET_OP] ∈ {B.<, B.|, B.>}`

This produces a single directed constraint edge.

---

## 6.5 Operator Pair Table (Illustrative Expansion)

This table is **not a semantic definition**, only a visualization of `A × A`:

| Source \ Target |   `<`   |   `|`   |   `>`   |
|-----------------|---------|---------|---------|
|             `<` | `< → <` | `< → |` | `< → >` |
|             `|` | `| → <` | `| → |` | `| → >` |
|             `>` | `> → <` | `> → |` | `> → >` |

Each cell corresponds to exactly one anchor pair.

---

## 6.6 No Object Semantics Rule

Alignment operators MUST NOT be interpreted as:

- anchors
- graph nodes
- runtime objects
- first-class semantic entities

They exist only as:

> syntactic selectors resolving into anchor-pair constraints

---

## 6.7 Determinism Rule

Each alignment expression MUST resolve to exactly one anchor pair.

No ambiguity, probabilistic interpretation, or contextual override is permitted.

---

## 6.8 Compilation Invariant

All alignment expressions compile to:

```
Edge = (source_span.anchor, target_span.anchor, δ)
```

Where:

- anchors are selected via `(SOURCE_OP, TARGET_OP)`
- `δ` is provided by optional offset system
- no intermediate representation retains operator identity

---

## 6.9 Conceptual Summary

Alignment in PÂTE is:

> a Cartesian-product selection over a fixed anchor basis, compiled into deterministic affine edges

Operators are purely syntactic and disappear after compilation.

They do not participate in the graph model.

---

## 7. Offset System

```ebnf
SPAN := "{{" SOURCE_SPAN? SOURCE_ALIGN_OPERATOR TARGET_ALIGN_OPERATOR TARGET_HORIZONTAL_SPAN H_OFFSET? (TARGET_VERTICAL_SPAN V_OFFSET?)? ("::" CONTENT)? "}}"

SOURCE_SPAN := SPAN_ID

SOURCE_ALIGN_OPERATOR := ALIGN_OPERATOR

TARGET_ALIGN_OPERATOR := ALIGN_OPERATOR

ALIGN_OPERATOR := { "<" | "|" | ">" }

TARGET_HORIZONTAL_SPAN := SPAN_ID

H_OFFSET := OFFSET+

TARGET_VERTICAL_SPAN := SPAN_ID

V_OFFSET := "," OFFSET+

OFFSET :=  SIGN OFFSET_BODY

SIGN := "+" | "-"

OFFSET_BODY := SCALE SPAN_ID

SCALE := RATIONAL_NUMBER?

RATIONAL_NUMBER :=
    POSITIVE_INTEGER "/" POSITIVE_INTEGER
  | POSITIVE_INTEGER

POSITIVE_INTEGER :=
  [1-9][0-9]*
```

### SCALE Semantics

SCALE denotes a rational scalar multiplier:

$$
k \in \mathbb{Q}^+
$$

If `SCALE = ε`, it is interpreted as:

$$
k = 1
$$

---

# PÂTE Affine Vector Formulation

## Span Geometry Model

A PÂTE span is a positioned geometric text object embedded in a 2D affine vector space.

Each span \(S\) possesses:

- a center position vector:

$$
\vec{p}(S) \in \mathbb{R}^2
$$

- intrinsic dimensions:

$$
w(S), h(S) \in \mathbb{R}^+
$$

---

## Coordinate Projection Operators

Horizontal and vertical positional components are projected independently.

### Horizontal Projection

$$
\pi_x(\vec{p}(S))
=
x(S)\hat{x}
$$

### Vertical Projection

$$
\pi_y(\vec{p}(S))
=
y(S)\hat{y}
$$

---

# Alignment Operators

PÂTE alignment operators are not anchors.

They are directional projection operators over span geometry.

---

## Horizontal Alignment Operators

### Leading

$$
\alpha_{<}(S)
=
-\frac{1}{2}w(S)\hat{x}
$$

### Center

$$
\alpha_{|}(S)
=
0
$$

### Trailing

$$
\alpha_{>}(S)
=
+\frac{1}{2}w(S)\hat{x}
$$

---

## Offset Vectors

Each offset term is a signed affine displacement vector.

### General Form

$$
\delta_i
=
\pm k_i \cdot d(S_i)
$$

Where:

- \(k_i \in \mathbb{Q}^+\)
- sign is carried explicitly by the grammar
- \(d(S_i)\) is a directional span extent vector

---

### Horizontal Offset

$$
\delta_i
=
\pm k_i \cdot w(S_i)\hat{x}
$$

### Vertical Offset

$$
\delta_i
=
\pm k_i \cdot h(S_i)\hat{y}
$$

---

# Canonical PÂTE Equation

Given:

```PÂTE
{{A<>B+1/10B-3/2C,D-7/8E+1/2F}}
```

The corresponding affine vector equation is:

$$
\vec{p}(A)
=
\pi_x(\vec{p}(B))
+
\pi_y(\vec{p}(D))
+
\alpha_{>}(B)
-
\alpha_{<}(A)
+
\sum_i \delta_i
$$

Where:

### Horizontal Offsets

$$
\delta_1
=
+\frac{1}{10}w(B)\hat{x}
$$

$$
\delta_2
=
-\frac{3}{2}w(C)\hat{x}
$$

### Vertical Offsets

$$
\delta_3
=
-\frac{7}{8}h(E)\hat{y}
$$

$$
\delta_4
=
+\frac{1}{2}h(F)\hat{y}
$$

---

# Interpretation

PÂTE defines a deterministic affine vector constraint system over typographic spans.

A span position is determined by:

- projected positional components
- alignment operator projections
- signed affine offset vectors

All geometry is compositional, deterministic, and graph-resolvable.
