//! Program owner — role graph.
//!
//! Declarations only: this file states the nouns of executable meaning and the
//! co-seated Knowledge owner. Thin operation signatures live in `ops.rs`;
//! construction laws are enforced by private fields and owner-module
//! constructors when this owner is realized.
//!
//! Cross-owner names are written bare and resolve to their owning contracts
//! when the dependency probe seats real imports: `Cut`, `FederationCut`,
//! `SourceSet`, `SourceSetId`, `EventProposal`, `ReferenceFrameId`,
//! `AcceptedEvent` (event); `PortFamilyId`, `PortOperationId`,
//! `PortContractVersion` (port); `ContinuationByteLimit`, `DeadlinePolicy`
//! (runtime); `SchemaId`,
//! `FieldId`, `BoundClass`, `Truth`, `Decision` (core). `EventProposal` is
//! the event owner's noun — this owner carries it, never declares it. The
//! durable `EffectIntent` record is runtime-owned; this owner declares only
//! the inert `EffectProposal` that admission strengthens.
//! Where a field must agree with another field, neither field is public.
//!
//! Identity and commitment widths are never chosen here: every digest and
//! opaque identity closes with the identity/canon profile (candidate rows
//! recovered from the archived corpus; ratified at the canon packet). A
//! `/* closes with … */` interior is a representation deferred to its
//! profile; every correctness-bearing relationship is a carried field.
//!
//! Derives are semantic claims. Plain limits are `Copy`; budgets are affine and
//! deliberately derive nothing that could duplicate capacity. No type here
//! derives serialization: canonical bytes are the canon owner's law, and Rust
//! layout is not a wire format.

// ---------------------------------------------------------------------------
// Identity and commitments — role-distinct; widths per the identity profile
// ---------------------------------------------------------------------------

/// Identity of one program's semantic commitment.
///
/// Derived from the canonical bytes of the checked semantic form under the
/// identity owner's preimage law. Two programs with one `ProgramId` carry one
/// semantic commitment; a changed commitment is a different program, never a
/// new "version" of the same identity.
pub struct ProgramId {
    /* semantic-commitment digest; closes with the identity profile */
}

/// Identity of one packaged `ProgramImage` artifact (semantic commitment plus
/// execution commitment plus closure), distinct from `ProgramId`: one program
/// may lawfully have more than one image realization over time.
pub struct ProgramImageId {
    /* image digest; closes with the identity profile */
}

/// The canonical-byte commitment of one lowered execution form. Never
/// substitutable for `ProgramId`: semantic and execution commitments are
/// distinct commitments inside one artifact.
pub struct ExecutionCommitment {
    /* execution-form digest; closes with the identity profile */
}

/// The exact-byte commitment of one image artifact as received, before any
/// validation. Evidence of which bytes were decoded — never authority.
pub struct ImageBytesCommitment {
    /* exact-byte digest; closes with the identity profile */
}

/// The content digest binding one referenced image component under the
/// ImmutableBound or Hybrid packaging roads. An unresolvable or
/// digest-mismatched reference refuses the image.
pub struct ComponentDigest {
    /* content digest; closes with the identity profile */
}

/// Version identity of the image artifact grammar itself. Compatibility
/// claims per horizon; never substitutable for any content identity.
pub struct ImageFormatVersion {
    /* closes with the identity profile */
}

/// Version identity of the Execution Form grammar a lowering targets.
pub struct ExecutionFormVersion {
    /* closes with the identity profile */
}

/// Version identity of the semantic-kernel operation inventory an image was
/// checked against. Semantic, execution, image-bytes, and kernel
/// compatibility do not move together; each identity carries its own claim.
pub struct SemanticKernelVersion {
    /* closes with the identity profile */
}

/// Deterministic position of one operation inside one descriptor's closed
/// roster. Declared order, never discovery order.
pub struct OperationOrdinal {
    index: u32,
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

/// The closed roster of operation declarations inside one descriptor.
/// Bounded by construction; a descriptor with an open operation set does not
/// exist.
pub struct OperationDescriptors {
    operations: Vec<OperationDescriptor>,
}

/// One declared operation: its position, posture, typed input and output
/// roles, source and Cut requirements, refusal family, work formula, and
/// explanation contract.
pub struct OperationDescriptor {
    ordinal: OperationOrdinal,
    posture: OperationPosture,
    inputs: Vec<InputRequirement>,
    output: OutputDeclaration,
    sources: SourceSet,
    cut_requirements: Vec<ExactCutRequirement>,
    refusals: DeclaredRefusalFamily,
    work: SemanticWorkFormula,
    explanation: ExplanationContract,
}

/// One typed input role an operation consumes, as a schema commitment. The
/// value arrives frozen at exact Cuts; no ambient read exists.
pub struct InputRequirement {
    schema: SchemaId,
}

/// The typed output role an operation produces, as a schema commitment.
pub struct OutputDeclaration {
    schema: SchemaId,
}

/// The application-declared domain refusal family of one operation, bound as
/// a schema commitment. Program's own three refusal families are below; a
/// domain refusal is application vocabulary kept honest by this binding.
pub struct DeclaredRefusalFamily {
    schema: SchemaId,
}

/// The declared requirement that one source be read at an exact Cut.
pub struct ExactCutRequirement {
    source: SourceSetId,
}

/// The declared explanation relationships of one operation under the
/// progressive-explanation rail: the typed semantic signature is bound here;
/// the four readings derive from one evaluation and a shorter reading may
/// omit detail but never contradict the expansion.
pub struct ExplanationContract {
    signature: SchemaId,
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
/// charging points, boundary-request points, and continuation layout — the
/// form body's grammar closes with the execution-form profile.
///
/// "Candidate" is load-bearing: this form has not yet earned agreement and
/// grants nothing.
pub struct CandidateExecutionForm {
    lowered_from: ProgramId,
    commitment: ExecutionCommitment,
    form: ExecutionFormVersion,
}

/// The packaged artifact binding one semantic commitment and one execution
/// commitment with the operation table, schema closure, required port
/// profiles, bounds, entrypoints, and packaging. Not executable: only the
/// gate mints `ExecutableProgramImage`.
pub struct ProgramImage {
    id: ProgramImageId,
    format: ImageFormatVersion,
    semantic_commitment: ProgramId,
    execution_commitment: ExecutionCommitment,
    execution_form: ExecutionFormVersion,
    kernel: SemanticKernelVersion,
    operations: OperationDescriptors,
    schema_closure: SchemaClosure,
    port_profiles: RequiredPortProfiles,
    bounds: DeclaredBounds,
    entrypoints: Entrypoints,
    packaging: Packaging,
}

/// The closed roster of schema commitments an image's operations reference.
/// An image whose schema references do not close refuses at the gate.
pub struct SchemaClosure {
    schemas: Vec<SchemaId>,
}

/// The closed roster of port families and contract versions an image may
/// request. A requirement, never a grant.
pub struct RequiredPortProfiles {
    requirements: Vec<RequiredPortProfile>,
}

/// One required port profile: the family and the exact contract version the
/// image was checked against.
pub struct RequiredPortProfile {
    family: PortFamilyId,
    contract: PortContractVersion,
}

/// The closed roster of operations an image exposes for invocation.
pub struct Entrypoints {
    operations: Vec<OperationOrdinal>,
}

/// How an image's components are carried. Three lawful roads, all satisfying
/// the same dual-form closure and standalone-inspection requirement.
/// `SelfContained` is the selected paved-road default (owner-ruled;
/// asymmetric classification — a lawful override selects another road; the
/// depot row records the selection).
pub enum Packaging {
    /// Every component inline in the image bytes.
    SelfContained,
    /// Components referenced by content identity from an immutable store.
    ImmutableBound {
        bindings: Vec<ImageComponentBinding>,
    },
    /// A declared mix of inline and content-bound components.
    Hybrid {
        inline: Vec<ImageComponentRole>,
        bound: Vec<ImageComponentBinding>,
    },
}

/// One content-bound component record: role, exact content digest, and
/// length. An unresolvable or digest-mismatched reference refuses the image.
pub struct ImageComponentBinding {
    role: ImageComponentRole,
    digest: ComponentDigest,
    length_bytes: u64,
}

/// The closed roster of component roles inside one image.
pub enum ImageComponentRole {
    SemanticForm,
    ExecutionForm,
    OperationTable,
    SchemaClosure,
    PortProfiles,
    Bounds,
    Entrypoints,
}

// ---------------------------------------------------------------------------
// Gate 1: the image-strengthening ladder
//
// Affine typestate: each stage consumes the prior value and returns the
// stronger type or a typed refusal. Foreign bytes and locally built images
// meet the same ladder; there is no local shortcut.
// ---------------------------------------------------------------------------

/// Bounded foreign image bytes as received — the only input the decode stage
/// accepts. Bounded by `ImageByteLimit` before allocation.
pub struct ImageBytes {
    /* bounded foreign bytes; never interpreted before bounded decode */
}

/// An untrusted image after bounded decode and before any validation.
/// Decode proves shape, never truth: the decoded record carries every
/// declared roster the later stages judge — commitments, versions, the
/// operation table, schema closure, port profiles, bounds, entrypoints,
/// and packaging — and every one of those claims remains unvalidated until
/// its stage strengthens it. The bytes commitment is evidence of exactly
/// which received bytes produced this record.
pub struct DecodedProgramImage {
    bytes: ImageBytesCommitment,
    format: ImageFormatVersion,
    /// The decoded, still-untrusted image record — the material
    /// `validate_semantic_image` judges. Possession claims nothing.
    image: ProgramImage,
}

/// An image whose structure closed and whose Semantic Form and recursion and
/// bound witnesses passed validation. Not yet agreement-checked.
pub struct SemanticImage {
    image: ProgramImage,
}

/// A `SemanticImage` whose independent lowering agreement is established.
/// Private construction: only the agreement route mints the witness inside.
pub struct AgreementCheckedImage {
    image: SemanticImage,
    agreement: AgreementEstablished,
}

/// The only image PakVM accepts. Private construction: the sole road here is
/// the complete Gate 1 chain — agreement plus effect, capability, source,
/// and profile closure. Possession is proof the gate ran.
pub struct ExecutableProgramImage {
    image: AgreementCheckedImage,
}

/// The independent route established the Semantic-to-Execution relation.
/// Private construction: only the agreement route mints this, and the
/// agreement route shares no load-bearing lowering or verdict logic with the
/// production lowerer.
pub struct AgreementEstablished {
    semantic: ProgramId,
    execution_commitment: ExecutionCommitment,
}

/// An independently checked relation does not hold — red evidence naming the
/// disagreeing operation and the two readings. Never collapsed with
/// `AgreementNotEstablished`.
pub struct DisagreementEstablished {
    semantic: ProgramId,
    operation: OperationOrdinal,
    production_reading: ExecutionCommitment,
    independent_reading: ExecutionCommitment,
}

/// The required independent route could not run to a verdict. Missing
/// required evidence, not a defect finding. Never collapsed with
/// `DisagreementEstablished`.
pub struct AgreementNotEstablished {
    semantic: ProgramId,
    missing: AgreementEvidenceGap,
}

/// What kept the independent route from a verdict.
pub enum AgreementEvidenceGap {
    /// The independent route is not available for this form version.
    IndependentRouteUnavailable,
    /// A required decoded input was unavailable to the independent route.
    RequiredInputUnavailable,
    /// The route ran but could not complete its verdict within bounds.
    VerdictIncomplete,
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
    result: Option<ImmediateResult>,
    explanation: Explanation,
    work: ConsumedSemanticWork,
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

/// One proposed external effect, complete enough to later realize the exact
/// physical request: the port family, the named port operation and the exact
/// contract version it is declared under, the typed request role, the
/// canonical request-value commitment, and the typed request value itself.
/// Inert data — it performs nothing and commits nothing. Only the runtime's
/// REQUEST and PEND admission strengthens it into the durable, runtime-owned
/// `EffectIntent`, exactly as only event admission strengthens an
/// `EventProposal` into an `AcceptedEvent`. The durable intent references
/// this proposal by its commitment; the runtime's publication contract keeps
/// the proposal durably reachable, so the exact physical request is
/// realizable from the intent without reconstruction. Recovery posture and
/// duplication-recognition identity bind through `operation`'s declared
/// `RecoveryContract` — cited, never mirrored here.
pub struct EffectProposal {
    port_family: PortFamilyId,
    /// The named port operation this proposal requests — the referent of the
    /// `RecoveryContract` citation above.
    operation: PortOperationId,
    /// The exact port contract version `operation` is declared under.
    contract: PortContractVersion,
    request_role: SchemaId,
    /// Canonical commitment over the request value — what the durable
    /// `EffectIntent` references. Must agree with `request`; neither field
    /// is public.
    commitment: RequestValueCommitment,
    /// The typed request value, riding inert inside the `Transition`.
    request: EffectRequestValue,
}

/// The canonical request value of one effect proposal, bound at construction
/// to the proposal's declared `request_role` schema commitment. The canonical
/// value body closes with the codec profile.
pub struct EffectRequestValue {
    /* canonical request-value body; closes with the codec profile */
}

/// The canonical-byte commitment of one effect proposal's request value.
/// Evidence and reference identity — never authority, and never
/// substitutable for the port response or any outcome fact.
pub struct RequestValueCommitment {
    /* canonical request-value digest; closes with the identity profile */
}

/// One complete bounded batch of effect proposals built by the atomic-
/// planning recursion road. The batch is data; constructing it crosses
/// nothing. If the recursion refuses, no batch exists and nothing external
/// happened.
pub struct EffectBatch {
    proposals: EffectProposals,
}

/// The optional immediate result value of one evaluation, bound to its
/// schema commitment and bounded by `ResultValueLimit`. The canonical value
/// body closes with the codec profile.
pub struct ImmediateResult {
    schema: SchemaId,
}

/// The explanation of one evaluation. Binds the evidence the evaluation
/// actually consumed; the four progressive readings derive from this one
/// record, and an explanation citing evidence the evaluation did not use is
/// a falsifier, never a rendering choice. Reading bodies close with the
/// explanation profile.
pub struct Explanation {
    cited: SourceSet,
}

/// The consumed-work account of one evaluation: exact charges per declared
/// portable work dimension. Never CPU cycles, wall time, or scheduler
/// observations.
pub struct ConsumedSemanticWork {
    terms: Vec<ConsumedWorkTerm>,
}

/// One consumed-work entry: the register dimension and the exact count
/// charged in that dimension's declared unit.
pub struct ConsumedWorkTerm {
    dimension: WorkDimensionId,
    consumed: u64,
}

/// The frozen typed input bundle one evaluation reads: its source set and
/// the exact Cuts every input was frozen at. The application's typed input
/// values — view-owned Fix results and admitted observations — are bound by
/// construction to exactly these sources and Cuts; no ambient read exists.
/// (Draft spelling; replaces the earlier illustrative `Snapshot`.)
pub struct DecisionInputs {
    sources: SourceSet,
    at: FederationCut,
}

/// One operation invocation's request value, as a schema commitment.
pub struct OperationInput {
    schema: SchemaId,
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
/// recursion lawful. An operation whose witness does not close refuses at
/// checked construction; runtime metering remains active after static
/// admission as a second lock.
pub struct RecursionWitness {
    /// The explicit decreasing measure, drawn from the closed measure
    /// algebra — bounded naturals and lexicographic tuples of them under an
    /// admitted well-founded order, never an arbitrary callback.
    measure: DecreasingMeasure,
    /// The closed set of mutually recursive operations the measure covers.
    mutual_closure: MutualRecursionClosure,
    depth: RecursionDepthLimit,
    interleaved_effects: EffectProposalLimit,
    effect_order: DeclaredEffectOrder,
    capabilities: Vec<CapabilityRequirement>,
    work: SemanticWorkFormula,
    memory: MemoryByteLimit,
    output: OutputByteLimit,
    suspension_depth: SuspensionLimit,
    /// Runtime-owned bound on captured continuation bytes, carried here so
    /// the witness closes before any boundary is crossed.
    continuation_bytes: ContinuationByteLimit,
    /// Runtime-owned durable deadline commitment this recursion runs under.
    deadline: DeadlinePolicy,
    recovery: RecoveryClosure,
}

/// A typed value from the closed measure algebra; representation closes with
/// the measure profile.
pub struct DecreasingMeasure {
    /* bounded naturals and lexicographic tuples; closes with the measure
     * profile */
}

/// The closed set of operations one decreasing measure jointly covers.
pub struct MutualRecursionClosure {
    operations: Vec<OperationOrdinal>,
}

/// The declared order of boundary crossings inside one interleaved
/// recursion; representation closes with the execution-form profile
/// (boundary-point sequence).
pub struct DeclaredEffectOrder {
    /* closes with the execution-form profile */
}

/// The witness clause that every referenced port family's declared
/// `RecoveryContract` routes exist before the first irreversible crossing.
/// Postures are cited from the port contracts, never mirrored here.
pub struct RecoveryClosure {
    families: Vec<PortFamilyId>,
}

/// One clause of the recursion witness — the closed roster refusals name.
pub enum RecursionClause {
    DecreasingMeasure,
    MutualClosure,
    EffectCount,
    EffectOrder,
    Capabilities,
    Work,
    Memory,
    Output,
    SuspensionDepth,
    ContinuationBytes,
    Deadline,
    Recovery,
}

/// Declared, portable semantic work as a function of the affected input set —
/// never CPU cycles, wall time, or scheduler observations. The enforceable
/// form of the shallow-push design intent: a declared formula, explicit
/// bounds, and parity with reference recomputation. Composition law:
/// sequential composition sums; choice takes the maximum lawful branch
/// bound, never the sum.
pub struct SemanticWorkFormula {
    formula: WorkFormulaBody,
}

/// Body of one declared work formula: bounded terms over the shared portable
/// work-dimension register.
pub struct WorkFormulaBody {
    terms: Vec<WorkTerm>,
}

/// One term of a declared work formula: the register dimension it scales
/// with and the exact per-unit charge in that dimension's declared unit.
pub struct WorkTerm {
    dimension: WorkDimensionId,
    coefficient: u64,
}

/// Identity of one portable work dimension in the shared bound-dimension
/// register (a depot closed roster). Closes with the identity profile.
/// Backend instruction count is never a dimension.
pub struct WorkDimensionId {
    /* closes with the identity profile */
}

// ---------------------------------------------------------------------------
// Least authority
// ---------------------------------------------------------------------------

/// One statically declared authority requirement. A requirement is inert: it
/// names a relationship the invocation will need, and grants nothing. An
/// image serializes requirements, never live authority.
pub enum CapabilityRequirement {
    /// The right to attempt operations of one port family.
    PortAttempt { family: PortFamilyId },
    /// The right to read one declared event source.
    SourceRead { source: SourceSetId },
    /// The right for one protected field to cross into evaluation.
    ProtectedFieldCross { field: FieldId },
}

/// The static least-authority projection of one program: which event sources
/// it reads, which exact Cuts it requires, which frames and relations it
/// traverses, which ports it may request, which capabilities it requires,
/// which bound classes it consumes, which protected fields may cross, and
/// its suspension ceiling. Recovery postures are read from the referenced
/// port operations' declared `RecoveryContract`s — cited, never mirrored.
///
/// Not a grant, and no proof of runtime behavior: Bvisor compares this
/// projection against installed authority and observed crossings.
pub struct RequirementsProjection {
    sources: SourceSet,
    cut_requirements: Vec<ExactCutRequirement>,
    frames: Vec<ReferenceFrameId>,
    relations: Vec<TraversedRelation>,
    ports: Vec<RequiredPortProfile>,
    capabilities: Vec<CapabilityRequirement>,
    bound_classes: Vec<BoundClass>,
    protected_fields: Vec<FieldId>,
    suspensions: SuspensionLimit,
}

/// One relation a program declares it traverses, per the event owner's
/// relation contract. Closes with the identity profile.
pub struct TraversedRelation {
    /* relation identity per the event owner's relation contract */
}

// ---------------------------------------------------------------------------
// Bounds (classes per root law; numeric values live in the depot)
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

/// Memory-class copyable limit on bytes one evaluation may retain.
#[derive(Clone, Copy)]
pub struct MemoryByteLimit {
    limit_bytes: u64,
}

/// Output-class copyable limit on artifact bytes one evaluation may emit.
#[derive(Clone, Copy)]
pub struct OutputByteLimit {
    limit_bytes: u64,
}

/// Memory-class copyable limit on image bytes one bounded decode accepts.
#[derive(Clone, Copy)]
pub struct ImageByteLimit {
    limit_bytes: u64,
}

/// Work-class copyable limit on structural nesting one bounded decode
/// traverses.
#[derive(Clone, Copy)]
pub struct DecodeDepthLimit {
    limit: u32,
}

/// Memory-class copyable limit on components one bounded decode admits.
#[derive(Clone, Copy)]
pub struct ComponentCountLimit {
    limit: u32,
}

/// The complete declared bound roster of one descriptor or image, under the
/// seven closed classes of root law. Work is declared per operation through
/// its formula; the roster carries the remaining ceilings.
pub struct DeclaredBounds {
    memory: MemoryByteLimit,
    result: ResultValueLimit,
    output: OutputByteLimit,
    events: EventProposalLimit,
    effects: EffectProposalLimit,
    suspension: SuspensionLimit,
    recursion: RecursionDepthLimit,
}

// ---------------------------------------------------------------------------
// Profiles — the owner-declared configuration algebra. The depot selects
// exact rows inside these algebras; operations receive the selected row as
// an explicit argument, never through any ambient lookup.
// ---------------------------------------------------------------------------

/// The evaluation profile one decide invocation runs under: the selected
/// bound roster. The affine work budget is minted per invocation from the
/// profile's selected work value.
pub struct EvaluationProfile {
    bounds: DeclaredBounds,
}

/// The lowering profile: which Execution Form grammar and semantic-kernel
/// inventory a lowering targets. Both roads of Gate 1 read the same profile;
/// they share no load-bearing logic.
pub struct LoweringProfile {
    execution_form: ExecutionFormVersion,
    kernel: SemanticKernelVersion,
}

/// The bounded-decode profile: the decode bound algebra an image decode
/// enforces before allocation.
pub struct ImageDecodeProfile {
    bytes: ImageByteLimit,
    depth: DecodeDepthLimit,
    components: ComponentCountLimit,
}

// ---------------------------------------------------------------------------
// Knowledge: the co-seated honesty owner
// ---------------------------------------------------------------------------

/// Identity of one application model. The application owns the model; this
/// identity keeps its use honest. Closes with the identity profile.
pub struct SemanticModelId {
    /* closes with the identity profile */
}

/// Version of one application model. One family's bump must never rename
/// identities under another.
pub struct ModelVersion {
    /* closes with the identity profile */
}

/// Identity of one estimator contract an application model is used under.
pub struct EstimatorContractId {
    /* closes with the identity profile */
}

/// Binds one application model's identity and version to its estimator
/// contract.
pub struct ModelBinding {
    model: SemanticModelId,
    version: ModelVersion,
    estimator: EstimatorContractId,
}

/// One application statement kept honest by its schema commitment: the
/// content is application vocabulary; the binding is ThreadPak's. The
/// statement's canonical bytes are committed under the canon profile.
pub struct SchemaBoundStatement {
    schema: SchemaId,
    /* canonical statement commitment; closes with the canon profile */
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

/// What one assumption is about: the sources and Cut it premises, and the
/// premise itself as a schema-bound statement.
pub struct AssumptionSubject {
    sources: SourceSet,
    at: Cut,
    premise: SchemaBoundStatement,
}

/// How uncertain inputs relate — independence is a claim, never a default.
pub enum DependencePosture {
    /// Independence is explicitly claimed.
    DeclaredIndependent,
    /// A declared dependence relationship, stated schema-bound.
    DeclaredDependence(SchemaBoundStatement),
    /// The relation is unknown; composition must treat inputs conservatively
    /// and may never sharpen a claim on this posture.
    DependenceUnknown,
}

/// The dependence declaration one claim carries.
pub struct Dependence {
    posture: DependencePosture,
}

/// Evidence connecting one model-and-estimator pair's output to observed
/// outcomes. Semantic model calibration; module-scoped, sharing nothing with
/// any physical-resource calibration another owner declares.
pub struct Calibration {
    model: SemanticModelId,
    estimator: EstimatorContractId,
    observed: SourceSet,
    evidence_cut: Cut,
}

/// The closed roster of information-loss crossing kinds. There is no
/// implicit exact-to-estimate collapse and no default that silently widens
/// or sharpens a claim.
pub enum LossKind {
    ExactToInterval,
    ExactToDistribution,
    EstimateToEstimate,
}

/// One declared lossy crossing: its kind, the input and output claims, the
/// policy that selected it, exactly what it discarded, its reversibility
/// posture, its disclosure statement, and the evidence Cut. Undeclared loss
/// is a `KnowledgeRefusal`; loss never grants truth or authority to the
/// mechanism that performed it.
pub struct InformationLossCrossing {
    kind: LossKind,
    input_claim: SchemaId,
    output_claim: SchemaId,
    policy: SchemaBoundStatement,
    discarded: DiscardedInformation,
    reversibility: ReversibilityPosture,
    disclosure: SchemaBoundStatement,
    evidence_cut: Cut,
}

/// Exactly what a lossy crossing discarded, stated schema-bound in the
/// crossing's own vocabulary.
pub struct DiscardedInformation {
    description: SchemaBoundStatement,
}

/// Whether a lossy crossing can be undone from what was retained.
pub enum ReversibilityPosture {
    Reversible,
    /// Irreversible; the entropy posture rides the crossing's policy
    /// statement.
    Irreversible,
}

/// An immutable derived claim bound to its complete honesty context: model
/// binding, exact accepted inputs and Cuts, assumptions, dependence,
/// calibration, and every information-loss crossing on its road. Conditioning
/// mints an immutable successor at a new Cut; nothing mutates a prior claim.
pub struct BoundClaim {
    model: ModelBinding,
    sources: SourceSet,
    at: FederationCut,
    assumptions: AssumptionSet,
    dependence: Dependence,
    calibration: Calibration,
    losses: Vec<InformationLossCrossing>,
}

/// A typed description of what additional admitted evidence could close a
/// decision. Inert data consumed by the shared logic axis
/// `Decision::Defer(EvidenceRequirement)`; the acquisition policy later
/// chooses REQUEST or PEND, a source, a deadline, and authority. The
/// requirement itself performs nothing and grants nothing.
pub struct EvidenceRequirement {
    subject: EvidenceSubject,
}

/// What evidence a requirement asks for: the question, schema-bound, and the
/// acceptable sources that could answer it.
pub struct EvidenceSubject {
    question: SchemaBoundStatement,
    acceptable_sources: SourceSet,
}

/// Work-class affine budget for conditioning and binding operations.
/// Deliberately neither `Clone` nor `Copy`.
pub struct KnowledgeBudget {
    remaining: u64,
}

// ---------------------------------------------------------------------------
// Refusals
// ---------------------------------------------------------------------------

/// Checked construction refused a descriptor. Composes the same typed bodies
/// the gate uses; no fourth refusal vocabulary exists.
pub enum ConstructionRefusal {
    /// The declaration is structurally open, unbounded, or contradictory.
    Semantic(SemanticFormViolated),
    /// The recursion witness could not close.
    Recursion(RecursionWitnessViolated),
}

/// A transition could not be lawfully produced. Every variant names the
/// violated law, the typed owner, the offending value, and the repair
/// direction.
pub enum DecisionRefusal {
    /// A declared input requirement was not satisfied at the frozen Cuts.
    UnsatisfiedInput(UnsatisfiedInput),
    /// An admitted bound was exhausted before the evaluation completed.
    BoundExhausted(BoundExhausted),
    /// The recursion witness was violated during evaluation.
    RecursionWitnessViolated(RecursionWitnessViolated),
    /// The operation's declared posture forbids what the evaluation attempted.
    PostureViolated(PostureViolated),
}

/// Gate 1 refused an image. `LoweringMismatch` is the refusal proving the
/// Semantic Form versus Execution Form wall; `AgreementNotEstablished` is
/// missing required evidence and is deliberately a distinct variant from any
/// disagreement finding.
pub enum ImageRefusal {
    /// Bounded decode failed: a decode bound was violated.
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
// Refusal bodies — typed roles, no byte sacks. Refusal prose lives in the
// depot keyed by refusal identity and adds no variant and no condition.
// ---------------------------------------------------------------------------

/// A declared input requirement unsatisfied at the frozen Cuts: which
/// requirement, from which source.
pub struct UnsatisfiedInput {
    requirement: InputRequirement,
    source: SourceSetId,
}

/// Which admitted bound was exhausted: its class, its register dimension,
/// and the exact consumed count against the exact limit, in the dimension's
/// declared unit.
pub struct BoundExhausted {
    class: BoundClass,
    dimension: WorkDimensionId,
    consumed: u64,
    limit: u64,
}

/// Which clause of the recursion witness failed to close or was violated.
pub struct RecursionWitnessViolated {
    clause: RecursionClause,
}

/// What the evaluation attempted that its declared posture forbids.
pub struct PostureViolated {
    declared: OperationPosture,
    attempted: AttemptedViolation,
}

/// The closed roster of posture violations, each named by a hostile the
/// harness plants.
pub enum AttemptedViolation {
    /// A pure ASK evaluation attempted an effect crossing.
    EffectFromPureAsk,
    /// Event-publication intent was produced outside a DO posture.
    PublicationOutsideDo,
    /// A boundary crossing occurred that the operation never declared.
    UndeclaredBoundaryCrossing,
}

/// Which decode bound the image bytes violated, with the exact observation.
pub struct DecodeBounded {
    bound: DecodeBound,
    observed: u64,
    limit: u64,
}

/// The closed roster of decode bounds.
pub enum DecodeBound {
    SizeExceeded,
    DepthExceeded,
    ComponentCountExceeded,
}

/// Which declared roster fails to close.
pub struct StructurallyOpen {
    roster: OpenRoster,
}

/// The closed roster of image rosters that must close.
pub enum OpenRoster {
    Operations,
    Effects,
    Capabilities,
    Sources,
    Schemas,
    PortProfiles,
    Entrypoints,
}

/// Which semantic-form law the checked commitment violated, at which
/// operation.
pub struct SemanticFormViolated {
    operation: OperationOrdinal,
    clause: SemanticFormClause,
}

/// The closed roster of semantic-form violations — the checked-construction
/// triad.
pub enum SemanticFormClause {
    DeclarationOpen,
    DeclarationUnbounded,
    DeclarationContradictory,
}

/// Which recursion or bound witness failed image validation, at which
/// operation.
pub struct RecursionOrBoundViolated {
    operation: OperationOrdinal,
    clause: RecursionClause,
}

/// Which execution-form law the lowered representation violated, at which
/// operation.
pub struct ExecutionFormViolated {
    operation: OperationOrdinal,
    clause: ExecutionFormClause,
}

/// The closed roster of execution-form validation clauses.
pub enum ExecutionFormClause {
    TypeViolation,
    RegionViolation,
    ControlFlowViolation,
    SuspensionViolation,
}

/// Which closure domain failed.
pub struct ClosureIncomplete {
    domain: ClosureDomain,
}

/// The closed roster of closure domains the gate's final stage validates.
pub enum ClosureDomain {
    Effects,
    Capabilities,
    Sources,
    Profiles,
}

/// The Cut a claim referenced and the requirement it failed.
pub struct StaleCut {
    cut: Cut,
    requirement: ExactCutRequirement,
}

/// Which premises are missing from the assumption set.
pub struct AssumptionsOpen {
    missing: Vec<AssumptionSubject>,
}

/// Where information was narrowed without a declared crossing.
pub struct UndeclaredInformationLoss {
    kind: LossKind,
    site: OperationOrdinal,
}

/// Which model and estimator pair the calibration evidence fails to cover.
pub struct CalibrationMismatch {
    model: SemanticModelId,
    estimator: EstimatorContractId,
}
