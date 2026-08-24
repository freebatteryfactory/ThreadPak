//! Port owner — role declarations.
//!
//! Declarations only: the nouns of the boundary grammar. No behavior lives
//! here; thin operations arrive beside this file, and guarded construction
//! arrives with them. Nothing in this file claims implementation support.
//!
//! Foreign roles referenced by name and declared by their own owners:
//! `AttemptId` (Bvisor), `EffectIntentId` and `TurnId` (runtime),
//! `Cut` (event), `AcceptedHlc` (event chronology).
//! Their declaration seats are fixed by the dependency probe, never by this
//! file. Referencing a foreign role grants nothing and mints nothing.

// ---------------------------------------------------------------------------
// Identity roles
// ---------------------------------------------------------------------------

/// Identifies one port family — one closed boundary vocabulary declared by a
/// semantic owner (storage by the event owner, external effects by programs,
/// observation ports below). Role-distinct from every other identity even
/// where byte widths match.
pub struct PortFamilyId(/* guarded */ u128);

/// Identifies one operation inside one family. Meaningless without its
/// `PortFamilyId`; two families may reuse ordinals without relation.
pub struct PortOperationId(/* guarded */ u32);

/// Identifies one physical request instance. Minted once per issued request,
/// never reused across Attempts, retries, or reconnects. A carrier-scoped
/// correlation identity never substitutes for it.
pub struct PortRequestId(/* guarded */ u128);

/// Carrier-scoped correlation identity supplied by a transport adapter.
/// It correlates frames on one carrier and nothing else: it never
/// substitutes for `PortRequestId`, never binds an Attempt, and never
/// carries authority across the boundary.
pub struct CarrierRequestId(/* guarded */ u128);

/// Version of one family's declared contract. A response role, refusal
/// role, or bound from one version never validates material issued under
/// another without an explicit, fallible conversion.
pub struct PortContractVersion(/* guarded */ u32);

// ---------------------------------------------------------------------------
// Contract declarations
// ---------------------------------------------------------------------------

/// The declared boundary of one port family: its closed operation set and
/// the laws every operation in it obeys. Declared by the family's semantic
/// owner using this grammar; validated before any request is admitted.
/// A contract declaration is data. It grants nothing and performs nothing.
pub struct PortContract {
    family: PortFamilyId,
    version: PortContractVersion,
    /// Closed set: an operation outside it does not exist at this boundary.
    operations: PortOperationSet,
    request_byte_limit: PortRequestByteLimit,
    response_byte_limit: PortResponseByteLimit,
}

/// The closed roster of operations one contract declares. Bounded;
/// membership is exact, never inferred from adapter capability.
pub struct PortOperationSet(/* guarded, bounded */ Vec<PortOperation>);

/// One operation inside a family: which request role it consumes, which
/// response role it produces, which refusal role it emits, and its
/// operation-specific recovery contract.
pub struct PortOperation {
    operation: PortOperationId,
    request_role: PortValueRole,
    response_role: PortValueRole,
    refusal_role: PortValueRole,
    recovery: RecoveryContract,
}

/// Names one schema-bound value role crossing the boundary. A role is a
/// semantic commitment, not a byte layout; equal bytes under different
/// roles never substitute.
pub struct PortValueRole(/* guarded */ u64);

// ---------------------------------------------------------------------------
// Recovery contracts — operation-specific, never one universal enum
// ---------------------------------------------------------------------------

/// One operation's declared recovery posture, from the five-posture
/// vocabulary: same-key idempotent, queryable outcome, compensatable,
/// at-least-once, nonreplayable. Each capability binds its exact routes;
/// a bare tag with no bindings is not a recovery contract. Every claimed
/// route must exist before the irreversible Attempt.
pub struct RecoveryContract {
    /// Present only when the external system honors same-key retry.
    same_key: Option<SameKeyIdempotency>,
    /// Present only when a real outcome-query operation exists.
    outcome_query: Option<OutcomeQueryRoute>,
    /// Present only when a real compensating operation exists.
    compensation: Option<CompensationRoute>,
    replay: ReplaySafety,
}

/// Same-key idempotency: which request role carries the key the external
/// system deduplicates on. The key is identity for the external operation
/// only — it is not a grant, not ingress identity, and not a checkpoint.
pub struct SameKeyIdempotency {
    key_role: PortValueRole,
}

/// The declared operation that can later establish outcome knowledge for
/// this operation. Querying is a fresh port operation under its own
/// Attempt; it never resumes the original one.
pub struct OutcomeQueryRoute {
    family: PortFamilyId,
    operation: PortOperationId,
}

/// The declared operation that compensates this one. Compensation is a new
/// effect with its own intent and Attempt — never a rollback of history.
pub struct CompensationRoute {
    family: PortFamilyId,
    operation: PortOperationId,
}

/// Whether an operation tolerates duplicate physical delivery.
pub enum ReplaySafety {
    /// Duplicates are tolerated by external semantics; reconciliation
    /// deduplicates by declared identity.
    AtLeastOnceTolerated,
    /// A duplicate physical execution is unlawful; retry requires the
    /// same-key or outcome-query route, or remains `OutcomeUnknown`.
    Nonreplayable,
}

// ---------------------------------------------------------------------------
// Authority
// ---------------------------------------------------------------------------

/// Generation of one port-grant relationship. Role-specific — no shared
/// authority-generation type exists until two real grant families prove
/// identical attenuation behavior (the core owner's extraction bar).
pub struct PortGrantGeneration(/* guarded */ u64);

/// Generation of one quarantine-grant relationship. Role-distinct from
/// `PortGrantGeneration` even where widths match.
pub struct QuarantineGrantGeneration(/* guarded */ u64);

/// The admitted authority to attempt operations of one family under one
/// scope, current under one grant generation. This is the admitted
/// record; the live installed handle is Bvisor custody and is not this
/// type. A serialized grant is data — decoding one installs nothing.
#[must_use]
pub struct PortGrant {
    family: PortFamilyId,
    /// Exact permitted operations; never widened by composition.
    operations: PortOperationSet,
    generation: PortGrantGeneration,
}

/// The admitted authority to store or read quarantined foreign material.
/// Distinct from every other grant; audit and debug access is explicit,
/// and no quarantine grant authorizes re-admission.
#[must_use]
pub struct QuarantineGrant {
    generation: QuarantineGrantGeneration,
}

// ---------------------------------------------------------------------------
// Requests and responses
// ---------------------------------------------------------------------------

/// One physical request: exactly one operation of one family, bound to one
/// Attempt, one grant, and one absolute deadline. Issued once; a retry is
/// a new request under a fresh Attempt.
pub struct PortRequest {
    request: PortRequestId,
    family: PortFamilyId,
    contract_version: PortContractVersion,
    operation: PortOperationId,
    /// The one physical effort this request belongs to (Bvisor-owned role).
    attempt: AttemptId,
    /// Authority this request executes under, at issue-time generation.
    grant_generation: PortGrantGeneration,
    /// Carried, never minted here: the operation's absolute deadline or a
    /// narrower allowance derived from it. Nothing downstream extends it.
    deadline: CarriedAbsoluteDeadline,
    /// Request payload under the operation's declared request role.
    payload: PortRequestPayload,
}

/// The absolute deadline as carried across the boundary. Minting and
/// enforcement belong to the runtime deadline owner; this role exists so
/// no adapter signature can accept a request without one.
pub struct CarriedAbsoluteDeadline(/* guarded */ u64);

/// Request bytes under a declared role, within the family's request byte
/// limit. Opaque to adapters beyond what the role admits.
pub struct PortRequestPayload(/* guarded, bounded */ Vec<u8>);

/// Foreign response material as received: a claim, not a response. It
/// becomes a `ValidatedResponse` only through response validation, or a
/// typed refusal. Holding one proves delivery of bytes and nothing else.
pub struct ForeignResponse {
    /// The request the carrier claims this answers — a claim to verify,
    /// not a binding.
    claimed_request: PortRequestId,
    carrier: CarrierRequestId,
    payload: /* bounded */ Vec<u8>,
}

/// A response validated against the exact request, Attempt, response role,
/// contract version, and grant generation it was issued under. The only
/// response form semantic work ever receives.
#[must_use]
pub struct ValidatedResponse {
    request: PortRequestId,
    attempt: AttemptId,
    role: PortValueRole,
    payload: /* bounded, role-checked */ Vec<u8>,
}

/// An authentic response whose Attempt is no longer live. Physical
/// evidence for reconciliation; carries no resume authority and never
/// converts into a `ValidatedResponse` for any other Attempt.
pub struct LateResponseEvidence {
    request: PortRequestId,
    attempt: AttemptId,
    payload: /* bounded */ Vec<u8>,
}

// ---------------------------------------------------------------------------
// Clock observation ports — two contracts, never one
// ---------------------------------------------------------------------------

/// Identifies one physical clock domain. Values from different domains
/// never compare, subtract, or rebase across domains without an explicit
/// fallible conversion declared by the temporal owner consuming them.
pub struct ClockDomainId(/* guarded */ u64);

/// Declared profile of one clock source: identity, version, resolution,
/// monotonicity claim, regression and suspend behavior. A profile is a
/// claim roster the harness qualifies — a compile is not a clock.
pub struct ClockSourceProfile {
    domain: ClockDomainId,
    profile_version: PortContractVersion,
}

/// A raw wall-time observation: foreign temporal material. It becomes
/// accepted chronology only through the chronology owner's admission.
/// It is never an `AcceptedHlc`, a `CommitPoint`, an order, or a Cut.
pub struct RawWallObservation {
    domain: ClockDomainId,
    /// Uninterpreted source value; interpretation belongs to admission.
    reported: i128,
}

/// A raw monotonic observation in one clock domain. Process-local
/// physics: it never serializes into durable state and never crosses
/// clock domains. Consumed by the runtime deadline owner's rebase.
pub struct RawMonotonicObservation {
    domain: ClockDomainId,
    reported: u128,
}

/// The wall-observation contract. Role-specific: this trait is not a
/// clock, not chronology, and not deadline authority. One host adapter
/// may implement both observation contracts; the contracts never merge.
pub trait WallObservationPort {
    fn observe_wall(
        &mut self,
        request: WallObservationRequest,
    ) -> Result<RawWallObservation, WallObservationRefusal>;
}

/// The monotonic-observation contract. Same law as above: an adapter of
/// both contracts is implementation reuse, not a merged temporal role.
pub trait MonotonicObservationPort {
    fn observe_monotonic(
        &mut self,
        request: MonotonicObservationRequest,
    ) -> Result<RawMonotonicObservation, MonotonicObservationRefusal>;
}

/// Names the domain and profile an observation is requested under.
pub struct WallObservationRequest {
    domain: ClockDomainId,
}

/// Names the domain and profile an observation is requested under.
pub struct MonotonicObservationRequest {
    domain: ClockDomainId,
}

// ---------------------------------------------------------------------------
// Quarantine port — four guardrails
// ---------------------------------------------------------------------------

/// One request to physically store rejected foreign material under an
/// ingress-owned quarantine disposition. The disposition and its intent
/// are event-ingress facts; this boundary only realizes them, bounded.
pub struct QuarantineStoreRequest {
    /// Ingress-owned disposition identity this storage realizes.
    disposition: QuarantineDispositionRef,
    retention: QuarantineRetention,
    payload: /* bounded */ Vec<u8>,
}

/// Reference to the ingress-owned disposition fact. Declared here as a
/// reference role only; the fact itself is owned by event ingress.
pub struct QuarantineDispositionRef(/* guarded */ u128);

/// The declared retention envelope for one quarantine store: count, byte,
/// and age ceilings, and the deletion route that makes expiry real.
/// Key-shred deletion is lawful only where the quarantine holds its own
/// key scope and the key authority was actually destroyed.
pub struct QuarantineRetention {
    max_items: u32,
    max_bytes: u64,
    max_age_ticks: u64,
    max_work: QuarantineWorkLimit,
    deletion: QuarantineDeletionRoute,
}

/// How expiry becomes actual absence of the material.
pub enum QuarantineDeletionRoute {
    /// The stored bytes are physically destroyed.
    PhysicalErase,
    /// The quarantine's own key scope is destroyed, making every stored
    /// item unrecoverable. Unlawful without an owned key scope.
    OwnedKeyScopeShred,
}

/// Durable evidence that one quarantine store occurred under its declared
/// retention. Proves custody, never content validity, and never grants
/// re-admission — quarantined bytes re-enter only as a fresh foreign
/// claim through ordinary ingress validation.
#[must_use]
pub struct QuarantineReceipt {
    disposition: QuarantineDispositionRef,
}

// ---------------------------------------------------------------------------
// Bounds — declared by contracts, enforced at physical admission
// ---------------------------------------------------------------------------

/// Ceiling on one request's payload bytes. Output class.
pub struct PortRequestByteLimit(/* nonzero */ u64);

/// Ceiling on one response's accepted bytes; material beyond it refuses
/// before allocation. Result class.
pub struct PortResponseByteLimit(/* nonzero */ u64);

/// Ceiling on the work one quarantine store or expiry pass may consume —
/// the fourth guardrail dimension the quarantine contract promises.
/// Work class.
pub struct QuarantineWorkLimit(/* nonzero */ u64);

// ---------------------------------------------------------------------------
// Refusals — role-specific; exact body shapes finalize in the type pass
// ---------------------------------------------------------------------------

/// Refusal of a family contract declaration.
pub enum PortContractRefusal {
    EmptyOperationSet,
    DuplicateOperation,
    UnknownValueRole,
    MissingRecoveryRoute,
    BoundOutOfProfile,
}

/// Pre-flight refusal of one request before any physical work.
pub enum PortRequestRefusal {
    UnknownFamily,
    UnknownOperation,
    ContractVersionMismatch,
    RequestBytesOverLimit,
    GrantAbsent,
    GrantGenerationStale,
    DeadlineAlreadyExpired,
}

/// Refusal of foreign response material during validation.
pub enum PortResponseRefusal {
    UnknownRequest,
    WrongAttempt,
    RoleMismatch,
    ContractVersionMismatch,
    GenerationStale,
    ResponseBytesOverLimit,
    Malformed,
    Noncanonical,
    DuplicateDelivery,
    DeadlineExpired,
}

/// Refusal of a wall observation.
pub enum WallObservationRefusal {
    UnknownDomain,
    SourceUnavailable,
    ProfileViolation,
}

/// Refusal of a monotonic observation.
pub enum MonotonicObservationRefusal {
    UnknownDomain,
    SourceUnavailable,
    ProfileViolation,
}

/// Refusal of a quarantine store.
pub enum QuarantineRefusal {
    GrantAbsent,
    RetentionCeilingReached,
    DeletionRouteUnavailable,
    PayloadOverLimit,
}
