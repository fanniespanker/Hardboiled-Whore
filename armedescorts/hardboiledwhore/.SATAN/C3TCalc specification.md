# C3TCalc Specification v0.2 (Draft)

## 1. Foundations

C3TCalc (Contextual Compositional Concept Transformation Calculus) is a context-sensitive semantic transformation calculus for constructing, composing, and transforming conceptual structures represented as typed, reifiable graph systems.

C3TCalc is not a closed ontology, logical system, or reduction-to-normal-form language.

It defines:

* a compositional graph construction system,
* a contextual transformation calculus over graphs,
* a relation-reification model,
* and a framework for interoperability between heterogeneous semantic systems.

C3TCalc assumes that conceptual meaning is:

* context-dependent,
* partially underspecified,
* compositional,
* and not reducible to a single canonical interpretation.

---

## 2. Ontological Model

### 2.1 Nodes as Universal Primitives

All entities are nodes.

A node may represent:

* a concept,
* a relation,
* a context,
* a composite structure,
* or a graph fragment.

Thus:

> Node := atomic | graph | composite structure

Nodes are not assumed to have global identity equivalence; they are interpreted via contextual substitutability.

---

### 2.2 Node Types

Nodes may optionally carry type annotations:

* Concept
* Relation
* Context
* Alignment
* Composite

Typing is interpretive, not globally enforced.

---

## 3. Relations as First-Class Nodes

Relations are nodes of type Relation.

A relation may be:

* ontology-defined,
* user-defined,
* or constructed via composition.

Relations are not primitive edges; they are objects used in edge instantiation.

Relations may be aligned across ontologies using `@`.

---

## 4. Edge Model

Edges are instantiated structures, not primitive entities.

An edge is defined as:

> Edge(Source, RelationNode, Target)

Edges are created through the `?` operator.

---

## 5. Core Operators

### 5.1 Edge Instantiation Operator `?`

The `?` operator introduces edge construction from a source node.

#### Single-edge form

```
A ? R = B
```

Meaning:

> Instantiate an edge from A to B labeled by relation R.

#### Multi-edge form

```
A ? R1 = B; R2 = C
```

Meaning:

> Instantiate multiple edges from A in a single construction frame.

Parentheses are optional and used only for grouping:

```
A ? (R1 = B; R2 = C)
```

---

### 5.2 Composition Operator `*`

```
A * B
```

Represents conceptual fusion producing a composite node.

---

### 5.3 Aggregation Operator `+`

```
A + B
```

Represents non-fused grouping or collection semantics.

---

### 5.4 Alignment Operator `@`

```
A @ B
```

Represents contextual correspondence or alignment between nodes.

Alignment does not imply identity.

---

### 5.5 Equivalence Operator `≡`

```
A ≡ B
```

Represents contextual substitutability or equivalence within an evaluation frame.

Equivalence is not global identity.

---

## 6. Syntax Rules

### 6.1 Parentheses

Parentheses are purely structural grouping devices and do not alter operator semantics.

### 6.2 Semicolons

Semicolons separate independent edge declarations within a single `?` frame.

---

## 7. Execution Model

### 7.1 No Convergence Requirement

C3TCalc does not define:

* termination conditions,
* normal forms,
* or convergence requirements.

---

### 7.2 Trace-Based Semantics

Execution is defined as a transformation trace:

```
G0 → G1 → G2 → ... → Gn (or infinite)
```

Each transition Gi → Gi+1 must obey operator rules.

---

### 7.3 Evaluator-Controlled Execution

Stopping conditions are external to C3TCalc and may include:

* resource limits,
* time constraints,
* user intervention,
* or heuristic policies.

C3TCalc itself does not define termination.

---

## 8. Validity Conditions

An execution is valid if:

1. The initial graph is well-formed.
2. Each transformation step obeys operator semantics.
3. All edges are constructed via `?` rules.
4. No step violates node or relation typing constraints (when present).

No requirement exists for reaching a terminal or stable state.

---

## 9. Identity and Equivalence

C3TCalc does not assume global identity.

Instead it supports:

* contextual equivalence (`≡`),
* alignment (`@`),
* and substitutability relations.

Nodes may be equivalent in one context and distinct in another.

---

## 10. Graph Structure Principle

Graphs may contain:

* atomic nodes,
* composite nodes,
* and graph fragments as nodes.

Graph nesting is allowed and does not require flattening.

Nested graphs are treated as first-class nodes unless explicitly expanded by interpretation rules.

---

## 11. Non-Goals

C3TCalc does NOT aim to:

* enforce global consistency,
* define canonical outputs,
* implement logical completeness,
* or guarantee convergence.

---

## 12. Relationship to Interpretation Systems

C3TCalc defines structural and transformational semantics only.

Interpretation, meaning resolution, and convergence behavior are external to the calculus and may be defined by systems such as GEL or other evaluators.

---

## 13. Summary

C3TCalc is a trace-based, context-sensitive graph transformation calculus with reified relations and non-convergent semantics.

## 14. Relationship to GEL (Contextual Projection Layer)

C3TCalc serves as the structural and transformational substrate for higher-order interpretive systems such as GEL (Graph/Generative Expression Layer).

### 14.1 Separation of Roles

C3TCalc does not define meaning convergence or interpretation.
Instead, it produces transformation traces over graph structures.

GEL operates over these traces as a projection system that generates multiple simultaneous interpretations.

* **C3TCalc:** generates graph transformation traces (what is structurally valid)
* **GEL:** generates interpretation spaces over traces (what can be read from structure)
* **Interpretation systems (e.g., Neurative):** may select or stabilize interpretations from GEL outputs

### 14.2 Non-Collapse Principle

C3TCalc explicitly does not collapse meaning into a canonical form.

There is no requirement that a graph state, or sequence of states, converge.

GEL may define multiple coexisting interpretations over the same trace without requiring reduction.

### 14.3 Projection Model

A GEL projection can be conceptualized as:

```
GEL_View = Project(C3TCalc_Trace, ContextFrame)
```

Where:

* C3TCalc_Trace is a sequence of graph transformations
* ContextFrame defines interpretive constraints, ontology overlays, or narrative framing
* GEL_View is an interpretation graph (not a reduced form of the original)

### 14.4 Example Interaction: Structural Generation → Interpretation

#### C3TCalc input

```
A ? knows = B;
A ? is_a_friend_of = C
```

#### Resulting trace segment

```
G0 → G1 → G2
```

(where edges are instantiated across states)

#### GEL projections

**Context Frame: Social Ontology**

```
A is connected to B via KNOWS
A is connected to C via FRIENDSHIP
```

**Context Frame: Epistemic Interpretation**

```
A has informational relation to B
A has social bond relation to C
```

**Context Frame: Structural View**

```
Node A has outgoing relations:
  - knows → B
  - is_a_friend_of → C
```

All interpretations are valid simultaneously.
No single projection is privileged by C3TCalc.

### 14.5 Example Interaction: Partial Structure and Underspecification

#### C3TCalc input

```
A ? knows = _;
_ ? is_a_friend_of = C
```

#### Interpretation behavior (via GEL)

GEL may produce different readings depending on context:

**Inference-oriented frame:**

```
There exists some X such that:
A knows X and X is a friend of C
```

**Structural frame:**

```
A connects to an unresolved node which connects to C
```

**Narrative frame:**

```
A's knowledge relationship is incomplete but linked to C through an intermediary
```

C3TCalc itself does not resolve or prefer any of these interpretations.

### 14.6 Neurative (Optional Stabilization Layer)

Neurative, if used, operates over GEL projections rather than C3TCalc structures.
It may:

* select stable interpretations
* compress interpretation sets
* prioritize context frames
* bind interpretations into operational outputs

Neurative does not modify C3TCalc graphs.

### 14.7 Architectural Summary

This establishes a three-layer separation:

1. **C3TCalc (Structural Layer)**

   * defines valid graph transformations
   * produces traces
   * does not converge

2. **GEL (Interpretation Layer)**

   * defines contextual projections over traces
   * allows multiple simultaneous meanings

3. **Neurative (Stabilization Layer, optional)**

   * selects or compresses interpretations
   * produces operational commitments

---

## 15. Final Note on System Boundary

C3TCalc remains agnostic to interpretation, truth, and convergence.

All semantic stabilization, selection, or “meaning formation” occurs outside the calculus in layered systems such as GEL and Neurative.
