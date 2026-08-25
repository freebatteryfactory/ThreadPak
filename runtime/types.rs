//! Runtime owner nouns: the logical runtime, PakVM, and Bvisor role graph.
//!
//! Declarations only.
//! Operations are declared in `ops.rs` beside this file; no behavior is defined here.
//! Crossing types named in field positions are foreign-owned: their declarations live with their owners and resolve by import when the dependency probe seats the homes.
//! This file keeps no manual mirror of that inventory — the generated contract probe derives it.
//! Bare names, no `crate::` paths, until the probe exists.
//!
//! Every field here is private, and every field is a correctness-bearing relationship the owner contract promises.
//! Where a leaf representation is written `/* closes with … */`, the relationship is law and only the exact byte shape remains owner-derived machining.
//!
//! Derive discipline: `Clone`, `Copy`, `Serialize`, `Deserialize`, `Ord`, and `Hash` are semantic claims.
//! A type without them here must not acquire them for convenience; custody and one-shot types must never implement `Clone` or any serialization.
//! Live custody types must also never become `Send` or `Sync` by accident: an auto-derived crossing would carry live authority between workers without an admission decision, so custody interiors are chosen at the guard pass to make the wrong crossing unrepresentable.

use core::num::{NonZeroU32, NonZeroU64};

// ---------------------------------------------------------------------------
// Logical runtime: identities, Turn, Stitch, processes
// ---------------------------------------------------------------------------

/// Identity of one logical operation — the application-meaningful unit one or more Turns realize.
///
/// One of the four never-substitutable lineage identities (logical operation, Turn, EffectIntent, Attempt).
pub struct LogicalOperationId {
    /* closes with the identity profile */
}

/// Identity of one logical transition.
///
/// Derived from the Turn's identity-bearing inputs; replay reconstructs the same Turn where lawful, and a changed identity input is a different Turn.
/// The exact preimage roster and width close with the identity profile; the recovered preimage candidates are depot rows, not code.
pub struct TurnId {
    /* derived identity; closes with the identity profile */
}

/// Stable identity of one logical process.
///
/// Not an OS thread, task, actor, mailbox, or physical worker.
pub struct ProcessId {
    /* closes with the identity profile */
}

/// Generation of one logical process.
///
/// A wake, continuation, or observation bound to a prior generation is not runnable work in a later one.
pub struct ProcessGeneration {
    generation: u64,
}

/// Identity of one delivery relationship this owner drives.
///
/// Role-distinct from `ProcessId` and from the view owner's `SubscriptionId`.
pub struct DeliveryId {
    /* closes with the identity profile */
}

/// The durable admitted state of one logical process — the state Stitch consumes and succeeds.
///
/// Recovery reconstructs it from accepted history selected by the process's input contract plus its durable checkpoint; queues, channels, and notifications are acceleration, never this state.
pub struct ProcessState {
    /* owner-derived durable representation */
}

/// The declared contract of one logical process: its input contract (which accepted work it consumes), its deferred-input bounds, its ordering relationship, and its no-progress disposition.
///
/// Declared once per process role; numeric values live in the depot.
pub struct ProcessContract {
    deferred_count: DeferredInputLimit,
    deferred_bytes: DeferredInputByteLimit,
    deferred_age: DeferredInputAgeLimit,
    /* input contract, ordering relationship, and no-progress disposition close with the stimulus-algebra seam */
}

/// The frozen half of one logical-book entry: the Turn record.
///
/// Binds the logical operation, the process and generation it advances, the frozen typed inputs at exact Cuts, the one admitted program invocation, the declared semantic bounds, the recovery posture, and the checkpoint consequence.
/// What the Turn concluded is a separate append-only record (`TurnConclusion`); the accepted composition of the two is the complete logical-book entry.
/// The physical book (Attempts) references this record and never edits it.
pub struct Turn {
    id: TurnId,
    operation: LogicalOperationId,
    process: ProcessId,
    generation: ProcessGeneration,
    inputs: FrozenTurnInputs,
    /// The one admitted program invocation (program-owned image identity).
    invocation: ProgramImageId,
    bounds: DeclaredTurnBounds,
    /// The operation's declared recovery posture (port-owned contract role).
    recovery: RecoveryContract,
    checkpoint: CheckpointConsequence,
}

/// The frozen input binding of one Turn: the exact source set and one exact Cut per participating source — never a generic frontier, HLC, page cursor, route, delivery sequence, or wall-clock instant.
///
/// A missing source stays explicit and is never silently replaced by the newest reachable one.
pub struct FrozenTurnInputs {
    sources: SourceSet,
    /* one exact Cut per source, in canonical participant order, plus the input identity commitment; closes with the canon profile */
}

/// The declared semantic bounds one Turn runs under.
///
/// The bound set's members are owner-named types classified per root law; numeric values live in the depot.
pub struct DeclaredTurnBounds {
    /* owner-named bound roster; closes with the process-contract seam */
}

/// Which checkpoint subject this Turn feeds and what prerequisite evidence it contributes toward that subject's next lawful advance.
///
/// A Turn that feeds no checkpoint chain says so explicitly; nothing infers a consequence from silence.
pub struct CheckpointConsequence {
    subject: CheckpointSubject,
    /* contributed prerequisite evidence; decomposition closes with the checkpoint contract */
}

/// One semantic conclusion: a result or its typed refusal — both program-owned roles, preserved without collapse.
///
/// Carried per Turn in `TurnConclusion` and per join branch in `JoinBranchRecord`.
pub enum SemanticConclusion {
    Returned(ImmediateResult),
    Refused(DecisionRefusal),
}

/// What one Turn concluded — the second half of the logical-book entry.
///
/// `Turn` freezes the invocation; this record joins that Turn's identity to its semantic conclusion, its proposals, its portable work, and its explanation.
/// Append-only: later evidence never edits it, and the accepted composition of `Turn` plus `TurnConclusion` is the complete logical-book entry.
pub struct TurnConclusion {
    /// The typed join to the frozen invocation this record concludes.
    turn: TurnId,
    /// The Turn's semantic conclusion — result or typed refusal, preserved without collapse.
    conclusion: SemanticConclusion,
    /// Program-owned carrier over the event owner's noun.
    events: EventProposals,
    /// Program-owned inert proposals; admission strengthens each into a durable `EffectIntent`.
    effects: EffectProposals,
    /// Program-owned portable work accounting.
    work: ConsumedSemanticWork,
    /// Program-owned progressive-explanation binding (rail 14).
    explanation: Explanation,
}

/// One member of the typed-observation family Stitch consumes.
///
/// The roster closes with the stimulus-algebra seam (a recorded open seam of this contract); a wake is awareness and is never a member of this family.
pub struct StitchStimulus {
    /* stimulus role and payload binding; roster closes with the stimulus-algebra seam */
}

/// Explicit context supplied to the Stitch transition: the Turn this transition realizes (carrying the operation, process, generation, frozen inputs, admitted invocation, bounds, recovery posture, and checkpoint consequence), the applicable process contract, and the operation's durable deadline commitment.
///
/// The process identity and generation ride on the Turn — no twin fields exist here.
/// Carries no ambient clock, no executor, and no I/O capability.
pub struct StitchContext {
    /// The frozen invocation this transition realizes; the produced `TurnConclusion` joins to its `TurnId`.
    turn: Turn,
    contract: ProcessContract,
    deadline: DeadlinePolicy,
}

/// The result of one Stitch transition: the next admitted state plus the concluded Turn's record — the semantic conclusion, event proposals for event admission, effect proposals for REQUEST/PEND admission, portable semantic work, and the evaluation's explanation, all joined to the `TurnId` they conclude.
///
/// Producing this value publishes nothing; only event admission makes a proposal an accepted fact, and only effect admission mints an intent.
#[must_use]
pub struct StitchAdvance {
    next: ProcessState,
    /// The concluded Turn's record — the typed join the logical book assembles beside the frozen `Turn`.
    conclusion: TurnConclusion,
}

/// Closed refusal family of the Stitch transition.
///
/// Every variant names the violated law, the typed owner, the offending value's role, and the repair direction when realized.
#[must_use]
pub enum StitchRefusal {
    /// The stimulus is bound to a prior process generation and is not runnable work in this one.
    StaleGeneration,
    /// The stimulus lies outside the process's declared input contract.
    StimulusOutsideContract,
    /// A declared Turn bound was exhausted before the transition completed.
    BoundExhausted,
    /* the roster closes with the stimulus-algebra seam */
}

/// One operation–observer relationship.
///
/// Resolves at most once with the strongest honest terminal observation available at that boundary.
/// It is not the outcome, the continuation, a receipt, a checkpoint, or retry authority.
/// Never `Clone`.
#[must_use]
pub struct Completion {
    /* private resolution state */
}

/// One lawfully admitted input that cannot run in the current process state: the retained stimulus, why it is deferred, the exact state or evidence that could unblock it, its absolute deadline, and its collation position.
///
/// Reconsidering it under unchanged state and unchanged evidence is no progress, and the runtime recognizes no progress rather than spinning.
pub struct DeferredInput {
    input: StitchStimulus,
    reason: DeferralReason,
    unblock: UnblockCondition,
    deadline: AbsoluteDeadline,
    /* collation position and retained-age evidence */
}

/// Why one admitted input cannot run now.
///
/// Names the blocking fact, never a generic "pending".
pub struct DeferralReason {
    /* blocking-fact binding; closes with the stimulus-algebra seam */
}

/// The exact state or evidence that could make one deferred input admissible.
///
/// Inert data; it performs nothing.
pub struct UnblockCondition {
    /* condition binding; closes with the stimulus-algebra seam */
}

/// Closed refusal family for deferred input.
#[must_use]
pub enum DeferredInputRefusal {
    /// The input can no longer become admissible — the terminal refusal.
    CanNeverBecomeAdmissible,
    /// Retaining the input would exceed the process's deferred count bound.
    CountBoundExceeded,
    /// Retaining the input would exceed the process's deferred byte bound.
    ByteBoundExceeded,
    /// The input's retained age passed the declared age bound.
    AgeBoundExceeded,
    /* the roster closes with the process-contract seam */
}

/// A separately authorized request that physical work stop.
///
/// Distinct from observer abandonment, deadline expiry, close, drain, and shutdown.
/// The authorizing grant is role-specific and closes with the guard pass.
pub struct CancellationRequest {
    target: CancellationTarget,
    /* authorizing-grant binding; closes with the guard pass */
}

/// What one cancellation request targets.
///
/// (Draft spelling; the two arms are law — Bvisor cancels physical Attempts, the runtime cancels logical operations.)
pub enum CancellationTarget {
    Attempt(AttemptId),
    Operation(LogicalOperationId),
}

/// Live custody of one runtime lane.
///
/// One drive call owns one lane; same-lane reentrancy is unavailable by default.
/// Never `Clone`, never serialized, never accidentally `Send`/`Sync`.
#[must_use]
pub struct RuntimeLane {
    /* private lane custody */
}

// ---------------------------------------------------------------------------
// The durable EffectIntent record and reconciliation
// ---------------------------------------------------------------------------

/// Identity of one durably admitted external intent.
///
/// One of the four never-substitutable lineage identities; minted only by REQUEST/PEND admission.
pub struct EffectIntentId {
    /* closes with the identity profile */
}

/// Which admission road minted one durable intent.
///
/// REQUEST returns without waiting; PEND additionally drives one immediate bounded Attempt.
/// The posture never changes the record's meaning or survival.
pub enum EffectAdmissionPosture {
    Request,
    Pend,
}

/// The durable, runtime-owned record of one admitted external intent — minted by REQUEST or PEND admission consuming a program-produced `EffectProposal`.
///
/// Append-only fact: it survives any later semantic refusal, and no later evidence edits it.
/// Effect *meaning* stays program-declared; this record owns admission, custody, and recovery accounting.
pub struct EffectIntent {
    id: EffectIntentId,
    turn: TurnId,
    posture: EffectAdmissionPosture,
    proposal: EffectProposalCommitment,
    /// The operation's declared recovery posture (port-owned contract role).
    recovery: RecoveryContract,
    idempotency: EffectIdempotencyIdentity,
    deadline: DeadlinePolicy,
}

/// Commitment over the canonical bytes of the admitted proposal, so the durable record can prove exactly which proposal it strengthened.
///
/// The proposal's canonical bytes — naming its port operation, contract version, and request-value commitment — publish with the intent record through the runtime storage family, so the exact physical request remains realizable from the durable record alone; this commitment proves the correspondence.
pub struct EffectProposalCommitment {
    /* closes with the canon profile */
}

/// The identity under which duplication of one intent is recognized, per the idempotency ladder: natural business identity, reservation token, generated per-call key, or explicit key.
///
/// Content-derived, wall-clock, session, route, connection, host, shard, and Attempt identities are all unlawful substitutes.
pub struct EffectIdempotencyIdentity {
    /* ladder-rung binding; closes with the identity profile */
}

/// The retry-discharging evidence that one proposal was durably admitted as one intent.
///
/// Claims exactly the admission boundary — never physical execution, outcome, or checkpoint progress.
#[must_use]
pub struct EffectAdmissionReceipt {
    intent: EffectIntentId,
    /* publication evidence of the admitted record */
}

/// Closed refusal family of effect admission.
#[must_use]
pub enum EffectAdmissionRefusal {
    /// The proposal's recovery posture claims a route that is not declared before the irreversible Attempt.
    RecoveryRouteMissing,
    /// No lawful idempotency identity exists for an effectful admission.
    NoLawfulIdempotencyIdentity,
    /// The operation's deadline had already expired at admission.
    DeadlineAlreadyExpired,
    /// Admission would exceed a declared effect bound.
    EffectBoundExceeded,
    /* the roster closes with the effect-admission contract */
}

/// Whether the external consequence of one effect is established.
///
/// `Known` carries the establishing conclusion; `OutcomeUnknown` is a stable honest result, never relabeled as failure.
/// (Draft spelling on the payload.)
pub enum OutcomeKnowledge {
    /// Admitted evidence establishes the outcome; the payload binds the establishing reconciliation conclusion.
    Known(EstablishedOutcome),
    /// An external consequence may exist; evidence cannot establish it.
    OutcomeUnknown,
}

/// The established external conclusion of one effect, bound to the evidence that establishes it.
///
/// A claim of knowledge never travels without its establishing basis.
pub struct EstablishedOutcome {
    intent: EffectIntentId,
    /* establishing evidence binding; closes with the reconciliation contract */
}

/// Whether reconciliation is owed for one intent.
///
/// Carrying the disposition inside `Complete` makes a disposition-without-completion unrepresentable, and `Outstanding` can never masquerade as a resolved handling.
pub enum ReconciliationLifecycle {
    NotRequired,
    Outstanding,
    Complete(ReconciliationDisposition),
}

/// How one completed reconciliation concluded — meaning available only at completion.
///
/// The lifecycle answers *whether*; this answers *how*.
pub enum ReconciliationDisposition {
    Reconciled,
    CompensationProposed,
    ManualInterventionRequired,
    AutomaticActionRefused,
}

/// The durable relationship around one admitted EffectIntent that has no terminal outcome knowledge: the intent, its known Attempts, the append-only evidence so far, and the applicable recovery posture.
pub struct OutstandingEffect {
    intent: EffectIntentId,
    /* bounded known-Attempt roster, append-only evidence references (sealed AttemptReports, acknowledgments, outcome-query evidence), and the recovery-contract binding */
}

/// The durable fact that outcome knowledge for one intent remains unresolved.
///
/// A checkpoint that forgets this obligation cannot lawfully skip the work it covers.
pub struct ReconciliationObligation {
    intent: EffectIntentId,
    /* recorded obligation evidence */
}

/// The current lawful conclusion and next action derived purely from append-only evidence.
///
/// Never rewrites an earlier Attempt or observation; later evidence supersedes the conclusion while the record stands.
#[must_use]
pub struct ReconciliationDecision {
    intent: EffectIntentId,
    action: ReconciliationNextAction,
    /* evidence basis consumed by this conclusion */
}

/// The closed roster of lawful next actions one reconciliation may conclude.
///
/// A physical inconvenience never adds a variant.
pub enum ReconciliationNextAction {
    Observe,
    Wait,
    OneFreshLawfulAttempt,
    ProposeCompensation,
    AwaitAuthorizedDecision,
    AcceptDurablePartialOutcome,
    TerminateUnresolved,
}

/// Closed refusal family for reconciliation.
#[must_use]
pub enum ReconciliationRefusal {
    /// The concluded action lies outside the operation's declared recovery contract.
    ActionOutsideRecoveryContract,
    /// The step budget was exhausted before a lawful conclusion.
    StepBudgetExhausted,
    /// The evidence basis is incomplete for the requested conclusion.
    EvidenceBasisIncomplete,
    /* the roster closes with the reconciliation contract */
}

/// Bounded allowance for one reconciliation pass.
///
/// Charging consumes; no widening method exists anywhere.
pub struct ReconciliationStepBudget {
    remaining: u64,
}

// ---------------------------------------------------------------------------
// Drivers, wakes, and delivery bounds
// ---------------------------------------------------------------------------

/// Bounded work allowance for one cooperative drive call.
///
/// Charging consumes; no widening method exists anywhere.
pub struct PumpWorkBudget {
    remaining: u64,
}

/// What one drive call established.
///
/// Progress reporting only: none of these values carries semantic results, parity, or checkpoint meaning.
pub enum DriveOutcome {
    /// The transition advanced at least one bounded unit of logical work.
    MadeProgress,
    /// A typed boundary action awaits host realization.
    NeedsHostAction,
    /// No admitted stimulus is currently runnable.
    Quiescent,
    /// The driven operation reached a terminal state.
    Terminal,
}

/// Generation binding for wake interest.
///
/// A wake carrying a stale generation is not runnable work.
pub struct WakeGeneration {
    generation: u64,
}

/// One registered wake interest, generation-bound.
///
/// A wake may be lost, duplicated, or coalesced; loss costs latency, never work.
pub struct WakeToken {
    /* private: interest identity, generation */
}

/// Maximum retained deferred inputs for one process.
pub struct DeferredInputLimit {
    limit: NonZeroU32,
}

/// Maximum retained deferred-input bytes for one process.
pub struct DeferredInputByteLimit {
    limit: NonZeroU64,
}

/// Maximum retained age of one deferred input.
///
/// A Time-class bound; the numeric value lives in the depot.
pub struct DeferredInputAgeLimit {
    /* a Time-class bound; closes with the depot profile */
}

/// Bounded work allowance for reconsidering deferred inputs in one pass.
///
/// Charging consumes; no widening method exists anywhere.
pub struct DeferredInputReconsiderationBudget {
    remaining: u64,
}

/// Maximum lag between accepted history and one consumer's checkpoint before the profile's declared posture applies.
pub struct CheckpointLagLimit {
    limit: NonZeroU64,
}

// ---------------------------------------------------------------------------
// Deadlines: the three-object split
// ---------------------------------------------------------------------------

/// Durable semantic deadline commitment for one operation.
///
/// Survives process death; never process-monotonic by itself.
/// Exactly one policy governs one operation across admission, execution, suspension, retry, outcome query, reconciliation, and every host adapter.
/// (Shape roster recovered; the paved-default classification is a depot row.)
pub enum DeadlinePolicy {
    /// A durable duration budget — the paved road.
    DurationBudget(DurationBudgetPolicy),
    /// A bound against a durable chronology coordinate.
    ChronologyBound(ChronologyBoundPolicy),
    /// Wall-anchored with declared tolerance — expressible, explicit, never a default.
    WallAnchoredWithTolerance(WallAnchoredPolicy),
}

/// Body of the duration-budget policy shape.
pub struct DurationBudgetPolicy {
    /* durable allowance commitment; closes with the time profile */
}

/// Body of the chronology-bound policy shape.
pub struct ChronologyBoundPolicy {
    /* durable coordinate binding; closes with the chronology profile */
}

/// Body of the wall-anchored policy shape, with its explicit tolerance.
pub struct WallAnchoredPolicy {
    /* anchor and tolerance; closes with the time profile */
}

/// Persisted named observations of already-spent allowance.
///
/// Evidence, not enforcement state; a policy has no "remainder" — the remaining allowance is derived at rebase.
pub struct ConsumedBudgetEvidence {
    /* owner-derived persisted observations */
}

/// Process-local enforcement state in one monotonic clock domain.
///
/// Never serialized, never transplanted to another host, dies with the process; reconstructed by rebasing policy against a monotonic observation and consumed evidence.
/// Remaining allowance never grows without a new explicit authority decision.
/// Never `Clone`.
pub struct LiveDeadline {
    /// The one monotonic clock domain this state is meaningful in (port-owned role).
    domain: ClockDomainId,
    /* private expiry point */
}

/// Closed refusal family for deadline rebase and checks.
///
/// Lost provenance defaults to refusal — the safety-relevant posture.
#[must_use]
pub enum DeadlineRefusal {
    /// Spent-allowance provenance is lost or incomplete; refusal, never an optimistic rebase.
    LostProvenance,
    /// The rebased remaining allowance is exhausted.
    AllowanceExhausted,
    /// The observation's clock domain does not match the live state's.
    ClockDomainMismatch,
    /* the roster closes with the time profile */
}

// ---------------------------------------------------------------------------
// Checkpoint authority
// ---------------------------------------------------------------------------

/// One runtime-owned checkpoint authority region.
///
/// Holds accepted checkpoint-advance records; never inhabits or mutates an observed domain region.
/// One writer per region and epoch.
pub struct CheckpointRegionId {
    /* closes with the identity profile */
}

/// Epoch of one checkpoint authority region.
pub struct CheckpointEpoch {
    epoch: u64,
}

/// The role-branded subject one checkpoint chain is keyed by.
///
/// A process checkpoint never substitutes for a subscription or delivery checkpoint merely because the identities share a width.
pub enum CheckpointSubject {
    Process(ProcessId),
    /// View-owned identity; the durable skip authority stays here.
    Subscription(SubscriptionId),
    Delivery(DeliveryId),
}

/// Generation of one checkpoint subject, for wrong-generation refusal at admission.
pub struct CheckpointSubjectGeneration {
    /* closes with the identity profile */
}

/// Identity of one accepted checkpoint-advance record.
pub struct CheckpointAdvanceId {
    /* closes with the identity profile */
}

/// The expected predecessor of one proposed advance: the prior accepted advance, or the explicit genesis posture of a fresh chain.
pub struct ExpectedCheckpointPredecessor {
    /* predecessor binding or genesis posture; closes with the checkpoint contract */
}

/// A proposed checkpoint advance.
///
/// Admissible only when every skip prerequisite exists; advancing before prerequisites is unrepresentable or refuses.
#[must_use]
pub struct CheckpointAdvanceProposal {
    region: CheckpointRegionId,
    epoch: CheckpointEpoch,
    subject: CheckpointSubject,
    generation: CheckpointSubjectGeneration,
    predecessor: ExpectedCheckpointPredecessor,
    cuts: ReferencedDomainCuts,
    completed: CompletedTurnSet,
    receipts: RequiredReceiptSet,
    obligations: AccountedObligationSet,
}

/// The exact domain Cuts whose completed work one advance covers.
///
/// References domain regions; never inhabits or mutates them.
pub struct ReferencedDomainCuts {
    /* exact Cut references per source; closes with the checkpoint contract */
}

/// The bounded set of completed Turns one advance accounts.
pub struct CompletedTurnSet {
    turns: Vec<TurnId>,
}

/// The required output and publication receipts one advance must present.
pub struct RequiredReceiptSet {
    /* role-specific receipt references; closes with the checkpoint contract */
}

/// Every outstanding effect and reconciliation obligation accounted by one advance.
///
/// A forgotten obligation makes the proposal inadmissible.
pub struct AccountedObligationSet {
    /* obligation references; closes with the checkpoint contract */
}

/// One accepted checkpoint-advance record: the authoritative grant of skip authority.
///
/// Append-only; never a mutable pointer.
pub struct AcceptedCheckpointAdvance {
    id: CheckpointAdvanceId,
    region: CheckpointRegionId,
    epoch: CheckpointEpoch,
    subject: CheckpointSubject,
    /// Commitment over the admitted proposal's canonical bytes.
    proposal: CheckpointProposalCommitment,
    predecessor: ExpectedCheckpointPredecessor,
}

/// Commitment over the canonical bytes of one admitted advance proposal.
pub struct CheckpointProposalCommitment {
    /* closes with the canon profile */
}

/// Compact derived image of one subject's checkpoint chain, for fast start.
///
/// Rebuildable from accepted advances; never the authority.
pub struct CurrentCheckpoint {
    subject: CheckpointSubject,
    derived_from: CheckpointAdvanceId,
    /* derived image body */
}

/// Durable evidence that one checkpoint advance was accepted and published.
#[must_use]
pub struct CheckpointReceipt {
    advance: CheckpointAdvanceId,
    /* publication evidence */
}

/// Role-specific authority to admit a checkpoint advance for one subject.
///
/// No other grant substitutes.
/// Never `Clone`.
#[must_use]
pub struct CheckpointAdvanceGrant {
    /* private authority binding */
}

/// Closed refusal family for checkpoint admission.
#[must_use]
pub enum CheckpointRefusal {
    /// A required skip prerequisite does not exist.
    PrerequisiteMissing,
    /// The expected predecessor is not the chain's current tip.
    WrongPredecessor,
    /// The subject generation does not match.
    WrongGeneration,
    /// The presented grant does not cover this subject, region, and epoch.
    WrongAuthority,
    /* the roster closes with the checkpoint contract */
}

// ---------------------------------------------------------------------------
// JoinAll: the first semantic join family
// ---------------------------------------------------------------------------

/// Deterministic identity of one branch inside one join.
///
/// Branch order is declared, never completion order.
pub struct JoinBranchId {
    index: u32,
}

/// Everything one branch established, preserved without loss.
pub struct JoinBranchRecord {
    branch: JoinBranchId,
    /// The branch's semantic conclusion — the same two-road role the Turn record carries; declared once with the logical-runtime nouns.
    conclusion: SemanticConclusion,
    /// The intents this branch durably admitted (they survive regardless of sibling outcomes).
    effects: AdmittedIntentSet,
    outcome: OutcomeKnowledge,
    /// Program-owned progressive-explanation binding.
    explanation: Explanation,
    /// Program-owned portable work accounting.
    work: ConsumedSemanticWork,
    /* evidence references */
}

/// The bounded set of intents one branch durably admitted.
pub struct AdmittedIntentSet {
    intents: Vec<EffectIntentId>,
}

/// The complete join conclusion: every branch's record in deterministic branch order.
///
/// A physical race winner never becomes a semantic winner.
#[must_use]
pub struct JoinAllOutcome {
    branches: Vec<JoinBranchRecord>,
}

/// Maximum branches of one join.
///
/// Work class.
pub struct JoinBranchLimit {
    limit: NonZeroU32,
}

// ---------------------------------------------------------------------------
// PakVM
// ---------------------------------------------------------------------------

/// One value of the closed PakVM algebra: exact scalars, role-specific identities and references, records and variants, bounded collections, exact numeric roles, admitted address and cut values, immutable image constants, bounded continuation captures, and opaque typed handles.
///
/// Never `Any`, a host object, raw pointer, callback, Future, socket, ambient capability, or serialized live authority.
/// The exact representation and operator inventory remain a recorded open seam.
pub struct VmValue {
    /* closed algebra; exact representation owner-derived */
}

/// What one synchronous PakVM step established.
///
/// PakVM never awaits the host; it stops at exactly one boundary.
pub enum VmStep {
    /// The machine may step again.
    Continue,
    /// The program returned a value.
    Returned(VmValue),
    /// Execution-state integrity was violated; this is not image validation.
    Refused(ExecutionIntegrityRefusal),
    /// The program produced event proposals (the program owner's carrier over the event owner's noun; event admission decides acceptance).
    Publication(EventProposals),
    /// The program produced one typed port request and one bounded one-shot suspension (`PortRequest` is port-owned).
    Requested {
        request: PortRequest,
        continuation: SuspendedVm,
    },
    /// An admitted semantic budget was exhausted.
    Exhausted(SemanticBudgetExhaustion),
}

/// One bounded suspension: request-bound, Attempt-bound, generation-bound, deadline-bound.
///
/// Resumed exactly once, terminated exactly once, or abandoned and sealed as physical evidence.
/// Process death destroys it; recovery mints a fresh Attempt.
/// The persisted continuation record carries the `DeadlinePolicy` reference plus `ConsumedBudgetEvidence` — never a live monotonic deadline.
/// Never `Clone`, never serialized live.
#[must_use]
pub struct SuspendedVm {
    request: PortRequestId,
    attempt: AttemptId,
    generation: ProcessGeneration,
    /* private: resume coordinate, bounded captures, deadline-policy and consumed-evidence references, cancellation posture */
}

/// Closed refusal family for execution-state integrity.
///
/// Semantic image validation is the program owner's and is not represented here.
#[must_use]
pub enum ExecutionIntegrityRefusal {
    /// Live execution state is corrupt.
    CorruptLiveState,
    /// A continuation received a response for the wrong coordinates.
    WrongContinuationResponse,
    /// An operator reached a state its algebra makes impossible.
    ImpossibleOperatorState,
    /* the roster closes with the VmValue seam */
}

/// Which admitted semantic bound was exhausted, with its exact dimension.
///
/// A typed terminal fact, never a panic.
pub struct SemanticBudgetExhaustion {
    /// The shared classification of the exhausted bound (core-owned).
    class: BoundClass,
    /* exact owner-named dimension and consumed value */
}

/// Maximum live frames on the explicit PakVM frame stack.
pub struct FrameLimit {
    limit: NonZeroU32,
}

/// Maximum bytes one PakVM value may occupy.
pub struct ValueByteLimit {
    limit: NonZeroU64,
}

/// Maximum bytes of transient PakVM scratch for one Turn.
pub struct ScratchByteLimit {
    limit: NonZeroU64,
}

/// Maximum bytes one suspension's captures may retain.
pub struct ContinuationByteLimit {
    limit: NonZeroU64,
}

// ---------------------------------------------------------------------------
// Bvisor
// ---------------------------------------------------------------------------

/// Identity of one physical effort.
///
/// Minted only by Bvisor admission; no Attempt exists before physical admission, and no retry reuses one.
pub struct AttemptId {
    /* closes with the identity profile */
}

/// The lineage relationship of one Attempt: edge references by identity, never containment.
///
/// Cancellation and termination propagate along typed fate-links, not ownership.
pub struct AttemptLineage {
    /* parent and fate-link references; closes with the admission contract */
}

/// The closed physical premise set one admission plan is checked against: required port bindings and grant identities with their expected generations and revocation posture, the target profile, the participating clock domains, the reservation dimensions, and the report requirements.
///
/// Derived from the Turn's admitted program invocation and its operation's declared port contract — program- and port-owned declarations carried here as data; this type mints no authority and installs nothing.
pub struct PhysicalAdmissionRequirements {
    /* requirement rosters; close with the admission contract */
}

/// The closed physical admission plan for one admitted logical invocation.
#[must_use]
pub struct AdmissionPlan {
    /// The validated image (program-owned identity) and its operation.
    image: ProgramImageId,
    operation: LogicalOperationId,
    turn: TurnId,
    cuts: FrozenTurnInputs,
    deadline: AbsoluteDeadline,
    /// The closed physical premise set this plan was checked against.
    requirements: PhysicalAdmissionRequirements,
    /* semantic and physical bound bindings close with the admission contract */
}

/// Closed refusal family for physical admission.
///
/// A refusal here means no Attempt ever existed and no reservation survived.
#[must_use]
pub enum PhysicalAdmissionRefusal {
    /// A required reservation dimension is unavailable; everything acquired was released.
    ReservationUnavailable,
    /// A required grant is absent or its generation is stale.
    GrantGenerationStale,
    /// The operation's absolute deadline had already expired.
    DeadlineAlreadyExpired,
    /// The selected target profile cannot realize a required dimension.
    TargetProfileUnsupported,
    /* the roster closes with the admission contract */
}

/// Live custody of one freshly admitted Attempt: installed grants, reservations, port bindings, deadline, and one-shot response authority.
///
/// Each lifecycle transition consumes the prior live value.
/// Never `Clone`, never serialized, never accidentally `Send`/`Sync`; a persisted description never resurrects custody.
#[must_use]
pub struct AdmittedAttempt {
    id: AttemptId,
    lineage: AttemptLineage,
    /* private custody: grants, reservations, ports, deadline */
}

/// Custody of one Attempt actively stepping PakVM.
///
/// Consumed on suspension or termination.
/// Same custody laws as `AdmittedAttempt`.
#[must_use]
pub struct RunningAttempt {
    id: AttemptId,
    /* private custody */
}

/// Custody of one Attempt with exactly one outstanding port request and one bounded suspension.
///
/// Consumed by the one lawful resume or by termination.
#[must_use]
pub struct LiveSuspendedAttempt {
    id: AttemptId,
    outstanding: PortRequestId,
    /* private custody */
}

/// Custody of one Attempt with no further physical work.
///
/// Consumed by sealing the report.
#[must_use]
pub struct TerminalAttempt {
    id: AttemptId,
    /* private custody */
}

/// Sealed physical evidence of one Attempt: identity and lineage, selected mechanism profile, installed grants, reservation and consumption evidence, requests and validated responses, cancellation and deadline observations, exits, and commit knowledge where a backend establishes it.
///
/// Never domain success, retry legality, compensation, checkpoint advancement, or semantic correctness.
pub struct AttemptReport {
    id: AttemptId,
    lineage: AttemptLineage,
    /* sealed evidence body */
}

/// Affine custody of one acquired physical resource: Attempt-bound, released exactly once, never reusable by a retry, never `Clone`, never serialized.
///
/// Partial acquisition releases everything acquired and refuses.
#[must_use]
pub struct ResourceReservation {
    attempt: AttemptId,
    dimension: ReservationDimension,
    /* private custody: quantity, release obligation */
}

/// One named physical resource dimension a reservation covers.
pub struct ReservationDimension {
    /* dimension identity; closes with the admission contract */
}

/// Closed refusal family for reservation acquisition.
#[must_use]
pub enum ReservationRefusal {
    /// The dimension's scope limit is reached.
    DimensionExhausted,
    /// Acquisition was partial; everything acquired was released.
    PartialAcquisitionReleased,
    /* the roster closes with the admission contract */
}

/// One-shot authority to apply exactly one validated response to exactly one suspended Attempt.
///
/// A response matching different correctness-bearing coordinates refuses; a late response for a dead Attempt resumes nothing and may remain authentic physical evidence.
/// Never `Clone`.
#[must_use]
pub struct AttemptResponseAuthority {
    request: PortRequestId,
    attempt: AttemptId,
    generation: ProcessGeneration,
    deadline: AbsoluteDeadline,
    /* private continuation binding */
}

/// Closed refusal family for applying a response to a suspension.
#[must_use]
pub enum AttemptResponseRefusal {
    /// A correctness-bearing coordinate (request, Attempt, role, generation, deadline) does not match.
    WrongCoordinates,
    /// The one-shot authority was already consumed.
    AuthorityConsumed,
    /// The target Attempt is no longer live; the response may remain physical evidence.
    AttemptNoLongerLive,
    /* the roster closes with the admission contract */
}

/// What Bvisor observed when attempting physical cancellation under the selected profile.
///
/// Never proof that the external consequence did not occur.
pub struct CancellationObservation {
    attempt: AttemptId,
    /* observation record */
}

/// Maximum concurrently live Attempts under one admission scope.
pub struct AttemptLimit {
    limit: NonZeroU32,
}

/// Maximum concurrently held reservations under one admission scope.
pub struct ReservationLimit {
    limit: NonZeroU32,
}

/// Maximum outstanding port requests for one Attempt.
pub struct PortRequestLimit {
    limit: NonZeroU32,
}

// The response byte ceiling is the port owner's `PortResponseByteLimit` — its declaration seat is the port contract; this owner consumes it inside response binding and declares no twin.

/// Identity of one admitted absolute-deadline commitment, so a carried allowance at the port boundary can name exactly which commitment it derives from.
pub struct AbsoluteDeadlineId {
    /* closes with the identity profile */
}

/// The admitted absolute deadline bound into one AdmissionPlan.
///
/// Derived from the operation's `DeadlinePolicy` at admission, meaningful in one clock domain; no adapter, retry, or fresh Attempt mints a later one.
pub struct AbsoluteDeadline {
    id: AbsoluteDeadlineId,
    /// The clock domain the expiry commitment is meaningful in (port-owned role).
    domain: ClockDomainId,
    /* expiry commitment; closes with the time profile */
}

// ---------------------------------------------------------------------------
// Profiles — the lawful configuration algebras this owner declares.
// Depot rows select coordinates inside them; operations receive the selected row as an explicit argument, never through an ambient lookup.
// ---------------------------------------------------------------------------

/// The declared algebra of one driver profile: who drives, when driving is required, the pump work allowance, runnable-lane fairness posture, starvation posture, wake registration and lost-wakeup prevention, spurious-wake and coalescing handling, reentrancy posture, callback custody, close/drain/shutdown/restart selections, panic containment selection, and which liveness claim becomes unavailable when the host stops driving.
///
/// Fairness is a declared claim, never queue-library folklore.
pub struct DriverProfile {
    /* declared axes above; exact representation closes with the paved driver escalation */
}

/// The declared algebra of one checkpoint storage profile: the publication family binding for accepted advances and the fast-start posture.
///
/// The storage mechanism is realization; this algebra is not.
pub struct CheckpointStorageProfile {
    /* declared axes; closes with the checkpoint-storage seam */
}

/// The declared algebra of one reconciliation profile: outcome-query cadence posture, compensation authorization posture, and partial-outcome admission posture — each selecting within the operation's recovery contract, never widening it.
pub struct ReconciliationProfile {
    /* declared axes; closes with the reconciliation contract */
}

/// The declared algebra of one effect-admission profile: the declared effect bounds REQUEST/PEND admission enforces and the required idempotency-ladder posture — each selecting within the operation's declared contracts, never widening one.
///
/// Depot rows select coordinates inside it.
pub struct EffectAdmissionProfile {
    /* declared axes; closes with the effect-admission contract */
}

/// The declared algebra of one panic containment profile.
///
/// The posture roster (caught unwind as physical observation, abort-and-fresh-Attempt, isolated worker) is the recorded owner escalation; this type seats the algebra without closing the fork.
pub struct PanicContainmentProfile {
    /* declared axes; the posture selection awaits the owner's ruling */
}
