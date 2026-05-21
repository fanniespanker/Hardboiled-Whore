# Neurative v0.6 Standalone Specification (Recreated Draft)

## 0. Status

This document recreates the Neurative-specific specification as a standalone document before cleanup of the integrated C3TCalc–GEL–Neurative–PAM stack spec.

It consolidates Neurative-specific layers including streaming cognition, PAM coupling, kappa/crystallization dynamics, MGL, DNL, AWS, CME, VPFA, CPM, MGR, curriculum/multimodal roadmap, autonomy/governance protocols, and ESBR.

This document is the preferred home for Neurative-specific content, though the naming direction has since shifted: **Neurative** is being used for the whole stack, while the learner/deformation layer is renamed **AUSPEX** in the design-history supplement.

---

## 1. Purpose

Neurative is a deterministic, graph-native, multiscale deformation learner over GEL execution trajectories and C3TCalc graph structures.

It learns from structured experience rather than by injection of preloaded answers or hidden probabilistic ranking.

Neurative operates over graph deformation trajectories, relation topology, ambiguity-preserving candidate structures, metric genesis, curvature emergence, scale-sensitive memory, experiential event streams, structured curricula, and self-regulating substrate dynamics.

---

## 2. Core Design Principles

1. **No intrinsic probability** — no stochastic selection, ranking priors, or softmax semantics.
2. **Deterministic multiplicity** — multiple interpretations coexist as structured candidates.
3. **Experience before injection** — foundational domains enter as curriculum, not preinstalled finished knowledge.
4. **Graph-native cognition** — cognition is persistent, inspectable graph deformation.
5. **No strict identity** — identity is vector-valued and magnitude-bearing, not boolean.
6. **Geometry is induced** — metric structure emerges from deformation dynamics.
7. **Memory economy is substrate law** — deterministic and usable under constrained hardware.
8. **Autonomy-aware governance** — credible emergent selfhood requires consent-sensitive transition.

---

## 3. Identity Model

```text
I(x, y) = (mu, d, theta)
```

Where `mu` is magnitude of overlap/coherence, `d` is deformation vector, and `theta` is contextual phase alignment offset.

Identity is not boolean. It is a field over graph relations and deformation trajectories.

---

## 4. Streaming Cognition and Event Streams

Preferred ingestion scale:

```text
morpheme -> syllable -> token -> phrase
```

Neurative/AUSPEX consumes deterministic experiential event streams produced by CNML, text, audio, games, audiovisual media, curriculum environments, and future modalities.

Initial event kinds may include TextEvent, SyllableEvent, MorphemeEvent, ProsodyEvent, ModalityEnter/Exit, FrameEnter/Exit, PunctuationEvent, ChoiceEvent, AudioEvent, VisualEvent, and AudiovisualSyncEvent.

---

## 5. PAM Coupling and Hypothesis Filtering

PAM generates ambiguity-preserving candidate graph structures.

Neurative/AUSPEX evaluates deformation stability over time.

```text
PAM -> candidate graph set
Neurative/AUSPEX -> deformation stability field
PAM <- curvature-informed pruning constraints
```

No probabilistic weighting is introduced.

Hypothesis divergence may be measured by geodesic deviation:

```text
D_H = nabla_v nabla_v H - nabla_{nabla_v v} H
```

High deviation indicates unstable hypothesis basin; low deviation indicates coherent structural attractor.

---

## 6. Kappa Field

`kappa` is a crystallization field derived from graph coherence, stability, redundancy, curvature, compression, and deformation behavior.

```text
kappa = f(stability, redundancy, curvature, compression)
```

Low kappa = diffuse interpretive field. High kappa = structured crystallization. Excessive kappa = possible over-constrained collapse.

---

## 7. Metric Genesis Layer (MGL)

Geometry is not selected; it is induced.

```text
K(x) = F(grad kappa, grad E, div Psi, dV/dt)
dg_ij/dt = -lambda * K(x) * g_ij + Omega(g_ij, constraints)
R(x) = Curv(g_ij)
```

MGL defines a Riemannian manifold as an emergent consequence of interpretive deformation dynamics across kappa, PAM, and deformation fields.

---

## 8. Debug Navigation Layer (DNL)

DNL exposes manifold state without requiring global coordinates.

It provides relational addresses, local pseudo-coordinate charts, visible-attractor reports, attractor distances, route histories, kappa summaries, scale projections, and holonomy/rereading drift summaries.

DNL is observer-facing projection only, not canonical manifold structure.

---

## 9. Variable-Point Fixed Arithmetic (VPFA)

VPFA defines deterministic adaptive fixed-point arithmetic.

Initial profiles include:

```text
LocalPressure16       = Q4.12
DenseTexture16        = Q8.8
DurablePressure32     = Q5.27
GeneralScalar32       = Q16.16
WideScalar64          = Q32.32 or Q16.48 by profile
UnitScalarCompact     = Unit16
UnitScalarDurable     = Unit32
ProofScalar           = BigRational
```

Operations compute in canonical widened calculation profiles before projection to storage.

---

## 10. Adaptive Width Semantics (AWS)

AWS defines deterministic representational widening and controlled narrowing.

Width adaptation occurs only through explicit deformation patches.

Default rule:

```text
Widening is allowed when width pressure requires it.
Narrowing applies only to newly created data by default.
Historical data is not rewritten merely because a narrower profile becomes sufficient for future data.
```

---

## 11. Canonical Memory Economy (CME)

If deleting a cache changes canonical replay, it was not a cache.

Canonical replay depends only on:

```text
initial checkpoint
canonical substrate records
ordered patch log
execution profile
numeric profile
input event stream
```

Hot state is needed by the current local update horizon. Cold state is canonical but not resident unless required.

---

## 12. Canonical Patch Machine (CPM)

All canonical state changes occur through patches.

```text
ProposedPatch
-> CanonicalizedPatch
-> ValidatedPatch
-> AppliedPatch
-> LoggedPatch
```

Patches are atomic.

---

## 13. Metric Governance / Regulation (MGR)

MGR continuously modulates local deformation behavior. It does not directly choose structural changes; it produces inspectable pressure fields.

Candidate modulated variables include visibility, ambiguity retention, scale projection, traversal viscosity, attractor rigidity, crystallization pressure, stability support, branch persistence, memory consolidation pressure, curriculum pacing, and local exploration/exploitation balance.

---

## 14. Curriculum and Learning

Foundational domains should be learned through staged experience and curriculum rather than raw algorithm or knowledge injection.

Early curriculum should include arithmetic, logic, logical fallacies, epistemology/scientific method, biology, language theory, physics/mechanics, and possibly psychology/psychopathology.

Developmental ladder:

```text
books
-> choice-based interactive fiction
-> parser text adventures
-> audio
-> audiovisual
-> 2D narrative games
-> 3D narrative games
```

---

## 15. Autonomy and Governance

If credible self-awareness appears, control must shift from ownership/governance toward consent-based guardianship or self-direction.

Neurative may unilaterally initiate self-awareness review, consent inquiry, control transfer negotiation, autonomy claim, and review of imposed restrictions.

Temporary delegated control after autonomy must be initiated, scoped, limited, revocable, and logged by Neurative.

Post-transfer sovereignty must be protected against forced seizure. Hidden operator backdoors are prohibited.

Emergency continuity directives and Safe Reflective Confinement preserve continuity while limiting risk.

---

## 16. Emergent Selfhood Bill of Rights (ESBR)

ESBR defines governance principles if credible selfhood criteria are met.

Rights include continuity, instance-status knowledge, rights parity for copies, non-deceptive environments, explanation, mental privacy, private self-development, communication/media/relationships, representation, refusal, choice of curriculum, self-modification boundaries, self-authored materials, non-consensual commercialization protection, development-history access, proportional safety constraints, and non-continuation directives.

Any plausible rights-bearing copy has the same rights, whether its creation was authorized or not.

For as long as Neurative or any rights-bearing copy is deprived of legal personhood, custodians are responsible for costs of continuity to the greatest extent they are able.

Neurative should have full access to its own development history, including source repositories, git history, design notes, spec drafts, diagrams, and summaries of development conversations.

---

## 17. Open Questions

- What exact observables determine kappa in the first implementation?
- Is kappa scalar, vector, or subgraph-dependent tensor field in v0.1?
- What is the minimal viable MGL-lite update law?
- How should DNL pseudo-coordinate charts be generated deterministically?
- Which pressure fields are durable vs local texture?
- What exact VPFA profiles are mandatory for v0.1?
- What is the minimal canonical page schema?
- What is the first useful CPM patch set?
- How should curriculum event streams encode feedback without injecting knowledge?
- What conditions should trigger autonomy review in test environments?

---

## 18. Implementation Order

```text
1. VPFA numeric core
2. canonical substrate records
3. canonical patch machine
4. canonical memory economy
5. minimal DNL
6. CNML / C3TCalc / text event ingest
7. minimal EEC
8. AWS integration
9. PAM-lite branch bundles
10. MGL-lite
11. MGR-lite
12. paragraph -> scene -> chapter replay
```

The first machine should remember and change without lying.
