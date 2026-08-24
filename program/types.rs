//! Program owner — role graph.
//!
//! Declarations only: this file states the nouns of executable meaning and the
//! co-seated Knowledge owner. Operations are stated in this owner's README;
//! construction laws are enforced by private fields and owner-module
//! constructors when this owner is realized.
//!
//! Cross-owner names (`Cut`, `SourceSet`, `EventProposal`, `PortFamilyId`,
//! `CapabilityRequirement`, `AdmittedObservation`, `Decision`) resolve to their
//! owning contracts; their declaration seats follow the dependency probe.
//! `EventProposal` is the event owner's noun — this owner carries it, never
//! declares it. The durable `EffectIntent` record is runtime-owned; this owner
//! declares only the inert `EffectProposal` that admission strengthens.
//! Where a field must agree with another field, neither field is public.
//!
//! Derives are semantic claims. Plain limits are `Copy`; budgets are affine and
//! deliberately derive nothing that could duplicate capacity. No type here
//! derives serialization: canonical bytes are the canon owner's law, and Rust
//! layout is not a wire format.

// ---------------------------------------------------------------------------
// Identity
// ---------------------------------------------------------------------------

/// Identity of one program's semantic commitment.
///
/// Derived from the canonical bytes of the checked semantic form under the
/// identity owner's preimage law. Two programs with one `ProgramId` carry one
/// semantic commitment; a changed commitment is a different program, never a
/// new "version" of the same identity.
pub struct ProgramId {
    id: [u8; 32],
}

/// Identity of one packaged `ProgramImage` artifact (semantic commitment plus
/// execution commitment plus closure), distinct from `ProgramId`: one program
/// may lawfully have more than one image realization over time.
pub struct ProgramImageId {
    id: [u8; 32],
}

// ---------------------------------------------------------------------------
// The authoring pipeline: descriptor → program → execution form → image
// ---------------------------------------------------------------------------

/// Authored data describing one program: operations, inputs, outputs, source
/// and Cut requirements, effect posture, bounds, refusal families, and
/// explanation relationships. A descriptor asserts nothing; checked
/// construction is the only road to a `Program`.
pub struct ProgramDescriptor {
    operations: OperationDescriptors,
    requirements: RequirementsProjection,
    declared_bounds: DeclaredBounds,
}

/// The closed roster of operation declarations inside one descriptor, each
/// carrying its posture, inputs, outputs, and refusal family. Bounded by
/// construction; a descriptor with an open operation set does not exist.
pub struct OperationDescriptors {
    operations: Vec<OperationDescriptor>,
}

/// One declared operation: its posture, its typed input and output roles, its
/// source and Cut requirements, and its refusal family.
pub struct OperationDescriptor {
    posture: OperationPosture,
    work: SemanticWorkFormula,
}

/// The checked semantic form: a descriptor that survived checked construction.
/// This is the semantic commitment PakVM never reinterprets.
pub struct Program {
    id: ProgramId,
    descriptor: ProgramDescriptor,
    recursion: RecursionWitness,
}

/// A `Program` that passed Semantic Form validation inside Gate 1 and may be
/// lowered. Distinct from `Program` so the gate's stages strengthen by type,
/// never by flag.
pub struct ValidatedSemanticProgram {
    program: Program,
}

/// The portable execution representation produced by the production lowerer:
/// explicit operators, control flow, value movement, frame layout, captures,
/// charging points, boundary-request points, and continuation layout.
///
/// "Candidate" is load-bearing: this form has not yet earned agreement and
/// grants nothing.
pub struct CandidateExecutionForm {
    lowered_from: ProgramId,
}

/// The packaged artifact binding one semantic commitment and one execution
/// commitment with the operation table, schema closure, required port
/// profiles, bounds, and entrypoints. Not executable: only the gate mints
/// `ExecutableProgramImage`.
pub struct ProgramImage {
    id: ProgramImageId,
    semantic_commitment: ProgramId,
}

/// An untrusted image after bounded decode and before any validation.
/// Foreign bytes and locally built images meet the gate as the same type:
/// there is no local shortcut.
pub struct DecodedProgramImage {
    bytes_commitment: [u8; 32],
}

/// The only image PakVM accepts. Private construction: the sole road here is
/// the complete Gate 1 chain, ending in `AgreementEstablished` plus closure
/// validation. Possession is proof the gate ran.
pub struct ExecutableProgramImage {
    image: ProgramImage,
    agreement: AgreementEstablished,
}

// ---------------------------------------------------------------------------
// Gate 1: independent lowering agreement
// ---------------------------------------------------------------------------

/// The independent route established the Semantic-to-Execution relation.
/// Private construction: only the agreement route mints this, and the
/// agreement route shares no load-bearing lowering or verdict logic with the
/// production lowerer.
pub struct AgreementEstablished {
    semantic: ProgramId,
    execution_commitment: [u8; 32],
}

/// An independently checked relation does not hold — red evidence naming the
/// disagreeing operation and the two readings. Never collapsed with
/// `AgreementNotEstablished`.
pub struct DisagreementEstablished {
    semantic: ProgramId,
}

/// The required independent route could not run to a verdict. Missing
/// required evidence, not a defect finding. Never collapsed with
/// `DisagreementEstablished`.
pub struct AgreementNotEstablished {
    semantic: ProgramId,
}

/// The closed outcome family of the agreement route. Variants wrap
/// private-construction witnesses, so no caller can mint a verdict.
pub enum AgreementOutcome {
    Established(AgreementEstablished),
    Disagreement(DisagreementEstablished),
    NotEstablished(AgreementNotEstablished),
}

// ---------------------------------------------------------------------------
// Transition: what one evaluation produces
// ---------------------------------------------------------------------------

/// The complete output of one pure program evaluation: proposed events,
/// proposed effects, an optional immediate result, the explanation, and
/// consumed-work accounting. A `Transition` performs nothing.
pub struct Transition {
    events: EventProposals,
    effects: EffectProposals,
}

/// The bounded, ordered set of event proposals inside one `Transition`.
/// Bounded by `EventProposalLimit` at construction. `EventProposal` is the
/// event owner's noun, carried here — admitted only by the event owner's
/// single admission operation at an expected Cut.
pub struct EventProposals {
    proposals: Vec<EventProposal>,
}

/// The bounded, ordered set of effect proposals inside one `Transition`.
/// Bounded by `EffectProposalLimit` at construction.
pub struct EffectProposals {
    proposals: Vec<EffectProposal>,
}

/// One proposed external effect: the port family, the typed request role,
/// the recovery posture, and the identity under which duplication is
/// recognized. Inert data — it performs nothing and commits nothing. Only
/// the runtime's REQUEST and PEND admission strengthens it into the durable,
/// runtime-owned `EffectIntent`, exactly as only event admission strengthens
/// an `EventProposal` into an `AcceptedEvent`.
pub struct EffectProposal {
    port_family: PortFamilyId,
}

/// One complete bounded batch of effect proposals built by the atomic-
/// planning recursion road. The batch is data; constructing it crosses
/// nothing. If the recursion refuses, no batch exists and nothing external
/// happened.
pub struct EffectBatch {
    proposals: EffectProposals,
}

// ---------------------------------------------------------------------------
// Postures and recursion
// ---------------------------------------------------------------------------

/// The declared posture of one program operation. The posture states meaning;
/// the runtime owner owns the REQUEST and PEND operations themselves.
/// Settled spellings: ASK, DO, REQUEST, PEND — and PEND is never AWAIT.
pub enum OperationPosture {
    /// Pure evaluation over supplied immutable inputs at exact Cuts.
    Ask,
    /// Admit local event-publication intent once applicable requirements close.
    Do,
    /// Admit this operation's effect proposal as a durable, runtime-owned
    /// `EffectIntent` and return without waiting.
    Request,
    /// Admit the same proposal durably and drive one immediate bounded
    /// Attempt.
    Pend,
}

/// The closed witness that makes well-founded, possibly effect-interleaved
/// recursion lawful: maximum effect count, effect ordering, capabilities,
/// semantic work, memory, output, suspension depth, continuation state, the
/// absolute deadline, and the recovery posture. An operation whose witness
/// does not close refuses at checked construction.
pub struct RecursionWitness {
    depth: RecursionDepthLimit,
    interleaved_effects: EffectProposalLimit,
}

/// Declared, portable semantic work as a function of the affected input set —
/// never CPU cycles, wall time, or scheduler observations. The enforceable
/// form of the shallow-push design intent: a declared formula, explicit
/// bounds, and parity with reference recomputation.
pub struct SemanticWorkFormula {
    formula: WorkFormulaBody,
}

// ---------------------------------------------------------------------------
// Bounds (classes per root law; numeric profiles live in the depot)
// ---------------------------------------------------------------------------

/// Work-class affine budget for one evaluation. Charging consumes the budget
/// and returns a smaller successor; no widening method exists anywhere.
/// Deliberately neither `Clone` nor `Copy`: duplicating it would fabricate
/// capacity.
pub struct SemanticWorkBudget {
    remaining: u64,
}

/// Work-class copyable limit on recursion depth for one operation.
#[derive(Clone, Copy)]
pub struct RecursionDepthLimit {
    limit: u32,
}

/// Output-class copyable limit on event proposals in one `Transition`.
#[derive(Clone, Copy)]
pub struct EventProposalLimit {
    limit: u32,
}

/// Effect-class copyable limit on effect proposals in one `Transition` or
/// one `EffectBatch`.
#[derive(Clone, Copy)]
pub struct EffectProposalLimit {
    limit: u32,
}

/// Result-class copyable limit on the immediate result value of one
/// evaluation.
#[derive(Clone, Copy)]
pub struct ResultValueLimit {
    limit_bytes: u64,
}

/// Suspension-class copyable limit on boundary suspensions one operation may
/// declare.
#[derive(Clone, Copy)]
pub struct SuspensionLimit {
    limit: u32,
}

// ---------------------------------------------------------------------------
// Knowledge: the co-seated honesty owner
// ---------------------------------------------------------------------------

/// Binds one application model's identity and version to its estimator
/// contract. The application owns the model; this binding keeps its use
/// honest. One bump of one family must not rename identities under another.
pub struct ModelBinding {
    model_identity: [u8; 32],
}

/// The declared premises one derived claim depends on. An absent assumption
/// set is a refusal, never an implicit "no assumptions".
pub struct AssumptionSet {
    assumptions: Vec<Assumption>,
}

/// One declared premise: what is assumed, over which sources, at which Cuts.
pub struct Assumption {
    subject: AssumptionSubject,
}

/// How uncertain inputs relate — independence is a claim, never a default.
pub struct Dependence {
    posture: DependencePosture,
}

/// Evidence connecting model output to observed outcomes. Semantic model
/// calibration; module-scoped, sharing nothing with any physical-resource
/// calibration another owner declares.
pub struct Calibration {
    evidence_cut: Cut,
}

/// One declared lossy crossing: what operation narrowed the information and
/// exactly what it discarded. Undeclared loss is a `KnowledgeRefusal`.
pub struct InformationLossCrossing {
    discarded: DiscardedInformation,
}

/// An immutable derived claim bound to its complete honesty context: model
/// binding, exact accepted inputs and Cuts, assumptions, dependence,
/// calibration, and every information-loss crossing on its road. Conditioning
/// mints an immutable successor at a new Cut; nothing mutates a prior claim.
pub struct BoundClaim {
    model: ModelBinding,
    inputs_cut: Cut,
    assumptions: AssumptionSet,
    dependence: Dependence,
}

/// A typed description of what additional admitted evidence could close a
/// decision. Inert data consumed by the shared logic axis
/// `Decision::Defer(EvidenceRequirement)`; the acquisition policy later
/// chooses REQUEST or PEND, a source, a deadline, and authority. The
/// requirement itself performs nothing and grants nothing.
pub struct EvidenceRequirement {
    subject: EvidenceSubject,
}

/// Work-class affine budget for conditioning and binding operations.
/// Deliberately neither `Clone` nor `Copy`.
pub struct KnowledgeBudget {
    remaining: u64,
}

// ---------------------------------------------------------------------------
// Least authority
// ---------------------------------------------------------------------------

/// The static least-authority projection of one program: which event sources
/// it reads, which exact Cuts it requires, which frames and relations it
/// traverses, which ports it may request, which capabilities it requires,
/// which bounds it consumes, which protected fields may cross, and which
/// suspension and recovery postures exist.
///
/// Not a grant, and no proof of runtime behavior: Bvisor compares this
/// projection against installed authority and observed crossings.
pub struct RequirementsProjection {
    sources: SourceSet,
    capabilities: Vec<CapabilityRequirement>,
}

// ---------------------------------------------------------------------------
// Refusals
// ---------------------------------------------------------------------------

/// A transition could not be lawfully produced. Every variant names the
/// violated law, the typed owner, the offending value, and the repair
/// direction when it is realized.
pub enum DecisionRefusal {
    /// A declared input requirement was not satisfied at the frozen Cuts.
    UnsatisfiedInput(UnsatisfiedInput),
    /// An admitted bound was exhausted before the evaluation completed.
    BoundExhausted(BoundExhausted),
    /// The recursion witness was violated or could not close.
    RecursionWitnessViolated(RecursionWitnessViolated),
    /// The operation's declared posture forbids what the evaluation attempted.
    PostureViolated(PostureViolated),
}

/// Gate 1 refused an image. `LoweringMismatch` is the refusal proving the
/// Semantic Form versus Execution Form wall; `AgreementNotEstablished` is
/// missing required evidence and is deliberately a distinct variant from any
/// disagreement finding.
pub enum ImageRefusal {
    /// Bounded decode failed: size, depth, or structural bounds violated.
    DecodeBounded(DecodeBounded),
    /// The image's declared rosters do not close.
    StructurallyOpen(StructurallyOpen),
    /// Semantic Form validation refused the checked semantic commitment.
    SemanticFormViolated(SemanticFormViolated),
    /// A recursion or bound witness failed validation.
    RecursionOrBoundViolated(RecursionOrBoundViolated),
    /// The independent route established that execution disagrees with
    /// semantics — red evidence.
    LoweringMismatch(DisagreementEstablished),
    /// The independent agreement route could not run to a verdict — missing
    /// required evidence, not a defect finding.
    AgreementNotEstablished(AgreementNotEstablished),
    /// Execution Form validation refused the lowered representation.
    ExecutionFormViolated(ExecutionFormViolated),
    /// Effect, capability, source, or profile closure failed.
    ClosureIncomplete(ClosureIncomplete),
}

/// A knowledge binding or conditioning could not be lawfully produced.
pub enum KnowledgeRefusal {
    /// A claim referenced inputs at a Cut it cannot lawfully read.
    StaleCut(StaleCut),
    /// The assumption set does not close over the claim's road.
    AssumptionsOpen(AssumptionsOpen),
    /// A lossy crossing occurred without a declared
    /// `InformationLossCrossing`.
    UndeclaredInformationLoss(UndeclaredInformationLoss),
    /// Calibration evidence does not cover the model and estimator pair.
    CalibrationMismatch(CalibrationMismatch),
    /// The knowledge budget was exhausted.
    BudgetExhausted(BoundExhausted),
}

// ---------------------------------------------------------------------------
// Owner-local detail payloads
//
// Second-tier declarations closing this file's role graph. Each carries the
// violated law, the offending value, and the repair direction for its refusal
// variant, or the body of its first-tier owner. Fields are private; their
// exact composition is fixed by this owner's realization under these docs.
// ---------------------------------------------------------------------------

/// Body of one declared work formula: portable work as a function of the
/// affected input set.
pub struct WorkFormulaBody {
    terms: Vec<WorkTerm>,
}

/// One term of a declared work formula, naming the input dimension it scales
/// with.
pub struct WorkTerm {
    dimension_bytes: [u8; 16],
}

/// What one assumption is about: the sources, Cuts, and claim it premises.
pub struct AssumptionSubject {
    subject_bytes: Vec<u8>,
}

/// The declared relation among uncertain inputs; independence is one explicit
/// posture among others, never a default.
pub struct DependencePosture {
    posture_bytes: Vec<u8>,
}

/// Exactly what a lossy crossing discarded, stated in the crossing's own
/// vocabulary.
pub struct DiscardedInformation {
    description_bytes: Vec<u8>,
}

/// What evidence a requirement asks for: the question, the acceptable
/// sources, and the closure it would provide.
pub struct EvidenceSubject {
    subject_bytes: Vec<u8>,
}

/// Refusal body: a declared input requirement unsatisfied at the frozen Cuts.
pub struct UnsatisfiedInput {
    requirement_bytes: Vec<u8>,
}

/// Refusal body: which admitted bound was exhausted, at what consumed value.
pub struct BoundExhausted {
    bound_bytes: Vec<u8>,
}

/// Refusal body: which clause of the recursion witness failed to close or was
/// violated.
pub struct RecursionWitnessViolated {
    clause_bytes: Vec<u8>,
}

/// Refusal body: what the evaluation attempted that its declared posture
/// forbids.
pub struct PostureViolated {
    attempted_bytes: Vec<u8>,
}

/// Refusal body: which decode bound (size, depth, structure) the image bytes
/// violated.
pub struct DecodeBounded {
    bound_bytes: Vec<u8>,
}

/// Refusal body: which declared roster fails to close.
pub struct StructurallyOpen {
    roster_bytes: Vec<u8>,
}

/// Refusal body: which semantic-form law the checked commitment violated.
pub struct SemanticFormViolated {
    law_bytes: Vec<u8>,
}

/// Refusal body: which recursion or bound witness failed image validation.
pub struct RecursionOrBoundViolated {
    witness_bytes: Vec<u8>,
}

/// Refusal body: which execution-form law the lowered representation
/// violated.
pub struct ExecutionFormViolated {
    law_bytes: Vec<u8>,
}

/// Refusal body: which effect, capability, source, or profile closure failed.
pub struct ClosureIncomplete {
    closure_bytes: Vec<u8>,
}

/// Refusal body: the Cut a claim referenced and why it is unreadable for that
/// claim.
pub struct StaleCut {
    cut: Cut,
}

/// Refusal body: which premises are missing from the assumption set.
pub struct AssumptionsOpen {
    missing_bytes: Vec<u8>,
}

/// Refusal body: where information was narrowed without a declared crossing.
pub struct UndeclaredInformationLoss {
    site_bytes: Vec<u8>,
}

/// Refusal body: which model and estimator pair the calibration evidence
/// fails to cover.
pub struct CalibrationMismatch {
    pair_bytes: Vec<u8>,
}
