//! Event owner — thin semantic operation signatures.
//!
//! Signature declarations only, semicolon form: what each operation consumes,
//! what it establishes, what it refuses, and what bounds it runs under. No
//! bodies — bodies land with their construction cuts. This file is contract,
//! not compilable code, until the dependency probe seats the manifest.
//!
//! Every operation receives its profile and its authority explicitly. Depot
//! rows are passed, never fetched (`depot/README.md`); no operation reads an
//! ambient clock, store, environment, or registry. Pure operations return
//! successor values and commit nothing — thin stateful shells publish what
//! pure operations admit.
//!
//! Foreign roles by owner: `RawWallObservation` (port). Storage physically
//! realizes publication through the storage port family (`StorageOperation`
//! roster; contract declared in the port grammar, projected as a depot row).

// ---------------------------------------------------------------------------
// Accepted history
// ---------------------------------------------------------------------------

/// Admit one proposed fact into accepted history — the single domain-fact
/// admission in all of ThreadPak.
///
/// Consumes: the admission profile, the caller's append authority, the
/// current region epoch, the store's current accepted boundary, the caller's
/// expected Cut, the proposal, and already-admitted chronology evidence.
/// Establishes: one `AcceptedEvent` at the next `AuthoritySequence` position.
/// Refuses: `StaleExpectedCut` before publication (no silent rebase),
/// `StaleEpoch`, `WrongAuthority`, `FrameMismatch`, `ReservedEventClass`,
/// `BoundExceeded` (per the profile's limits), `Causation(_)`.
/// Bounds: `EventByteLimit`, `BatchEventLimit` (via batch composition),
/// `CausalParentLimit`, `UnresolvedCausalClaimLimit`.
pub fn admit_event(
    profile: &EventAdmissionProfile,
    grant: &AppendGrant,
    epoch: &AuthorityEpoch,
    current: &Cut,
    expected: ExpectedCut,
    proposal: EventProposal,
    accepted_hlc: AcceptedHlc,
    source_chronology: SourceChronologyEvidence,
) -> Result<AcceptedEvent, EventAdmissionRefusal>;

/// Relate two federation cuts over compatible source sets.
///
/// Consumes: the federation profile and two cuts. Establishes: one
/// `FederationCutRelation` — a knowledge summary, never coexistence, never
/// atomicity. Refuses: nothing; `Incompatible` is an answer, not a refusal.
/// Bounds: `FederationSourceLimit` (already enforced at `SourceSet`
/// construction).
pub fn relate_federation_cuts(
    profile: &FederationProfile,
    left: &FederationCut,
    right: &FederationCut,
) -> FederationCutRelation;

// ---------------------------------------------------------------------------
// Chronology
// ---------------------------------------------------------------------------

/// Pure chronology admission: policy + prior state + one admitted wall
/// observation + optional source chronology → the successor state and an
/// `AcceptedHlc`, or a typed refusal.
///
/// Reads no ambient clock, performs no I/O, persists nothing, commits
/// nothing; the thin stateful shell commits the returned state. Excessive-
/// future source values are preserved and classified, never clamped.
/// Refuses: `RegressionBeyondPolicy`, `FutureBeyondPolicy`,
/// `LogicalCounterOverflow` (no wrap, no saturation; prior accepted state
/// intact), `ProfileMismatch`.
pub fn admit_chronology(
    policy: &ChronologyPolicy,
    prior: &ChronologyState,
    observation: RawWallObservation,
    source: Option<SourceHlc>,
) -> Result<ChronologyAdvance, ChronologyRefusal>;

/// Merge two chronology summaries — pure, same-profile, componentwise
/// maximum. Total over validated same-profile summaries; consults no wall
/// clock, evaluates no trust, stamps no event, claims no durable progress.
/// Refuses: `ChronologyMergeRefusal::ProfileMismatch` — the only guard.
pub fn merge_chronology_summaries(
    left: &ChronologySummary,
    right: &ChronologySummary,
) -> Result<ChronologySummary, ChronologyMergeRefusal>;

// ---------------------------------------------------------------------------
// Ingress
// ---------------------------------------------------------------------------

/// Idempotent `Reserve`: mint or recover the reservation for one client
/// nonce and intent commitment.
///
/// Same nonce + same commitment returns the same reservation, consuming
/// bounded lookup work and no new slot. Establishes: one durable
/// `IngressReservation`. Refuses: `ReservationConflict` (same nonce,
/// different commitment — no overwrite, no second token),
/// `ReservationCapacityExhausted` (never resolved by evicting an unexpired
/// reservation), `ReservationRateExceeded`, `NonceExceedsByteLimit`.
/// Bounds: the complete `ReservationProfile` family.
pub fn reserve_ingress(
    profile: &ReservationProfile,
    grant: &IngressGrant,
    nonce: ClientNonce,
    intent: ReservationIntentCommitment,
) -> Result<IngressReservation, IngressRefusal>;

/// Admit one validated foreign claim into durable ingress custody — the
/// ClaimFirst terminal boundary.
///
/// Establishes: the `AdmittedClaim` custody fact and its retry-discharging
/// `ClaimAdmissionReceipt`, both bound to the exact submission identity.
/// Refuses: `NoLawfulIdempotencyIdentity` (effectful ingress with no ladder
/// rung), `ReservationExpired` (a retry never silently becomes a fresh
/// intent), `ClaimValidationRefused` upstream dispositions.
pub fn admit_claim(
    grant: &IngressGrant,
    validated: ValidatedClaim,
    submission: SubmissionIdentity,
) -> Result<(AdmittedClaim, ClaimAdmissionReceipt), IngressRefusal>;

/// Record the outcome of one admitted claim's processing obligation.
///
/// The custody fact is never deleted; what closes is the obligation.
/// `DomainRefusal` is the application's typed domain-refusal family, carried
/// verbatim. Refuses: `ResolutionBeforeOutcome` (the referenced event is not
/// accepted — a claim is never marked resolved first),
/// `ClaimAlreadyResolved` (no overwrite of a closed obligation).
pub fn resolve_claim<DomainRefusal>(
    claim: &AdmittedClaim,
    resolution: ClaimResolution<DomainRefusal>,
) -> Result<ClaimResolution<DomainRefusal>, IngressRefusal>;

/// Mint the DomainFirst terminal receipt after the accepted event's
/// publication boundary is durable: the binding of this exact submission's
/// idempotency identity to the accepted identity and its commit point.
/// Refuses: `ResolutionBeforeOutcome` when the commit boundary does not yet
/// cover the accepted event.
pub fn mint_domain_admission_receipt(
    submission: SubmissionIdentity,
    event: &AcceptedEvent,
    commit: &CommitPoint,
    operation: IngressOperationFamilyId,
) -> Result<DomainAdmissionReceipt, IngressRefusal>;

// ---------------------------------------------------------------------------
// Recovery
// ---------------------------------------------------------------------------

/// Pure recovery classification over bounded scan evidence: establish the
/// exact recovered boundary and classify everything encountered.
///
/// Committed-boundary-bounded, never caller-acknowledgement-bounded.
/// Within-boundary unreadable material classifies refuse-and-hold — a
/// classification, not a refusal; committed-but-unacknowledged material is
/// never discarded. Refuses: `ScanBudgetExhausted` (no partial boundary is
/// reported as recovered). Bounds: `RecoveryScanBudget`.
pub fn recover_prefix(
    profile: &RecoveryProfile,
    scanned: SegmentScanEvidence,
) -> Result<RecoveryReceipt, RecoveryRefusal>;

// ---------------------------------------------------------------------------
// Partition
// ---------------------------------------------------------------------------

/// Seal one region's writable epoch at an exact Cut. A sealed parent
/// accepts no later write. Refuses: `WrongAuthority`, `ParentNotSealed`
/// preconditions of downstream steps are established here.
pub fn seal_region(
    grant: &PartitionGrant,
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
    at: Cut,
) -> Result<RegionSealWitness, PartitionRefusal>;

/// Prove one lawful split of a sealed parent: children pairwise disjoint,
/// union exactly the parent, no gap, no overlap. Refuses: `ParentNotSealed`,
/// `ChildrenOverlap`, `CoverageGap`.
pub fn compute_split_witness(
    sealed: &RegionSealWitness,
    children: Vec<AuthorityRegionId>,
) -> Result<SplitWitness, PartitionRefusal>;

/// Translate a parent position into a successor scope — the only lawful
/// bridge between parent and child cuts. Refuses:
/// `SuccessionEvidenceMissing` preconditions downstream are established
/// here; `WrongAuthority`.
pub fn compute_cut_succession(
    grant: &PartitionGrant,
    sealed: &RegionSealWitness,
    successor_region: AuthorityRegionId,
    successor_epoch: AuthorityEpoch,
) -> Result<CutSuccessionWitness, PartitionRefusal>;

/// Activate one fresh epoch on a successor region. Activation precedes
/// routing publication. Refuses: `WrongAuthority`, `ParentNotSealed`.
pub fn activate_epoch(
    grant: &PartitionGrant,
    split: &SplitWitness,
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
) -> Result<EpochActivation, PartitionRefusal>;

/// Publish routing for a succession — last, after every activation.
/// Routing reports authority and never grants it. Refuses:
/// `RoutingBeforeActivation`, `WrongAuthority`.
pub fn publish_routing(
    grant: &PartitionGrant,
    split: &SplitWitness,
    activations: Vec<EpochActivation>,
) -> Result<RoutingPublication, PartitionRefusal>;

// ---------------------------------------------------------------------------
// Removal
// ---------------------------------------------------------------------------

/// Admit one removal plan into affine removal authority. A grant is not an
/// admission — this boundary mints the ladder's own affine authority.
/// Refuses: `WrongAuthority`, `ScopeMismatch`, `StaleCut`.
pub fn admit_removal(
    grant: &RemovalGrant,
    plan: RemovalPlan,
) -> Result<RemovalAdmission, RemovalRefusal>;

/// Cross the destructive boundary exactly once, consuming the admission.
/// Establishes: the `RemovalCommitment` fact and the completed-ladder
/// `RemovalReceipt`. Authorized removal is never historical absence.
/// Refuses: `ScopeMismatch` (never destroys wider than admitted).
pub fn commit_removal(
    admission: RemovalAdmission,
) -> Result<(RemovalCommitment, RemovalReceipt), RemovalRefusal>;
