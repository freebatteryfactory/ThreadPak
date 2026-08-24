//! Port owner — thin operation signatures.
//!
//! Signature declarations only, in semicolon form: each states its exact
//! inputs, outputs, refusal family, and bounds. Bodies land with the
//! construction cuts; nothing here claims implementation support. The two
//! observation contracts (`WallObservationPort`, `MonotonicObservationPort`)
//! are trait-shaped and live in `types.rs` — they are real substitution
//! seams, not convenience traits.
//!
//! Depot threading: every operation receives the exact profile or contract
//! it consumes as an ordinary argument. No operation reaches into an ambient
//! registry, and no signature accepts a god-context
//! (`depot/README.md`, "Rows are passed, never fetched").

/// Validate one family declaration against the admitted bounds profile.
///
/// Inputs: the bounds profile (depot-selected, passed in) and the raw
/// declaration. Output: the admitted contract — the only form requests are
/// ever validated against; private construction proves the rosters close,
/// every claimed recovery route exists, and the declared bounds fit the
/// profile. Refuses with `PortContractRefusal` (empty or duplicate
/// operations, an operation naming a role outside the roster, a missing
/// recovery route, a bound outside the profile). Work is bounded by the
/// profile's operation and role count ceilings; validation allocates
/// nothing beyond them.
pub fn declare_contract(
    profile: &PortBoundsProfile,
    contract: PortContract,
) -> Result<AdmittedPortContract, PortContractRefusal>;

/// Pre-flight one request before any physical work.
///
/// Inputs: the admitted contract, the grant the request claims to execute
/// under, and the request. Output: the dispatchable request — the only form
/// an adapter may dispatch; private construction proves family, operation,
/// version, grant coverage and generation, payload bound, and deadline
/// domain checks ran. Refuses with `PortRequestRefusal`; a carried deadline
/// naming a foreign clock domain refuses as `DeadlineDomainMismatch`, and an
/// already-expired allowance refuses before dispatch. Consumes the request;
/// a refused request is returned to no one — a retry is a new request under
/// a fresh Attempt.
pub fn validate_request(
    contract: &AdmittedPortContract,
    grant: &PortGrant,
    request: PortRequest,
) -> Result<DispatchableRequest, PortRequestRefusal>;

/// Validate foreign response material against the exact outstanding request.
///
/// Inputs: the admitted contract, the outstanding request the response must
/// answer, and the foreign material. Output: the validated response — the
/// only response form semantic work ever receives, bound to the exact
/// request, Attempt, role, contract version, and grant generation. Refuses
/// with `PortResponseRefusal`: wrong request or Attempt, role mismatch,
/// stale version or generation, bytes over the contract's response limit
/// (refused before allocation), malformed or noncanonical content, duplicate
/// delivery, expired deadline. Foreign bytes have exactly this one lawful
/// morphism; no other operation over them exists.
pub fn validate_response(
    contract: &AdmittedPortContract,
    outstanding: &PortRequest,
    foreign: ForeignResponse,
) -> Result<ValidatedResponse, PortResponseRefusal>;

/// Seal an authentic response whose Attempt is no longer live as physical
/// evidence.
///
/// Inputs: the request the material answers and the foreign material.
/// Output: late-response evidence for reconciliation — it resumes nothing,
/// converts into no `ValidatedResponse`, and carries no authority. Refuses
/// with `PortResponseRefusal` when the material cannot be authenticated
/// against the request at all: unknown request, role mismatch, malformed
/// content, bytes over limit. Authenticity here proves delivery of bytes for
/// a dead Attempt and nothing more.
pub fn seal_late_response(
    outstanding: &PortRequest,
    foreign: ForeignResponse,
) -> Result<LateResponseEvidence, PortResponseRefusal>;

/// Physically store rejected foreign material under an ingress-owned
/// quarantine disposition.
///
/// Inputs: the quarantine grant and the store request carrying its declared
/// retention envelope. Output: the quarantine receipt — proof of custody
/// under the declared retention, never of content validity, never of
/// re-admissibility. Refuses with `QuarantineRefusal`: absent grant, any
/// retention ceiling reached (count, bytes, age), work ceiling exhausted
/// mid-store, deletion route unavailable (key-shred without an owned key
/// scope), payload over limit. All four guardrail ceilings bind this
/// operation; expiry passes consume the same work ceiling.
pub fn quarantine_store(
    grant: &QuarantineGrant,
    request: QuarantineStoreRequest,
) -> Result<QuarantineReceipt, QuarantineRefusal>;
