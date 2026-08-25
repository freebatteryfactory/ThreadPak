//! Port owner — thin operation signatures.
//!
//! Declaration form: each operation is a function-type alias (`…Fn`) stating its exact inputs, outputs, refusal family, and bounds — valid Rust, so the dependency probe sees real compiler-visible edges.
//! Bodies land with the construction cuts; each body is checked against its alias by a Macroonz-generated conformance assertion (`const _: NameFn = name;`).
//! Nothing here claims implementation support.
//! The two observation contracts (`WallObservationPort`, `MonotonicObservationPort`) are trait-shaped and live in `types.rs` — they are real substitution seams, not convenience traits.
//!
//! Depot threading: every operation receives the exact profile or contract it consumes as an ordinary argument.
//! No operation reaches into an ambient registry, and no signature accepts a god-context (`depot/README.md`, "Rows are passed, never fetched").

/// `declare_contract` — validate one family declaration against the admitted bounds profile.
///
/// Inputs: the bounds profile (depot-selected, passed in) and the raw declaration.
/// Output: the admitted contract — the only form requests are ever validated against; private construction proves the rosters close, every claimed recovery route exists, and the declared bounds fit the profile.
/// Refuses with `PortContractRefusal` (empty or duplicate operations, an operation naming a role outside the roster, a missing recovery route, a bound outside the profile).
/// Work is bounded by the profile's operation and role count ceilings; validation allocates nothing beyond them.
pub type DeclareContractFn = fn(
    profile: &PortBoundsProfile,
    contract: PortContract,
) -> Result<AdmittedPortContract, PortContractRefusal>;

/// `validate_request` — pre-flight one request before any physical work.
///
/// Inputs: the admitted contract, the grant the request claims to execute under, the current admitted monotonic observation in the deadline's clock domain (the temporal evidence "already expired" is judged against — a claim with no evidence input cannot be established), and the request.
/// Output: the dispatchable request — the only form an adapter may dispatch; private construction proves family, operation, version, grant coverage and generation, payload bound, and deadline domain checks ran.
/// Refuses with `PortRequestRefusal`; a carried deadline naming a clock domain other than the observation's refuses as `DeadlineDomainMismatch`, and an allowance the observation's enclosure proves expired refuses as `DeadlineAlreadyExpired` before dispatch.
/// Consumes the request; a refused request is returned to no one — a retry is a new request under a fresh Attempt.
pub type ValidateRequestFn = fn(
    contract: &AdmittedPortContract,
    grant: &PortGrant,
    now: &MonotonicObservation,
    request: PortRequest,
) -> Result<DispatchableRequest, PortRequestRefusal>;

/// `validate_response` — validate foreign response material against the exact outstanding request.
///
/// Inputs: the admitted contract, the outstanding request the response must answer, the response already validated for that request if one exists (the duplicate-delivery evidence — its presence makes any further foreign material a duplicate), the current admitted monotonic observation in the deadline's clock domain, and the foreign material.
/// Output: the validated response — the only response form semantic work ever receives, bound to the exact request, Attempt, role, contract version, and grant generation.
/// Refuses with `PortResponseRefusal`: wrong request or Attempt, role mismatch, stale version or generation, bytes over the contract's response limit (refused before allocation), malformed or noncanonical content, duplicate delivery (a prior validated response exists), expired deadline (proven by the observation's enclosure).
/// Foreign bytes have exactly this one lawful morphism; no other operation over them exists.
pub type ValidateResponseFn = fn(
    contract: &AdmittedPortContract,
    outstanding: &PortRequest,
    prior: Option<&ValidatedResponse>,
    now: &MonotonicObservation,
    foreign: ForeignResponse,
) -> Result<ValidatedResponse, PortResponseRefusal>;

/// `seal_late_response` — seal an authentic response whose Attempt is no longer live as physical evidence.
///
/// Inputs: the admitted contract (the role and byte-bound facts authentication is judged against — sealing without the contract could not establish either), the request the material answers, and the foreign material.
/// Output: late-response evidence for reconciliation — it resumes nothing, converts into no `ValidatedResponse`, and carries no authority.
/// Refuses with `PortResponseRefusal` when the material cannot be authenticated against the request at all: unknown request, role mismatch, malformed content, bytes over the contract's response limit.
/// Authenticity here proves delivery of bytes for a dead Attempt and nothing more.
pub type SealLateResponseFn = fn(
    contract: &AdmittedPortContract,
    outstanding: &PortRequest,
    foreign: ForeignResponse,
) -> Result<LateResponseEvidence, PortResponseRefusal>;

/// `admit_wall_observation` — strengthen one raw wall observation into the admitted enclosure (owner-ruled 2026-08-24).
///
/// Inputs: the clock source profile (depot-threaded — the declared resolution, monotonicity, regression, suspend, and uncertainty-posture rows ride this argument, swap-friendly) and the raw observation carrying its reading, requested profile, and reported uncertainty.
/// Output: the admitted `WallObservation` — earliest/latest bounds derived from the reading and its uncertainty; a stated zero uncertainty yields the degenerate point enclosure.
/// `Unstated` uncertainty resolves by the profile's posture: `DeclaredMaximum` widens the enclosure by the declared maximum, `RefuseUnknown` refuses as `UncertaintyUnknown` — never silently zero.
/// Refuses with `WallObservationRefusal`: unknown domain, a raw value whose carried profile is not this profile (`ProfileMismatch`), a reading violating the profile's declared facts (`ProfileViolation`), unknown uncertainty under the refusing posture.
/// The chronology owner's admission consumes only this admitted form, never the raw one.
pub type AdmitWallObservationFn = fn(
    source: &ClockSourceProfile,
    raw: RawWallObservation,
) -> Result<WallObservation, WallObservationRefusal>;

/// `admit_monotonic_observation` — strengthen one raw monotonic observation into the admitted enclosure.
///
/// Same law as the wall sibling; the runtime deadline owner's rebase consumes only this admitted form, never the raw one.
/// Refuses with `MonotonicObservationRefusal`.
pub type AdmitMonotonicObservationFn = fn(
    source: &ClockSourceProfile,
    raw: RawMonotonicObservation,
) -> Result<MonotonicObservation, MonotonicObservationRefusal>;

/// `quarantine_store` — physically store rejected foreign material under an ingress-owned quarantine disposition.
///
/// Inputs: the quarantine grant and the store request carrying its declared retention envelope.
/// Output: the quarantine receipt — proof of custody under the declared retention, never of content validity, never of re-admissibility.
/// Refuses with `QuarantineRefusal`: absent grant, any retention ceiling reached (count, bytes, age), work ceiling exhausted mid-store, deletion route unavailable (key-shred without an owned key scope), payload over limit.
/// All four guardrail ceilings bind this operation; expiry passes consume the same work ceiling.
pub type QuarantineStoreFn = fn(
    grant: &QuarantineGrant,
    request: QuarantineStoreRequest,
) -> Result<QuarantineReceipt, QuarantineRefusal>;
