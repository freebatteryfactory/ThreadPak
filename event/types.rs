//! Event owner — type declarations.
//!
//! Declarations only: this file states the role graph of the `event` home.
//! Operations live in the home's verb files; constructors and invariant
//! strengthening arrive with the guard pass. No impl blocks, no function
//! bodies, and deliberately no derives — every derive (`Clone`, `Ord`,
//! `Serialize`, …) is a semantic claim, minted per type with the guard pass,
//! never a convenience sticker.
//!
//! Where a representation is written `/* closes with … */`, the semantic role
//! is law and the byte-level shape is an owner-derived contract still to be
//! machined (for example, the chronology profile's exact widths). An empty
//! body is a visible open seam, never a placeholder API.
//!
//! Cross-home types are referenced as bare names in field position, owners
//! noted here — `WallObservation` (port; the admitted observation enclosure —
//! raw readings never reach this owner), `SchemaId` and `SchemaVersion`
//! (core schema) — never as `crate::` paths; the dependency probe seats the
//! real imports. Application payload value roles and knowledge axes remain
//! doc-referenced until their owners force fields.
//!
//! Profiles declared at the end of this file are the owner's configuration
//! algebra: operations receive them as explicit arguments, selected values
//! live in `depot/event.md`, and nothing here is fetched ambiently
//! (`depot/README.md`, "Rows are passed, never fetched").

use core::num::{NonZeroU32, NonZeroU64};

// ---------------------------------------------------------------------------
// Coordinate owner
// ---------------------------------------------------------------------------

/// A named coordinate system. Historical facts stay bound to the frame under
/// which they were admitted; a newer frame reinterprets explicitly, never
/// retroactively.
pub struct ReferenceFrame {
    /* frame identity and axis roster; closes with the identity profile */
}

/// Identity of one `ReferenceFrame`. Role-distinct from every other identity;
/// consumers binding a frame bind this identity plus a `FrameVersion`.
pub struct ReferenceFrameId {
    /* closes with the identity profile */
}

/// The version of a `ReferenceFrame`. Frame meaning changes only by minting a
/// new version, never by mutating an existing one.
pub struct FrameVersion {
    /* closes with the identity profile */
}

/// One declared dimension of a `ReferenceFrame`. An axis supports exactly the
/// operations its `AxisCapability` set declares — nothing is inferred from
/// the Rust representation of values on the axis.
pub struct Axis {
    /* axis identity, value role, and declared capability set */
}

/// The closed roster of operations an `Axis` may declare lawful.
pub enum AxisCapability {
    /// Values on this axis may be compared for equality.
    Equality,
    /// Values admit one total order declared by the axis, not by `Ord`.
    TotalOrder,
    /// Values admit a declared partial order.
    PartialOrder,
    /// Values participate in a declared hierarchy.
    Hierarchy,
    /// Values admit declared intervals.
    Intervals,
    /// Values participate in declared set membership.
    Sets,
    /// Values participate in application-declared typed relationships.
    TypedRelationship,
    /// Values admit metric distance under one named profile only.
    MetricDistance {
        profile: MetricProfileId,
    },
    /// A qualified approximate mechanism may propose candidates for this
    /// axis. Candidates never establish truth; exact verification remains
    /// downstream.
    QualifiedApproximation,
}

/// Names the exact metric profile under which distance on an axis is lawful.
pub struct MetricProfileId {
    /* closes with the identity profile */
}

/// A typed semantic address inside one `ReferenceFrame`: where a fact enters
/// semantic space. An admission address is never current state — the derived
/// `Fix` (view owner) answers "where are we now".
pub struct Coordinate {
    /* frame binding plus per-axis position; shape closes with the frame
     * contract */
}

/// A declared transformation between frames. It states its source and target
/// frame versions, domain, multiplicity, loss, exactness, reversibility,
/// authority posture, and work and expansion bounds. An undeclared
/// transformation does not exist.
pub struct FrameTransformation {
    /* the eight declared facts above; closes with the frame contract */
}

// ---------------------------------------------------------------------------
// Accepted history owner
// ---------------------------------------------------------------------------

/// Identity of one accepted fact. Minted only by event admission; readers
/// never parse structure out of it.
pub struct EventId {
    /* closes with the identity profile */
}

/// The canonical-byte commitment an `EventId` derives from. Hashes are
/// receipts of bytes, never authority over meaning.
pub struct EventCommitment {
    /* closes with the canonical-byte profile */
}

/// The application-declared class of one event. Reserved internal classes
/// are unmintable by public writers; authority is validated before frame
/// construction.
pub struct EventClass {
    /* closes with the identity profile */
}

/// The schema commitment one event body is proposed and accepted under.
/// `SchemaId` and `SchemaVersion` are the core schema owner's roles, carried
/// here — this binding never substitutes for codec or layout identity.
pub struct SchemaBinding {
    schema: SchemaId,
    version: SchemaVersion,
}

/// The bounded candidate bytes of one proposed event body, within
/// `EventByteLimit`. Canonical only after admission proves it; holding this
/// value proves nothing about canonicality or acceptance.
pub struct ProposedEventBody {
    /* bounded candidate bytes; canonical form closes with the canon profile */
}

/// A proposed fact. It carries no authority, no position, and no chronology;
/// only admission can turn it into an `AcceptedEvent`. Public writers cannot
/// propose reserved internal event classes — authority is validated before
/// frame construction.
pub struct EventProposal {
    coordinate: Coordinate,
    class: EventClass,
    schema: SchemaBinding,
    body: ProposedEventBody,
    causation: ProposedCausation,
}

/// An immutable fact admitted into accepted history. The accepted record
/// binds identity, address, authority, exact local order, admitted
/// chronology evidence, and explicit relationships. Commit knowledge rides
/// its own knowledge axis — never a field here, because the `CommitPoint`
/// only exists after publication (the four-object split: event body,
/// accepted record, batch publication, commit receipt).
pub struct AcceptedEvent {
    id: EventId,
    commitment: EventCommitment,
    coordinate: Coordinate,
    class: EventClass,
    schema: SchemaBinding,
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
    sequence: AuthoritySequence,
    predecessor: ImmediateHistoryPredecessor,
    accepted_hlc: AcceptedHlc,
    source_chronology: SourceChronologyEvidence,
    causation: AdmittedCausation,
    /* payload bytes are bound through `commitment` — the record references
     * the body it commits to; it never carries a second copy */
}

/// Identity of one bounded write-authority region. Live write regions for one
/// authority family and epoch are disjoint; query regions may overlap freely.
pub struct AuthorityRegionId {
    /* closes with the identity profile */
}

/// One epoch of write authority over one region. Exactly one active semantic
/// authority may extend the accepted order per region per epoch — an
/// authority, not a thread, task, process, or machine.
pub struct AuthorityEpoch {
    /* closes with the identity profile */
}

/// Exact accepted position under one writer authority. Scope-bound: the scope
/// lives in the value's own canonical bytes, there is no derived `Ord`, and
/// comparison is lawful only within one proven scope. An ordering-
/// interpretation change that would make old and new values incomparable
/// mints a new authority generation. There is no global sequence.
pub struct AuthoritySequence {
    /* scope binding plus position; closes with the canonical-byte profile */
}

/// Exact durable accepted-prefix progress under one authority. A commit
/// point is evidence of publication, never chronology and never a
/// checkpoint.
pub struct CommitPoint {
    /* scope binding plus exact prefix; closes with the canonical-byte
     * profile */
}

/// An exact read boundary over accepted history: what "now" means for one
/// read. Every derived result names its Cut.
pub struct Cut {
    /* scope binding plus exact boundary; closes with the canonical-byte
     * profile */
}

/// The caller's claimed current Cut for expected-base admission. A stale
/// expected Cut refuses before publication; an adapter may not silently
/// rebase and retry, because that changes the operation's meaning.
pub struct ExpectedCut {
    expected: Cut,
}

/// Names the exact participating authorities, regions, and epochs of one
/// multi-source claim, in canonical representation order. Changing any
/// participant creates a different source set; a result may never silently
/// drop an unavailable source and keep the original set's name.
pub struct SourceSet {
    /* participant roster; closes with the identity profile */
}

/// Identity of a `SourceSet`.
pub struct SourceSetId {
    /* closes with the identity profile */
}

/// One exact `CommitPoint` per participating authority of one `SourceSet`.
/// A federation cut proves no coexistence, no snapshot, no global order, and
/// no distributed atomicity. A componentwise join is a knowledge summary
/// only.
pub struct FederationCut {
    source_set: SourceSetId,
    /* one exact CommitPoint per participant, in canonical order */
}

/// The lawful comparisons between two `FederationCut`s over compatible
/// source sets.
pub enum FederationCutRelation {
    Equal,
    Advances,
    Precedes,
    Concurrent,
    Incompatible,
}

/// Authorizes one append relationship to one region. Role-specific; grants
/// are minted, never authored — a caller cannot construct authority by
/// writing a convincing value.
pub struct AppendGrant {
    /* scope, generation, and attenuation close with the guard pass */
}

/// The typed refusal family of event admission.
pub enum EventAdmissionRefusal {
    /// The caller's `ExpectedCut` is not the current accepted boundary.
    StaleExpectedCut,
    /// The target epoch is sealed or superseded.
    StaleEpoch,
    /// The caller's grant does not cover this region and epoch.
    WrongAuthority,
    /// The proposal's frame or coordinate role does not match the target.
    FrameMismatch,
    /// The proposal declares a reserved internal event class.
    ReservedEventClass,
    /// The proposal exceeds a declared admission bound.
    BoundExceeded,
    /// A declared causal relationship failed causation admission.
    Causation(CausationRefusal),
    /* the roster closes with the admission contract */
}

// ---------------------------------------------------------------------------
// Chronology owner
// ---------------------------------------------------------------------------

/// Chronology supplied by another source. Preserved as supplied — an
/// excessive-future source value is retained and classified, never clamped
/// into a false source value.
pub struct SourceHlc {
    /* closes with the chronology profile */
}

/// Chronology admitted under local clock policy. Minted only by pure
/// chronology admission; never a deadline, never an order, never a cut.
pub struct AcceptedHlc {
    /* closes with the chronology profile */
}

/// Immutable envelope over already-admitted chronology. Its independently
/// maximal components may never have co-occurred in one observation; there
/// is no road from a summary back to a `SourceHlc` or `AcceptedHlc`. Merge
/// is pure and same-profile; algebraic claims hold per profile only. The
/// name is never abbreviated.
pub struct ChronologySummary {
    /* closes with the chronology profile */
}

/// Identity of one chronology profile — the compatibility boundary of every
/// chronology value. Numerically newer never implies compatible.
pub struct ChronologyProfileId {
    /* closes with the identity profile */
}

/// Time-class bound: how far a wall observation or source value may lead
/// accepted chronology before classification refuses advancement. The
/// numeric ceiling is a depot chronology row (withheld until selected).
pub struct SkewCeiling {
    /* a Time-class bound; value in depot/event.md */
}

/// The declared local admission policy: profile identity, skew ceiling,
/// regression posture, future posture, and counter behavior.
pub struct ChronologyPolicy {
    profile: ChronologyProfileId,
    skew_ceiling: SkewCeiling,
    /* regression posture, future posture, and counter behavior rosters
     * close with the chronology profile pass */
}

/// Whether a submission supplied source chronology — a typed absence role,
/// never a hole. Supplied values are preserved exactly; an excessive-future
/// value is preserved and classified, never clamped.
pub enum SourceChronologyEvidence {
    /// Source chronology was supplied and is preserved as supplied.
    Preserved(SourceHlc),
    /// No source chronology was supplied — a genuine recorded fact, not a
    /// defaulted value.
    NoneSupplied,
}

/// The prior admitted chronology state consumed by pure admission. Owned by
/// the thin stateful shell; the pure operation returns the successor and
/// commits nothing.
pub struct ChronologyState {
    /* closes with the chronology profile */
}

/// Classification evidence of one chronology admission: what the observation
/// and source looked like against policy (in-window, regression-classified,
/// future-classified). Evidence about the admission, never a truth value.
pub struct ChronologyAdmissionEvidence {
    /* classification record; closes with the chronology profile */
}

/// The result of one pure chronology admission: the successor state, the
/// admitted value, preserved source evidence, and admission evidence.
pub struct ChronologyAdvance {
    next: ChronologyState,
    accepted: AcceptedHlc,
    source: SourceChronologyEvidence,
    evidence: ChronologyAdmissionEvidence,
}

/// The typed refusal family of chronology admission. Overflow refuses — no
/// wrap, no saturation, no invented chronology; prior accepted state remains
/// intact.
pub enum ChronologyRefusal {
    RegressionBeyondPolicy,
    FutureBeyondPolicy,
    LogicalCounterOverflow,
    ProfileMismatch,
    /* the roster closes with the chronology profile */
}

/// The typed refusal family of `ChronologySummary` merge — deliberately its
/// own single-cause family, not the admission family: merge is total over
/// validated same-profile summaries, so the domain boundary is its only
/// guard, and no overflow cause exists (componentwise max of valid values
/// cannot overflow). Profile identity subsumes profile version.
pub enum ChronologyMergeRefusal {
    ProfileMismatch,
}

// ---------------------------------------------------------------------------
// Causation owner
// ---------------------------------------------------------------------------

/// Append-integrity lineage: the immediate predecessor relationship inside
/// one authority's accepted order. Integrity, not domain meaning.
pub struct ImmediateHistoryPredecessor {
    /* predecessor binding; genesis posture closes with the admission
     * contract */
}

/// A typed, bounded, multi-parent semantic dependency between accepted
/// facts. Every cause is already accepted; accepted causation is acyclic by
/// admission. Edge kinds are application-declared through the closed
/// relation contract — chronology, order, delivery, and correlation prove no
/// edge.
pub struct DomainCausation {
    causes: InlineCauseSet,
    /* application-declared edge kind, via the relation contract */
}

/// The bounded inline cause set of one `DomainCausation`. Admission refuses
/// beyond `CausalParentLimit`; the refusal names an external relation extent
/// as the earned future road, and no such extent exists until a real
/// consumer reaches the bound (owner-ruled 2026-08-24).
pub struct InlineCauseSet {
    causes: Vec<EventId>,
}

/// A foreign causal assertion as submitted, before admission: the asserted
/// foreign cause reference and its claimed relationship. Inert input data —
/// admission may record it as an `UnresolvedCausalClaim` and nothing else.
pub struct ForeignCausalAssertion {
    /* asserted foreign reference and claimed relationship; the foreign
     * reference role closes with the identity profile */
}

/// An admitted, unverifiable foreign causal assertion. It never counts as
/// `DomainCausation`, never closes a causal traversal, and never establishes
/// causal completeness. Later evidence may admit a separate resolved
/// relationship (owner-ruled 2026-08-24).
pub struct UnresolvedCausalClaim {
    assertion: ForeignCausalAssertion,
    /* admission provenance; closes with the identity profile */
}

/// The causation a proposal declares: already-accepted causes plus any
/// foreign assertions. Empty rosters are the lawful measured "no declared
/// causes" — a recorded zero, never absence-as-hole.
pub struct ProposedCausation {
    causes: InlineCauseSet,
    assertions: Vec<ForeignCausalAssertion>,
}

/// The causation an accepted record carries: the admitted typed edges and
/// the admitted-but-unresolved foreign claims, kept structurally apart so
/// no reader can count a claim as an edge. Total inline parents across all
/// edges are bounded by `CausalParentLimit`; unresolved claims by
/// `UnresolvedCausalClaimLimit`.
pub struct AdmittedCausation {
    edges: Vec<DomainCausation>,
    unresolved: Vec<UnresolvedCausalClaim>,
}

/// Membership and correlation relationships — entity, process, case,
/// subscription, application views. Never causal proof; a distinct relation
/// family from `DomainCausation` by construction.
pub struct CorrelationMembership {
    /* relation identity and members, via the relation contract */
}

/// The typed refusal family of causation admission.
pub enum CausationRefusal {
    /// A declared cause is not an accepted fact under a reachable source.
    CauseNotAccepted,
    /// The declared edges would make accepted causation cyclic.
    CausalCycle,
    /// Inline fan-in exceeds `CausalParentLimit`. The external relation
    /// extent is the earned future road; it is not built yet.
    FanInExceedsInlineBound,
    /* the roster closes with the relation contract */
}

// ---------------------------------------------------------------------------
// Partition owner
// ---------------------------------------------------------------------------

/// Evidence that one region's writable epoch is sealed at an exact Cut. A
/// sealed parent accepts no later write.
pub struct RegionSealWitness {
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
    seal_cut: Cut,
}

/// The declared membership geometry of one region — the coordinate owner's
/// region-membership role, carried here as split evidence. Geometry is a
/// declared fact about which addresses a region covers; it is never write
/// authority and never physical placement.
pub struct RegionGeometry {
    /* declared membership geometry over coordinate axes; representation
     * closes with the region-geometry contract */
}

/// One proposed child of a split: its identity together with its declared
/// geometry. Identity alone can prove nothing about disjointness or
/// coverage — the geometry is the evidence the split proof judges.
pub struct ChildRegionDeclaration {
    region: AuthorityRegionId,
    geometry: RegionGeometry,
}

/// The spatial proof of one lawful split: children pairwise disjoint, union
/// equal to the complete parent region, no gap, no overlap. Inherited events
/// retain their identities and original positions; children own only fresh
/// suffixes under fresh epochs.
pub struct SplitWitness {
    parent: RegionSealWitness,
    children: Vec<ChildRegionDeclaration>,
    /* the pairwise-disjointness and exact-coverage proof over the children's
     * declared geometry against the parent's; representation closes with the
     * region-geometry contract */
}

/// Explicit evidence relating positions across an authority succession. A
/// parent cut and a child cut are never interchangeable without it.
pub struct CutSuccessionWitness {
    parent: RegionSealWitness,
    successor_region: AuthorityRegionId,
    successor_epoch: AuthorityEpoch,
    /// The parent boundary as lawfully translated into the successor's
    /// scope. A translated boundary, never a fresh mint.
    translated: Cut,
}

/// The activation of one fresh epoch on a successor region. Activation
/// precedes routing publication.
pub struct EpochActivation {
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
}

/// The routing publication for a succession — published last. Routing
/// reports authority; it never grants it, and physical placement is never
/// partition identity.
pub struct RoutingPublication {
    successors: Vec<EpochActivation>,
    /* physical placement and reachability rows close with the deployment
     * contract — they report, never grant */
}

/// The closed roster of partition operations a `PartitionGrant` may be
/// scoped to. Pure geometry and succession proofs appear nowhere here —
/// proofs consume evidence, never authority. `RetireParent`'s operation
/// arrives with its consumer (the succession witness's stale-parent leg);
/// the authority role is declared with the roster so a grant can be scoped
/// to it from the first mint.
pub enum PartitionOperation {
    /// Seal one region's writable epoch at an exact Cut.
    Seal,
    /// Activate one fresh epoch on a successor region.
    ActivateSuccessor,
    /// Publish routing for a succession — last, after every activation.
    PublishRouting,
    /// Retire a stale parent: reachable for reads, never again a writer.
    RetireParent,
}

/// The closed member set of partition operations one grant covers.
pub struct PartitionOperationSet {
    /* closed membership over `PartitionOperation`; representation closes
     * with the guard pass */
}

/// Generation of one partition grant, for expected-generation validation of
/// authority state changes. A grant never survives its declared generation.
pub struct PartitionGrantGeneration {
    /* closes with the identity profile */
}

/// Authorizes an explicit set of partition operations for one region family
/// (owner-ruled 2026-08-24: spelling kept, authority operation-scoped).
/// Role-specific like every grant here. Pure geometry and cut proofs need
/// evidence, never this grant; admitting or publishing a partition
/// transition requires the grant scoped to that exact operation.
pub struct PartitionGrant {
    operations: PartitionOperationSet,
    generation: PartitionGrantGeneration,
    /* region-family scope and attenuation close with the guard pass */
}

/// The typed refusal family of partition operations.
pub enum PartitionRefusal {
    /// The claimed seal boundary is not the region's current accepted
    /// boundary — sealing behind it would orphan accepted material; sealing
    /// ahead of it would seal history that does not exist.
    StaleSealCut,
    /// The parent epoch is not sealed at the claimed Cut.
    ParentNotSealed,
    /// Two proposed child regions overlap.
    ChildrenOverlap,
    /// The proposed children leave part of the parent uncovered.
    CoverageGap,
    /// A parent and child position were related without succession evidence.
    SuccessionEvidenceMissing,
    /// Routing publication was attempted before child activation.
    RoutingBeforeActivation,
    /// The caller's grant does not cover this region family, or is not
    /// scoped to the partition operation being admitted.
    WrongAuthority,
    /* the roster closes with the region-geometry contract */
}

/// Evidence that one complete handoff — seal, split, succession, activation,
/// route-last publication — occurred lawfully. The parent seal rides inside
/// the split witness.
#[must_use]
pub struct PartitionHandoffReceipt {
    split: SplitWitness,
    succession: CutSuccessionWitness,
    activations: Vec<EpochActivation>,
    routing: RoutingPublication,
}

// ---------------------------------------------------------------------------
// Removal owner
// ---------------------------------------------------------------------------

/// Authorizes participation in one removal ladder. Role-specific; a grant
/// is not an admission — the ladder still mints its own affine authority.
pub struct RemovalGrant {
    /* scope, generation, and attenuation close with the guard pass */
}

/// Identity of one removal plan.
pub struct RemovalPlanId {
    /* closes with the identity profile */
}

/// Identity of one removal admission.
pub struct RemovalAdmissionId {
    /* closes with the identity profile */
}

/// Identity of one removal commitment.
pub struct RemovalCommitmentId {
    /* closes with the identity profile */
}

/// The exact scope of one requested destruction: which region, at which
/// exact Cut, over which extent. Never a pattern, never "everything since".
pub struct RemovalScope {
    region: AuthorityRegionId,
    at: Cut,
    /* the exact extent within the region; closes with the storage contract */
}

/// A caller-authored removal request. It grants nothing.
pub struct RemovalPlan {
    id: RemovalPlanId,
    scope: RemovalScope,
}

/// Affine, boundary-minted authority for one exact destructive operation.
/// Consumed by the crossing it authorizes; never cloneable, never reusable.
#[must_use]
pub struct RemovalAdmission {
    id: RemovalAdmissionId,
    plan: RemovalPlanId,
    /// The exact scope re-bound at admission — a plan cannot widen between
    /// authoring and admission.
    scope: RemovalScope,
    /* granting-authority binding closes with the guard pass */
}

/// The fact that the destructive boundary actually crossed. Authorized
/// removal is never historical absence: never-existed, incomplete, corrupt,
/// unauthorized, shredded, lawfully-removed, and not-retained remain
/// distinguishable answers (the read-outcome family is declared with the
/// query surface).
pub struct RemovalCommitment {
    id: RemovalCommitmentId,
    admission: RemovalAdmissionId,
    /// The exact extent actually destroyed — never wider than admitted.
    destroyed: RemovalScope,
}

/// Evidence of one completed removal ladder.
#[must_use]
pub struct RemovalReceipt {
    plan: RemovalPlanId,
    admission: RemovalAdmissionId,
    commitment: RemovalCommitmentId,
}

/// The typed refusal family of the removal ladder.
pub enum RemovalRefusal {
    /// The caller's grant does not cover this removal ladder.
    WrongAuthority,
    /// The admission's scope does not match the plan it claims.
    ScopeMismatch,
    /// The plan's Cut binding is not the current accepted boundary state
    /// the ladder requires.
    StaleCut,
    /* the roster closes with the removal contract */
}

// ---------------------------------------------------------------------------
// Storage publication contract
// ---------------------------------------------------------------------------

/// Physical succession identity of stored material. Compaction mints a new
/// generation of segments; it never mints event identity, order, or meaning.
/// Predecessor material remains authoritative until the replacement is
/// durably published and selected.
pub struct StorageGeneration {
    /* closes with the storage contract */
}

/// One accepted identity at its exact accepted position — one row of an
/// `AppendReceipt`'s published batch.
pub struct AcceptedPosition {
    event: EventId,
    sequence: AuthoritySequence,
}

/// Evidence of one durable append publication: which proposals became
/// accepted, at which positions, through which commit point. Claims exactly
/// the publication boundary and nothing more — not namespace publication,
/// not derived progress, not checkpoint advancement. The batch roster is
/// bounded by `BatchEventLimit`.
#[must_use]
pub struct AppendReceipt {
    commit: CommitPoint,
    accepted: Vec<AcceptedPosition>,
}

/// The lawful classifications of material found during accepted-prefix
/// recovery. Recovery is committed-boundary-bounded, never
/// caller-acknowledgement-bounded.
pub enum RecoveryClassification {
    /// Inside the committed boundary and unreadable: refuse-and-hold.
    /// Committed material is never silently discarded; committed-but-
    /// unacknowledged material is never discarded at all.
    WithinCommittedBoundary,
    /// Beyond the last valid `CommitPoint`: lawful discard, evidenced by a
    /// `RecoveryReceipt`.
    BeyondCommittedBoundary,
}

/// One classified extent encountered during recovery: its classification
/// and the exact material bounds it covers.
pub struct ClassifiedExtent {
    classification: RecoveryClassification,
    /* exact extent bounds; closes with the storage contract */
}

/// Evidence of one recovery pass: the exact recovered boundary and the
/// classified disposition of everything encountered beyond it.
#[must_use]
pub struct RecoveryReceipt {
    recovered: CommitPoint,
    classified: Vec<ClassifiedExtent>,
}

/// The typed refusal family of a recovery pass. Classification is total —
/// refuse-and-hold is a classification, not a refusal — so this family
/// covers only the pass itself failing to complete lawfully.
pub enum RecoveryRefusal {
    /// The scan budget was exhausted before the committed boundary was
    /// established. No partial boundary is reported as recovered.
    ScanBudgetExhausted,
    /* the roster closes with the storage contract */
}

/// Bounded scan evidence handed to pure recovery classification by the
/// storage boundary: what material was physically encountered, as claims to
/// classify — never as trusted structure.
pub struct SegmentScanEvidence {
    /* scanned frame and segment claims; closes with the storage contract */
}

/// One bounded exact read over the accepted prefix at an exact Cut — the
/// input surface derivation consumes (view advance and resolve read through
/// this role). A read, never authority: it mints nothing, advances nothing,
/// and its extent is bounded by the consuming operation's declared bounds.
/// The seven-way history-read outcome family is declared with the query
/// surface, not here.
pub struct ExactHistoryRead {
    at: Cut,
    /* bounded extent binding; closes with the storage contract */
}

/// The closed roster of operations the storage port family declares. The
/// family's `PortContract` declaration (port grammar) selects exactly these;
/// the declaration itself is data and is projected as a depot row.
pub enum StorageOperation {
    /// Append a bounded admitted batch against an `ExpectedCut`.
    Append,
    /// Read an exact accepted prefix.
    ExactPrefixRead,
    /// Freeze an exact Cut for readers.
    FreezeCut,
    /// Recover the accepted prefix after a crash.
    Recover,
    /// Reopen idempotently.
    Reopen,
    /// Compact into a fresh `StorageGeneration`.
    Compact,
}

// ---------------------------------------------------------------------------
// Ingress owner
// ---------------------------------------------------------------------------

/// The two lawful intake compositions. The operation's contract selects the
/// mode; the remote sender never does.
pub enum IngressMode {
    /// Received → Validated → ClaimAdmitted; domain interpretation and event
    /// admission follow later, and the claim resolution is recorded.
    ClaimFirst,
    /// Received → Validated → domain-event acceptance; no separate durable
    /// claim fact is created. DomainFirst does not mean "trusted" — the
    /// foreign-content firewall applies identically.
    DomainFirst,
}

/// How much progress is exposed before the terminal milestone. Interface-
/// selected; no ThreadPak-wide default. Both projections preserve the same
/// discharge invariant: only the matching terminal admitted receipt
/// discharges retry.
pub enum AcknowledgmentProjection {
    TerminalOnly,
    Progressive,
}

/// A client-minted stable retry identity. Byte-bounded by
/// `ClientNonceByteLimit`. Content-derived, wall-clock, session, route,
/// connection, host, shard, and Attempt identities are all unlawful
/// substitutes.
pub struct ClientNonce {
    /* opaque client-minted bytes */
}

/// Authorizes one foreign-submission relationship. Role-specific; never a
/// substitute for any other owner's grant.
pub struct IngressGrant {
    /* scope, generation, and attenuation close with the guard pass */
}

/// Identity of one ingress operation family — the scope reservations,
/// quotas, and receipts are keyed by ("per operation" in the bounds).
pub struct IngressOperationFamilyId {
    /* closes with the identity profile */
}

/// The canonical commitment of one submission's intent, bound at `Reserve`.
/// Same-nonce duplicate detection compares this commitment: equal commitment
/// is one retry, different commitment is a typed conflict. A commitment,
/// never the intent bytes themselves.
pub struct ReservationIntentCommitment {
    /* closes with the canon profile */
}

/// The idempotency identity one effectful submission carries — the settled
/// four-rung ladder, in order. Effectful ingress with none of these refuses
/// before admission (`IngressRefusal::NoLawfulIdempotencyIdentity`); no
/// content-derived key, wall-clock bucket, AttemptId, session, route,
/// connection, host, or shard may substitute.
pub enum SubmissionIdentity {
    /// Natural business identity carried by the operation itself. The
    /// application's key role, carried — never interpreted here.
    NaturalBusinessIdentity(/* application key role; bounded, carried */),
    /// A reservation token obtained through idempotent `Reserve` under a
    /// stable client-minted nonce.
    Reservation(IngressReservationToken),
    /// A generated client key minted per logical call instance — never per
    /// source-code call site.
    GeneratedClientKey(/* bounded opaque key bytes */),
    /// An explicit client-supplied key.
    ExplicitClientKey(/* bounded opaque key bytes */),
}

/// The durable reservation minted or recovered by idempotent `Reserve` under
/// one `ClientNonce`. Repeating Reserve with the same nonce and intent
/// returns the same reservation; conflicting intent refuses; capacity
/// exhaustion refuses and never evicts an unexpired reservation into an
/// ambiguous duplicate.
pub struct IngressReservation {
    token: IngressReservationToken,
    generation: ReservationGeneration,
    /* bound intent commitment, principal scope, operation family, authority
     * lineage, and retention contract */
}

/// The recoverable token of one reservation. An ingress identity
/// relationship only — no grant, no admission, no Attempt, no checkpoint,
/// no proof of truth.
pub struct IngressReservationToken {
    /* closes with the identity profile */
}

/// Generation of one reservation relationship, for expected-generation
/// admission of reservation state changes.
pub struct ReservationGeneration {
    /* closes with the identity profile */
}

/// How long a reservation token may authorize or correlate admission. After
/// it passes, a retry may refuse as expired; it may never silently become a
/// fresh intent.
pub struct TokenUsabilityHorizon {
    /* a Time-class bound; closes with the depot profile */
}

/// How long the system must still recognize reuse of the nonce identity —
/// strictly independent of token usability. It never closes before the same
/// reservation's `TokenUsabilityHorizon`: a still-usable token whose duplicate
/// recognition had lapsed would let a retry become a fresh intent, and guarded
/// construction refuses the inversion.
pub struct DuplicateRecognitionHorizon {
    /* a Time-class bound; closes with the depot profile */
}

/// The compact retired-identity record that outlives token usability and
/// prevents identity resurrection until the duplicate-recognition horizon
/// closes.
pub struct ReservationTombstone {
    /* compact identity commitment; closes with the identity profile */
}

/// Identity of one admitted foreign claim.
pub struct ClaimIdentity {
    /* closes with the identity profile */
}

/// Stage witness: framed foreign input was received, with bounded physical
/// observations. The wrong type to discharge any retry duty.
pub struct ReceivedClaim {
    /* framed input identity and bounded observations */
}

/// Stage witness: protocol, authentication, replay, role, and firewall
/// checks passed. Still the wrong type to discharge any retry duty. A
/// progress witness states exactly what survived — recorded stage metadata
/// is not crash-recoverable claim bytes unless it says so.
pub struct ValidatedClaim {
    /* validation evidence and survival posture */
}

/// The durable-custody publication boundary of one admitted claim: evidence
/// that the claim's custody bytes actually became durable. Custody evidence
/// only — not domain truth, not an accepted event.
pub struct ClaimCustodyCommitment {
    /* custody publication binding; closes with the storage contract */
}

/// Durable ingress custody of one foreign claim. The ClaimFirst terminal
/// fact: its receipt — and nothing earlier — discharges the sender's
/// claim-submission retry duty. Claim admission is not domain truth, not an
/// accepted event, not a process run, and not view progress.
pub struct AdmittedClaim {
    identity: ClaimIdentity,
    /// Which submission's retry duty this custody fact can discharge.
    submission: SubmissionIdentity,
    custody: ClaimCustodyCommitment,
}

/// The retry-discharging receipt of one ClaimFirst submission: bound to the
/// exact submission identity and the durable custody boundary — the two
/// facts that make "this exact submission's duty is discharged" checkable.
#[must_use]
pub struct ClaimAdmissionReceipt {
    identity: ClaimIdentity,
    submission: SubmissionIdentity,
    custody: ClaimCustodyCommitment,
}

/// The retry-discharging receipt of one DomainFirst submission (draft
/// spelling). An `AcceptedEvent` alone does not prove *this* submission's
/// duty was discharged; the receipt binds the submission's idempotency
/// identity to the accepted identity and its exact publication boundary,
/// under the operation family it was submitted through.
#[must_use]
pub struct DomainAdmissionReceipt {
    submission: SubmissionIdentity,
    event: EventId,
    commit: CommitPoint,
    operation: IngressOperationFamilyId,
}

/// The recorded outcome of one admitted claim's processing obligation. The
/// custody fact is never deleted; what closes is the obligation. Never
/// recorded before the outcome actually exists.
///
/// `DomainRefusal` is the application's typed domain-refusal family, carried
/// generically exactly as `Decision::Defer` carries its demand (core logic
/// precedent): ingress records it verbatim and interprets nothing. It is an
/// application-constructed value under the application's own redaction
/// discipline — never raw foreign bytes, which the rejected-content law
/// already forbids inside any refusal value.
pub enum ClaimResolution<DomainRefusal> {
    /// The claim was interpreted and its proposal was accepted into history.
    ResolvedAsAcceptedEvent {
        claim: ClaimIdentity,
        event: EventId,
    },
    /// The claim was interpreted and refused by the application's domain
    /// interpretation. The refusal that explains what happened rides the
    /// record — a resolution may never lose it.
    ResolvedWithDomainRefusal {
        claim: ClaimIdentity,
        refusal: DomainRefusal,
    },
}

/// The lawful dispositions of rejected foreign content. The default is the
/// bounded, typed, redacted diagnostic.
pub enum RejectedContentDisposition {
    /// Retain nothing beyond the typed refusal.
    Forget,
    /// Retain a bounded, classification-aware, redacted diagnostic. Any
    /// fingerprint is keyed and scope-bound — never a public unkeyed digest
    /// of low-entropy input.
    MinimalProtectedAudit,
    /// Opt-in raw retention through the quarantine port, under four
    /// guardrails: bounded; expiring with real deletion; access-controlled;
    /// never directly re-admittable.
    ProtectedQuarantine,
}

/// The bounded, redacted diagnostic evidence of one rejection. Carries no
/// raw attacker bytes.
pub struct RejectedContentDiagnostic {
    /* bounded classification-aware evidence; keyed scope-bound fingerprint
     * where retained */
}

/// A declared intent to retain rejected material in protected quarantine.
/// Crosses to the outside world via the port owner's quarantine contract;
/// quarantined material never enters accepted history and is never directly
/// re-admitted.
pub struct QuarantineIntent {
    disposition: RejectedContentDisposition,
    /* bounded extent and retention binding */
}

/// The typed refusal family of ingress.
pub enum IngressRefusal {
    /// Same nonce, conflicting intent. The original relationship is
    /// unchanged — no overwrite, no second token.
    ReservationConflict,
    /// A reservation bound is exhausted. Never resolved by evicting an
    /// unexpired reservation.
    ReservationCapacityExhausted,
    /// Reservation creation rate exceeded.
    ReservationRateExceeded,
    /// The nonce exceeds `ClientNonceByteLimit`.
    NonceExceedsByteLimit,
    /// The token's usability horizon has passed. The retry does not become
    /// a fresh intent.
    ReservationExpired,
    /// Effectful ingress with no lawful idempotency identity refuses before
    /// admission.
    NoLawfulIdempotencyIdentity,
    /// Validation failed; the rejected-content disposition governs what is
    /// retained.
    ClaimValidationRefused,
    /// A claim resolution was submitted before its outcome actually existed
    /// (the referenced event is not accepted). Never recorded.
    ResolutionBeforeOutcome,
    /// The claim's processing obligation is already closed; a second
    /// resolution never overwrites the first.
    ClaimAlreadyResolved,
    /* the roster closes with the ingress contract */
}

// ---------------------------------------------------------------------------
// Bounds — owner-local limits. Numeric values and paved profiles live in the
// depot. None of these is affine; each is a plain declared limit consumed by
// the operation that names it. (`RecoveryScanBudget` and the reservation
// work budgets become affine only if duplication could fabricate capacity —
// decided at the guard pass, per bound.)
// ---------------------------------------------------------------------------

/// Maximum canonical bytes of one event.
pub struct EventByteLimit(NonZeroU64);

/// Maximum events in one admission batch.
pub struct BatchEventLimit(NonZeroU32);

/// Maximum inline causal parents of one event (the fan-in bound,
/// owner-ruled 2026-08-24).
pub struct CausalParentLimit(NonZeroU32);

/// Maximum admitted unresolved causal claims on one event. A bounded roster
/// like every admitted roster — an unbounded claim list would be a foreign
/// storage grant.
pub struct UnresolvedCausalClaimLimit(NonZeroU32);

/// Maximum participating authorities of one `SourceSet`.
pub struct FederationSourceLimit(NonZeroU32);

/// Bounded work allowance for one recovery scan.
pub struct RecoveryScanBudget(NonZeroU64);

/// Maximum live reservations overall.
pub struct ReservationCountLimit(NonZeroU64);

/// Maximum retained reservation bytes overall.
pub struct ReservationByteLimit(NonZeroU64);

/// Maximum live reservations per principal (including the bounded anonymous
/// bucket).
pub struct ReservationsPerPrincipalLimit(NonZeroU32);

/// Maximum live reservations per tenant scope.
pub struct ReservationsPerTenantLimit(NonZeroU32);

/// Maximum live reservations per operation family.
pub struct ReservationsPerOperationLimit(NonZeroU32);

/// Maximum reservation creations per declared window.
pub struct ReservationCreationRateLimit(NonZeroU32);

/// Bounded work allowance for one reservation lookup.
pub struct ReservationLookupWorkBudget(NonZeroU64);

/// Bounded work allowance for one reservation creation.
pub struct ReservationCreateWorkBudget(NonZeroU64);

/// Maximum bytes of one `ClientNonce`.
pub struct ClientNonceByteLimit(NonZeroU32);

/// Maximum bytes of one `IngressReservationToken`.
pub struct ReservationTokenByteLimit(NonZeroU32);

/// Maximum age of one active reservation.
pub struct ActiveReservationAgeLimit {
    /* a Time-class bound; closes with the depot profile */
}

/// Maximum retained conflict evidence per reservation.
pub struct ConflictEvidenceLimit(NonZeroU32);

// ---------------------------------------------------------------------------
// Profiles — this owner's configuration algebra. A profile is a typed bundle
// of the owner's selectable facts; operations receive the exact profile as an
// explicit argument, selected values are depot rows (depot/event.md), and no
// operation fetches a profile ambiently. A profile selects coordinates inside
// the algebra declared here; it can never widen it.
// ---------------------------------------------------------------------------

/// The admission profile: the bounds one event admission runs under.
/// Reserved-class roster and genesis posture close with the admission
/// contract.
pub struct EventAdmissionProfile {
    event_bytes: EventByteLimit,
    batch_events: BatchEventLimit,
    causal_parents: CausalParentLimit,
    unresolved_claims: UnresolvedCausalClaimLimit,
}

/// The federation profile: the bounds multi-source claims run under.
pub struct FederationProfile {
    sources: FederationSourceLimit,
}

/// The reservation profile: every bound of the ingress reservation family
/// plus the two horizons. Guarded construction refuses a
/// `DuplicateRecognitionHorizon` that closes before the
/// `TokenUsabilityHorizon`.
pub struct ReservationProfile {
    count: ReservationCountLimit,
    bytes: ReservationByteLimit,
    per_principal: ReservationsPerPrincipalLimit,
    per_tenant: ReservationsPerTenantLimit,
    per_operation: ReservationsPerOperationLimit,
    creation_rate: ReservationCreationRateLimit,
    lookup_work: ReservationLookupWorkBudget,
    create_work: ReservationCreateWorkBudget,
    nonce_bytes: ClientNonceByteLimit,
    token_bytes: ReservationTokenByteLimit,
    active_age: ActiveReservationAgeLimit,
    conflict_evidence: ConflictEvidenceLimit,
    token_usability: TokenUsabilityHorizon,
    duplicate_recognition: DuplicateRecognitionHorizon,
}

/// The recovery profile: the work one recovery pass may consume.
pub struct RecoveryProfile {
    scan: RecoveryScanBudget,
}

/// The storage profile: the selected physical-store facts a qualified store
/// is configured with. Segment sizing, compaction posture, and generation
/// retention close with the storage contract; the profile is mechanism
/// configuration and never semantic identity.
pub struct StorageProfile {
    /* segment, compaction, and retention selections; close with the storage
     * contract */
}
