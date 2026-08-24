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
//! Cross-home types (payload values, schema bindings, knowledge axes, grants
//! of other owners) are referenced in doc comments, not as fields, until the
//! dependency probe seats the real imports.

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

/// A proposed fact. It carries no authority, no position, and no chronology;
/// only admission can turn it into an `AcceptedEvent`. Public writers cannot
/// propose reserved internal event classes — authority is validated before
/// frame construction.
pub struct EventProposal {
    /* coordinate binding, payload reference, declared class, proposed
     * causation; payload and schema binding are value/schema-owner types */
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
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
    sequence: AuthoritySequence,
    predecessor: ImmediateHistoryPredecessor,
    /* admitted chronology evidence (AcceptedHlc, optional SourceHlc),
     * causation, and the payload/schema binding complete the record */
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

/// The declared local admission policy: regression posture, future posture,
/// counter behavior, and profile identity.
pub struct ChronologyPolicy {
    /* closes with the chronology profile */
}

/// The prior admitted chronology state consumed by pure admission. Owned by
/// the thin stateful shell; the pure operation returns the successor and
/// commits nothing.
pub struct ChronologyState {
    /* closes with the chronology profile */
}

/// The result of one pure chronology admission: the successor state, the
/// admitted value, preserved source evidence, and admission evidence.
pub struct ChronologyAdvance {
    next: ChronologyState,
    accepted: AcceptedHlc,
    /* preserved SourceHlc where supplied, plus admission evidence */
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

/// An admitted, unverifiable foreign causal assertion. It never counts as
/// `DomainCausation`, never closes a causal traversal, and never establishes
/// causal completeness. Later evidence may admit a separate resolved
/// relationship (owner-ruled 2026-08-24).
pub struct UnresolvedCausalClaim {
    /* asserted foreign cause reference and its provenance; the foreign
     * reference role closes with the identity profile */
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

/// The spatial proof of one lawful split: children pairwise disjoint, union
/// equal to the complete parent region, no gap, no overlap. Inherited events
/// retain their identities and original positions; children own only fresh
/// suffixes under fresh epochs.
pub struct SplitWitness {
    parent: RegionSealWitness,
    /* child region roster plus the disjointness and coverage proof; closes
     * with the region-geometry contract */
}

/// Explicit evidence relating positions across an authority succession. A
/// parent cut and a child cut are never interchangeable without it.
pub struct CutSuccessionWitness {
    /* predecessor and successor scope bindings plus the exact translated
     * boundary */
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
    /* successor roster; closes with the deployment contract */
}

/// Evidence that one complete handoff — seal, succession, activation,
/// route-last publication — occurred lawfully.
#[must_use]
pub struct PartitionHandoffReceipt {
    /* binds the seal, split witness, succession evidence, activations, and
     * routing publication */
}

// ---------------------------------------------------------------------------
// Removal owner
// ---------------------------------------------------------------------------

/// Authorizes participation in one removal ladder. Role-specific; a grant
/// is not an admission — the ladder still mints its own affine authority.
pub struct RemovalGrant {
    /* scope, generation, and attenuation close with the guard pass */
}

/// A caller-authored removal request. It grants nothing.
pub struct RemovalPlan {
    /* exact scope and cut binding of the requested destruction */
}

/// Affine, boundary-minted authority for one exact destructive operation.
/// Consumed by the crossing it authorizes; never cloneable, never reusable.
#[must_use]
pub struct RemovalAdmission {
    /* binds the plan, the exact scope and cut, and the granting authority */
}

/// The fact that the destructive boundary actually crossed. Authorized
/// removal is never historical absence: never-existed, incomplete, corrupt,
/// unauthorized, shredded, lawfully-removed, and not-retained remain
/// distinguishable answers (the read-outcome family is declared with the
/// query surface).
pub struct RemovalCommitment {
    /* binds the admission and the exact destroyed extent */
}

/// Evidence of one completed removal ladder.
#[must_use]
pub struct RemovalReceipt {
    /* binds plan, admission, and commitment identities */
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

/// Evidence of one durable append publication: which proposals became
/// accepted, at which positions, through which commit point. Claims exactly
/// the publication boundary and nothing more — not namespace publication,
/// not derived progress, not checkpoint advancement.
#[must_use]
pub struct AppendReceipt {
    commit: CommitPoint,
    /* accepted identities and positions of the published batch */
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

/// Evidence of one recovery pass: the exact recovered boundary and the
/// classified disposition of everything beyond it.
#[must_use]
pub struct RecoveryReceipt {
    recovered: CommitPoint,
    /* classified extents and their dispositions */
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
/// strictly independent of token usability.
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

/// Durable ingress custody of one foreign claim. The ClaimFirst terminal
/// fact: its receipt — and nothing earlier — discharges the sender's
/// claim-submission retry duty. Claim admission is not domain truth, not an
/// accepted event, not a process run, and not view progress.
pub struct AdmittedClaim {
    identity: ClaimIdentity,
    /* durable custody binding and exact admission evidence */
}

/// The retry-discharging receipt of one ClaimFirst submission.
#[must_use]
pub struct ClaimAdmissionReceipt {
    identity: ClaimIdentity,
    /* exact admission evidence */
}

/// The recorded outcome of one admitted claim's processing obligation. The
/// custody fact is never deleted; what closes is the obligation. Never
/// recorded before the outcome actually exists.
pub enum ClaimResolution {
    /// The claim was interpreted and its proposal was accepted into history.
    ResolvedAsAcceptedEvent {
        claim: ClaimIdentity,
        event: EventId,
    },
    /// The claim was interpreted and refused by the application's domain
    /// interpretation.
    ResolvedWithDomainRefusal {
        claim: ClaimIdentity,
        /* the typed domain refusal, application-owned */
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
