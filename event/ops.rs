//! Event owner — thin semantic operation signatures.
//!
//! Declaration form: each operation is authored as a Rust function-pointer type alias (`pub type NameFn = fn(...) -> ...;`) preserving the signature shape the operation must have.
//! Foreign owner names stand unresolved in this fragment, so this file alone claims neither compilation nor resolved dependency edges; that evidence is the generated contract probe's to produce.
//! No bodies — bodies land with their construction cuts, each pinned to its declared signature by a Macroonz-generated conformance assertion (`const _: AdmitEventFn = admit_event;`).
//! Nothing in this file executes, and nothing here claims implementation support.
//!
//! Every operation receives its profile and its authority explicitly.
//! Depot rows are passed, never fetched (`depot/README.md`); no operation reads an ambient clock, store, environment, or registry.
//! Pure operations return successor values and commit nothing — thin stateful shells publish what pure operations admit.
//!
//! Foreign roles by owner: `WallObservation` (port — the admitted observation enclosure produced by the port owner's `admit_wall_observation` under a declared `ClockSourceProfile`; raw readings never reach this owner).
//! Storage physically realizes publication through the storage port family (`StorageOperation` roster; contract declared in the port grammar, projected as a depot row).

// ---------------------------------------------------------------------------
// Accepted history
// ---------------------------------------------------------------------------

/// Operation `admit_event` — admit one proposed fact into accepted history: the single domain-fact admission in all of ThreadPak.
///
/// Consumes: the admission profile, the caller's append authority, the current region epoch, the store's current accepted boundary, the caller's expected Cut, the proposal, and already-admitted chronology evidence.
/// Establishes: one `AcceptedEvent` at the next `AuthoritySequence` position.
/// Refuses: `StaleExpectedCut` before publication (no silent rebase), `StaleEpoch`, `WrongAuthority`, `FrameMismatch`, `ReservedEventClass`, `BoundExceeded` (per the profile's limits), `Causation(_)`.
/// Bounds: `EventByteLimit`, `BatchEventLimit` (via batch composition), `CausalParentLimit`, `UnresolvedCausalClaimLimit`.
pub type AdmitEventFn = fn(
    profile: &EventAdmissionProfile,
    grant: &AppendGrant,
    epoch: &AuthorityEpoch,
    current: &Cut,
    expected: ExpectedCut,
    proposal: EventProposal,
    accepted_hlc: AcceptedHlc,
    source_chronology: SourceChronologyEvidence,
) -> Result<AcceptedEvent, EventAdmissionRefusal>;

/// Operation `relate_federation_cuts` — relate two federation cuts over compatible source sets.
///
/// Consumes: the federation profile and two cuts.
/// Establishes: one `FederationCutRelation` — a knowledge summary, never coexistence, never atomicity.
/// Refuses: nothing; `Incompatible` is an answer, not a refusal.
/// Bounds: `FederationSourceLimit` (already enforced at `SourceSet` construction).
pub type RelateFederationCutsFn = fn(
    profile: &FederationProfile,
    left: &FederationCut,
    right: &FederationCut,
) -> FederationCutRelation;

// ---------------------------------------------------------------------------
// Chronology
// ---------------------------------------------------------------------------

/// Operation `admit_chronology` — pure chronology admission: policy + prior state + one admitted wall-observation enclosure + optional source chronology → the successor state and an `AcceptedHlc`, or a typed refusal.
///
/// The observation is the port owner's admitted enclosure (`WallObservation`: earliest and latest bounds under a declared `ClockSourceProfile`) — never a raw reading.
/// A point reading is the degenerate enclosure only when the source explicitly claims zero uncertainty; unstated uncertainty was already resolved at the port boundary by the profile's posture — declared maximum or refusal, never zero.
///
/// Reads no ambient clock, performs no I/O, persists nothing, commits nothing; the thin stateful shell commits the returned state.
/// Excessive-future source values are preserved and classified, never clamped.
/// Refuses: `RegressionBeyondPolicy`, `FutureBeyondPolicy`, `LogicalCounterOverflow` (no wrap, no saturation; prior accepted state intact), `ProfileMismatch`.
pub type AdmitChronologyFn = fn(
    policy: &ChronologyPolicy,
    prior: &ChronologyState,
    observation: WallObservation,
    source: Option<SourceHlc>,
) -> Result<ChronologyAdvance, ChronologyRefusal>;

/// Operation `merge_chronology_summaries` — merge two chronology summaries: pure, same-profile, componentwise maximum.
///
/// Total over validated same-profile summaries; consults no wall clock, evaluates no trust, stamps no event, claims no durable progress.
/// Refuses: `ChronologyMergeRefusal::ProfileMismatch` — the only guard.
pub type MergeChronologySummariesFn = fn(
    left: &ChronologySummary,
    right: &ChronologySummary,
) -> Result<ChronologySummary, ChronologyMergeRefusal>;

// ---------------------------------------------------------------------------
// Ingress
// ---------------------------------------------------------------------------

/// Operation `reserve_ingress` — idempotent `Reserve`: mint or recover the reservation for one client nonce and intent commitment.
///
/// Same nonce + same commitment returns the same reservation, consuming bounded lookup work and no new slot.
/// Establishes: one durable `IngressReservation`.
/// Refuses: `ReservationConflict` (same nonce, different commitment — no overwrite, no second token), `ReservationCapacityExhausted` (never resolved by evicting an unexpired reservation), `ReservationRateExceeded`, `NonceExceedsByteLimit`.
/// Bounds: the complete `ReservationProfile` family.
pub type ReserveIngressFn = fn(
    profile: &ReservationProfile,
    grant: &IngressGrant,
    nonce: ClientNonce,
    intent: ReservationIntentCommitment,
) -> Result<IngressReservation, IngressRefusal>;

/// Operation `admit_claim` — admit one validated foreign claim into durable ingress custody: the ClaimFirst terminal boundary.
///
/// Establishes: the `AdmittedClaim` custody fact and its retry-discharging `ClaimAdmissionReceipt`, both bound to the exact submission identity.
/// Refuses: `NoLawfulIdempotencyIdentity` (effectful ingress with no ladder rung), `ReservationExpired` (a retry never silently becomes a fresh intent), `ClaimValidationRefused` upstream dispositions.
pub type AdmitClaimFn = fn(
    grant: &IngressGrant,
    validated: ValidatedClaim,
    submission: SubmissionIdentity,
) -> Result<(AdmittedClaim, ClaimAdmissionReceipt), IngressRefusal>;

/// Operation `resolve_claim` — record the outcome of one admitted claim's processing obligation.
///
/// The custody fact is never deleted; what closes is the obligation.
/// `DomainRefusal` is the application's typed domain-refusal family, carried verbatim.
/// Refuses: `ResolutionBeforeOutcome` (the referenced event is not accepted — a claim is never marked resolved first), `ClaimAlreadyResolved` (no overwrite of a closed obligation).
pub type ResolveClaimFn<DomainRefusal> = fn(
    claim: &AdmittedClaim,
    resolution: ClaimResolution<DomainRefusal>,
) -> Result<ClaimResolution<DomainRefusal>, IngressRefusal>;

/// Operation `mint_domain_admission_receipt` — mint the DomainFirst terminal receipt after the accepted event's publication boundary is durable: the binding of this exact submission's idempotency identity to the accepted identity and its commit point.
///
/// Refuses: `ResolutionBeforeOutcome` when the commit boundary does not yet cover the accepted event.
pub type MintDomainAdmissionReceiptFn = fn(
    submission: SubmissionIdentity,
    event: &AcceptedEvent,
    commit: &CommitPoint,
    operation: IngressOperationFamilyId,
) -> Result<DomainAdmissionReceipt, IngressRefusal>;

// ---------------------------------------------------------------------------
// Recovery
// ---------------------------------------------------------------------------

/// Operation `recover_prefix` — pure recovery classification over bounded scan evidence: establish the exact recovered boundary and classify everything encountered.
///
/// Committed-boundary-bounded, never caller-acknowledgment-bounded.
/// Within-boundary unreadable material classifies refuse-and-hold — a classification, not a refusal; committed-but-unacknowledged material is never discarded.
/// Refuses: `ScanBudgetExhausted` (no partial boundary is reported as recovered).
/// Bounds: `RecoveryScanBudget`.
pub type RecoverPrefixFn = fn(
    profile: &RecoveryProfile,
    scanned: SegmentScanEvidence,
) -> Result<RecoveryReceipt, RecoveryRefusal>;

// ---------------------------------------------------------------------------
// Partition — admissions consume a grant scoped to their exact operation; pure geometry and succession proofs consume evidence, never authority.
// ---------------------------------------------------------------------------

/// Operation `seal_region` — seal one region's writable epoch at an exact Cut.
///
/// A sealed parent accepts no later write.
///
/// Consumes: a grant scoped to `PartitionOperation::Seal`, the region and epoch, the region's current accepted boundary, and the caller's claimed seal boundary.
/// The claimed boundary must be the current accepted boundary — sealing behind it would orphan accepted material; sealing ahead of it would seal history that does not exist.
/// Refuses: `WrongAuthority`, `StaleSealCut` (the claimed boundary is not the current accepted boundary).
/// `ParentNotSealed` preconditions of downstream steps are established here.
pub type SealRegionFn = fn(
    grant: &PartitionGrant,
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
    current: &Cut,
    claimed: ExpectedCut,
) -> Result<RegionSealWitness, PartitionRefusal>;

/// Operation `compute_split_witness` — prove one lawful split of a sealed parent: children pairwise disjoint, union exactly the parent, no gap, no overlap.
///
/// A pure geometric proof: evidence in, witness out, no authority consumed.
/// Child identities alone can prove nothing about coverage — every child carries its declared geometry, judged against the parent's declared geometry.
/// Refuses: `ParentNotSealed`, `ChildrenOverlap`, `CoverageGap`.
pub type ComputeSplitWitnessFn = fn(
    sealed: &RegionSealWitness,
    parent: &RegionGeometry,
    children: Vec<ChildRegionDeclaration>,
) -> Result<SplitWitness, PartitionRefusal>;

/// Operation `compute_cut_succession` — translate a parent position into a successor scope: the only lawful bridge between parent and child cuts.
///
/// A pure proof: it consumes succession evidence, not authority — the split witness is the relation that makes the translation lawful.
/// Refuses: `SuccessionEvidenceMissing` (the successor is not a proven child of this exact sealed parent).
/// `WrongAuthority` belongs to the admission operations, never to this proof.
pub type ComputeCutSuccessionFn = fn(
    sealed: &RegionSealWitness,
    split: &SplitWitness,
    successor_region: AuthorityRegionId,
    successor_epoch: AuthorityEpoch,
) -> Result<CutSuccessionWitness, PartitionRefusal>;

/// Operation `activate_epoch` — activate one fresh epoch on a successor region.
///
/// Activation precedes routing publication.
/// Consumes a grant scoped to `PartitionOperation::ActivateSuccessor`.
/// Refuses: `WrongAuthority`, `ParentNotSealed`.
pub type ActivateEpochFn = fn(
    grant: &PartitionGrant,
    split: &SplitWitness,
    region: AuthorityRegionId,
    epoch: AuthorityEpoch,
) -> Result<EpochActivation, PartitionRefusal>;

/// Operation `publish_routing` — publish routing for a succession: last, after every activation.
///
/// Routing reports authority and never grants it.
/// Consumes a grant scoped to `PartitionOperation::PublishRouting`.
/// Refuses: `RoutingBeforeActivation`, `WrongAuthority`.
pub type PublishRoutingFn = fn(
    grant: &PartitionGrant,
    split: &SplitWitness,
    activations: Vec<EpochActivation>,
) -> Result<RoutingPublication, PartitionRefusal>;

// ---------------------------------------------------------------------------
// Removal
// ---------------------------------------------------------------------------

/// Operation `admit_removal` — admit one removal plan into affine removal authority.
///
/// A grant is not an admission — this boundary mints the ladder's own affine authority.
/// Refuses: `WrongAuthority`, `ScopeMismatch`, `StaleCut`.
pub type AdmitRemovalFn = fn(
    grant: &RemovalGrant,
    plan: RemovalPlan,
) -> Result<RemovalAdmission, RemovalRefusal>;

/// Operation `commit_removal` — cross the destructive boundary exactly once, consuming the admission.
///
/// Establishes: the `RemovalCommitment` fact and the completed-ladder `RemovalReceipt`.
/// Authorized removal is never historical absence.
/// Refuses: `ScopeMismatch` (never destroys wider than admitted).
pub type CommitRemovalFn = fn(
    admission: RemovalAdmission,
) -> Result<(RemovalCommitment, RemovalReceipt), RemovalRefusal>;
