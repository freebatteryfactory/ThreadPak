//! Port owner — role declarations.
//!
//! Declarations only: the nouns of the boundary grammar. No behavior lives
//! here; the thin operation signatures live in `ops.rs` beside this file, and
//! guarded construction arrives with them. Nothing in this file claims
//! implementation support.
//!
//! Foreign roles referenced by name and declared by their own owners:
//! `AttemptId` (Bvisor), `TurnId` (runtime), `EffectIntentId` and
//! `AbsoluteDeadlineId` (runtime — minted with its durable-EffectIntent and
//! deadline families), `Cut` (event), `AcceptedHlc` (event chronology),
//! `SchemaId` and `SchemaVersion` (core schema). Their declaration seats are
//! fixed by the dependency probe, never by this file. Referencing a foreign
//! role grants nothing and mints nothing.
//!
//! No identity in this file chooses its own byte width: every identity and
//! version interior cites its row in `depot/port.md`, which cites the
//! identity/canon profile. A payload is opaque bounded bytes under a declared
//! role; the machine-wide byte-role register is the core canon owner's seam.

use core::num::{NonZeroU32, NonZeroU64};

// ---------------------------------------------------------------------------
// Identity roles
// ---------------------------------------------------------------------------

/// Identifies one port family — one closed boundary vocabulary declared by a
/// semantic owner (storage by the event owner, external effects by programs,
/// observation ports below). Role-distinct from every other identity even
/// where byte widths match.
pub struct PortFamilyId {
    /* fresh-opaque identity; width per depot/port.md row port.family-id-width
     * (identity profile) */
}

/// Identifies one operation inside one family. Meaningless without its
/// `PortFamilyId`; two families may reuse ordinals without relation.
pub struct PortOperationId {
    /* registered id; width per depot/port.md row port.registered-id-width —
     * that row preserves a recovered contradiction (u16 register law vs the
     * previously declared u32) */
}

/// Identifies one physical request instance. Minted once per issued request,
/// never reused across Attempts, retries, or reconnects. A carrier-scoped
/// correlation identity never substitutes for it.
pub struct PortRequestId {
    /* fresh-opaque identity; width per depot/port.md row
     * port.request-id-width */
}

/// Carrier-scoped correlation identity supplied by a transport adapter.
/// It correlates frames on one carrier and nothing else: it never
/// substitutes for `PortRequestId`, never binds an Attempt, and never
/// carries authority across the boundary. Equal widths with `PortRequestId`
/// are exactly the substitution hazard the role split exists to refuse.
pub struct CarrierRequestId {
    /* fresh-opaque identity; width per depot/port.md row
     * port.request-id-width */
}

/// Version of one family's declared contract. A response role, refusal
/// role, or bound from one version never validates material issued under
/// another without an explicit, fallible conversion. Role-distinct from
/// every other version line — a contract version upgrades nothing else.
pub struct PortContractVersion {
    /* scoped version scalar (Class C); width per depot/port.md row
     * port.version-scalar-width */
}

// ---------------------------------------------------------------------------
// Contract declarations
// ---------------------------------------------------------------------------

/// The declared boundary of one port family: its closed operation set, its
/// closed value-role roster, and the laws every operation in it obeys.
/// Declared by the family's semantic owner using this grammar; validated by
/// `declare_contract` before any request is admitted. A contract declaration
/// is data. It grants nothing and performs nothing.
pub struct PortContract {
    family: PortFamilyId,
    version: PortContractVersion,
    /// Closed set: an operation outside it does not exist at this boundary.
    operations: PortOperationSet,
    /// Closed set: the roster `PortContractRefusal::UnknownValueRole` checks
    /// against. A role not bound here does not exist at this boundary.
    roles: PortValueRoleSet,
    request_byte_limit: PortRequestByteLimit,
    response_byte_limit: PortResponseByteLimit,
}

/// A family contract that survived `declare_contract` validation. Private
/// construction: possession proves the declaration's rosters close, its
/// recovery routes exist, and its bounds fit the admitted profile. Requests
/// are validated only against an admitted contract, never a raw declaration.
#[must_use]
pub struct AdmittedPortContract {
    contract: PortContract,
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
pub struct PortValueRole {
    /* registered id; width per depot/port.md row port.registered-id-width */
}

/// Binds one declared value role to the schema commitment that gives its
/// bytes meaning. The binding is contract data; it grants nothing.
/// (`SchemaId` and `SchemaVersion` are core-schema roles.)
pub struct PortValueRoleBinding {
    role: PortValueRole,
    schema: SchemaId,
    schema_version: SchemaVersion,
}

/// The closed roster of value-role bindings one contract declares.
pub struct PortValueRoleSet(/* guarded, bounded */ Vec<PortValueRoleBinding>);

// ---------------------------------------------------------------------------
// Recovery contracts — operation-specific, never one universal enum
// ---------------------------------------------------------------------------

/// One operation's declared recovery posture, from the five-posture
/// vocabulary: same-key idempotent, queryable outcome, compensatable,
/// at-least-once, nonreplayable. Each capability binds its exact routes;
/// a bare tag with no bindings is not a recovery contract. Every claimed
/// route must exist before the irreversible Attempt.
///
/// The recovered corpus carries wider rosters for this declaration (a
/// five-axis and a nine-property form); those rows are preserved in
/// `depot/port.md` and widen this shape only when the runtime's
/// effect-admission pass consumes them.
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
pub struct PortGrantGeneration {
    /* scoped generation scalar (Class C); width per depot/port.md row
     * port.version-scalar-width */
}

/// Generation of one quarantine-grant relationship. Role-distinct from
/// `PortGrantGeneration` even where widths match.
pub struct QuarantineGrantGeneration {
    /* scoped generation scalar (Class C); width per depot/port.md row
     * port.version-scalar-width */
}

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
/// Attempt, one grant, and one carried deadline allowance. Issued once; a
/// retry is a new request under a fresh Attempt.
pub struct PortRequest {
    request: PortRequestId,
    family: PortFamilyId,
    contract_version: PortContractVersion,
    operation: PortOperationId,
    /// The one physical effort this request belongs to (Bvisor-owned role).
    attempt: AttemptId,
    /// Authority this request executes under, at issue-time generation.
    grant_generation: PortGrantGeneration,
    /// Carried, never minted here: the operation's deadline allowance,
    /// rebased into a named clock domain. Nothing downstream extends it.
    deadline: CarriedAbsoluteDeadline,
    /// Request payload under the operation's declared request role.
    payload: PortRequestPayload,
}

/// A request that survived pre-flight validation against its admitted
/// contract and grant. Private construction: possession proves the family,
/// operation, version, grant, bounds, and deadline checks ran. The only
/// request form an adapter may dispatch. (Draft spelling; renaming changes
/// no law.)
#[must_use]
pub struct DispatchableRequest {
    request: PortRequest,
}

/// The deadline allowance as carried across the boundary. Minting and
/// enforcement belong to the runtime deadline owner; this role exists so no
/// adapter signature can accept a request without one, and so a value from
/// one clock domain cannot wander into another. What crosses is a derived
/// allowance rebased into this boundary's clock domain — never a
/// transplanted raw instant, and never the `DeadlinePolicy` itself. Every
/// derivation narrows; retry, reconnect, and carrier conversion cannot
/// reset it.
pub struct CarriedAbsoluteDeadline {
    /// The runtime-owned deadline commitment this allowance derives from.
    source: AbsoluteDeadlineId,
    /// The clock domain the allowance is rebased into. Cross-domain use
    /// refuses at validation; there is no implicit conversion.
    domain: ClockDomainId,
    /// The clock profile the rebase was performed under.
    profile: ClockProfileVersion,
    allowance: CarriedAllowance,
}

/// The two lawful allowance forms. Both only narrow.
pub enum CarriedAllowance {
    /// The absolute expiry point, rebased into the named domain.
    AbsoluteExpiry {
        /* expiry point in the named domain; representation closes with the
         * clock profile rows (depot/port.md) */
    },
    /// A narrower remaining allowance derived from the expiry.
    RemainingAllowance {
        /* unsigned span (core Duration role); representation closes with
         * the clock profile rows */
    },
}

/// Request bytes under a declared role, within the family's request byte
/// limit. Opaque to adapters beyond what the role admits.
pub struct PortRequestPayload(/* guarded, bounded */ Vec<u8>);

/// Foreign response material as received: a claim, not a response. It
/// becomes a `ValidatedResponse` only through response validation, or a
/// typed refusal. Holding one proves delivery of bytes and nothing else —
/// foreign bytes have exactly one lawful morphism, validation.
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
    contract_version: PortContractVersion,
    grant_generation: PortGrantGeneration,
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

/// Identifies one physical clock domain: one clock lineage (one boot of one
/// clock kind), minted fresh, never derived from content. Values from
/// different domains never compare, subtract, or rebase across domains
/// without an explicit fallible conversion declared by the temporal owner
/// consuming them.
pub struct ClockDomainId {
    /* fresh-opaque identity; width per depot/port.md row
     * port.clock-domain-id-width */
}

/// Version of one clock source's declared profile. Role-distinct from
/// `PortContractVersion` — a contract version and a clock profile version
/// answer different questions and never substitute.
pub struct ClockProfileVersion {
    /* scoped version scalar (Class C); width per depot/port.md row
     * port.version-scalar-width */
}

/// Declared profile of one clock source. A profile is a claim roster the
/// harness qualifies — a compile is not a clock. Every field is a declared
/// fact; none is inferred from a host library.
pub struct ClockSourceProfile {
    domain: ClockDomainId,
    profile: ClockProfileVersion,
    /// The declared reading resolution.
    resolution: DeclaredResolution,
    /// What monotonicity, if any, this source claims.
    monotonicity: MonotonicityClaim,
    /// The declared behavior when the source moves backward.
    regression: RegressionBehavior,
    /// The declared behavior across host suspend and resume.
    suspend: SuspendBehavior,
    /// How observations without stated uncertainty are treated.
    uncertainty: UncertaintyPosture,
}

/// The declared reading resolution of one clock source.
pub struct DeclaredResolution {
    /* declared granularity; roster closes with the clock profile rows
     * (depot/port.md) */
}

/// The monotonicity claim one clock source declares.
pub struct MonotonicityClaim {
    /* declared claim; roster closes with the clock profile rows */
}

/// The declared regression behavior of one clock source.
pub struct RegressionBehavior {
    /* declared behavior; roster closes with the clock profile rows */
}

/// The declared suspend/resume behavior of one clock source.
pub struct SuspendBehavior {
    /* declared behavior; roster closes with the clock profile rows */
}

/// How an observation whose uncertainty is unknown is treated. Unknown
/// uncertainty is never zero uncertainty. Safety-relevant: the paved
/// posture is refusal (depot/port.md row port.uncertainty-posture).
pub enum UncertaintyPosture {
    /// Assume the profile's configured maximum uncertainty.
    DeclaredMaximum {
        /* configured maximum; value in depot/port.md, currently withheld */
    },
    /// Refuse the observation.
    RefuseUnknown,
}

/// A raw wall-time observation: foreign temporal material. It becomes
/// accepted chronology only through the chronology owner's admission.
/// It is never an `AcceptedHlc`, a `CommitPoint`, an order, or a Cut.
/// Whether a reading carries per-observation uncertainty bounds or leans on
/// the profile's posture is a recorded escalation (README, Escalation 1).
pub struct RawWallObservation {
    domain: ClockDomainId,
    /// Uninterpreted source reading; interpretation belongs to admission.
    /// Representation closes with the clock profile rows (depot/port.md).
    reported: WallReading,
}

/// The uninterpreted wall reading role.
pub struct WallReading {
    /* signed reading; representation closes with the clock profile rows */
}

/// A raw monotonic observation in one clock domain. Process-local
/// physics: it never serializes into durable state and never crosses
/// clock domains. Consumed by the runtime deadline owner's rebase.
pub struct RawMonotonicObservation {
    domain: ClockDomainId,
    /// Representation closes with the clock profile rows (depot/port.md).
    reported: MonotonicReading,
}

/// The uninterpreted monotonic reading role.
pub struct MonotonicReading {
    /* unsigned reading; representation closes with the clock profile rows */
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

/// Names the domain and profile a wall observation is requested under.
pub struct WallObservationRequest {
    domain: ClockDomainId,
    profile: ClockProfileVersion,
}

/// Names the domain and profile a monotonic observation is requested under.
pub struct MonotonicObservationRequest {
    domain: ClockDomainId,
    profile: ClockProfileVersion,
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
pub struct QuarantineDispositionRef {
    /* fresh-opaque identity; width per depot/port.md row
     * port.request-id-width */
}

/// The declared retention envelope for one quarantine store: the four
/// guardrail ceilings — count, bytes, age, and work — and the deletion
/// route that makes expiry real. Key-shred deletion is lawful only where
/// the quarantine holds its own key scope and the key authority was
/// actually destroyed.
pub struct QuarantineRetention {
    max_items: QuarantineItemLimit,
    max_bytes: QuarantineByteLimit,
    max_age: QuarantineAgeLimit,
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
// Profiles — the owner's configuration algebra. Selected values live in
// depot/port.md and are passed into operations explicitly; no operation
// reaches into an ambient registry.
// ---------------------------------------------------------------------------

/// The admitted ceilings a family declaration's bounds must fit — the
/// profile `PortContractRefusal::BoundOutOfProfile` checks against.
/// Selected values live in the depot; this type is the algebra.
pub struct PortBoundsProfile {
    max_request_bytes: PortRequestByteLimit,
    max_response_bytes: PortResponseByteLimit,
    max_operations: PortOperationCountLimit,
    max_roles: PortValueRoleCountLimit,
}

// ---------------------------------------------------------------------------
// Bounds — declared by contracts, enforced at physical admission
// ---------------------------------------------------------------------------

/// Ceiling on one request's payload bytes. Output class.
pub struct PortRequestByteLimit(NonZeroU64);

/// Ceiling on one response's accepted bytes; material beyond it refuses
/// before allocation. Result class.
pub struct PortResponseByteLimit(NonZeroU64);

/// Ceiling on the operations one family contract may declare. Memory class.
pub struct PortOperationCountLimit(NonZeroU32);

/// Ceiling on the value-role bindings one family contract may declare.
/// Memory class.
pub struct PortValueRoleCountLimit(NonZeroU32);

/// Ceiling on quarantined items retained under one disposition. Memory
/// class.
pub struct QuarantineItemLimit(NonZeroU32);

/// Ceiling on quarantined bytes retained under one disposition. Memory
/// class.
pub struct QuarantineByteLimit(NonZeroU64);

/// Ceiling on the age of quarantined material. Time class: the age names
/// its clock domain — a tick count without a domain is not an age.
pub struct QuarantineAgeLimit {
    /* domain-bound span; closes with the clock profile rows
     * (depot/port.md) */
}

/// Ceiling on the work one quarantine store or expiry pass may consume —
/// the fourth guardrail dimension the quarantine contract promises.
/// Work class.
pub struct QuarantineWorkLimit(NonZeroU64);

// ---------------------------------------------------------------------------
// Refusals — role-specific; every variant is reachable by a public input
// ---------------------------------------------------------------------------

/// Refusal of a family contract declaration.
pub enum PortContractRefusal {
    EmptyOperationSet,
    DuplicateOperation,
    /// An operation names a value role absent from the contract's roster.
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
    /// The carried allowance names a clock domain this boundary does not
    /// hold; cross-domain deadlines never convert implicitly.
    DeadlineDomainMismatch,
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
    /// The requested profile is not the declared profile of the domain.
    ProfileMismatch,
    SourceUnavailable,
    ProfileViolation,
    /// The source stated no uncertainty and the profile's posture refuses
    /// unknown uncertainty.
    UncertaintyUnknown,
}

/// Refusal of a monotonic observation.
pub enum MonotonicObservationRefusal {
    UnknownDomain,
    ProfileMismatch,
    SourceUnavailable,
    ProfileViolation,
}

/// Refusal of a quarantine store.
pub enum QuarantineRefusal {
    GrantAbsent,
    RetentionCeilingReached,
    /// The store's work ceiling was exhausted before completion.
    WorkCeilingExhausted,
    DeletionRouteUnavailable,
    PayloadOverLimit,
}
