//! Program owner — thin semantic operation signatures.
//!
//! Signature declarations only: each item states one lawful transformation —
//! its exact inputs, its result, its refusal family, and the bounds it
//! consumes. Bodies land with the construction cuts (A3 for this owner); a
//! trailing-semicolon item here is a contract, not compilable Rust, and no
//! manifest exists yet by design.
//!
//! Every operation receives the exact profile or budget it consumes as an
//! explicit argument (`depot/README.md`, "Rows are passed, never fetched").
//! No operation reads an ambient clock, store, environment, or global
//! profile. Pure operations take no `&mut` except the affine budgets they
//! charge.

// ---------------------------------------------------------------------------
// Authoring: descriptor → Program
// ---------------------------------------------------------------------------

/// Checked construction — the only road to a `Program`. Refuses a descriptor
/// that is structurally open, unbounded, or self-contradictory, or whose
/// recursion witness does not close. Total over its refusal family; performs
/// nothing.
pub fn construct_program(
    descriptor: ProgramDescriptor,
    recursion: RecursionWitness,
) -> Result<Program, ConstructionRefusal>;

/// The static least-authority projection of one program. Total: every
/// checked program projects. The projection is not a grant and proves
/// nothing about runtime behavior.
pub fn project_requirements(program: &Program) -> RequirementsProjection;

// ---------------------------------------------------------------------------
// Gate 1: lowering and the image-strengthening ladder
//
// One owner, two judgment roads: `lower` is the production road;
// `check_lowering_agreement` is the independent road. They share the public
// form declarations and the decoded immutable inputs, and never share
// lowering dispatch, reconstruction, or the verdict predicate.
// ---------------------------------------------------------------------------

/// Semantic Form validation inside the gate: strengthens a checked program
/// into the form that may be lowered.
pub fn validate_semantic_form(
    program: Program,
) -> Result<ValidatedSemanticProgram, ImageRefusal>;

/// The production lowerer: produces the candidate execution form under one
/// lowering profile. The candidate has not earned agreement and grants
/// nothing.
pub fn lower(
    profile: &LoweringProfile,
    program: &ValidatedSemanticProgram,
) -> Result<CandidateExecutionForm, ImageRefusal>;

/// The independent agreement route. Returns the three-verdict outcome —
/// established, disagreement (red evidence), or not-established (missing
/// evidence) — never a boolean and never a refusal: a verdict is an outcome,
/// not an error. Shares no load-bearing lowering or verdict logic with
/// `lower`.
pub fn check_lowering_agreement(
    profile: &LoweringProfile,
    program: &ValidatedSemanticProgram,
    candidate: &CandidateExecutionForm,
) -> AgreementOutcome;

/// Bounded decode: the only door from foreign or local image bytes into the
/// gate. Enforces the decode profile's byte, depth, and component bounds
/// before allocation; refuses malformed or noncanonical bytes.
pub fn decode_image(
    profile: &ImageDecodeProfile,
    bytes: ImageBytes,
) -> Result<DecodedProgramImage, ImageRefusal>;

/// Structural closure plus Semantic Form plus recursion-and-bound
/// validation, consuming the decoded image. One sealed transition of the
/// affine ladder; its refusal names the exact stage that failed.
pub fn validate_semantic_image(
    decoded: DecodedProgramImage,
) -> Result<SemanticImage, ImageRefusal>;

/// The independent lowering agreement applied to an image's two forms.
/// `LoweringMismatch` and `AgreementNotEstablished` are distinct refusals —
/// red evidence versus missing evidence — and a locally built image receives
/// no shortcut.
pub fn check_image_agreement(
    profile: &LoweringProfile,
    image: SemanticImage,
) -> Result<AgreementCheckedImage, ImageRefusal>;

/// Effect, capability, source, and profile closure — the gate's final stage.
/// The only constructor of `ExecutableProgramImage`; possession downstream
/// is proof the complete gate ran.
pub fn validate_image_closure(
    image: AgreementCheckedImage,
) -> Result<ExecutableProgramImage, ImageRefusal>;

// ---------------------------------------------------------------------------
// Evaluation: the paved transition
// ---------------------------------------------------------------------------

/// The paved pure bounded transition. Reads only the frozen typed inputs and
/// the invocation's request value; charges the affine budget; produces the
/// complete `Transition` — proposals, optional immediate result, explanation,
/// consumed work — or a typed refusal. Performs nothing, admits nothing.
pub fn decide(
    profile: &EvaluationProfile,
    inputs: &DecisionInputs,
    input: OperationInput,
    budget: &mut SemanticWorkBudget,
) -> Result<Transition, DecisionRefusal>;

// ---------------------------------------------------------------------------
// Knowledge: binding and conditioning
// ---------------------------------------------------------------------------

/// Binds one application-derived claim to its complete honesty context:
/// model binding, exact inputs and Cuts, assumptions, dependence,
/// calibration. Refuses on a stale Cut, an open assumption set, a
/// calibration gap, or budget exhaustion.
pub fn bind_claim(
    model: &ModelBinding,
    inputs: &DecisionInputs,
    assumptions: AssumptionSet,
    dependence: Dependence,
    calibration: &Calibration,
    budget: &mut KnowledgeBudget,
) -> Result<BoundClaim, KnowledgeRefusal>;

/// Conditioning: prior claim plus one admitted observation — an
/// `AcceptedEvent`, re-entered through ordinary event admission at a new
/// Cut — plus assumptions yields an immutable successor bound to that Cut.
/// Never a mutation of the prior claim, which stays valid at its own Cut.
pub fn condition(
    prior: &BoundClaim,
    observation: &AcceptedEvent,
    assumptions: AssumptionSet,
    budget: &mut KnowledgeBudget,
) -> Result<BoundClaim, KnowledgeRefusal>;

/// Declares one lossy crossing on a claim's road, minting the successor that
/// carries it. The only lawful way a claim narrows; an undeclared narrowing
/// is `KnowledgeRefusal::UndeclaredInformationLoss`. `#[must_use]`: the
/// crossing evidence is the point.
pub fn declare_loss(
    prior: &BoundClaim,
    crossing: InformationLossCrossing,
    budget: &mut KnowledgeBudget,
) -> Result<BoundClaim, KnowledgeRefusal>;
