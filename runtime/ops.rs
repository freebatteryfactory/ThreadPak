//! Runtime owner — thin semantic operation signatures.
//!
//! Declarations only: each signature states its exact inputs, outputs,
//! refusal family, and consumed bounds. Bodies land with construction cuts
//! A1–A6; nothing here claims implementation support. Signatures are written
//! in semicolon form deliberately — this file is contract, not compilable
//! mechanism, until the dependency probe seats the homes.
//!
//! Depot law applies to every signature (`depot/README.md`, "Rows are
//! passed, never fetched"): profiles and budgets arrive as explicit
//! arguments; no operation reads an ambient registry, clock, or global
//! context. Affine budgets are consumed by value and returned smaller.
//!
//! Foreign roles by owner: `EffectProposal`, `ImmediateResult`,
//! `DecisionRefusal` (program); `ValidatedResponse`, `RawMonotonicObservation`
//! (port); `AcceptedCheckpointAdvance` consumers see only this owner's types.

// ---------------------------------------------------------------------------
// Logical runtime
// ---------------------------------------------------------------------------

/// The runtime transition: prior admitted state plus one typed observation
/// plus explicit context yields the next state and its proposals, or a typed
/// refusal. Pure and sans-I/O; produces proposals only — event admission and
/// effect admission strengthen them elsewhere. A plain free function: no
/// trait exists until two real, substitutable transition providers exist.
pub fn stitch(
    context: &StitchContext,
    prior: &ProcessState,
    stimulus: StitchStimulus,
) -> Result<StitchAdvance, StitchRefusal>;

/// One cooperative drive call over one runtime lane. Charges the pump budget
/// and returns the smaller successor beside the outcome; progress reporting
/// only — no semantic result, parity, or checkpoint meaning rides here.
pub fn drive(
    profile: &DriverProfile,
    lane: &mut RuntimeLane,
    budget: PumpWorkBudget,
) -> (DriveOutcome, PumpWorkBudget);

/// Register generation-bound wake interest on one lane, per the
/// check-register-check-park protocol. A wake may be lost, duplicated, or
/// coalesced; loss costs latency, never work.
pub fn register_wake(
    lane: &RuntimeLane,
    generation: ProcessGeneration,
) -> WakeToken;

// ---------------------------------------------------------------------------
// Effect admission and reconciliation
// ---------------------------------------------------------------------------

/// REQUEST/PEND admission: consume one program-produced proposal and mint
/// the durable, runtime-owned intent plus its retry-discharging receipt.
/// The admitted record survives any later semantic refusal. The authorizing
/// grant binding closes with the guard pass.
pub fn admit_effect_intent(
    posture: EffectAdmissionPosture,
    proposal: EffectProposal,
    turn: &Turn,
) -> Result<(EffectIntent, EffectAdmissionReceipt), EffectAdmissionRefusal>;

/// One pure bounded reconciliation pass over append-only evidence: yields
/// the current lawful conclusion and next action, charging the step budget.
/// Never rewrites an earlier Attempt, observation, or external event.
pub fn reconcile(
    profile: &ReconciliationProfile,
    outstanding: &OutstandingEffect,
    budget: ReconciliationStepBudget,
) -> Result<(ReconciliationDecision, ReconciliationStepBudget), ReconciliationRefusal>;

// ---------------------------------------------------------------------------
// Checkpoint authority
// ---------------------------------------------------------------------------

/// Assemble one advance proposal from its prerequisites. Refuses when any
/// skip prerequisite is missing — a proposal that cannot present its
/// receipts and accounted obligations never exists.
pub fn propose_checkpoint_advance(
    subject: CheckpointSubject,
    predecessor: ExpectedCheckpointPredecessor,
    cuts: ReferencedDomainCuts,
    completed: CompletedTurnSet,
    receipts: RequiredReceiptSet,
    obligations: AccountedObligationSet,
) -> Result<CheckpointAdvanceProposal, CheckpointRefusal>;

/// Admit one advance under the role-specific grant, appending the accepted
/// record and returning its publication receipt. Wrong predecessor, wrong
/// generation, or wrong authority refuses before publication.
pub fn admit_checkpoint_advance(
    grant: CheckpointAdvanceGrant,
    proposal: CheckpointAdvanceProposal,
) -> Result<(AcceptedCheckpointAdvance, CheckpointReceipt), CheckpointRefusal>;

/// Derive the compact fast-start image of one subject's chain from its
/// accepted advances. Pure; the derived image is never the authority.
pub fn derive_current_checkpoint(
    subject: CheckpointSubject,
    advances: &[AcceptedCheckpointAdvance],
) -> CurrentCheckpoint;

// ---------------------------------------------------------------------------
// JoinAll
// ---------------------------------------------------------------------------

/// Collate branch records into the complete join conclusion in declared
/// branch order, preserving every branch's result or refusal, admitted
/// intents, outcome posture, explanation, evidence, and work. A physical
/// race winner never becomes a semantic winner.
pub fn collate_join(
    limit: JoinBranchLimit,
    branches: Vec<JoinBranchRecord>,
) -> JoinAllOutcome;

// ---------------------------------------------------------------------------
// Bvisor — physical admission and Attempt custody
// ---------------------------------------------------------------------------

/// Close one physical admission plan for one admitted logical invocation.
/// Pure planning; nothing physical happens and no Attempt exists yet.
pub fn plan_admission(
    turn: &Turn,
    deadline: AbsoluteDeadline,
) -> Result<AdmissionPlan, PhysicalAdmissionRefusal>;

/// All-or-release physical admission: acquire every reservation, install
/// grants and port bindings, and mint the fresh Attempt. Partial
/// acquisition releases everything acquired and refuses — no partially
/// admitted Attempt and no leaked reservation exists.
pub fn admit_attempt(
    plan: AdmissionPlan,
) -> Result<AdmittedAttempt, PhysicalAdmissionRefusal>;

/// Apply exactly one validated response to exactly one suspended Attempt,
/// consuming the one-shot authority and the suspended custody. A response
/// matching different correctness-bearing coordinates refuses; a late
/// response for a dead Attempt resumes nothing.
pub fn apply_response(
    authority: AttemptResponseAuthority,
    suspended: LiveSuspendedAttempt,
    response: ValidatedResponse,
) -> Result<RunningAttempt, AttemptResponseRefusal>;

/// Consume any live Attempt custody into terminal custody. Termination is a
/// physical fact; it proves nothing about the external consequence.
pub fn terminate_attempt(
    live: RunningAttempt,
    observation: CancellationObservation,
) -> TerminalAttempt;

/// Seal the terminal Attempt's physical evidence. Consumes the custody;
/// the sealed report is data and never resurrects anything.
pub fn seal_attempt_report(
    terminal: TerminalAttempt,
) -> AttemptReport;

// ---------------------------------------------------------------------------
// Deadlines
// ---------------------------------------------------------------------------

/// Rebase the durable policy against one monotonic observation and the
/// persisted spent-allowance evidence, deriving process-local enforcement
/// state in the observation's clock domain. Remaining allowance never grows;
/// lost provenance refuses.
pub fn rebase_deadline(
    policy: &DeadlinePolicy,
    consumed: &ConsumedBudgetEvidence,
    observation: RawMonotonicObservation,
) -> Result<LiveDeadline, DeadlineRefusal>;
