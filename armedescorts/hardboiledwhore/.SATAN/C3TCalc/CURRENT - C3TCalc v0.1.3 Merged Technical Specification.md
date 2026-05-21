# C3TCalc v0.1.3 Merged Technical Specification (Draft)

## 0. Status

C3TCalc v0.1.3 merges the useful implementation and ontology scaffolding from **C3TCalc v0.1 Standalone Specification (Draft)** into the newer v0.1.2 technical model.

This document supersedes:

- C3TCalc v0.1 Standalone Specification (Draft)
- C3TCalc v0.1.1 Specification
- C3TCalc v0.1.2 Technical Specification

The lay-facing explanation should remain separate: **C3TCalc for Non-Pisces**.

Core decisions:

- a complete fish is itself a first-class resource;
- ordinary fish use `&` as the tail marker;
- unresolved and negative relation states are tail-mode modifiers: `&?`, `&!`, `&?!`;
- `_` is anonymous resource binding;
- `*X` is named scoped binding introduction;
- `$X` is query / return binding;
- relations are positional templates, e.g. `is_[owner]_of`;
- canonical lists are comma-separated `ListExpr` values;
- schools / fish blocks use `{ ... }`;
- `fuse` and `aggregate` are ordinary relation templates;
- bracketed parameter lists are removed from canonical syntax;
- relation-chain syntax is reserved future-valid syntax but unimplemented in v0.1.3.

---

## 1. Purpose

C3TCalc defines a compact graph-native expression syntax for deterministic semantic relations, graph patterns, scoped bindings, query bindings, context grounding, typed declarations, local relation definitions, ontology-defined relation templates, and canonical graph-expression serialization.

C3TCalc is independently useful outside Neurative.

---

## 2. Design Principles

C3TCalc is graph-anatomical rather than algebraic.

Relations have tails and heads. A complete relation expression is a **fish**. A complete fish is itself a **resource**.

Bindings may be referential, anonymous, existential, declarative, or returned. Context is explicit. Fusion and aggregation are relation kinds rather than primitive operators. Relation meaning belongs to relation templates / ontologies, not to the parser core.

---

## 3. Core Concepts

### Resource

A resource is any addressable semantic thing in C3TCalc: people, characters, scenes, books, concepts, roles, locations, relation templates, relation-template instances, complete fish, schools, anonymous resources, query-bound resources, and scoped declared resources.

Resource paths use `/` traversal:

```text
Fannie_Spanker/books/Hardboiled_Whore/characters/Judith
roles/femme_fatale
politics/movements/antifascism
```

Prefixed names use `:`:

```text
hw:locations/Santa_Virginia
geo:US/CA/San_Diego
mdo:Movements/antifascism
```

### Fish

Canonical anatomy:

```text
tail&relation@head
```

A fish has a tail resource/expression, relation resource/template instance, head resource/expression, tail mode, and optional context.

A fish is itself a first-class resource.

### School / Fish Block

```text
{
  fish;
  fish;
}
```

A school is a scoped block of fish and is itself a structured resource.

---

## 4. Core Symbol Summary

```text
&      relation tail marker
&?     unresolved/contextual tail mode
&!     asserted negative tail mode
&?!    unresolved/contextual negative tail mode
@      relation head marker
~      context / scope grounding
_      anonymous resource binding
*X     named scoped binding introducer
$X     query / return binding
X      referential resource use
/      resource / concept / type / scope path traversal
:      prefix separator inside prefixed names
^      mapping / projection
(...)  canonical ListExpr
{...}  school / fish block / binding scope
;      fish / statement separator inside schools
[...]  positional relation-template slot inside relation names
"..."  universal literal surface
%      literal escape inside quoted literals
```

Reserved or currently unassigned:

```text
= + . | ! < > #
```

Removed from canonical v0.1.3:

```text
relation[param=value]
A?relation@B
bare * as anonymous binding
```

---

## 5. Identifier, Literal, and Prefix Rules

Identifiers SHOULD use Unicode XID classes:

```text
identifier_initial   = Unicode XID_Start
identifier_continue  = Unicode XID_Continue, excluding structural delimiters
```

Structural delimiters include active/reserved syntax characters.

Quoted literals use a universal literal surface:

```text
"..."
```

Quoted literals do not intrinsically mean string; they are interpreted by expected type/template/declaration/ontology context.

Percent escapes are used inside literals:

```text
%22 = quote
%25 = percent
```

Control characters MUST be percent-encoded. Percent escapes use UTF-8 byte encoding and SHOULD use uppercase hexadecimal.

Base64 MAY be used as a typed literal codec, not as the universal escape mechanism.

Prefix declarations use declaration templates:

```text
*mdo&declare_[prefix]@"https://fanniespanker.github.io/Media-Description-Ontology/";
```

---

## 6. Fish Anatomy, Tail Modes, and Reversal

Canonical directed fish form:

```text
A&relation@B
```

Meaning:

```text
A --relation--> B
```

Tail modes:

```text
&      asserted positive fish
&?     unresolved/contextual/interrogative positive fish
&!     asserted negative fish
&?!    unresolved/contextual/interrogative negative fish
```

`?` is not probability. `!` marks negative relation polarity only as part of a tail-mode marker.

Reverse-oriented surface form:

```text
A@relation&B
```

means:

```text
B --relation--> A
```

Examples:

```text
A&fan_of@B
B@fan_of&A
```

canonicalize to the same fish.

Reverse-oriented modes attach to the tail side:

```text
B@role&?A
```

normalizes to:

```text
A&?role@B
```

Surface reversal is not inverse-relation semantics.

---

## 7. Inverse Relations

Ontologies SHOULD explicitly define inverse relations. Inverse behavior MUST NOT be inferred from reverse surface syntax alone.

```text
A&parent_of@B
B@parent_of&A
```

are the same fish written from opposite orientations.

```text
B&child_of@A
```

is a distinct relation kind unless the ontology declares `parent_of inverse child_of`.

---

## 8. Context, Mapping, and Paths

The marker `~` grounds an expression in a context, scope, frame, interpretation, or local condition:

```text
expression~context
```

The marker `^` is reserved for mapping / projection:

```text
source^target
```

Mapping can also be expressed as ordinary fish:

```text
source&maps_to@target
```

The slash `/` denotes traversal within a resource-like address space. A path expression is not itself a relation assertion.

---

## 9. Binding Taxonomy

```text
X      referential resource use
_      anonymous resource binding
*X     named scoped binding introducer
$X     query / return binding
```

`*X` has existential or declarative interpretation depending on the consuming relation template.

A scoped binding name has exactly one binding kind per scope.

Query examples:

```text
$A&is_sibling_of@B
A&$rel@B
$A&$rel@$B
```

Bare `*` as anonymous binding is deprecated; use `_`.

---

## 10. Lists and Schools

Parentheses define canonical list expressions:

```text
(A,B,C)
```

A list is not automatically a graph, set, tuple, school, multiple heads, or expanded into multiple relations.

Example:

```text
A&rel@(B,C)
```

means the head is one `ListExpr(B,C)`.

Curly braces define schools:

```text
{
  Andrea&is_[owner]_of@Judith;
  Judith&is_[pet]_of@Andrea;
}
```

A school is itself a structured resource.

---

## 11. Relation Templates

Relations are template instances. Square brackets inside relation names define positional slots.

Example:

```text
Andrea&is_[owner]_of@Judith
```

Template shape:

```text
is_[role]_of
```

Multi-slot example:

```text
hw:locations/Santa_Virginia&is_[20]_[minutes]_[East]_of@geo:US/CA/San_Diego
```

v0.1.3 removes canonical `relation[param=value]` syntax. Use positional templates or ordinary fish.

---

## 12. Declarations and Expressions as Values

Declarations are ordinary fish using declaration templates:

```text
*binding&declare_[kind]@target
```

Examples:

```text
*name&declare_[string]@"Bob";
*R&declare_[relation]@parent_of;
*expr&declare_[c3t_expr]@"A&parent_of@B";
```

Declaration records SHOULD remain inspectable.

---

## 13. Fusion and Aggregation

Fusion and aggregation are ordinary relation templates:

```text
A&fuse@(B,C)
A&aggregate@(B,C)
```

The tail is the result and the head list supplies inputs. Surface canonicalization never performs fusion.

---

## 14. Complete Fish as Resources

A complete fish is not merely syntax; it is itself a resource.

Example:

```text
Judith&role@roles/femme_fatale
```

Potential operations over fish resources include assertion provenance, denial, contradiction, support, grounding, temporal scoping, citation, versioning, transformation, and inclusion in a school.

Fish-resource quoting/addressing syntax is not finalized in v0.1.3. Working illustrative notation:

```text
[Judith&role@roles/femme_fatale]&grounded_in@Act_2/Chapter_17
```

---

## 15. Relation-Chain Continuation

Relation-chain syntax is reserved as future-valid syntax and planned no later than v1.0.

v0.1.3 parsers SHOULD recognize chain-like forms and emit a deterministic reserved/unimplemented diagnostic.

Example reserved future syntax:

```text
A&parent_of@*X@parent_of&B
```

Intended future desugaring:

```text
{
  A&parent_of@*X;
  B&parent_of@*X;
}
```

---

## 16. Strict and Diagnostic Modes

Strict mode rejects malformed expressions and emits no repair interpretation as canonical output.

Diagnostic mode may suggest repairs, infer likely intended graph shape when safe, explain malformed fish anatomy, and show canonical candidates.

Only strict-mode parsed expressions may become canonical graph input.

---

## 17. Canonicalization

Canonical fish record:

```text
Fish {
  tail: Expr
  relation: RelationExpr
  head: Expr
  mode: TailMode
  context: Optional<Expr>
}
```

Canonical formatting SHOULD remove insignificant whitespace, preserve grouping, canonicalize lists with commas, normalize reverse surface orientation, canonicalize tail-mode order, preserve source order in schools unless a profile defines otherwise, normalize literals to NFC where applicable, uppercase percent escapes, and preserve declarations in canonical AST.

---

## 18. Parser Core vs Ontology/Registry

C3TCalc core syntax MUST NOT bake in semantic relations such as `parent_of`, `role`, `fuse`, `aggregate`, `is_[owner]_of`, or `maps_to`.

The parser recognizes fish anatomy, tail modes, bindings, lists, schools, literals, paths, prefixes, and relation-template surface forms.

Validation, inverse expansion, slot checking, query expansion, canonicalization beyond syntax, and diagnostics depend on a relation registry, Herring Bones module, or ontology definition profile.

---

## 19. Relation Template Metadata

A relation template may specify:

```text
name
template_shape
slot_count
slot_names
slot_types
slot_order_policy
arity
directionality
inverse
inverse_policy
symmetry
asymmetry
transitivity
reflexivity
acyclicity
tail_types
head_types
canonicalization_rule
materialization_policy
query_expansion_policy
validation_diagnostics
human_readable_gloss
```

---

## 20. Storage and Indexing

A directed fish SHOULD be stored once in canonical form.

Recommended indexes:

```text
by_tail
by_head
by_relation_template
by_tail_kind
by_head_kind
by_context
by_fish_resource
```

Fish identity/hashing/addressing must be stable under canonicalization.

---

## 21. Herring Bones and Gradual Formalization

Herring Bones (HB) is the standard relation/template library.

It is not part of parser core.

Herring Bones SHOULD support local relation declaration, local type declaration, glosses, unknown-relation warnings, promotion from local relation to formal ontology template, and migration without destroying authorial intent.

Example:

```text
{
  *haunts&declare_[relation]@"recurring affective presence";
  Memory&haunts@Narrator;
}
```

---

## 22. Ontology Definition Profile

Ontologies may be written in C3TCalc using a restricted Ontology Definition Profile.

C3TCalc core provides syntax, AST, fish anatomy, tail modes, bindings, lists, schools, literals, declarations, canonicalization framework, and diagnostics framework.

Ontologies provide types, constructors, relation templates, slot schemas, inverse declarations, validation rules, literal interpretations, canonicalization profiles, and query expansion policies.

Ontology constructors and validators MUST be pure, deterministic, inspectable, and canonicalizable.

---

## 23. Locked v0.1.3 Implementation Decisions

```text
relation_chain_continuation = reserved_valid_future_syntax_unimplemented_in_v0_1_3_planned_by_v1_0
relation_anatomy = single_tail_single_relation_single_head_expression
list_expression_order = source_preserved_template_interpreted
unknown_relation_parse_policy = allowed
unknown_relation_permissive_check = warning
unknown_relation_strict_check = error_unless_locally_declared_or_loaded
prefix_expansion = simple_concatenation
literal_encoding = UTF-8
literal_unicode_normalization = NFC
literal_percent_escapes = uppercase_canonical
raw_quote_inside_literal = forbidden_use_%22
raw_percent_inside_literal = forbidden_unless_valid_escape_or_%25
declaration_preservation = canonical_ast_always_preserves
relation_declaration_emission = profile_defined
ontology_source_language = C3TCalc_Ontology_Definition_Profile
initial_implementation_language = Rust
parameter_lists = removed_from_canonical_syntax
anonymous_binding = underscore
named_scoped_binding = star_name
query_return_binding = dollar_name
```

---

## 24. CLI and Serialization

Recommended CLI:

```text
c3t parse
c3t fmt
c3t check
c3t canon
c3t explain
c3t emit-json
```

Canonical identity in v0.1.3 is defined by canonical C3TCalc surface text, canonical parser/AST semantics, and deterministic canonicalization rules.

JSON AST output may be provided for development, tests, snapshots, and tooling. Optional S-expression export may be added later as non-default/non-canonical tooling.

---

## 25. Diagnostics

Recommended diagnostic categories:

```text
MalformedFishAnatomy
ReservedUnimplementedChain
UnknownRelationTemplate
BindingKindConflict
DuplicateScopedBinding
UndeclaredPrefix
InvalidLiteralEscape
InvalidListSyntax
InvalidTemplateSlot
IllegalParameterBracketSyntax
FishResourceSyntaxUnfinalized
DeprecatedQuestionTailSyntax
DeprecatedParameterListSyntax
DeprecatedBareStarAnonymousBinding
```

---

## 26. Migration Notes

Deprecated v0.1 unresolved syntax:

```text
A?role@B
```

Canonical v0.1.3:

```text
A&?role@B
```

Deprecated v0.1 anonymous binding:

```text
*
```

Canonical v0.1.3:

```text
_
```

Deprecated v0.1 parameter syntax:

```text
A&parent_of[kind=biological]@B
```

Canonical alternatives:

```text
A&is_[biological_parent]_of@B
[A&parent_of@B]&kind@relation_kinds/biological
```

Old STL naming is replaced by **Herring Bones (HB)**.

---

## 27. Repository Naming

```text
repo: purple-herring
language: C3TCalc
source extension: .fish
toolchain: SARDINE
standard library: Herring Bones
```

Release channels:

```text
Surströmming / surstromming = alpha / unstable
pickled = beta
smoked = release candidate
herring / stable = stable release
```

---

## 28. Foundational Claim

C3TCalc is not just a prettier triple syntax.

A fish is a first-class resource.

The fish is both statement and thing.

C3TCalc is an ontology priesthood machine with a layperson parish interface.
