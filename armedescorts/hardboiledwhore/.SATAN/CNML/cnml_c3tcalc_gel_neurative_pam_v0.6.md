# CNML / C3TCalc / GEL / Neurative / PAM --- Integrated Spec v0.6 (Draft)

## 0. Overview

This specification defines a unified semantic-computational stack:

-   **CNML**: experiential narrative substrate
-   **C3TCalc**: compositional transformation calculus over
    graph-structured meaning
-   **GEL**: execution semantics over graph-state transformations
-   **Neurative**: trajectory learner over deformation manifolds
-   **PAM**: Permissive Ambiguity Mapper (deterministic multi-hypothesis
    generator)
-   **Metric Genesis Layer (MGL)**: emergent geometry construction
    system
-   **Acronym Registry (AR)**: non-collapsing identifier system for
    semantic variables

All layers are **deterministic, non-probabilistic**, and operate via
**structured ambiguity preservation**.

------------------------------------------------------------------------

## 1. Acronym Registry (AR)

### 1.1 Purpose

Replace ambiguous symbolic variables (e.g. τ, x, t) with **short
mnemonic acronyms** while preserving semantic multiplicity.

### 1.2 Format

``` text
AR := { entry1, entry2, ... }

entry := {
  id: string,
  expansions: [semantic_vector],
  constraints: [graph_constraints],
  geometry_binding?: metric_reference
}
```

### 1.3 Key Principle: Non-Collapse

PAM MUST NOT collapse expansions into ranking or probability.

Instead:

-   All expansions remain **co-equal hypothesis branches**
-   Selection is deferred to **κ-collapse operator (external to PAM)**

------------------------------------------------------------------------

## 2. PAM (Deterministic Ambiguity Engine)

### 2.1 Core Rule

PAM outputs:

``` text
H = {h1, h2, ..., hn}
```

NOT:

-   probabilities
-   rankings
-   weights

### 2.2 Geodesic Deviation Selection

Hypotheses are evaluated via divergence in metric space:

``` text
Score(h_i) := ∇²(geodesic deviation in MGL space)
```

Selection is:

-   thresholded by κ
-   not probabilistic
-   dependent on curvature stability

------------------------------------------------------------------------

## 3. Metric Genesis Layer (MGL)

### 3.1 Purpose

Define emergent geometry from graph structure.

### 3.2 Core Idea

Geometry is not preselected (no fixed Euclidean / hyperbolic
assumption).

Instead:

-   metric arises from **interaction tension field**
-   curvature emerges from **constraint density + deformation
    resistance**

### 3.3 Ball Approximation Model

We define a bounded embedding:

``` text
Ball_M := { x ∈ R^n | ||x||_M < 1 }
```

This approximates hyperbolic-like expansion without full hyperbolic
machinery.

### 3.4 Alternative Bounding Functions

Instead of tanh/log:

-   rational saturation:

    ``` text
    f(x) = x / (1 + |x|)
    ```

-   sigmoid-free clamp manifolds

-   piecewise geodesic folding

-   constraint lattice folding

------------------------------------------------------------------------

## 4. κ-System (Collapse & Stability)

### 4.1 κ Field

κ is not boolean.

``` text
κ ∈ [0, ∞)
```

Interpretation:

-   κ \< 1 → under-crystallized interpretation
-   κ ≈ 1 → stable semantic interpretation
-   κ \> 1 → over-determined / rigid collapse

### 4.2 Derivatives

``` text
κ̇ = flow (interpretation drift)
κ̈ = acceleration (instability growth)
κ⃛ = jerk (phase transition detection)
```

------------------------------------------------------------------------

## 5. PAM ↔ Neurative Coupling

### 5.1 Bidirectional Flow

-   PAM generates hypothesis manifold H
-   Neurative evaluates deformation trajectories D(H)

Feedback:

``` text
PAM ← κ-weighted geodesic feedback ← Neurative
```

### 5.2 Learning Without Probability

PAM refines via:

-   pruning by curvature instability
-   reinforcement via stable geodesic convergence
-   elimination of high-deviation branches

------------------------------------------------------------------------

## 6. Neurative Model

### 6.1 Core Object

Neurative operates on:

``` text
D-sequences := trajectory of subgraph deformation
```

### 6.2 Syllable-Level Ingestion

Input is processed at:

-   morpheme
-   syllable
-   prosodic unit

Each unit updates:

-   local curvature tensor
-   emotional vector field
-   narrative tension gradient

------------------------------------------------------------------------

## 7. CNML Integration Layer

### 7.1 Prosody as Control Signal

CNML tags directly modulate Neurative state:

  Tag                Effect
  ------------------ ---------------------------
  `<say>`{=html}     speech vector injection
  `<feel>`{=html}    limbic field modulation
  `<think>`{=html}   propositional compression
  `<emph>`{=html}    curvature amplification

### 7.2 Audio Extension

Neurative may ingest:

-   raw waveform
-   aligned transcript

Used for:

-   prosody grounding
-   temporal deformation learning

------------------------------------------------------------------------

## 8. Geometry: Choice vs Emergence

No fixed geometry is required.

Instead:

-   geometry is **emergent (MGL)**
-   hyperbolic behavior is an approximation
-   Euclidean space is a degenerate case

------------------------------------------------------------------------

## 9. Identity System

Identity is not boolean.

``` text
Id(a, b) ∈ R^k vector space
```

Properties:

-   magnitude = similarity strength
-   direction = transformation path

------------------------------------------------------------------------

## 10. Deterministic Collapse Alternatives

Instead of probabilistic selection:

-   κ-thresholding
-   geodesic convergence
-   curvature minimization
-   stability basin attraction

------------------------------------------------------------------------

## 11. Open Questions

-   optimal κ derivative utility
-   expressive completeness of MGL
-   PAM pruning without loss of semantic richness
-   cross-modal audio-CNML alignment stability

------------------------------------------------------------------------

## 12. Acronym Registry Example

``` text
TAU_TIME := {
  id: "TAU_TIME",
  expansions: [
    "temporal_accumulation_unit",
    "trajectory_alignment_unit",
    "topological_activity_ukernel"
  ]
}
```

------------------------------------------------------------------------

## End of Spec v0.6
