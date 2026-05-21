# PÂTE — Unified Specification (v0.5 aligned synthesis)

## Positional Affine Text Encoding for CNML + Poem DSL

---

# 1. Core Unification Principle

PÂTE is a **surface span language** that compiles into a:

> directed affine graph over a 3-anchor basis per span

All syntax variants reduce to one primitive:

```
⟨Span A, Anchor i⟩ → ⟨Span B, Anchor j⟩ + δ
```

Where:
- i, j ∈ {L (leading), C (center), R (trailing)}
- δ ∈ ℚ or symbolic width-expression
- evaluation is SCC + fixed-point based (Poem DSL)

---

# 2. Anchor System (canonical)

| Symbol | Meaning |
|--------|--------|
| `<` | Leading anchor (L) |
| `|` | Center anchor (C) |
| `>` | Trailing anchor (R) |

Anchors are:
- discrete geometric reference points
- immutable per span
- graph nodes in constraint resolution

---

# 3. Span Syntax (canonical surface form)

```
{{ SOURCE_SPAN? ALIGN_OP TARGET_SPEC OFFSET :: CONTENT }}
```

Whitespace is semantically inert.

---

## 3.1 Span identity

```
SOURCE_SPAN ::= IDENTIFIER
```

If omitted → implicit current span.

---

## 3.2 Alignment operator (sugar)

```
ALIGN_OP ::= "<|" | "|>" | "|<" | ">|" | "||" | "<<" | ">>" | "<>" | "><"
```

But all operators compile to:

```
SOURCE.anchor_i → TARGET.anchor_j
```

---


# 4. Target Specification (resolved form)

```
TARGET_SPEC ::= TARGET_SPAN TARGET_ANCHOR
TARGET_ANCHOR ::= "<" | "|" | ">"
```

Meaning:

| Form | Meaning |
|------|--------|
| `B<` | B.L |
| `B|` | B.C |
| `B>` | B.R |

This removes ambiguity present in earlier drafts.

---

# 5. Offset System

```
OFFSET ::= RATIONAL | SYMBOL | WIDTH_EXPR
```

Examples:
- `1/2`
- `-3`
- `τ`
- `C.width`
- `1/2 B.width`

---

## 5.1 Semantics

Offset applies AFTER anchor binding:

```
pos(A.anchor_i) =
pos(B.anchor_j) + δ
```

---

# 6. Fully reduced semantic model (core)

Every PÂTE expression compiles to:

```
Edge:
  source = (A, i)
  target = (B, j)
  weight = δ
```

Where:
- i, j ∈ {L, C, R}

This yields a 3×3 anchor relation space.

---

# 7. Alignment as anchor-pair compression

All operators are syntactic compression of:

```
A[i] → B[j]
```

Examples:

| Expression | Expansion |
|-----------|----------|
| A<|B | A.R → B.C |
| A>|B | A.L → B.C |
| A|B  | A.C → B.C |
| A<B  | A.R → B.L |
| A>B  | A.L → B.R |

---

# 8. Chaining model

Multiple spans define a graph:

```
A |> B| :: x
B |> C| :: y
```

Compiles to:

```
A → B → C (via anchor edges)
```

Propagation occurs through SCC evaluation.

---

# 9. Propagation chains (explicit form)

Optional rooted form:

```
A :> B <| C |> D
```

Meaning:
- A is propagation root
- constraints form a directed path
- chain is syntactic grouping only (no new semantics)

---

# 10. Full evaluation pipeline

1. Lexical scan → span construction
2. Anchor extraction (L/C/R)
3. Operator desugaring → anchor edges
4. Offset binding → weighted edges
5. Graph construction
6. SCC decomposition (Poem DSL)
7. Fixed-point iteration
8. Convergence check (topological stability)

---

# 11. Fixed-point semantics

System resolves:

- cyclic dependencies
- affine feedback loops
- overlapping constraints

Termination condition:

- no change in topology class
- residual δ < ε (BigRational stable threshold)

---

# 12. Design invariants

- No absolute positioning primitives
- No attribute system in surface syntax
- All geometry is relational
- Anchors are the only primitives
- Operators are syntactic projections only
- All constructs reduce to anchor-edge graph

---

# 13. Canonical IR (compiled form)

After desugaring:

```
Node = (Span, Anchor)
Edge = (Node → Node, δ)
Graph = Directed Weighted Multigraph
```

This is the true execution model of PÂTE.

---

# 14. Relationship to CNML + Poem DSL

- CNML = structural framing + scope
- PÂTE = geometric constraint generation
- Poem DSL = SCC + fixed-point resolver
- GEL = external semantic annotation layer

---

# 15. Final unified interpretation

PÂTE is:

> a span-based surface language that compiles into a directed affine multigraph over a 3-anchor basis, evaluated via SCC-fixed-point convergence.

Meaning emerges from stable geometric structure, not syntax.

---