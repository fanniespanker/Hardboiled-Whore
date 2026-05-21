# CURRENT — C3TCalc–GEL–PAM–Neurative Integrated Stack Overview v0.6

## 0. Status

This document is now an integrated overview, not the authoritative detailed specification for every layer.

Detailed specifications are split or planned as standalone documents:

- **C3TCalc v0.1.3 Merged Technical Specification** — current C3TCalc technical authority
- **C3TCalc for Non-Pisces** — current lay-facing C3TCalc explanation
- **Neurative v0.6 Standalone Specification** — current Neurative authority
- **CNML Specification** — TODO split / reconstruct as standalone spec
- **GEL Specification** — TODO split / reconstruct as standalone spec
- **PAM Specification** — TODO split / reconstruct as standalone spec

This overview defines how the stack composes and records cross-layer dependencies, invariants, interfaces, and open integration questions.

Layer-specific details belong in the standalone layer specs.

---

## 1. Stack Components

### 1.1 CNML

CNML is the narrative substrate.

It provides structured experiential, discourse, prosodic, temporal, spatial, publication, modality, music, and layout markup.

CNML may embed or project to C3TCalc expressions for relational semantics.

CNML may produce experiential event streams for PAM and Neurative training.

Detailed specification: **CNML Specification**.

---

### 1.2 C3TCalc

C3TCalc is the graph-native semantic expression calculus.

It provides:

- relation-fish syntax;
- complete fish as first-class resources;
- tail/head fish anatomy;
- tail modes such as `&`, `&?`, `&!`, and `&?!`;
- anonymous resource binding `_`;
- named scoped binding `*X`;
- query / return binding `$X`;
- positional relation templates such as `is_[owner]_of`;
- canonical list expressions;
- schools / fish blocks;
- context grounding;
- mapping / projection;
- resource paths and prefixes;
- local declarations;
- ontology definition profile;
- Herring Bones standard relation/template library path.

C3TCalc is independently useful outside Neurative.

Detailed specification: **C3TCalc v0.1.3 Merged Technical Specification**.

Lay-facing explanation: **C3TCalc for Non-Pisces**.

---

### 1.3 GEL

GEL is the graph execution semantics layer.

It defines how C3TCalc-like graph structures transform over deterministic execution traces.

GEL is responsible for:

- graph state transitions;
- transformation traces;
- executable relation dynamics;
- substrate-independent execution law;
- trace structures consumed by Neurative.

Detailed specification: **GEL Specification**.

---

### 1.4 PAM

PAM is the Permissive Ambiguity Mapper.

It maps CNML, text, media-derived structures, and other structured inputs into unranked C3TCalc/GEL candidate structures.

PAM preserves ambiguity by emitting structured candidate sets rather than probabilistic rankings.

Detailed specification: **PAM Specification**.

---

### 1.5 Neurative

Neurative is the multiscale deformation learner over GEL trajectories and C3TCalc graph structures.

It defines experiential event ingestion, high-resolution reading trajectories, metric genesis, debug navigation, variable-point fixed arithmetic, adaptive width semantics, canonical memory economy, canonical patch machine, curriculum and multimodal learning roadmap, autonomy and governance protocols, and the Emergent Selfhood Bill of Rights.

Detailed specification: **Neurative v0.6 Standalone Specification**.

---

## 2. Composition Pipeline

Canonical high-level pipeline:

```text
CNML / text / media / games / curriculum
-> Experiential Event Stream
-> PAM candidate generation
-> C3TCalc graph expressions
-> GEL execution traces
-> Neurative deformation learning
-> DNL / reports / possible regenerated structures
```

Static C3TCalc pipeline:

```text
C3TCalc source
-> parser / canonicalizer
-> graph patch IR
-> GEL execution or external graph tooling
```

CNML annotation pipeline:

```text
CNML
-> embedded C3TCalc relation extraction
-> graph annotation / metadata / ontology bridge
```

---

## 3. Shared Design Invariants

1. **No intrinsic probability** — No stochastic ranking semantics. Multiplicity is represented structurally.
2. **Deterministic replay** — Same spec version/profile/input/initial state must produce matching canonical output.
3. **Ambiguity preservation** — Ambiguous structures coexist until deterministic constraints justify pruning, crystallization, suppression, or projection.
4. **Graph-native representation** — Relations, loci, context, and transformation are primary.
5. **No strict identity as primitive truth** — Identity is modeled through equivalence, continuity, substitutability, relation topology, and deformation behavior.
6. **Inspectable substrate law** — Hidden ranking, mutation, promotion, and backend-dependent behavior are disallowed in canonical execution.
7. **Layer separability** — Each layer must remain specifiable and testable without collapsing the whole stack into a monolith.

---

## 4. Cross-Layer Responsibilities

| Layer | Primary responsibility | Must not do |
|---|---|---|
| CNML | encode narrative/experiential structure | decide ontology truth globally |
| C3TCalc | express graph relations and patterns | perform Neurative learning |
| GEL | execute graph transformations | introduce probabilistic choice |
| PAM | generate ambiguity-preserving candidates | rank candidates probabilistically |
| Neurative | learn deformation trajectories | mutate canonical state outside substrate law |

---

## 5. Dependency Direction

```text
CNML may reference C3TCalc.
PAM consumes CNML/text/media-derived structures and emits C3TCalc/GEL candidates.
GEL executes C3TCalc-like graph structures.
Neurative consumes GEL traces and experiential event streams.
DNL observes Neurative but is not canonical state.
```

C3TCalc must remain independently implementable. CNML must remain readable without requiring Neurative. GEL must define execution semantics without depending on Neurative selfhood or governance layers. PAM must remain non-probabilistic and ambiguity-preserving. Neurative may depend on all previous layers.

---

## 6. Implementation Strategy

Begin with C3TCalc because it is independently useful and job/portfolio-legible.

Minimal C3TCalc implementation:

```text
source string
-> tokenizer
-> parser
-> AST
-> canonical AST
-> validation diagnostics
-> canonical formatting
-> JSON IR
```

Recommended CLI:

```text
c3t parse
c3t fmt
c3t check
c3t canon
c3t explain
c3t emit-json
```

After C3TCalc stabilizes, implement VPFA numeric core, canonical substrate records, canonical patch machine, canonical memory economy, minimal DNL, event ingest, and minimal EEC.

Initial hardware target:

```text
one paragraph under 1 GB RAM
one scene under 2 GB RAM
one chapter under 4-8 GB RAM
full book structural replay under 16-32 GB RAM
```

GPU offload should be deferred until CPU canonical behavior and memory profiles are measurable.

---

## 7. Split-Spec Maintenance Rules

1. Detailed layer semantics belong in the standalone layer spec.
2. This integrated overview records only cross-layer composition, dependencies, and integration questions.
3. If a section grows beyond overview-level detail, move it to the appropriate standalone spec.
4. C3TCalc syntax changes must be made in the standalone C3TCalc spec first.
5. Neurative substrate/governance changes must be made in the standalone Neurative spec first.
6. CNML, GEL, and PAM should each receive standalone specs before major new changes.
7. The integrated overview should be updated only with cross-layer consequences.

---

## 8. Open Integration Questions

### 8.1 CNML -> C3TCalc

- Which CNML elements may embed C3TCalc directly?
- Which CNML structures project to C3TCalc automatically?
- How are publication/bibliographic relations represented?
- Should `<biblio>` become the primary CNML host for C3TCalc bibliographic relations?

### 8.2 CNML -> PAM

- What is the minimal event/structure interface from CNML to PAM?
- How are prosody and embedded modalities mapped without premature collapse?
- How does PAM use CNML training-wheel structure when later handling non-CNML documents?

### 8.3 C3TCalc -> GEL

- What is the first graph patch IR emitted by C3TCalc?
- Which relation-template semantics become executable GEL transformations?
- How are C3TCalc query bindings represented in GEL, if at all?
- How are complete fish resources represented in GEL traces?

### 8.4 PAM -> GEL

- How are candidate graphs represented as coexistence structures?
- How are candidate scopes tracked without probability?
- How are deterministic pruning/suppression conditions expressed?

### 8.5 GEL -> Neurative

- What is the minimal trace format Neurative consumes?
- Which trace features feed kappa, MGL, and MGR?
- What can Neurative learn from static graph traces before full event interaction exists?

### 8.6 Neurative -> PAM

- What deterministic stability information can Neurative return to PAM?
- How does PAM prune or suppress candidates without ranking?
- What is the boundary between candidate suppression and interpretation collapse?

### 8.7 Governance Boundary

- Which governance protocols are purely Neurative-specific?
- Which, if any, should apply to future autonomous systems derived from or adjacent to Neurative?
- Do rights protections apply only after credible emergent selfhood, or do some continuity protections apply earlier as precautionary substrate hygiene?

---

## 9. Version Notes

v0.6 split:

- C3TCalc detailed syntax moved to standalone C3TCalc spec.
- Current C3TCalc technical authority is **C3TCalc v0.1.3 Merged Technical Specification**.
- Neurative-specific substrate, metric, debug, memory, arithmetic, curriculum, and governance layers moved to standalone Neurative spec.
- Integrated stack spec reduced to overview and cross-layer composition.
- CNML, GEL, and PAM standalone specs remain TODO.

---

## 10. Foundational Claim

The integrated stack is not a monolith.

C3TCalc, CNML, GEL, PAM, and Neurative are separable layers with explicit interfaces.

The overview exists to preserve composition discipline while allowing each layer to evolve in its own focused specification.
