# ThreadPak acceptance witness

This document is the frozen acceptance contract for the ThreadPak architecture. It realizes the Acceptance section of `ARCHITECTURE.md` in full detail and derives from the rulings accepted 2026-08-24. It states what must be **proved** and what would **falsify** each proof. Nothing in this document is claimed proven; a sentence here is a demand on evidence, never a report of it.

## The acceptance theorem

```text
ThreadPakArchitectureAccepted
=
WitnessA ∧ WitnessB

WitnessA
=
A1 ∧ A2 ∧ A3 ∧ A4 ∧ A5 ∧ A6
```

Witness A proves the complete local machine. Witness B proves authority succession and scale-out. Both are required; neither substitutes for the other, and Witness A holding alone is never grounds to treat partitioning as "probably fine."

Witness A is proved through ordered construction cuts A1–A6. The cuts are construction order, not product phases, release slices, or temporary architectures. Only after A6 holds does Witness A hold.

## The standing rule

Construction order does not weaken acceptance. Each cut uses final semantic roles, and no earlier cut is presented as a complete ThreadPak product or a compatibility promise.

Accordingly, in every cut:

- no temporary identities;
- no temporary bytes called canonical;
- no generic status wrapper;
- no API introduced with the intention of replacing it later.

The freeze that precedes the cuts consists of this contract together with the owner contracts — each owner's README law, its complete `types.rs` role graph, its thin semantic operation signatures, its profile and depot algebra with the initially selected rows, its exact crossings, and its refusal families. These close in the A0 contract-closure pass, and no cut begins before its owner's closure stands. The freeze proves nothing and claims nothing green; it closes what the witnesses must prove and what the hostile denominator contains.

Where this document names a milestone, identity, or refusal, the exact public Rust spelling binds in the owning contract's type pass; the semantic role named here is the law the spelling must realize.

---

## Witness A

### A1 — Accepted-history kernel

**Proves:** one admitted fact can become accepted history honestly.

**Exercises:** the event owner contract (coordinates, accepted history, authority regions); the port owner contract for durable publication as declared by event; depot profiles for paved limits.

**Scope:**

```text
typed Coordinate
ReferenceFrame
AuthorityRegion
AuthorityEpoch
EventProposal
ExpectedCut
ordinary event admission
AuthoritySequence
CommitPoint
exact prefix read
role-specific AppendReceipt
```

**Required falsifiers:**

```text
wrong frame
wrong coordinate role
stale expected Cut
stale epoch
wrong authority
publication failure
reopen after crash
```

**Non-claim:** A1 does not prove ThreadPak's computation, runtime, effects, ingress, or recovery thesis.

### A2 — The divided highway and late materialization

**Proves:** acceleration cannot dethrone authority.

**Exercises:** the view owner contract (query, Fix, View maintenance, temporal monitor, SelectionMask, materialization, DataBlock); the event owner contract as the source of exact Cuts; depot profiles.

**Scope:**

```text
pull reference query at exact Cut
push incremental advance from admitted delta
same-claim, same-cut parity
one bounded K3 × C3 temporal monitor
SelectionMask over one exact RowDomain
candidate acceleration
exact verification
late materialization
rebuild from accepted history
```

**Required falsifiers:**

```text
push and pull disagree
stale AppliedCut
wrong RowDomain with equal cardinality
approximate candidate claims truth
forbidden field materialized before authorization
corrupt materialization treated as accepted history
```

**Non-claim:** A2 does not prove program execution, effects, or durable recovery.

### A3 — Program, knowledge honesty, the image gate, and PakVM

**Proves:** executable meaning — not physical effects.

**Exercises:** the program owner contract (transition, knowledge bindings, descriptor, lowering law, image gate); the runtime owner contract for PakVM closed-value stepping; the event owner contract for frozen input Cuts.

**Scope:**

```text
ordinary Rust Program
pure bounded transition
knowledge bindings over application-owned estimates:
    ModelBinding
    Assumption
    Dependence
    Calibration
    EvidenceRequirement
production lowering
independent lowering agreement
ExecutableProgramImage
PakVM closed-value stepping
no ambient host authority
```

The lowering gate obeys the one-owner, two-roads law: Program owns the Semantic-to-Execution lowering law and consumes its agreement result, but the production lowerer and the independent agreement route must not share load-bearing lowering or verdict logic. `DisagreementEstablished` (an independently checked relation does not hold) and `AgreementNotEstablished` (the required independent route could not establish the relation) remain distinct outcomes. Every locally or externally produced image crosses the same agreement gate; a locally built image receives no shortcut.

**Required falsifiers:**

```text
Semantic Form / Execution Form disagreement
shared lowerer/checker defect
unbounded recursive path
missing effect closure
host callback hidden inside program
semantic work omitted
foreign image bypasses agreement
```

**Non-claim:** A3 does not prove any external effect, Attempt behavior, or recovery.

### A4 — Ingress membrane

**Proves:** foreign input crosses honestly without creating a second event authority.

**Exercises:** the event owner contract's ingress family (claim custody, Reserve, the acknowledgment ladder, rejected-content disposition) and its ordinary event admission; the port owner contract for the quarantine crossing; depot profiles for reservation limits.

**Scope — both lawful intake compositions:**

```text
ClaimFirst
DomainFirst
stable ClientNonce
idempotent Reserve
reservation recovery
terminal-only ack projection
progressive ack projection
typed redacted rejection evidence
optional protected quarantine
```

The terminal milestone is object-specific: the claim-admission receipt (`ClaimAdmissionReceipt`) discharges a ClaimFirst submission's retry duty; the domain-admission receipt — bound to the submission's idempotency identity and the accepted event's publication — discharges a DomainFirst submission's retry duty. No earlier progress stage discharges retry, and the exposure projection (terminal-only versus progressive) is interface-selected with no ThreadPak-wide default. Domain-fact admission is owned solely by event; ingress records claim custody and claim resolution and owns no second domain admission primitive.

**Required falsifiers:**

```text
lost acknowledgement at every exposed stage
lost reservation-token response
duplicate Reserve
same nonce with conflicting intent
quota exhaustion
crash after claim admission
raw rejected content escaping custody
unkeyed rejection digest becoming an oracle
```

The quota falsifier must show typed capacity refusal without silent eviction: an implementation may not evict an unexpired reservation and then accept a semantically ambiguous duplicate, and token-usability expiry must remain distinct from the duplicate-recognition horizon.

**Non-claim:** A4 does not prove external effects or durable process recovery.

### A5 — Effect membrane and physical Attempts

**Proves:** honest contact with external reality.

**Exercises:** the runtime owner contract (Turn, EffectIntent admission, reconciliation) and its Bvisor family (AdmissionPlan, reservation custody, Attempt, AttemptReport); the port owner contract (typed request/response, recovery posture); the program owner contract as the origin of EffectProposals; the event owner contract for observations re-entering through ordinary admission.

**Scope:**

```text
EffectProposal
EffectIntent admission
Turn
Bvisor AdmissionPlan
fresh Attempt
typed PortRequest
same-Attempt response
timeout
disconnect
observer abandonment
OutcomeUnknown
late response
pure reconciliation
```

**Required falsifiers:**

```text
late response resumes fresh Attempt
disconnect becomes cancellation
timeout becomes nonexecution
retry reuses AttemptId
reservation leaks after failed admission
physical success becomes domain success
reconciliation rewrites an earlier Attempt
```

**Non-claim:** A5 does not prove durable continuity, checkpoint recovery, or driver invariance.

### A6 — Durable continuity and the whole loop

**Proves:** the complete local machine closes.

**Exercises:** the runtime owner contract (checkpoint authority, its accepted checkpoint-advance records in runtime-owned checkpoint authority regions, their publication family, delivery, drivers, replay, reconciliation obligations); the view owner contract for pull recovery; the program owner contract for the knowledge-acquisition loop; the port owner contract for PEND acquisition.

**Scope:**

```text
accepted checkpoint-advance record
derived current checkpoint
lost wake
duplicate wake
pull recovery from exact Cut
same logical Turn replayed through fresh Attempt
two physical drivers
same logical trace

knowledge-acquisition loop:
    Defer(EvidenceRequirement)
    → REQUEST or PEND
    → foreign observation
    → ordinary admission at new Cut
    → conditioned estimate
    → final decision

progressive explanation:
    concise description
    typed semantic signature
    structured explanation
    complete definitional expansion
```

**Required falsifiers:**

```text
checkpoint advances before prerequisites
output commits while checkpoint lags
wake delivery impersonates progress
checkpoint forgets reconciliation obligation
driver schedule changes semantic work
replay changes logical result
explanation cites evidence not used by evaluation
```

The output-commits-while-checkpoint-lags falsifier must show the lawful gap: the committed output remains real, replay plus idempotency and reconciliation handle the duplication, and the checkpoint never lies that work may be skipped. The inverse — a checkpoint advanced without its required output commitments — must be unrepresentable or refused at admission.

Only after A6 does Witness A hold.

---

## Witness B — Authority succession and scale-out

**Proves:** write scale by splitting authority, with nothing lost, re-minted, or silently reordered.

**Exercises:** the event owner contract (authority regions, epochs, partition, lineage, cut succession); the view owner contract for query and View parity across succession; the runtime owner contract for checkpoint and recovery meaning across handoff.

**Must prove:**

```text
seal parent authority at exact Cut

child regions:
    pairwise disjoint
    union equals parent
    no gap
    no overlap

fresh child epochs activated

routing published last

preserved across succession:
    EventIds
    original AuthoritySequences
    frame and coordinate meaning
    causation
    accepted prefix

only fresh child suffixes appended

explicit cut-succession evidence binding
parent and child authority generations

query result and View parity
across parent-to-child succession

checkpoint and recovery meaning
across lawful handoff

stale parent remains reachable
without retaining write authority
```

**Required falsifiers — each attack must refuse:**

```text
overlapping child regions offered for activation
child set leaving a coverage gap
child re-minting an inherited event or position
routing published before child activation
parent accepting a write after seal
parent cut compared to child cut without a succession witness
checkpoint silently rebased across handoff without succession evidence
```

**Non-claim:** Witness B does not prove federation beyond succession — cross-authority read postures and coordinated snapshots carry their own contracts and are not certified by a split.

---

## Comparison law

Acceptance evidence compares **logical traces**: accepted events, semantic results and refusals, Turn identities, EffectIntent identities, declared result order, semantic work, checkpoint consequences, reconciliation meaning, and explanation meaning.

Physical traces lawfully differ: fresh AttemptIds, wall-clock durations, scheduling order, wake timing, batch sizes, worker placement, and physical resource observations are not part of the logical trace and must not be demanded to match.

An operation escapes trace invariance only by explicitly declaring schedule-sensitive semantics in its owning contract. Absent that declaration, a driver or schedule change that alters the logical trace is a falsification, not an implementation detail.

## Escalations

None. Every requirement above carries or derives from rulings already accepted; no new closure requiring an explicit mint was encountered while freezing this contract.
