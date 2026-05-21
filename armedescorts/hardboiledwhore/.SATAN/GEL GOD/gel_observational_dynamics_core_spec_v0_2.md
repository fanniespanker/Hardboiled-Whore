# GEL Observational Dynamics Core Spec (Draft)

Version: 0.2-preformal
Status: unstable / semantically active
Scope: core substrate + observational dynamics only

---

## 0. Foundational Assumptions

- No primitive identity.
- No globally accessible state.
- No privileged observational scale.
- Operators deterministic.
- Semantics emergent.
- Persistence observationally derived.
- Local continuity not guaranteed.
- Global coherence bias assumed.
- Universal graph reconstruction considered intractable.

Undefined / partially defined:
- exact topology of G
- dimensionality semantics of τ
- exact rewrite algebra
- constraint language ℛ
- ontology encoding Σ
- scale parameterization s
- persistence operator Ψ
- continuity metric definitions
- evaluator/policy formalism

---

## 1. Substrate

Universal graph substrate:

    G ∈ 𝒢

Notes:
- G treated as latent/inaccessible at universal scale.
- Higher layers SHOULD operate on projections/invariants, not full graph state.
- "Node" and "edge" semantics remain implementation-relative.

---

## 2. Primitive Dynamical Fields

### 2.1 Temporal Field

    τ : G → ℝⁿ

Semantics:
- temporal perception geometry
- perceptual resolution
- maturity/crystallization scale
- nonuniform over G

Properties:
- continuous where possible
- globally coherence-biased
- local smoothness not guaranteed
- n implementation-defined and potentially variable

Open questions:
- whether τ-space is vector, tensor, quaternionic, polyernionic, etc.
- whether n dynamically scales with graph locality/state

---

### 2.2 Structural Deformation Field

    ω = ∇ᴳτ

Semantics:
- local graph deformation
- change propagation
- tension/evolution pressure

Undefined:
- exact graph differential operator
- whether ω is local/global/multiscale
- admissible discontinuities

---

## 3. Derived Invariant Fields

Derived fields are:
- compressive
- observational
- nonprimitive
- intended to stabilize across scale

---

### 3.1 Reinforcement / Coherence

    Aᵢ = τᵢ · ωᵢ

Semantics:
- aligned coevolution
- reinforcement pressure
- coherent adaptation

Interpretation:
- energy-like projection
- alignment observable

---

### 3.2 Misalignment / Novelty

    Bᵢ = ||τᵢ × ωᵢ||

Semantics:
- directional conflict
- surprise
- rupture/novelty pressure

Interpretation:
- orthogonal deformation observable

Notes:
- assumes cross-product-compatible structure
- generalized exterior products may replace ×

---

### 3.3 Persistence / Inertia

    Mᵢ = Ψ(Δτᵢ, Δωᵢ, Aᵢ, Bᵢ, Cᵢ)

Semantics:
- resistance to meaningful deformation
- persistence under rewrite
- crystallized coherence

NOT:
- primitive mass
- explicit identity
- static weight

Interpretation:
- canonicalized persistence compression

Undefined:
- Ψ
- temporal integration windows
- persistence thresholds

---

### 3.4 Constraint Field

    Cᵢ = 𝒱(Gᵢ, ℛ)

Semantics:
- rewrite legality
- local validity
- invariant preservation pressure

Undefined:
- rule language ℛ
- validation algebra
- conflict resolution semantics

---

## 4. Observable Basis

Observable manifold:

    𝔽ₛ = (τ, ω, A, B, M, C)ₛ

Meaning:
- higher layers consume observables
- not full graph state

State therefore becomes:
- reconstructed
- projected
- scale-relative
- noncanonical

---

## 5. Equivalence

Identity not primitive.

Scale-dependent equivalence:

    x ~ₛ y  ⟺  φₛ(x) = φₛ(y)

Semantics:
- observational indistinguishability
- projection-relative sameness

Properties:
- no globally privileged equivalence
- equivalence may vary by scale/context

Undefined:
- φₛ
- scale topology
- equivalence closure semantics

---

## 6. Substitutability

Substitutability ≠ equivalence.

Constraint-preserving substitution:

    x ≈ₛ y  ⟺  𝒞(x → y) = 0

Semantics:
- operational interchangeability
- safe rewrite replacement
- context-sensitive equivalence

Interpretation:
- equivalence filtered through constraints

Undefined:
- 𝒞
- replacement locality semantics
- acceptable degradation metrics

---

## 7. Ontology Layer

Ontological projection:

    𝒪 : 𝔽ₛ → Σ

Semantics:
- semantic regime classification
- trajectory compression
- pattern stabilization

Restrictions:
- SHOULD NOT assume primitive identity
- SHOULD NOT require full graph state
- SHOULD NOT directly mutate G

Undefined:
- ontology representation
- semantic regime topology
- learning/update rules

---

## 8. Decision Layer

Decision operator:

    D = Ψ(𝒪(𝔽ₛ), 𝒞)

Semantics:
- rewrite authorization
- policy projection
- action selection

Properties:
- decisions exist above dynamics
- decisions consume observables/ontology

Undefined:
- evaluator semantics
- optimization criteria
- conflict arbitration

---

## 9. Rewrite Semantics

Structural evolution:

    Gₜ₊₁ = 𝒮(Gₜ, Dₜ)

Interpretation:
- graph rewritten through transformations
- persistence is emergent, not assumed

Undefined:
- rewrite operator algebra
- transaction semantics
- concurrency/conflict handling
- reversibility guarantees

---

## 10. Architectural Flow

    G
      → (τ, ω)
      → (A, B, M, C)
      → 𝔽ₛ
      → ~ₛ
      → 𝒪
      → D
      → 𝒮
      → Gₜ₊₁

---

## 11. Compression Principle

System does NOT attempt:
- total graph reconstruction
- universal state materialization

System instead operates through:
- stable projections
- local observables
- invariant compression
- scale-relative reconstruction

---

## 12. Open Structural Questions

Unresolved:
- whether τ is fundamentally continuous
- whether τ dimensionality is dynamic
- whether observational scales are continuous/discrete
- whether equivalence is locally or globally composable
- whether ontology coevolves with G
- whether rewrite legality emerges or is explicitly encoded
- whether field basis is minimal
- whether A/B should remain orthogonal or dynamically coupled
- whether graph differential geometry should be formalized categorically
