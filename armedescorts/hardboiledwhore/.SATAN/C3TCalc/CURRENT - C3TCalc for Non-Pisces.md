# C3TCalc for Non-Pisces

## 0. What This Is

C3TCalc is a compact language for making **meaning-maps**.

It lets people and software write down relationships between things clearly enough that they can be parsed, searched, compared, transformed, and discussed.

The technical spec says exactly how the language works.

This document explains the idea in plain language.

---

## 1. The Basic Shape

The basic shape of C3TCalc is:

```text
tail & relation @ head
```

Read it as:

```text
tail --relation--> head
```

Or in ordinary English:

```text
this thing has this relationship to that thing
```

A complete relation statement is called a **fish**.

A fish has three main parts:

```text
TAIL RESOURCE
  the thing the relation starts from

RELATION
  the named connection

HEAD RESOURCE
  the thing the relation points to
```

Example teaching form:

```text
Fannie_Spanker/books/Hardboiled_Whore/characters/Judith
  & role
  @ roles/femme_fatale
```

Compact form:

```text
Fannie_Spanker/books/Hardboiled_Whore/characters/Judith&role@roles/femme_fatale
```

Plain English:

```text
Judith, a character in Hardboiled Whore by Fannie Spanker, has the role femme fatale.
```

---

## 2. Quick Syntax Reference

```text
TAIL & RELATION @ HEAD
```

Markers:

```text
&      asserted relation
&?     unresolved/contextual relation
&!     negative asserted relation
&?!    unresolved/contextual negative relation
@      relation target / head marker
~      context grounding
_      anonymous resource
*X     named scoped resource
$X     return this in a query
X      ordinary resource reference
/      resource path traversal
:      prefix separator
^      mapping / projection
(...)  list expression
{...}  school / fish block
;      fish separator inside schools
[...]  positional relation-template slot
"..."  literal
%      literal escape
```

Reserved for later:

```text
= + . | ! < > #
```

---

## 3. Reading a Basic Fish

```text
Fannie_Spanker/books/Hardboiled_Whore/characters/Judith
  & role
  @ roles/femme_fatale
```

This means:

```text
Judith --role--> femme_fatale
```

The path identifies Judith as the character inside *Hardboiled Whore*, under Fannie Spanker's books.

---

## 4. Unresolved Fish

```text
Fannie_Spanker/books/Hardboiled_Whore/characters/Judith&?role@(roles/femme_fatale,roles/conspirator)
```

Plain English:

```text
Judith's role is unresolved between femme fatale and conspirator.
```

---

## 5. Negative Fish

```text
Judith&!is_[owner]_of@Andrea
```

Plain English:

```text
Judith is not Andrea's owner.
```

---

## 6. Context-Grounded Fish

```text
Fannie_Spanker/books/Hardboiled_Whore/characters/Judith&role@roles/femme_fatale~Fannie_Spanker/books/Hardboiled_Whore/chapters/03
```

Plain English:

```text
In Chapter 3, Judith has the role femme fatale.
```

---

## 7. Fusion

```text
roles/femme_fatale-conspirator&fuse@(roles/femme_fatale,roles/conspirator)
```

Plain English:

```text
The role femme_fatale-conspirator is the fusion of femme fatale and conspirator.
```

Fusion is an ordinary relation, not punctuation magic.

---

## 8. Anonymous Fusion Inside a Relation Slot

```text
Shosh&?is_[_&fuse@(paranormal/ghost,family/mother)]_of@Andrea
```

Plain English:

```text
Shosh has an unresolved/contextual relation to Andrea whose role slot is filled by an anonymous resource that is the fusion of paranormal/ghost and family/mother.
```

---

## 9. Query Binding

```text
$person&is_sibling_of@people/Jordan
```

Plain English:

```text
Return every person who is a sibling of Jordan.
```

---

## 10. Existential Scoped Binding

```text
{
  people/Alex&is_friend_of@*friend;
  *friend&is_sibling_of@people/Jordan;
}
```

Plain English:

```text
There exists some scoped resource friend such that Alex is friends with it, and that same friend is Jordan's sibling.
```

---

## 11. Anonymous Resource Binding

```text
_&fuse@(paranormal/ghost,family/mother)
```

Plain English:

```text
some anonymous resource is the fusion of paranormal/ghost and family/mother
```

---

## 12. Lists

```text
(A,B,C)
```

A list is not automatically a graph, set, tuple, or multiple heads. The relation template decides how to interpret it.

---

## 13. Schools

```text
Act_2/Chapter_17&establishes@{
  Andrea&is_[owner]_of@Judith;
  Judith&is_[pet]_of@Andrea;
}
```

Plain English:

```text
Act 2, Chapter 17 establishes these fish: Andrea is Judith's owner; Judith is Andrea's pet.
```

---

## 14. Mapping Between Systems

```text
ontology-a/politics/movement^ontology-b/social/ideology
```

Plain English:

```text
Project or map politics/movement from Ontology A into social/ideology in Ontology B.
```

---

## 15. The Secret Cream Sauce

The most important idea in C3TCalc is:

```text
A complete fish is itself a resource.
```

The fish is both statement and thing.

---

## 16. Tiny Cheat Sheet

```text
TAIL & RELATION @ HEAD
```

Core vocabulary:

```text
fish      complete relation expression
school    block of fish
resource  addressable thing, including complete fish
```

Shortest summary:

```text
C3TCalc is a language for making meaning-maps where relations are fish, fish form schools, and every complete fish is itself a resource.
```
