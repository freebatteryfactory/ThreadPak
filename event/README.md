# Event

This document is the owner contract for the `event` home.
It states product law: what ThreadPak defines.
No sentence here claims current implementation support.
It stays consistent with `ARCHITECTURE.md`; where the two could ever disagree, that is a defect to fix, never a fork to interpret.

The home's question:

> **What fact became accepted, at which semantic address, under which authority, in what exact authority-local order, and through which durable cut?**

This home is the center of the machine.
Accepted event history owns domain fact (ARCHITECTURE.md §What owns fact).
Everything else in ThreadPak either feeds this home through admission, derives from it at exact Cuts, or is a role-specific durable record of another owner — runtime checkpoint advances, admitted EffectIntents, Attempt evidence — which owns exactly its own fact and references this home's Cuts without inhabiting them.

## Co-seated semantic owners

Per ARCHITECTURE.md §Owners are not directories, one dependency home may seat several semantic owners.
This home seats seven, each with its own question, types, operations, and refusals:

| Owner | Unique question |
| --- | --- |
| Coordinate | Where does a semantic fact live, and what operations are lawful on that location or relation? |
| Accepted history | What fact became accepted, under which authority, in what exact order, through which durable cut? |
| Chronology | What chronology may this store lawfully accept? |
| Causation | Which accepted facts stand in which explicit, typed relationships? |
| Partition | How does write authority over a region lawfully begin, seal, split, and succeed? |
| Removal | How is one exact destructive operation lawfully authorized, performed, and evidenced? |
| Ingress | What custody and semantic status does this foreign submission have, and which exact boundary may the sender rely upon? |

Co-seating is not soup: each owner's types and operations stay separately named, and none answers another's question.

---

## 1. Coordinate

An accepted fact lives at a typed **Coordinate** inside a named **ReferenceFrame** with an explicit **FrameVersion**.
A Coordinate is an admission address — where a fact entered semantic space.
It is never current state: a derived `Fix` (view owner) answers where the evidence places the application now.
The two are different coordinate roles and never substitute.

**Axis capability law.** Each Axis declares which operations are lawful on it: equality, total order, partial order, hierarchy, intervals, sets, typed relationships, metric distance under a named profile, qualified approximation.
Nothing infers an undeclared operation from the Rust representation — a string-displayed axis acquires no lexicographic semantic order; a graph library computing a number grants no causal distance meaning.

**Frame transformation law.** A transformation between frames declares: source and target frame versions, domain, multiplicity, loss, exactness, reversibility, authority posture, and work and expansion bounds.
Historical facts stay bound to the frame under which they were admitted; a newer frame may reinterpret or migrate explicitly, never rewrite old meaning by pretending the coordinate system was always different.

**The three storage layers** (never conflated): every inhabited semantic address exposes a logical journal view over accepted history; write authority belongs to one bounded AuthorityRegion for one AuthorityEpoch; physical stores host regions and their material without becoming semantic location or authority.

**The overlap law.** Query regions may overlap arbitrarily.
Live write-authority regions for one authority family and epoch are disjoint — no overlap, ever.
One accepted event may participate in several typed views, memberships, and relations without duplication or re-admission.

*Chronology: carries the settled coordinate rulings (owner-endorsed direction, carried); overlap and three-layer law derive from ARCHITECTURE.md §The rails (1, 3, 10).*

## 2. Accepted history

**Roles.** `EventProposal` is a proposed fact with no authority.
`AcceptedEvent` is an immutable fact admitted into history.
`EventId` identifies the accepted fact.
`EventCommitment` is the canonical-byte commitment the identity derives from.
None of Command, Observation, EffectIntent, Receipt, or View is an event (ARCHITECTURE.md §What owns fact).

**Admission.** Event admission takes an `EventProposal`, the caller's `ExpectedCut`, an `AppendGrant`, and the current region epoch.
A stale expected Cut refuses before publication — an adapter may not silently rebase and retry against a newer prefix, because that changes the operation's meaning.
Aborted staging leaves no visible gap: a bounded accepted batch occupies one contiguous membership under its region authority.

**The four-object publication split.** Event body, accepted record, batch publication, and the `CommitPoint` receipt are four objects.
Commit knowledge rides its own knowledge axis, never a field on the event — an event cannot contain the CommitPoint that only exists after publishing it.

**Order.** `AuthoritySequence` is the exact accepted order under one writer authority.
It is scope-bound: the scope lives in the value's own canonical bytes, there is no derived ordering across scopes, and comparison is lawful only within one proven scope.
Any change to the ordering interpretation that would make old and new values incomparable mints a new authority generation.
There is no global sequence anywhere (ARCHITECTURE.md §The rails, 10).

**Single writer, correctly read.** One writer per region per epoch means one active semantic authority may extend the accepted order — not one thread, task, process, or machine.
Preparation, validation, reads, and derived work may all proceed concurrently; the one serialized boundary is accepted publication.

**Reserved event classes.** Public writers cannot mint internal event classes; authority is validated before frame construction.

**Federation.** A `SourceSet` names the exact participating authorities, regions, and epochs, in canonical representation order — changing any participant creates a different SourceSet.
A `FederationCut` maps every source to one exact local CommitPoint.
Compatible cuts compare as equal, advancing, preceding, concurrent, or incompatible.
A componentwise join is a knowledge summary; it never proves the component cuts coexisted, formed a snapshot, or committed atomically.

*Chronology: carries ARCHITECTURE.md §The rails (1, 3, 10) and the settled accepted-history rulings; the FederationCut nonclaim carries the owner-ruled cut-vector law.*

## 3. Chronology

Chronology is admitted, never trusted.
The roles never substitute:

```text
SourceHlc            chronology supplied by another source
AcceptedHlc          chronology admitted under local clock policy
ChronologySummary    immutable envelope over already-admitted chronology
AuthoritySequence    exact accepted order — not chronology
CommitPoint / Cut    exact durable progress — not chronology
```

**Pure admission.** `admit_chronology` is a pure operation: policy, prior chronology state, an admitted wall observation, and an optional SourceHlc yield the next state, an AcceptedHlc, and evidence — or a typed refusal.
It reads no ambient clock, performs no I/O, persists nothing, and commits nothing; a thin stateful shell commits the returned state.
A wall observation reaches this owner only as the port owner's admitted enclosure (`WallObservation`: earliest and latest bounds under a declared clock-source profile) — never a raw reading.
Unstated source uncertainty was already resolved at the port boundary by the profile's posture — declared maximum or refusal, never zero — and a point reading is the degenerate enclosure only when the source explicitly claims zero uncertainty.

**Honesty rules.** An excessive-future source value is preserved and classified, never clamped into a false source value.
Counter overflow is a typed refusal — no wrap, no saturation, no invented chronology; prior accepted state remains intact.
`ChronologySummary` merge is pure, same-profile, involves no wall clock, no source trust decision, no event stamping, and no durable-progress claim; its independently maximal components may never have co-occurred in one observation, and there is no road from the summary back to a SourceHlc or AcceptedHlc.
Algebraic claims (associativity, commutativity, idempotence) are made per profile, only where they actually hold.

**Merge refusal is its own family.** Merge is total over validated same-profile summaries, so its refusal family (`ChronologyMergeRefusal`) has exactly one cause — profile mismatch — and deliberately shares nothing with the admission family: componentwise maximum of valid values has no overflow cause, and profile identity subsumes profile version.

**Nonclaims.** Chronology proves no causation, no durable order, no completeness, and no checkpoint progress.
Exact component widths, skew ceilings, persistence, and the compatibility profile close with the chronology profile (owner-derived contract; open machining, not a fork); the recovered width candidates and their preserved contradiction live in `depot/event.md`.

*Chronology: carries the settled chronology rulings and the clock-role table (owner-endorsed direction, carried); the pure-advance shape derives from ARCHITECTURE.md §The rails (8).*

## 4. Causation

Three separate relationships, never merged (ARCHITECTURE.md §Composition and causation):

```text
ImmediateHistoryPredecessor   append integrity and local lineage
DomainCausation               typed, bounded, multi-parent semantic dependency
Membership / Correlation      entity, process, case, subscription views — never causal proof
```

An accepted causal edge points only to already-accepted causes; accepted causation is acyclic by admission.
Chronology proves no edge.
AuthoritySequence proves no edge.
Delivery proves no edge.
Correlation proves no edge.

**Unresolved causation (owner-ruled 2026-08-24).** An unverifiable foreign causal assertion may be admitted as an `UnresolvedCausalClaim`.
It never counts as DomainCausation, never closes a causal traversal, and never establishes causal completeness.
Later evidence may admit a separate resolved causal relationship.

**Fan-in (owner-ruled 2026-08-24).** The first profile supports bounded inline causal fan-in.
Beyond the bound, admission refuses with a typed result that names an external relation extent as the earned future road.
No such extent exists until a real consumer reaches the bound.

## 5. Partition

**Split.** A lawful split seals the parent at an exact Cut and carries a spatial witness proving the children are pairwise disjoint, their union equals the complete parent region, and no gap exists.
Child identities alone prove nothing: each proposed child carries its declared region geometry, judged against the parent's.
Pure geometry and cut-succession proofs consume evidence, never authority; sealing, successor activation, and routing publication each require the `PartitionGrant` scoped to that exact operation, and sealing requires the region's current accepted boundary — a stale claimed seal boundary refuses.
Children activate fresh epochs.
Inherited accepted events retain their identities, original AuthoritySequences, frames, coordinates, and causation; children never re-mint the inherited prefix and own only their fresh suffixes.
Child write authority activates before routing changes, and routing publishes last — routing reports authority and never grants it.
A parent cut and a child cut are not interchangeable positions; continuation across succession requires explicit cut-succession evidence.

**Merge** (owner-derived contract, derived from the split law): seal all child authorities at exact Cuts, prove the children pairwise disjoint and their union equal to the merged region, preserve every child segment and identity, activate one fresh merged epoch, publish merged routing last, and write only a new merged suffix.
A merge never retroactively invents a total order among child histories.

**Diverged copies.** A scope guard verifies scope identity, not world identity.
Only the lineage-wide commitment structure can testify that two holders of one scope are the same world; no comparison of positions can ever establish authority.

*Chronology: carries the settled partition/handoff rulings; the merge contract is owner-derived, not yet exercised by any consumer.*

## 6. Removal

Removal is a three-stage authority ladder, with each stage a distinct fact:

```text
RemovalPlan          caller-authored request; grants nothing
RemovalAdmission     affine boundary-minted authority for one exact destructive operation
RemovalCommitment    the fact that the destructive boundary actually crossed
```

`AuthorizedlyRemoved` is never historical absence.
These remain distinguishable answers, and no reader may collapse them: never existed; source incomplete; source corrupt; access unauthorized; protected meaning shredded; lawfully removed under policy; not retained by this profile.
The exact read-outcome family is declared with the query surface (view crossing); this owner establishes the facts that make those answers honest.
A removal admitted past the last committed boundary is correctly discarded on recovery.

*Chronology: carries the settled removal-ladder rulings (authority is minted, never authored).*

## 7. Storage publication contract

The storage contract is behavioral and mechanism-free.
Its operation roster is closed (`StorageOperation`): append against an ExpectedCut; exact accepted-prefix read; freeze of an exact Cut; accepted-prefix recovery after crash; idempotent reopen; and compaction as physical succession.
The family's contract declaration uses the port owner's grammar and is itself data, projected as a depot row.

**Recovery law.** Recovery is committed-boundary-bounded, never caller-acknowledgement-bounded.
Material beyond the last valid CommitPoint is lawfully discarded with a typed recovery receipt.
Material within the committed boundary that cannot be read intact is refuse-and-hold — committed data may never be silently discarded, and committed-but-unacknowledged data may never be discarded at all.

**Compaction.** Compaction creates new physical segments and a new `StorageGeneration`.
It never creates new event identities, new semantic order, or new accepted meaning.
Predecessor material remains authoritative until the replacement is durably published and selected; a failed compaction cannot make accepted history disappear.

**Durability is not one Boolean.** Payload bytes durable, event accepted and visible, namespace published, derived progress advanced, checkpoint advanced, and remote acceptance are separate frontiers; each receipt claims exactly the boundary it establishes and nothing more.

## 8. Ingress

Ingress owns the custody of foreign submissions.
It is a membrane into this home's authority, not a second event authority and not a business-rule engine.

**Two intake compositions.** The operation's contract — never the remote sender — selects the mode:

```text
ClaimFirst    ReceivedClaim → ValidatedClaim → AdmittedClaim
              → later: ClaimResolution (accepted event or typed domain refusal)

DomainFirst   ReceivedClaim → ValidatedClaim → AcceptedEvent
```

**Retry discharge law.** Only the matching terminal admitted receipt discharges that submission's retry duty: the claim-admission receipt (`ClaimAdmissionReceipt`) for a ClaimFirst submission; the domain-admission receipt (`DomainAdmissionReceipt`, draft spelling) — bound to the submission's idempotency identity, the accepted `EventId`, its `CommitPoint`, and the operation family — for a DomainFirst submission.
Nothing earlier can do it, and there is no generic `Admitted` or `Accepted`.
How much progress is exposed before the terminal milestone (terminal-only versus progressive) is an interface-selected projection with no ThreadPak-wide default; both projections preserve the same discharge invariant.

**Milestone honesty.** Claim admission does not mean the domain assertion is true, that a domain event was accepted, that a process ran, or that a view caught up.
Domain-event acceptance does not mean downstream effects completed or any checkpoint advanced.
A progress witness states exactly what survived: recorded stage metadata is not crash-recoverable claim bytes unless the witness says so.

**There is no `admit_domain` primitive.** Domain-fact admission is owned once, by accepted history.
The ClaimFirst completion is a composition: application-owned interpretation of the validated claim into an EventProposal or domain refusal, ordinary event admission, and an ingress-recorded `ClaimResolution` relating the AdmittedClaim to its outcome.
The claim's custody fact is never deleted; what closes is the processing obligation.
When one qualified backend can co-publish the accepted event and the claim resolution under one proven local atomic boundary it may; the records remain semantically distinct, an accepted event is never rolled back because a resolution publication lagged, and a claim is never marked resolved before the event actually became accepted.

**Idempotency identity ladder.** The ladder is typed (`SubmissionIdentity`), in order: natural business identity carried by the operation; an `IngressReservationToken` obtained through idempotent `Reserve` under a stable client-minted `ClientNonce`; a generated-client key minted per logical call instance (never per source-code call site); an explicit client-supplied key.
Effectful ingress with none of these refuses before admission.
No content-derived key — identical bytes cannot distinguish one retry from two intents — and no wall-clock bucket, AttemptId, session, route, connection, host, or shard may serve as identity.

**Reserve laws.** Repeating Reserve with the same nonce and same intent returns the same token, consuming bounded lookup work and no new reservation slot.
The same nonce with conflicting intent is a typed reservation conflict — no overwrite, no second token.
Quota exhaustion is a typed capacity refusal; an implementation may not evict an unexpired reservation and then accept a semantically ambiguous duplicate.

**Reservation lifecycle.** Token usability and duplicate recognition are two horizons.
After the `TokenUsabilityHorizon`, a retry may refuse as expired but may not silently become a fresh intent; a compact tombstone preserves duplicate recognition through the `DuplicateRecognitionHorizon`, after which retirement is lawful.
The duplicate-recognition horizon never closes before the token-usability horizon: a still-usable token whose duplicate recognition had lapsed would let a retry become a fresh intent, and guarded construction refuses the inversion.
Reservation state is bounded per principal, tenant, and operation, with creation-rate and byte bounds; anonymous or pre-authentication callers draw from a bounded anonymous bucket, an established client scope, or are refused before durable reservation — an unauthenticated caller never receives an unbounded durable-state mint.
A reservation token is an ingress identity relationship, not authority: it is no grant, no admission, no Attempt, no checkpoint, and no proof of truth; later admission performs ordinary authority checks.

**Rejected content.** The default disposition is a bounded, typed, redacted diagnostic.
Any fingerprint is keyed and scope-bound, never a public unkeyed digest of low-entropy input.
Raw retention is opt-in through a `QuarantineIntent` that crosses to the outside world via the port owner's quarantine contract under four guardrails: bounded; expiring with real deletion; access-controlled; never directly re-admittable — reuse crosses the foreign-content firewall as a fresh claim.
Rejected bytes are never promoted into accepted history, and no raw attacker bytes ride inside a refusal value.

*Chronology: carries the settled ingress acknowledgment/idempotency/custody ruling; the object-specific milestones, Reserve/nonce law, two-horizon lifecycle, and no-admit_domain composition are owner-endorsed closures of that ruling (2026-08-24); the no-default ack projection is the symmetric case of the settled default doctrine.*

---

## Grants

`AppendGrant` authorizes one append relationship to one region.
`IngressGrant` authorizes one foreign-submission relationship.
`RemovalGrant` authorizes participation in one removal ladder.
`PartitionGrant` authorizes an explicit set of partition operations — seal, successor activation, routing publication, parent retirement — for one region family (owner-ruled 2026-08-24: spelling kept, authority operation-scoped).
Pure geometry and cut proofs need evidence, never a grant.
Each grant is role-specific: no grant here widens, substitutes for another owner's grant, or survives its declared generation.
Authority is minted, never authored — a caller cannot construct authority by writing a convincing value.

## Receipts

`AppendReceipt`, `RecoveryReceipt`, `PartitionHandoffReceipt`, `RemovalReceipt`, and the ingress stage receipts each prove exactly one boundary.
There is no universal receipt (ARCHITECTURE.md §The rails, 13).

## Bounds and profiles

Owner-local limits, consumed by the operations that declare them; numeric values live in `depot/event.md`: `EventByteLimit`, `BatchEventLimit`, `CausalParentLimit`, `UnresolvedCausalClaimLimit`, `FederationSourceLimit`, `RecoveryScanBudget`, and the ingress reservation family (`ReservationCountLimit`, `ReservationByteLimit`, `ReservationsPerPrincipalLimit`, `ReservationsPerTenantLimit`, `ReservationsPerOperationLimit`, `ReservationCreationRateLimit`, `ReservationLookupWorkBudget`, `ReservationCreateWorkBudget`, `ClientNonceByteLimit`, `ReservationTokenByteLimit`, `ActiveReservationAgeLimit`, `ConflictEvidenceLimit`) plus the two horizons.

The limits are bundled into this owner's profile algebra (`EventAdmissionProfile`, `FederationProfile`, `ReservationProfile`, `RecoveryProfile`, `StorageProfile`, `ChronologyPolicy`): every operation receives its exact profile as an explicit argument, and nothing fetches a row ambiently (`depot/README.md`, "Rows are passed, never fetched").
A profile selects coordinates inside the algebra declared here; it can never widen it.

## Crossings

Each crossing is stated per ARCHITECTURE.md §No orphan by distribution: fact — owner — establishing operation — carrier — substitution refusal — chronology.

1. **Storage publication.** Fact: durable acceptance of admitted material.
  Owner: this home (contract); a qualified store realizes it.
  Operation: publication of what pure admission admitted.
  Carrier: the storage port family, declared here in the port owner's contract grammar.
  Refusal: a storage mechanism's success claim never substitutes for a CommitPoint it did not establish.
  Chronology: carries the settled storage/recovery rulings.
2. **Quarantine.** Fact: retained rejected material.
  Owner: ingress declares disposition and intent; port owns the physical crossing; the physical attempt is admitted like any other.
  Carrier: the quarantine port contract.
  Refusal: quarantined bytes are never re-admitted directly and never enter accepted history.
  Chronology: carries the settled rejected-content ruling.
3. **Wall observations.** Fact: a physical clock reading and its uncertainty.
  Owner: port — the raw observation and its strengthening into the admitted enclosure (`WallObservation`) under a declared clock-source profile; chronology consumes only the enclosure.
  Operation: port-owned observation admission, then pure chronology admission.
  Carrier: the wall-observation port contract.
  Refusal: a raw reading is never an admitted enclosure, and neither is ever an AcceptedHlc.
  Chronology: carries the settled clock-role table and the owner-ruled observation-enclosure closure (2026-08-24).
4. **Derived results.** Fact: any Fix, query result, projection, monitor, or DataBlock.
  Owner: the view owner.
  Operation: view advance or resolve at exact Cuts over this home's accepted history.
  Carrier: view surfaces.
  Refusal: no derived result becomes accepted history; disagreement means the derived result is wrong.
  Chronology: carries ARCHITECTURE.md §The two lanes.
5. **Checkpoint authority.** Fact: the right to skip completed logical work.
  Owner: runtime, in runtime-owned checkpoint authority regions.
  Operation: runtime checkpoint admission.
  Carrier: checkpoint references naming this home's Cuts.
  Refusal: checkpoint records reference domain Cuts but never inhabit or mutate domain regions; no cursor, chronology value, or wake advances one.
  Chronology: owner-ruled 2026-08-24 (ARCHITECTURE.md §Checkpoints).
6. **Deadline enforcement.** Fact: remaining waiting or work allowance.
  Owner: runtime/Bvisor.
  Operation: deadline rebase and check.
  Carrier: monotonic-observation port contract.
  Refusal: no chronology value here is a deadline, and no deadline is chronology.
  Chronology: carries the settled deadline-split ruling.
7. **Application interpretation.** Fact: the meaning of a validated claim as a domain proposal.
  Owner: the application.
  Operation: pure application-owned interpretation.
  Carrier: ordinary Rust.
  Refusal: ingress never interprets domain meaning; interpretation never admits anything.
  Chronology: owner-endorsed closure 2026-08-24.
8. **Remote wrappers.** Fact: any stage witness or receipt from this home.
  Owner: unchanged by transport.
  Operation: none — carriage only.
  Carrier: generated wrappers and host adapters.
  Refusal: a wrapper may carry a witness and may never mint or strengthen one.
  Chronology: carries ARCHITECTURE.md §No orphan by distribution.

## Nonclaims

HLC proves no causation and no durable order.
Route grants no authority.
Physical placement is not partition identity.
A FederationCut is not a distributed transaction.
Authorized removal is not historical absence.
A reservation token is not a capability.
Claim admission is not domain truth.
Storage success is not namespace publication, derived progress, or checkpoint advancement.
Nothing in this home performs an external effect; the effectful shells publish what pure operations admit.

## Hostile denominator

The falsifiers this home must refuse, each with a typed refusal a test can demand:

1. Admission under a wrong or mismatched ReferenceFrame or coordinate role.
2. Append against a stale ExpectedCut; append under a sealed or stale epoch; append by a non-writer authority.
3. An undeclared axis operation inferred from representation.
4. Cross-scope comparison of AuthoritySequence or CommitPoint values.
5. Chronology used as causation, order, completeness, or progress; excessive-future clamping; counter-overflow wraparound.
6. Causal edge to a non-accepted cause; causal cycle; correlation or delivery order presented as causation; UnresolvedCausalClaim counted as DomainCausation; fan-in beyond the inline bound not refusing.
7. Split children that overlap, gap, or fail to cover the parent; routing published before child activation; inherited events re-minted; parent and child cuts interchanged without succession evidence.
8. Removal without the full ladder; removal read back as historical absence; discard of committed material during recovery; discard of committed-but-unacknowledged material ever.
9. Lost acknowledgement at every exposed ingress rung; an earlier milestone discharging retry; a generic admitted/accepted result.
10. Duplicate Reserve minting a second token; same nonce with conflicting intent overwriting; eviction of an unexpired reservation followed by an ambiguous duplicate; expiry silently converting a retry into a fresh intent.
11. Crash after claim admission losing claim custody; a progress witness claiming crash-recoverability the bytes do not have.
12. Raw rejected content escaping custody — in a refusal value, an unkeyed digest, or a direct re-admission.
13. A compaction or storage generation change presented as new event identity or order.
14. A carrier or wrapper minting or strengthening any witness.
15. A claim resolution recorded before its outcome exists, or a closed obligation resolved twice.

## Escalations

Recorded for the repository owner.
The three escalations this contract opened were ruled 2026-08-24; the dispositions are recorded here so the contract carries no stale open fork.

1. **Partition authority naming — resolved.** `PartitionGrant` is kept, and the authority is operation-scoped (an explicit `PartitionOperation` set per grant).
  Pure proofs take no grant.
2. **Federation source cap — resolved.** The cap stays as a typed admission bound (`FederationSourceLimit`); every actual operation is finitely bounded, and no eternal product-wide source count is implied.
  The numeric value remains withheld, profile-selected.
  The recovered no-cap posture stays preserved as provenance in `depot/event.md`.
3. **HLC width candidates — resolved.** The ch10 pair {physical u64, logical u32} is ratified as chronology encoding profile V1.
  Before these bytes become canonical, the profile must also bind: physical epoch, unit/resolution, admitted wall-reading interpretation, logical increment rule, regression rule, future-skew posture, and the overflow refusal — each a depot row, withheld until its value arrives (`depot/event.md`).
