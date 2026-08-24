//! Runtime owner nouns: the logical runtime, PakVM, and Bvisor role graph.
//!
//! Declarations only. Operations live in verb-named modules beside this file;
//! no behavior is defined here. Crossing types named in field positions —
//! `ExecutableProgramImage`, `EventProposals`, `EffectProposal` (program),
//! `Cut`, `AcceptedEvent` (event), `PortRequest`, `RawMonotonicObservation`
//! (port) — are foreign-owned; their declarations live with their owners and
//! resolve by import when the homes are seated. The durable `EffectIntent`
//! record — minted by REQUEST and PEND admission from a program's
//! `EffectProposal` — is this owner's, declared with the effect-admission
//! repair pass alongside its publication contract.
//!
//! Derive discipline: `Clone`, `Copy`, `Serialize`, `Deserialize`, `Ord`, and
//! `Hash` are semantic claims. A type without them here must not acquire them
//! for convenience; custody and one-shot types must never implement `Clone`
//! or any serialization.

use core::num::{NonZeroU32, NonZeroU64};

// ---------------------------------------------------------------------------
// Logical runtime: Turn, Stitch, processes
// ---------------------------------------------------------------------------

/// Identity of one logical transition. Derived from the Turn's identity
/// inputs; replay reconstructs the same Turn where lawful, and a changed
/// identity input is a different Turn. Exact width and preimage law live
/// with the identity owner.
pub struct TurnId {
    // derived identity bytes; representation owner-derived
}

/// Stable identity of one logical process. Not an OS thread, task, actor,
/// mailbox, or physical worker.
pub struct ProcessId {
    // identity bytes; representation owner-derived
}

/// Generation of one logical process. A wake, continuation, or observation
/// bound to a prior generation is not runnable work in a later one.
pub struct ProcessGeneration {
    generation: u64,
}

/// Explicit context supplied to the Stitch transition: applicable policy,
/// bounds, and generation facts. Carries no ambient clock, no executor, and
/// no I/O capability.
pub struct StitchContext {
    // owner-derived: policy, bounds, generation bindings
}

/// The result of one Stitch transition: the next admitted state, event
/// proposals for event admission, explicit effect intents, portable semantic
/// work, and explanation. Producing this value publishes nothing; only event
/// admission makes a proposal an accepted fact.
#[must_use]
pub struct StitchAdvance {
    // owner-derived: next state, proposals, intents, work, explanation
}

/// Closed refusal family for the Stitch transition. Variants are
/// owner-derived; no generic status exists.
#[must_use]
pub struct StitchRefusal {
    // owner-derived refusal variants
}

/// One operation–observer relationship. Resolves at most once with the
/// strongest honest terminal observation available at that boundary. It is
/// not the outcome, the continuation, a receipt, a checkpoint, or retry
/// authority. Never `Clone`.
#[must_use]
pub struct Completion {
    // private resolution state
}

/// One lawfully admitted input that cannot run in the current process state:
/// the input identity, the reason, the exact state or evidence that could
/// unblock it, age, absolute deadline, and collation position. Terminal
/// refusal when it can no longer become admissible.
pub struct DeferredInput {
    // owner-derived retained-input record
}

/// Closed refusal family for deferred input, including the terminal
/// can-never-become-admissible refusal and the bounds refusals.
#[must_use]
pub struct DeferredInputRefusal {
    // owner-derived refusal variants
}

/// A separately authorized request that physical work stop. Distinct from
/// observer abandonment, deadline expiry, close, drain, and shutdown.
pub struct CancellationRequest {
    // owner-derived: authority, scope, target
}

// ---------------------------------------------------------------------------
// Effect relationship and reconciliation
// ---------------------------------------------------------------------------

/// Whether the external consequence of one effect is established. `Known`
/// carries its conclusion through the reconciliation record; `OutcomeUnknown`
/// is a stable honest result, never relabeled as failure.
pub enum OutcomeKnowledge {
    /// Admitted evidence establishes the outcome.
    Known,
    /// An external consequence may exist; evidence cannot establish it.
    OutcomeUnknown,
}

/// The durable relationship around one admitted EffectIntent that has no
/// terminal outcome knowledge: known Attempts, evidence so far, and the
/// applicable recovery posture.
pub struct OutstandingEffect {
    // owner-derived durable relationship
}

/// The durable fact that outcome knowledge for one effect remains
/// unresolved. A checkpoint that forgets this obligation cannot lawfully
/// skip the work it covers.
pub struct ReconciliationObligation {
    // owner-derived obligation record
}

/// The current lawful conclusion and next action derived purely from
/// append-only evidence: observe, wait, one fresh lawful Attempt, a
/// separately admitted compensation, an authorized decision, a durable
/// partial outcome where the contract permits, or terminate unresolved.
/// Never rewrites an earlier Attempt or observation.
#[must_use]
pub struct ReconciliationDecision {
    // owner-derived conclusion record
}

/// Closed refusal family for reconciliation.
#[must_use]
pub struct ReconciliationRefusal {
    // owner-derived refusal variants
}

/// Bounded allowance for one reconciliation pass. Charging consumes; no
/// widening method exists anywhere.
pub struct ReconciliationStepBudget {
    remaining: u64,
}

// ---------------------------------------------------------------------------
// Drivers, wakes, and delivery bounds
// ---------------------------------------------------------------------------

/// Bounded work allowance for one cooperative drive call. Charging consumes;
/// no widening method exists anywhere.
pub struct PumpWorkBudget {
    remaining: u64,
}

/// What one drive call established. Progress reporting only: none of these
/// values carries semantic results, parity, or checkpoint meaning.
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

/// Generation binding for wake interest. A wake carrying a stale generation
/// is not runnable work.
pub struct WakeGeneration {
    generation: u64,
}

/// One registered wake interest, generation-bound. A wake may be lost,
/// duplicated, or coalesced; loss costs latency, never work.
pub struct WakeToken {
    // private: interest identity, generation
}

/// Maximum retained deferred inputs for one process.
pub struct DeferredInputLimit {
    limit: NonZeroU32,
}

/// Maximum retained deferred-input bytes for one process.
pub struct DeferredInputByteLimit {
    limit: NonZeroU64,
}

/// Maximum lag between accepted history and one consumer's checkpoint before
/// the profile's declared posture applies.
pub struct CheckpointLagLimit {
    limit: NonZeroU64,
}

// ---------------------------------------------------------------------------
// Deadlines: the three-object split
// ---------------------------------------------------------------------------

/// Durable semantic deadline commitment for one operation. Survives process
/// death. Never process-monotonic by itself.
pub struct DeadlinePolicy {
    // owner-derived durable commitment
}

/// Persisted named observations of already-spent allowance. Evidence, not
/// enforcement state.
pub struct ConsumedBudgetEvidence {
    // owner-derived persisted observations
}

/// Process-local enforcement state in one monotonic clock domain. Never
/// serialized, never transplanted to another host, dies with the process;
/// reconstructed by rebasing policy against a monotonic observation and
/// consumed evidence. Remaining allowance never grows without a new explicit
/// authority decision. Never `Clone`.
pub struct LiveDeadline {
    // private: clock domain binding, expiry point
}

/// Closed refusal family for deadline rebase and checks.
#[must_use]
pub struct DeadlineRefusal {
    // owner-derived refusal variants
}

// ---------------------------------------------------------------------------
// Checkpoint authority
// ---------------------------------------------------------------------------

/// One runtime-owned checkpoint authority region. Holds accepted
/// checkpoint-advance records; never inhabits or mutates an observed domain
/// region. One writer per region and epoch.
pub struct CheckpointRegionId {
    // identity bytes; representation owner-derived
}

/// Epoch of one checkpoint authority region.
pub struct CheckpointEpoch {
    epoch: u64,
}

/// The process, subscription, or delivery identity one checkpoint chain is
/// keyed by.
pub struct CheckpointSubjectId {
    // identity bytes; representation owner-derived
}

/// A proposed checkpoint advance: expected predecessor, subject generation,
/// exact referenced domain Cuts, completed Turns, required output and
/// publication receipts, and every outstanding effect and reconciliation
/// obligation accounted. Admissible only when every skip prerequisite
/// exists.
#[must_use]
pub struct CheckpointAdvanceProposal {
    // owner-derived proposal record
}

/// One accepted checkpoint-advance record: the authoritative grant of skip
/// authority. Append-only; never a mutable pointer.
pub struct AcceptedCheckpointAdvance {
    // owner-derived accepted record
}

/// Compact derived image of one subject's checkpoint chain, for fast start.
/// Rebuildable from accepted advances; never the authority.
pub struct CurrentCheckpoint {
    // owner-derived derived image
}

/// Durable evidence that one checkpoint advance was accepted and published.
pub struct CheckpointReceipt {
    // owner-derived receipt record
}

/// Role-specific authority to admit a checkpoint advance for one subject.
/// No other grant substitutes. Never `Clone`.
#[must_use]
pub struct CheckpointAdvanceGrant {
    // private authority binding
}

/// Closed refusal family for checkpoint admission, including
/// prerequisite-missing, wrong-predecessor, wrong-generation, and
/// wrong-authority refusals.
#[must_use]
pub struct CheckpointRefusal {
    // owner-derived refusal variants
}

// ---------------------------------------------------------------------------
// JoinAll: the first semantic join family
// ---------------------------------------------------------------------------

/// Deterministic identity of one branch inside one join. Branch order is
/// declared, never completion order.
pub struct JoinBranchId {
    index: u32,
}

/// Everything one branch established, preserved without loss: its result or
/// refusal, effect intents, outcome posture, explanation, evidence, and
/// semantic work.
pub struct JoinBranchRecord {
    // owner-derived preserved branch facts
}

/// The complete join conclusion: every branch's record in deterministic
/// branch order. A physical race winner never becomes a semantic winner.
#[must_use]
pub struct JoinAllOutcome {
    // owner-derived: branch records in declared order
}

// ---------------------------------------------------------------------------
// PakVM
// ---------------------------------------------------------------------------

/// One value of the closed PakVM algebra: exact scalars, role-specific
/// identities and references, records and variants, bounded collections,
/// exact numeric roles, admitted address and cut values, immutable image
/// constants, bounded continuation captures, and opaque typed handles.
/// Never `Any`, a host object, raw pointer, callback, Future, socket,
/// ambient capability, or serialized live authority.
pub struct VmValue {
    // closed algebra; exact representation owner-derived
}

/// What one synchronous PakVM step established. PakVM never awaits the host;
/// it stops at exactly one boundary.
pub enum VmStep {
    /// The machine may step again.
    Continue,
    /// The program returned a value.
    Returned(VmValue),
    /// Execution-state integrity was violated; this is not image validation.
    Refused(ExecutionIntegrityRefusal),
    /// The program produced event proposals (the program owner's carrier
    /// over the event owner's noun; event admission decides acceptance).
    Publication(EventProposals),
    /// The program produced one typed port request and one bounded one-shot
    /// suspension (`PortRequest` is port-owned).
    Requested {
        request: PortRequest,
        continuation: SuspendedVm,
    },
    /// An admitted semantic budget was exhausted.
    Exhausted(SemanticBudgetExhaustion),
}

/// One bounded suspension: request-bound, Attempt-bound, generation-bound,
/// deadline-bound. Resumed exactly once, terminated exactly once, or
/// abandoned and sealed as physical evidence. Process death destroys it;
/// recovery mints a fresh Attempt. Never `Clone`, never serialized.
#[must_use]
pub struct SuspendedVm {
    // private: resume coordinate, bounded captures, bindings
}

/// Closed refusal family for execution-state integrity: corrupt live state,
/// a wrong continuation response, an impossible operator state. Semantic
/// image validation is the program owner's and is not represented here.
#[must_use]
pub struct ExecutionIntegrityRefusal {
    // owner-derived refusal variants
}

/// Which admitted semantic bound was exhausted, with its exact dimension.
/// A typed terminal fact, never a panic.
pub struct SemanticBudgetExhaustion {
    // owner-derived exhaustion record
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

/// Identity of one physical effort. Minted only by Bvisor admission; no
/// Attempt exists before physical admission, and no retry reuses one.
pub struct AttemptId {
    // identity bytes; representation owner-derived
}

/// The closed physical admission plan for one admitted logical invocation:
/// validated image and operation, Turn relationship, exact source cuts and
/// generations, required ports and grants, authority generations and
/// revocation posture, semantic and physical bounds, target profile, clock
/// domains, the absolute deadline, reservation dimensions, and report
/// requirements.
#[must_use]
pub struct AdmissionPlan {
    // owner-derived plan record
}

/// Closed refusal family for physical admission. A refusal here means no
/// Attempt ever existed and no reservation survived.
#[must_use]
pub struct PhysicalAdmissionRefusal {
    // owner-derived refusal variants
}

/// Live custody of one freshly admitted Attempt: installed grants,
/// reservations, port bindings, deadline, and one-shot response authority.
/// Each lifecycle transition consumes the prior live value. Never `Clone`,
/// never serialized; a persisted description never resurrects custody.
#[must_use]
pub struct AdmittedAttempt {
    // private custody: id, grants, reservations, ports, deadline
}

/// Custody of one Attempt actively stepping PakVM. Consumed on suspension
/// or termination.
#[must_use]
pub struct RunningAttempt {
    // private custody
}

/// Custody of one Attempt with exactly one outstanding port request and one
/// bounded suspension. Consumed by the one lawful resume or by termination.
#[must_use]
pub struct LiveSuspendedAttempt {
    // private custody: outstanding request binding
}

/// Custody of one Attempt with no further physical work. Consumed by sealing
/// the report.
#[must_use]
pub struct TerminalAttempt {
    // private custody
}

/// Sealed physical evidence of one Attempt: identity and lineage, selected
/// mechanism profile, installed grants, reservation and consumption
/// evidence, requests and validated responses, cancellation and deadline
/// observations, exits, and commit knowledge where a backend establishes it.
/// Never domain success, retry legality, compensation, checkpoint
/// advancement, or semantic correctness.
pub struct AttemptReport {
    // owner-derived sealed evidence
}

/// Affine custody of one acquired physical resource: Attempt-bound, released
/// exactly once, never reusable by a retry, never `Clone`, never serialized.
/// Partial acquisition releases everything acquired and refuses.
#[must_use]
pub struct ResourceReservation {
    // private custody: dimension, quantity, release obligation
}

/// Closed refusal family for reservation acquisition.
#[must_use]
pub struct ReservationRefusal {
    // owner-derived refusal variants
}

/// One-shot authority to apply exactly one validated response to exactly one
/// suspended Attempt. A response matching different correctness-bearing
/// coordinates refuses; a late response for a dead Attempt resumes nothing
/// and may remain authentic physical evidence. Never `Clone`.
#[must_use]
pub struct AttemptResponseAuthority {
    // private: request binding, expiry
}

/// What Bvisor observed when attempting physical cancellation under the
/// selected profile. Never proof that the external consequence did not
/// occur.
pub struct CancellationObservation {
    // owner-derived observation record
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

/// Maximum bytes one validated port response may occupy.
pub struct PortResponseLimit {
    limit: NonZeroU64,
}

/// The admitted absolute deadline bound into one AdmissionPlan. Derived from
/// the operation's DeadlinePolicy; no adapter, retry, or fresh Attempt mints
/// a later one.
pub struct AbsoluteDeadline {
    // owner-derived: clock-domain-bound expiry commitment
}
