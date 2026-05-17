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

```EBNF
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
```

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

## 13.1 Purpose

This section defines how RSVP handles synthetic identities created by fusion (`*`) and whether identical constructions across separate expressions resolve to the same or different entities.

This is critical for determining whether RSVP behaves as:

* a semantic database (global identity collapse)
* or a compositional universe (local identity generation)

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

---

## 13.3 Global Identity Reuse Principle (Default Mode)

If two expressions produce identical canonical member sets, they MUST resolve to the same SID:

```text
A * B == B * A
A * B → SID_1
B * A → SID_1
```

This enforces:

* global structural identity consistency
* ontology-independent reuse
* deterministic graph compression

---

## 13.4 Cross-Expression Collision Rule

If identical fusion sets appear in different expressions or documents:

```text
A * B   (in expression X)
A * B   (in expression Y)
```

They resolve to the same synthetic node:

```text
SID(A,B)
```

This is a **global identity space**, not a local one.

---

## 13.5 Canonical Identity Invariance

Identity is invariant under:

* permutation of members
* re-bracketing of fusion trees
* repeated re-serialization

Example:

```text
(A * B) * C == A * (B * C) == A * B * C
```

All map to the same SID.

---

## 13.6 Identity Non-Collapse Rule (Exception Mode)

A system MAY optionally disable global reuse and treat each fusion as locally unique:

```text
A * B (instance-1)
A * B (instance-2)
```

This produces distinct SIDs:

```text
SID_1 != SID_2
```

This mode is ONLY valid if explicitly enabled by execution context.

---

## 13.7 Ontology Independence Constraint

Synthetic identity generation MUST NOT depend on:

* ontology selection
* relation annotations (?())
* instance typing (::)
* subtype structure (/)

Only the fusion set defines identity.

---

## 13.8 Interaction With Instance Typing (::)

If a fused node is classified:

```text
(A * B) :: C
```

then:

* SID is computed BEFORE classification
* classification does NOT affect SID

Thus:

```text
SID(A,B) remains stable regardless of :: annotations
```

---

## 13.9 Interaction With Relations (?())

Relations do NOT affect identity:

```text
A * B ?(about=X)
```

SID is computed solely from:

```text
[A, B]
```

---

## 13.10 System Implication

This design choice makes RSVP:

* a **globally deduplicating semantic system** by default
* capable of forming a shared "concept space"
* consistent with RDF graph merging semantics

---

## 13.11 Summary Rule

> Fusion defines identity. Everything else annotates it.

---
