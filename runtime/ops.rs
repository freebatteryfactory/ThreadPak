//! Runtime owner — thin semantic operation signatures.
//!
//! Declaration form: each operation is authored as a Rust function-pointer type alias stating its exact inputs, outputs, refusal family, and consumed bounds, preserving the signature shape the operation must have.
//! Foreign owner names stand unresolved in this fragment, so this file alone claims neither compilation nor resolved dependency edges; that evidence is the generated contract probe's to produce.
//! No bodies exist here and nothing claims implementation support: bodies land with construction cuts A5–A6, where Macroonz generates the conformance assertion (`const _: StitchFn = stitch;`) binding each realized operation to its declared alias.
//!
//! Depot law applies to every signature (`depot/README.md`, "Rows are passed, never fetched"): profiles and budgets arrive as explicit arguments; no operation reads an ambient registry, clock, or global context.
//! Affine budgets are consumed by value and returned smaller.
//!
//! Foreign roles named in these signatures stay owned by their declaring contracts; this file keeps no manual mirror of that inventory, which the generated contract probe derives.

// ---------------------------------------------------------------------------
// Logical runtime
// ---------------------------------------------------------------------------

/// Operation `stitch` — the runtime transition: prior admitted state plus one typed observation plus explicit context (the Turn being realized, the process contract, the deadline policy) yields the next state and the concluded Turn's record, or a typed refusal.
///
/// Pure and sans-I/O; produces proposals only — event admission and effect admission strengthen them elsewhere.
/// The produced `TurnConclusion` joins to the context Turn's identity.
/// A plain free function at realization: no trait exists until two real, substitutable transition providers exist.
pub type StitchFn = fn(
    context: &StitchContext,
    prior: &ProcessState,
    stimulus: StitchStimulus,
) -> Result<StitchAdvance, StitchRefusal>;

/// Operation `drive` — one cooperative drive call over one runtime lane.
///
/// Charges the pump budget and returns the smaller successor beside the outcome; progress reporting only — no semantic result, parity, or checkpoint meaning rides here.
pub type DriveFn = fn(
    profile: &DriverProfile,
    lane: &mut RuntimeLane,
    budget: PumpWorkBudget,
) -> (DriveOutcome, PumpWorkBudget);

/// Operation `register_wake` — register generation-bound wake interest on one lane, per the check-register-check-park protocol.
///
/// A wake may be lost, duplicated, or coalesced; loss costs latency, never work.
pub type RegisterWakeFn = fn(
    lane: &RuntimeLane,
    generation: ProcessGeneration,
) -> WakeToken;

// ---------------------------------------------------------------------------
// Effect admission and reconciliation
// ---------------------------------------------------------------------------

/// Operation `admit_effect_intent` — REQUEST/PEND admission: consume one program-produced proposal and mint the durable, runtime-owned intent plus its retry-discharging receipt.
///
/// The profile carries the declared effect bounds and required idempotency posture; the live deadline is the rebased evidence `EffectAdmissionRefusal::DeadlineAlreadyExpired` refuses against.
/// The admitted record survives any later semantic refusal.
/// The authorizing grant binding closes with the guard pass.
pub type AdmitEffectIntentFn = fn(
    profile: &EffectAdmissionProfile,
    posture: EffectAdmissionPosture,
    proposal: EffectProposal,
    turn: &Turn,
    deadline: &LiveDeadline,
) -> Result<(EffectIntent, EffectAdmissionReceipt), EffectAdmissionRefusal>;

/// Operation `reconcile` — one pure bounded reconciliation pass over append-only evidence: yields the current lawful conclusion and next action, charging the step budget.
///
/// Never rewrites an earlier Attempt, observation, or external event.
pub type ReconcileFn = fn(
    profile: &ReconciliationProfile,
    outstanding: &OutstandingEffect,
    budget: ReconciliationStepBudget,
) -> Result<(ReconciliationDecision, ReconciliationStepBudget), ReconciliationRefusal>;

// ---------------------------------------------------------------------------
// Checkpoint authority
// ---------------------------------------------------------------------------

/// Operation `propose_checkpoint_advance` — assemble one advance proposal from its prerequisites, under the authority region, epoch, and subject generation the proposal record binds.
///
/// Refuses when any skip prerequisite is missing — a proposal that cannot present its region, generation, receipts, and accounted obligations never exists.
pub type ProposeCheckpointAdvanceFn = fn(
    region: CheckpointRegionId,
    epoch: CheckpointEpoch,
    subject: CheckpointSubject,
    generation: CheckpointSubjectGeneration,
    predecessor: ExpectedCheckpointPredecessor,
    cuts: ReferencedDomainCuts,
    completed: CompletedTurnSet,
    receipts: RequiredReceiptSet,
    obligations: AccountedObligationSet,
) -> Result<CheckpointAdvanceProposal, CheckpointRefusal>;

/// Operation `admit_checkpoint_advance` — admit one advance under the role-specific grant, checking the expected predecessor against the chain's current durably published tip (`None` is the explicit fresh-chain posture), appending the accepted record, and returning its publication receipt.
///
/// Wrong predecessor, wrong generation, or wrong authority refuses before publication.
pub type AdmitCheckpointAdvanceFn = fn(
    grant: CheckpointAdvanceGrant,
    current: Option<&AcceptedCheckpointAdvance>,
    proposal: CheckpointAdvanceProposal,
) -> Result<(AcceptedCheckpointAdvance, CheckpointReceipt), CheckpointRefusal>;

/// Operation `derive_current_checkpoint` — derive the compact fast-start image of one subject's chain from its accepted advances.
///
/// Pure; the derived image is never the authority.
pub type DeriveCurrentCheckpointFn = fn(
    subject: CheckpointSubject,
    advances: &[AcceptedCheckpointAdvance],
) -> CurrentCheckpoint;

// ---------------------------------------------------------------------------
// JoinAll
// ---------------------------------------------------------------------------

/// Operation `collate_join` — collate branch records into the complete join conclusion in declared branch order, preserving every branch's result or refusal, admitted intents, outcome posture, explanation, evidence, and work.
///
/// A physical race winner never becomes a semantic winner.
pub type CollateJoinFn = fn(
    limit: JoinBranchLimit,
    branches: Vec<JoinBranchRecord>,
) -> JoinAllOutcome;

// ---------------------------------------------------------------------------
// Bvisor — physical admission and Attempt custody
// ---------------------------------------------------------------------------

/// Operation `plan_admission` — close one physical admission plan for one admitted logical invocation, checking the Turn against the closed physical premise set (required port bindings and grant identities with expected generations, target profile, clock domains, reservation dimensions, report requirements).
///
/// Pure planning; nothing physical happens and no Attempt exists yet.
pub type PlanAdmissionFn = fn(
    turn: &Turn,
    requirements: &PhysicalAdmissionRequirements,
    deadline: AbsoluteDeadline,
) -> Result<AdmissionPlan, PhysicalAdmissionRefusal>;

/// Operation `admit_attempt` — all-or-release physical admission: acquire every reservation, install grants and port bindings, and mint the fresh Attempt.
///
/// Partial acquisition releases everything acquired and refuses — no partially admitted Attempt and no leaked reservation exists.
pub type AdmitAttemptFn = fn(
    plan: AdmissionPlan,
) -> Result<AdmittedAttempt, PhysicalAdmissionRefusal>;

/// Operation `apply_response` — apply exactly one validated response to exactly one suspended Attempt, consuming the one-shot authority and the suspended custody.
///
/// A response matching different correctness-bearing coordinates refuses; a late response for a dead Attempt resumes nothing.
pub type ApplyResponseFn = fn(
    authority: AttemptResponseAuthority,
    suspended: LiveSuspendedAttempt,
    response: ValidatedResponse,
) -> Result<RunningAttempt, AttemptResponseRefusal>;

/// Operation `terminate_attempt` — consume any live Attempt custody into terminal custody.
///
/// Termination is a physical fact; it proves nothing about the external consequence.
pub type TerminateAttemptFn = fn(
    live: RunningAttempt,
    observation: CancellationObservation,
) -> TerminalAttempt;

/// Operation `seal_attempt_report` — seal the terminal Attempt's physical evidence.
///
/// Consumes the custody; the sealed report is data and never resurrects anything.
pub type SealAttemptReportFn = fn(
    terminal: TerminalAttempt,
) -> AttemptReport;

// ---------------------------------------------------------------------------
// Deadlines
// ---------------------------------------------------------------------------

/// Operation `rebase_deadline` — rebase the durable policy against one admitted monotonic observation (the port owner's validated enclosure; raw readings never cross this boundary) and the persisted spent-allowance evidence, deriving process-local enforcement state in the observation's clock domain.
///
/// Remaining allowance never grows; lost provenance refuses.
pub type RebaseDeadlineFn = fn(
    policy: &DeadlinePolicy,
    consumed: &ConsumedBudgetEvidence,
    observation: MonotonicObservation,
) -> Result<LiveDeadline, DeadlineRefusal>;
