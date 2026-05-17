# Resource Synthesis and Variation Protocol (RSVP)

## Working Spec v0.1 (Consolidated Draft)

RSVP is a URI/IRI-compatible semantic expression language for constructing and relating atomic, composite, and synthetic resources across one or more ontologies (e.g. MODO).

It extends identifier-based resource systems with:

* compositional algebra
* relational projection
* scoped ontology references
* synthetic resource construction

RSVP does not define ontology truth; it defines composition semantics.

---

# 1. Core Concept

An RSVP expression denotes:

> a resource, or a synthetic resource composed from resources via semantic operators and relations.

A resource may be:

* IRI (RFC 3987-compatible)
* ontology-qualified identifier
* fragment identifier (#)
* compact domain identifier (e.g. Fiction.crime/noir)

---

# 2. Expression Grammar (Informal EBNF)

EXPR := COMPOSITION

COMPOSITION := SYNTHESIS ("+" SYNTHESIS)*

SYNTHESIS := INTERSECTION ("*" INTERSECTION)*

INTERSECTION := PRIMARY ("/" PRIMARY)*

PRIMARY := RESOURCE
| GROUP
| RELATION_BLOCK

GROUP := "(" EXPR_LIST ")"

EXPR_LIST := EXPR ("," EXPR)*

RELATION_BLOCK := "?" "(" RELATION_LIST ")"

RELATION_LIST := RELATION (";" RELATION)*

RELATION := KEY "=" EXPR

RESOURCE := IRI | COMPACT_ID | FRAGMENT

---

# 3. Operators

## 3.1 Aggregation (+)

A + B

Meaning:

* co-presence
* tag aggregation
* unordered combination
* no implied fusion

Example:
Horror + Comedy

---

## 3.2 Fusion / Synthesis (*)

A * B

Meaning:

* emergent hybrid concept
* semantic fusion
* non-reducible combination

Example:
Love * Justice

---

## 3.3 Subtype Path (/)

A/B

Meaning:

* taxonomic refinement
* hierarchical specialization

Example:
Fiction.crime/noir

---

## 3.4 Grouping (...)

Defines scope for:

* precedence control
* relational blocks
* expression lists

---

## 3.5 List Separator (,)

(A, B, C)

Meaning:

* structural grouping
* tuple/list separation
* does NOT imply semantic union

---

## 3.6 Clause Separator (;)

A?( ... ; ... )

Meaning:

* separation of independent relational clauses
* multiple assertions in a single block

---

# 4. Relational Projection

Syntax:

A?(key=EXPR; key2=EXPR)

Meaning:
Attaches semantic relations to a resource without altering its identity.

---

## Core Relations (Reserved Vocabulary)

* about — thematic subject
* in — contextual domain
* as — interpretive role / framing
* through — mediation or mechanism
* of — compositional ownership
* has — attribute relation

---

## Examples

Women?as=Law_enforcement
Repentance?in=Justice
Knowledge?through=Language

---

# 5. Ontology Integration

RSVP supports external ontologies (e.g. MODO).

## Ontology namespace prefix

modo:Genre
wikidata:Q123

## Subtype qualification (/)

modo:Genre/fiction/crime/noir

## Membership / instance-of (::)

Columbo::modo:Media/television/series

Meaning:

* RHS defines category/type
* LHS is an instance/member

---

# 6. Semantic Distinctions

## 6.1 Aggregation vs Fusion

A + B → coexistence
A * B → fused hybrid

---

## 6.2 List vs Semantic Union

(A, B) → structural grouping
(A + B) → semantic aggregation

---

## 6.3 Relation vs Composition

A * B → symmetric synthesis
A?(as=B) → directed interpretation

---

# 7. Example Expressions

Fiction.crime/noir + Fiction.romance

about=(Love?as=Justice; Repentance?through=Suffering)

Women?as=Law_enforcement;
Law_enforcement?in=Politics.movements/feminism

Love * Justice

Columbo::modo:Media/television/series?(about=Crime; in=Law_enforcement; as=Detective_narrative)

---

# 8. Design Principles

## 8.1 Structural vs Semantic Separation

Structural: , ( ) ; =
Semantic: + * / :: ?

---

## 8.2 Progressive specificity

* → aggregation

- → synthesis
  ?() → explicit framing

---

## 8.3 Ontology neutrality

RSVP does not define truth or enforce ontology consistency.

---

## 8.4 Synthetic resources

Any RSVP expression MAY denote a derived or conceptual resource.

---

# 9. Open Questions

1. Should :: and / ever overlap in meaning?
2. Should * remain strictly symmetric?
3. Should ; be allowed at global scope?
4. Should relation blocks nest?
5. Should canonical equivalence rules be defined?

---

# 10. Current Status

RSVP is a semantic composition calculus for constructing structured, relational, and synthetic resource identifiers over one or more ontologies.

---

# 11. Operator Precedence Table (Formal)

## 11.1 Precedence Order (High → Low)

| Level | Construct             | Operator  | Meaning                                   |
| ----- | --------------------- | --------- | ----------------------------------------- |
| 0     | Grouping              | `( ... )` | explicit scope override                   |
| 1     | Relational projection | `?()`     | semantic relation block (postfix binding) |
| 2a    | Ontology membership   | `::`      | instance-of / category membership         |
| 2b    | Subtype path          | `/`       | hierarchical specialization               |
| 3     | Fusion / intersection | `*`       | symmetric semantic synthesis              |
| 4     | Aggregation           | `+`       | co-presence / non-fused grouping          |
| 5     | Structural separator  | `,`       | tuple/list separator (non-semantic)       |
| 5     | Structural separator  | `;`       | clause separator inside relation blocks   |
| 5     | Structural binding    | `=`       | relation assignment binding               |

---

## 11.2 Canonical Binding Rules

### Grouping overrides all precedence

```text
(A + B) * C
```

forces `+` evaluation before `*`.

---

### Relational projection binds to full preceding expression

```text
A + B ?(about=C)
```

is parsed as:

```text
(A + B)?(about=C)
```

---

### Ontology membership binds tighter than subtype

```text
A::B/C
```

is parsed as:

```text
A :: (B/C)
```

---

### Fusion binds tighter than aggregation

```text
A + B * C
```

is parsed as:

```text
A + (B * C)
```

---

## 11.3 Structural vs Semantic Layers

### Structural (non-semantic precedence layer)

* `,` list construction
* `;` clause separation
* `=` relation binding

### Semantic operators

* `+` aggregation
* `*` fusion
* `/` specialization
* `::` membership
* `?()` relational projection

---

## 11.4 Summary Order

```text
( )
→ ?()
→ ::
→ /
→ *
→ +
→ , ; =
```

---

---

# 12. Canonical AST + Normalization Specification (v0.1)

## 12.1 Purpose

This section defines the canonical abstract syntax tree (AST) for RSVP and the normalization rules required to ensure deterministic equivalence across implementations.

It enables:

* consistent parsing
* canonical identity generation
* RDF/SPARQL compilation stability
* cross-system interoperability

---

## 12.2 Canonical AST Node Types

All RSVP expressions compile into the following node types:

### Core Node Types

Node :=
Resource
| Aggregate
| Fusion
| Subtype
| Instance
| RelationBlock
| Relation
| List

---

## 12.3 Node Definitions

### Resource

Resource {
id: IRI | CompactID | Fragment
}

---

### Aggregate (+)

Aggregate {
members: Set<Node>
}

Properties:

* unordered
* non-identity-forming
* recursively flattened

---

### Fusion (*)

Fusion {
members: Set<Node>
canonical_id: SyntheticID
}

Properties:

* identity-forming
* symmetric
* order-independent

---

### Subtype (/)

Subtype {
parent: Node
child: Node
}

Meaning: hierarchical specialization (child ⊆ parent)

---

### Instance (::)

Instance {
instance: Node
type: Node
}

Meaning: rdf:type equivalence

---

### Relation Block (?())

RelationBlock {
subject: Node
relations: Set<Relation>
}

---

### Relation

Relation {
key: String
value: Node
}

---

### List (())

List {
items: Sequence<Node>
}

Properties:

* ordered
* purely structural

---

## 12.4 Canonicalization Rules

### 12.4.1 Aggregation (+)

Commutative:
A + B == B + A

Idempotent:
A + A == A

Flattening:
(A + B) + C == A + B + C

Canonical form:

* sorted by stable canonical ordering

---

### 12.4.2 Fusion (*)

Commutative:
A * B == B * A

Idempotent:
A * A == A

Flattening:
(A * B) * C == A * B * C

Synthetic identity:
Fusion(A,B,...) → SyntheticID(hash(sorted(A,B,...)))

---

### 12.4.3 Subtype (/)

Non-commutative:
A / B ≠ B / A

Associative chaining:
A / B / C == A / (B / C)

---

### 12.4.4 Instance (::)

Non-commutative:
A :: B ≠ B :: A

Deduplicated:
Repeated declarations collapse

---

### 12.4.5 Relation Blocks (?())

Subject binding:
A + B ?(k=C) == (A + B)?(k=C)

Relation deduplication:
A?(k=B; k=B) == A?(k=B)

Flattening:
Nested relation blocks are collapsed

---

### 12.4.6 Lists (,)

Ordered and non-normalized:
(A,B) ≠ (B,A)

---

## 12.5 Normalization Pipeline

1. Parse → Raw AST
2. Structural flattening
3. Canonical ordering
4. Identity resolution (fusion nodes)
5. Deduplication
6. Final canonical AST emission

---

## 12.6 Equivalence Rules

Aggregation:
A + B == B + A

Fusion:
A * B == B * A
(A * B) * C == A * B * C

Relations:
A?(k=B; m=C) == A?(m=C; k=B)

Structural equivalence ignores parentheses if structure identical

---

## 12.7 Synthetic Identity Model

Fusion creates canonical nodes:

Fusion(A,B,...) → Node(SID)

Where:
SID = hash(canonical_sorted([A,B,...]))

Properties:

* deterministic
* global
* ontology-independent
* reusable

---

## 12.8 Compilation Guarantees

* Deterministic output for identical input
* Compositional AST structure
* RDF/SPARQL compatibility
* Cross-ontology neutrality

---

## 12.9 System Implication

RSVP now functions as:

* a semantic expression calculus
* a canonical AST-driven compiler frontend
* a synthetic resource generation system

---

---

# 13. Synthetic Identity Collisions & Cross-Expression Reuse (v0.1)

## 13. Synthetic Identity Collisions & Cross-Expression Reuse (v0.2)

## 13.1 Purpose

This section defines how RSVP handles synthetic identities created by fusion (`*`) and how identical constructions behave across expressions and ontological contexts.

It establishes RSVP as a **contextual semantic composition system**, not a globally collapsing identity database.

---

## 13.2 Synthetic Identity Rule (Core)

For any fusion expression:

```text
A * B * C
```

a synthetic identity is generated:

```text
SID = hash(canonical_sorted([A, B, C]))
```

This rule is the sole source of identity formation in RSVP.

---

## 13.3 Identity Scope (Contextual Model)

RSVP operates under a **contextual identity space model (Option B)**.

This means:

* SIDs are deterministic within a given execution context
* identity reuse across contexts is OPTIONAL and externally controlled
* no global identity collapse is assumed by default

A context may be:

* a document
* a runtime session
* a compilation unit
* a distributed graph shard

Thus:

> identical fusion structures are identical *within a context*, not inherently across all contexts

---

## 13.4 Identity Invariance Rules (Unified)

Fusion identity is invariant under:

* permutation of members
* re-bracketing of fusion trees
* duplicate elimination
* serialization format differences

Example:

```text
(A * B) * C == A * (B * C) == A * B * C
```

All expressions are equivalent **within the same identity context**.

---

## 13.5 Ontology Independence Constraint (Clarified)

Synthetic identity generation MUST NOT depend on:

* ontology selection
* relation annotations (`?()`)
* instance typing (`::`)
* subtype structure (`/`)

Only the fusion member set determines identity.

### Explication

This guarantees a strict separation between:

* **identity formation (structural composition)**
* **semantic interpretation (ontological projection)**

Thus, RSVP identities are *pre-semantic*: they exist prior to ontological assignment.

---

## 13.6 Classification Rule (`::`)

```text
(A * B) :: C
```

means:

* compute SID first (fusion identity is unaffected)
* attach classification afterward
* classification does NOT modify identity

Therefore:

* identity layer is stable
* ontology layer is orthogonal

---

## 13.7 System Implication (Reframed)

RSVP defines a:

> **declarative compositional space for nonlinear relations of concepts**

Properties:

* identities are compositional rather than referential
* meaning emerges from structured combination, not lookup
* ontologies act as interpretive overlays, not identity generators

This positions RSVP as:

* a semantic composition calculus
* rather than a graph query or knowledge base system

---

## 13.8 Summary Rule

> Fusion defines identity. Everything else interprets or annotates it within a context.

---

# 14. Context Model (v0.1)

## 14.1 Purpose

This section defines the structure of *contexts* in RSVP, which determine the scope of identity, canonicalization, and evaluation.

A context is the fundamental unit of semantic isolation and composition.

---

## 14.2 Context Definition

A context is a named or anonymous evaluation environment:

```text
Context := {
  id: ContextID,
  parent: ContextID | null,
  rules: ContextRules
}
```

Where:

* `id` uniquely identifies the context
* `parent` defines optional nesting
* `rules` define scoping behavior (identity, ontology visibility, evaluation mode)

---

## 14.3 Contextual Identity Scope

All synthetic identities (SIDs) are scoped to a context.

Thus:

```text
SID(A * B) in Context X ≠ SID(A * B) in Context Y
```

unless explicitly linked via context mapping rules.

---

## 14.4 Context Nesting

Contexts may be nested:

```text
C_child ⊂ C_parent
```

Rules:

* child contexts inherit ontology visibility unless overridden
* child contexts do NOT inherit identity caches unless explicitly enabled

---

## 14.5 Context Isolation Principle

By default:

* identity generation is local to a context
* canonicalization is deterministic but not globally shared

This preserves RSVP as a compositional system rather than a global database.

---

## 14.6 Cross-Context Reference Model

To reference a SID across contexts, a qualified form is used:

```text
SID@ContextID
```

Meaning:

* resolve SID within specified context scope
* do not merge identity spaces implicitly

---

## 14.7 Context Merging (Explicit Operation)

Contexts may be merged only through an explicit operation:

```text
Merge(C1, C2)
```

Rules:

* identity spaces remain distinct unless merged
* SID collisions are re-evaluated under merged canonicalization rules

---

## 14.8 Evaluation Rule

All RSVP evaluation occurs relative to a context:

```text
Eval(Expression, Context)
```

No expression has meaning outside a context.

---

## 14.9 System Implication

This establishes RSVP as a:

> context-relative semantic composition system with deterministic but scoped identity generation

This prevents accidental global ontology collapse while preserving compositional consistency.

---

## 14.10 Summary Rule

> Identity is local. Structure is portable. Context defines meaning boundaries.

---

# 15. Context Linking Semantics (v0.1)

## 15.1 Purpose

This section defines how meanings, identities, and expressions are related across distinct contexts without merging identity spaces.

Context linking enables interoperability while preserving isolation.

---

## 15.2 Core Principle

Contexts are NEVER merged implicitly.

Instead, RSVP supports **explicit mapping relations between contexts**.

---

## 15.3 Context Mapping Relation

A mapping between contexts is defined as:

```text
Map(C1 → C2)
```

Meaning:

* expressions in C1 may be interpreted in C2
* identities are NOT unified
* only translation rules are shared

---

## 15.4 Identity Translation (Non-Merging Projection)

A SID may be projected across contexts:

```text
SID@C1 ⇝ SID@C2
```

This is a *projection*, not a re-generation.

Rules:

* original SID remains unchanged
* target SID is context-local alias or view
* no global identity collapse occurs

---

## 15.5 Semantic Alignment Rules

When mapping contexts, alignment may be defined over:

* ontology correspondences
* relation renamings
* subtype mappings

Example:

```text
A::B in C1 → A::B' in C2
```

This does not affect identity, only interpretation.

---

## 15.6 Expression Translation

An RSVP expression can be translated between contexts:

```text
Translate(Expression, C1 → C2)
```

Rules:

* re-evaluates expression under C2 rules
* preserves structural composition where possible
* recomputes SID only within C2 scope

---

## 15.7 Context Linking Graph

Contexts form a higher-order graph:

```text
C1 → C2 → C3
  ↘──────↗
```

Properties:

* edges represent mapping functions
* nodes remain isolated identity spaces
* cycles are allowed but do not imply identity merge

---

## 15.8 Non-Equivalence Guarantee

Even if two contexts are fully mapped:

```text
SID(A * B in C1) ≠ SID(A * B in C2)
```

unless explicitly normalized under a shared context.

---

## 15.9 System Implication

This enables RSVP to function as:

> a network of partially interoperable semantic spaces

rather than a single unified ontology or graph.

---

## 15.10 Summary Rule

> Contexts may communicate. They do not collapse.

---

# 16. Ontology Reference Neutralization (v0.1)

## 16.1 Purpose

All ontology references in RSVP are non-binding and serve only as examples of external semantic systems.

---

## 16.2 Neutral Reference Rule

Any mention of a named ontology (e.g., MODO) MUST be interpreted as:

> "an external ontology system, such as MODO"

and does NOT imply:

* integration
* dependency
* shared semantics
* structural coupling

---

## 16.3 Replacement Semantics

All prior ontology-specific references SHOULD be read as:

* "external ontology system (such as MODO)"
* or "ontology layer (such as MODO)"

This preserves RSVP ontology neutrality by default.

---

## 16.4 System Constraint

RSVP does NOT define, embed, or require any specific ontology.

Ontologies remain:

* pluggable
* externally defined
* translation-mapped only

---

## 16.5 Summary Rule

> Ontologies are examples, not dependencies.
