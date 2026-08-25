//! # view — role-graph declarations
//!
//! The derived-result owner's nouns.
//! This file declares roles and the law each role carries; behavior lives in this owner's thin operations (`ops.rs`).
//! Every derived resident is rebuildable from accepted history at exact Cuts; the read-side authority roles declared here (`ReadGrant`, `ProtectedResolutionGrant`) are declaration seats only — no view operation mints, advances, or widens authority (see `README.md`, "The wall").
//!
//! Cross-home types are referenced as bare names in field position, owners noted here — `Truth` (core); `AcceptedEvent`, `Cut`, `FederationCut`, `SourceSetId`, `ReferenceFrameId`, `FrameVersion`, `ExactHistoryRead` (event) — never as `crate::` paths; the dependency probe seats the real imports.
//! The runtime owner keys a subscription's durable checkpoint by wrapping this owner's `SubscriptionId` in its role-branded `CheckpointSubject`; no runtime type is referenced here.
//!
//! Profiles declared near the end of this file are the owner's configuration algebra: operations receive them as explicit arguments, selected values live in `depot/view.md`, and nothing here is fetched ambiently (`depot/README.md`, "Rows are passed, never fetched").
//!
//! Declarations only.
//! Fields appear where product law pins them; interiors marked as seams close with their named pass (identity profile, canon profile, guard pass, or an owner ruling), never silently.

use core::num::{NonZeroU32, NonZeroU64};
use core::time::Duration;

// ---------------------------------------------------------------------------
// Identities
// ---------------------------------------------------------------------------

/// Names exactly which population a mask, column, or materialization speaks about.
///
/// Equal cardinality is never equality of domains; two `RowDomainId`s are comparable only by identity, never by size or by byte-shape.
pub struct RowDomainId {
    /* closes with the identity profile */
}

/// Private-construction proof that two row domains are one population.
///
/// Minted only by the equality-proving operation; possession is the only lawful road past `SelectionRefusal::RowDomainEqualityUnproven`.
/// Equal cardinality mints nothing.
pub struct RowDomainEqualityWitness {
    /* the two proven-equal domain identities and the proving evidence; construction closes with the guard pass */
}

/// The named derived role and contract — which question a materialization answers.
///
/// One of four never-conflated identities (role, generation, block, occurrence).
pub struct MaterializationId {
    /* closes with the identity profile */
}

/// One realization generation of a materialization role.
///
/// A new generation is a new physical realization claim; it never renames the role.
pub struct MaterializationGenerationId {
    /* closes with the identity profile */
}

/// One bounded physical component under a materialization generation.
pub struct DataBlockId {
    /* closes with the identity profile */
}

/// One exact stored or in-memory realization of a DataBlock.
pub struct OccurrenceId {
    /* closes with the identity profile */
}

/// Identity of one `View` — the durable semantic definition of one maintained derived result.
///
/// Definition and state are separate roles (`View` / `ViewState`; owner ruling 2026-08-24).
pub struct ViewId {
    /* closes with the identity profile */
}

/// Version of one `View`'s definition semantics.
///
/// A changed definition is a new version; it never silently rebinds existing states.
pub struct ViewVersion {
    /* closes with the identity profile */
}

/// One rebuild generation of one maintained `ViewState`.
///
/// Discarding and rebuilding a stale or corrupt state mints a new generation; the `View` definition it maintains is unchanged.
pub struct ViewGeneration {
    /* closes with the identity profile */
}

/// One rebuild generation of a temporal monitor's persisted state.
pub struct MonitorGeneration {
    /* closes with the identity profile */
}

/// Identity of one temporal claim.
///
/// A settled fate binds to exactly one (`TemporalClaimId`, `TemporalClaimVersion`) pair: changing the claim, its version, its horizon, or its source set creates a NEW claim and never reopens an old fate.
pub struct TemporalClaimId {
    /* closes with the identity profile */
}

/// Version of a temporal claim's semantics.
///
/// Versions separate claims; they do not amend settled evaluations.
pub struct TemporalClaimVersion {
    /* closes with the identity profile */
}

/// Identity of one subscription relationship.
///
/// Declaration seat only: the durable checkpoint keyed by this identity is runtime-owned authority, and nothing in this owner may advance it.
pub struct SubscriptionId {
    /* closes with the identity profile */
}

// ---------------------------------------------------------------------------
// Derived answers
// ---------------------------------------------------------------------------

/// A demand-driven question against accepted history at exact Cuts.
///
/// A query names its source set, its frame expectations, its Cut requirements, and its bounds; every read names what "now" means.
pub struct Query {
    sources: SourceSetId,
    frame: ReferenceFrameId,
    frame_version: FrameVersion,
    /* question body (application-typed), Cut requirements, and the bound set; the body's exact shape closes beside Fix's public shape (Escalation 1 in README.md) */
}

/// Which road produced a derived answer.
///
/// Parity law: at the same claim, source set, exact Cuts, frame, relation versions, configuration, and profile, the two roads must agree — and on disagreement the maintained road loses.
/// Provenance is honesty about the road taken, never permission to skip parity.
pub enum DerivationRoad {
    /// Recomputed from accepted history by the reference road.
    Reference,
    /// Served from a push-maintained result.
    Maintained,
}

/// Declared portable work consumed by one advance or resolve: a function of the affected input set, never CPU cycles, wall time, or scheduler observations (rail 11).
pub struct ConsumedDerivationWork {
    /* per-dimension accounting; the dimension roster closes with the work-profile rows in depot/view.md */
}

/// Progressive-explanation bindings of one evaluation (rail 14): concise description, typed signature, structured explanation, complete definitional expansion — four readings of one evaluation, never four evaluations.
pub struct DerivationExplanation {
    /* explanation bindings; shape closes with the explanation pass */
}

/// A derived answer: what can be concluded from accepted history at exact Cuts — never what is authoritatively stored.
///
/// Shape, completeness, and freshness are orthogonal axes and never collapse into one another or into the value.
///
/// The exact public field surface is a recorded open seam (Escalation 1 in `README.md`); the axes and relationships below are law.
pub struct Fix<T> {
    value: T,
    sources: SourceSetId,
    at: FederationCut,
    completeness: Completeness,
    freshness: Freshness,
    road: DerivationRoad,
    work: ConsumedDerivationWork,
    explanation: DerivationExplanation,
}

/// The named unobserved or unclosed participants of one claim's source set.
///
/// Incompleteness names what is missing — never a bare flag.
pub struct UnobservedSources {
    /* named missing participants; representation closes with the identity profile */
}

/// This owner's completeness roster (evicted from any universal enum: each axis family keeps an owner-specific roster).
///
/// Incompleteness names what is missing; it is never silently dropped, and "nothing arrived yet" never becomes "nothing exists".
pub enum Completeness {
    /// The named source set was completely observed through the bound Cuts.
    Complete,
    /// One or more named sources were not completely observed.
    ///
    /// Closure-requiring conclusions (absence, negation, exhaustive search, final order, top-k) are unavailable in this state.
    Incomplete {
        missing: UnobservedSources,
    },
}

/// This owner's freshness roster: how the answer's Cuts relate to what was requested.
///
/// Freshness is evidence about staleness, never permission to substitute a newer or older result — an exact read that cannot be served at its requested Cut refuses; it never silently serves another Cut.
pub enum Freshness {
    /// The bound Cuts are exactly the requested Cuts.
    AtRequested,
    /// The bound Cuts lag the requested posture; the lag is named evidence.
    BehindRequested {
        /* the requested-versus-bound Cut relationship; representation closes with the guard pass */
    },
}

/// The durable semantic definition of one maintained derived result — the claim: sources, frame, advance law, resolve law, and parity contract.
///
/// Validated once; a definition never advances.
/// The plain database noun is kept deliberately: `View` is the declared derivation, `ViewState` is one maintained state of it, and `MaterializationGeneration` is one physical realization (owner ruling 2026-08-24).
/// `Projection` stays unminted — the lowercase word remains the root's generated-artifact category and the relational operator sense.
pub struct View {
    id: ViewId,
    version: ViewVersion,
    /* the definition body — sources, frame, advance law, resolve law, parity contract; exact roster closes with the guard pass */
}

/// One maintained derived state of one `View` at one `AppliedCut` — the push lane's resident.
///
/// Rebuildable from accepted history by the reference road at the same definition, source set, Cuts, frame, and profile; parity with that road is required, and on disagreement the maintained state loses.
/// The binding to its `View` is load-bearing: advance refuses a state that does not belong to the supplied definition (`AdvanceRefusal::StateClaimMismatch`).
pub struct ViewState {
    view: ViewId,
    view_version: ViewVersion,
    applied: AppliedCut,
    generation: ViewGeneration,
    /* maintained derived content; representation closes with the guard pass */
}

/// The result of one push-lane advance: the next maintained state and the declared work it consumed.
///
/// Producing this value publishes nothing and advances no checkpoint.
#[must_use]
pub struct ViewAdvance {
    next: ViewState,
    work: ConsumedDerivationWork,
}

/// The semantic relationship between one consumer and one `View`, with a bounded retained window.
///
/// A subscription binds the definition, never tonight's `ViewState` (owner ruling 2026-08-24).
/// A subscription's durable skip authority is its runtime-owned checkpoint; a delivered update, a wake, and a retained window are never progress.
pub struct Subscription {
    /// The identity the runtime owner keys this subscription's durable checkpoint by (`CheckpointSubject::Subscription`, runtime-owned).
    ///
    /// Nothing in this owner advances that checkpoint.
    id: SubscriptionId,
    /// The subscribed definition — id and version, never a state.
    view: ViewId,
    view_version: ViewVersion,
    window: SubscriptionWindowLimit,
}

/// Continuation of one traversal or query context.
///
/// A cursor continues work; it is never skip authority, never a checkpoint, and never durable recovery state.
/// It binds the exact operation it continues — changing any bound fact is an incompatible continuation, never a silent rebase.
pub struct Cursor {
    at: FederationCut,
    /* continuation bindings: operation identity, source lineage and generations, selectors and filters, declared order and tie-breaks, direction, page and work bounds; exact roster and encoding close with the canon profile (recovered thirteen-fact binding set is banked in depot/view.md) */
}

// ---------------------------------------------------------------------------
// Temporal claims
// ---------------------------------------------------------------------------

/// One bounded temporal claim over accepted history: identity + version + source set + frame + horizon + claim body.
///
/// Which claim families exist first is a recorded open seam (Escalation 3 in `README.md`).
pub struct TemporalClaim {
    id: TemporalClaimId,
    version: TemporalClaimVersion,
    sources: SourceSetId,
    frame: ReferenceFrameId,
    frame_version: FrameVersion,
    horizon: TemporalHorizon,
    /* claim body (the temporal proposition itself); closes with the temporal starter family (Escalation 3) */
}

/// Temporal fate — orthogonal to K3 knowledge (`Truth`) and to horizon; the three never merge into one enum.
///
/// Fate latches: `Open → Satisfied` and `Open → Violated`, never back, for the same exact claim.
pub enum TemporalFate {
    /// Lawful future extensions of history can still decide either way.
    Open,
    /// No lawful future extension can unsatisfy the claim.
    Satisfied,
    /// No lawful future extension can repair the claim.
    Violated,
}

/// A finite push-lane monitor's persisted state: derived and rebuildable, never temporal truth.
///
/// Binds the exact claim, its source set, the exact `AppliedCut` incorporated, and a monitor generation; when stale or corrupt it is recomputed from accepted history, never trusted.
pub struct TemporalMonitorState {
    claim: TemporalClaimId,
    claim_version: TemporalClaimVersion,
    sources: SourceSetId,
    applied: AppliedCut,
    generation: MonitorGeneration,
    /* bounded state body; representation closes with the monitor compilation contract */
}

/// One monitor advance outcome: the next state and, at most once per exact claim, a newly settled fate.
///
/// Knowledge (`Truth`) rides beside fate and is evaluated per claim law — `Truth::Pending` is an evidence statement, not a fate.
#[must_use]
pub struct MonitorAdvance {
    next: TemporalMonitorState,
    settled: Option<TemporalFate>,
    knowledge: Truth,
    work: ConsumedDerivationWork,
}

// ---------------------------------------------------------------------------
// Selection and release
// ---------------------------------------------------------------------------

/// Exact semantic membership over one row domain at one source Cut.
///
/// The physical representation may vary; membership meaning may not.
/// Masks compose only across proven-equal row domains at the same Cut; composition over unproven equality refuses (`SelectionRefusal::RowDomainEqualityUnproven`).
/// A mask never flattens `Truth::Pending` into `False`: fail-closed is not permission to report the stronger fact.
pub struct SelectionMask {
    row_domain: RowDomainId,
    at: Cut,
    representation: SelectionRepresentation,
    /* logical length (padding bits never enter membership or iteration — width per the canon profile; the recovered sketch and the count-width law disagree, preserved in depot/view.md), deterministic iteration order, and the private membership representation */
}

/// A set of candidate members proposed by a qualified approximate mechanism.
///
/// Candidates never establish truth, absence, order, completeness, or authority; the only lawful consumers are exact verification against accepted history or an honest incomplete result.
/// No conversion to `SelectionMask` or to any result type exists.
pub struct ApproximateCandidateSet {
    row_domain: RowDomainId,
    /* candidate roster and the proposing mechanism's profile binding */
}

/// Read authority over selected information.
///
/// Declared here; minted only by the host's authority installation, never by any view operation, and never widened by any result derived under it.
pub struct ReadGrant {
    /* scope, attenuation, generation, expiry bindings; close with the guard pass */
}

/// Authority to request physical resolution of protected payload extents.
///
/// The physical crossing is a typed port operation; view code never receives raw key authority.
pub struct ProtectedResolutionGrant {
    /* key-scope, extent-scope, generation bindings; close with the guard pass */
}

/// The result of one protected resolution.
///
/// Role named here; the exact result family (what a resolution may report besides the resolved extent) is owner-derived work that closes with its construction cut (README Escalations).
pub struct ProtectedResolution {
    /* resolved extents under the applicable grant; family closes with its cut */
}

// ---------------------------------------------------------------------------
// Parity
// ---------------------------------------------------------------------------

/// Evidence that the maintained road and the reference road agreed at one exact claim, source set, Cuts, frame, and profile.
///
/// Evidence about one comparison — never a standing certificate.
pub struct ParityWitness {
    /* the compared claim binding and both roads' result commitments */
}

/// Evidence that the two roads diverged.
///
/// The maintained result loses: it is discarded and rebuilt from accepted history.
/// Divergence evidence is never silently dropped.
pub struct ParityDivergence {
    /* the compared claim binding and the differing result commitments */
}

/// The outcome of one parity judgment.
///
/// Judging is an operation; an incomparable pairing (different claim, sources, Cut, frame, or profile) refuses as `QueryRefusal::ParityIncomparable` instead of producing this.
#[must_use]
pub enum ParityVerdict {
    Held(ParityWitness),
    Diverged(ParityDivergence),
}

// ---------------------------------------------------------------------------
// Materializations
// ---------------------------------------------------------------------------

/// Which authoritative source Cuts a derived generation incorporated.
///
/// Never a storage snapshot identifier: new physical bytes do not imply a newer `AppliedCut`, and a newer `AppliedCut` requires proof that newer source Cuts were actually incorporated.
/// `CommitPoint`, `AppliedCut`, and checkpoint remain three different facts.
pub struct AppliedCut {
    sources: SourceSetId,
    incorporated: FederationCut,
}

/// One realization generation of a materialization role: the binding among role, generation, source set, and incorporated Cuts.
///
/// Lifecycle stages (derive, validate, bind, publish, activate, select, retire, reclaim) are separate operations; the standing typestate roster for those stages is a realization seam under this contract.
pub struct MaterializationGeneration {
    role: MaterializationId,
    generation: MaterializationGenerationId,
    applied: AppliedCut,
    /* derivation-profile binding and incorporation evidence; the evidence shape closes with the qualification pass */
}

/// One bounded physical component under a generation: derived acceleration, never authority.
///
/// Describing itself proves neither source existence nor derivation correctness; layout and device hints are untrusted claims.
/// A corrupt block is discarded and rebuilt — never reported as absence.
pub struct DataBlock {
    generation: MaterializationGenerationId,
    block: DataBlockId,
    row_domain: RowDomainId,
    /* column/extent descriptor bindings (grouping lawful, omission not) and the descriptor's own decode/allocation/component/row/result bounds — a reader refuses before allocating; bound values are MaterializationProfile rows in depot/view.md */
}

// ---------------------------------------------------------------------------
// Bounds owned here (numeric values and paved profiles live in the depot)
// ---------------------------------------------------------------------------

/// Result-class limit: maximum rows one query may return.
///
/// A row limit binds discovery before decode, join, sort, or materialization — never read-everything-then-return-the-first-N — and a full page proves no completeness.
pub struct QueryRowLimit(NonZeroU32);

/// Work-class affine budget for one resolve invocation: charging consumes it and returns a smaller successor; no widening operation exists anywhere.
///
/// Deliberately neither `Clone` nor `Copy` — duplicating it would fabricate capacity.
#[must_use]
pub struct QueryWorkBudget(NonZeroU64);

/// Work-class limit: maximum navigation depth one resolve may traverse.
pub struct NavigationDepth(NonZeroU32);

/// Work-class limit: maximum relation fan-out one traversal step may follow.
pub struct RelationFanOutLimit(NonZeroU32);

/// Result-class limit: maximum selected-membership cardinality one selection may report.
pub struct SelectionCardinalityLimit(NonZeroU32);

/// Memory-class limit: maximum bytes one materialization pass may retain.
pub struct MaterializationByteLimit(NonZeroU64);

/// The dimension a temporal horizon is measured in.
///
/// The two dimensions never substitute or convert.
pub enum HorizonDimension {
    /// A count of admitted events over the claim's source set.
    AdmittedEventCount(NonZeroU64),
    /// An admitted span, carried as the standard `core::time::Duration` (the generic unsigned span; owner mint 2026-08-24 — std batteries, no custom span type).
    AdmittedSpan(Duration),
}

/// Time-class bound: the explicit horizon a bounded temporal claim carries.
///
/// A horizon is a bound riding beside K3 and fate — never a truth value.
pub struct TemporalHorizon {
    dimension: HorizonDimension,
}

/// Memory-class limit: maximum retained window one subscription may hold.
pub struct SubscriptionWindowLimit(NonZeroU32);

// ---------------------------------------------------------------------------
// Profiles — this owner's configuration algebra. Selected values are
// depot/view.md rows; operations receive the exact profile as an argument.
// ---------------------------------------------------------------------------

/// The selected bounds and work posture of the push lane's advance.
pub struct ViewAdvanceProfile {
    /* declared per-event work formula binding and advance bounds; rows in depot/view.md */
}

/// The selected bounds of one pull-lane resolve: row ceiling, traversal depth, fan-out, and the work-budget denomination budgets are minted from.
pub struct ViewResolveProfile {
    rows: QueryRowLimit,
    depth: NavigationDepth,
    fan_out: RelationFanOutLimit,
    /* work-budget denomination; rows in depot/view.md */
}

/// The selected bounds of one temporal monitor: state-size ceiling and advance work posture.
///
/// Fast-start posture is a recorded owner escalation, not a profile row.
pub struct MonitorProfile {
    /* monitor state and work bounds; rows in depot/view.md */
}

/// The selected bounds and mechanism postures of one materialization role: byte ceilings and the descriptor's decode/allocation/component/row/result bound values.
pub struct MaterializationProfile {
    bytes: MaterializationByteLimit,
    /* descriptor bound values and layout-mechanism selection; rows in depot/view.md */
}

/// Qualified physical representations one semantic mask may take (recovered mechanism roster; qualification rows in depot/view.md).
///
/// Conversion among qualified representations is a mask operation; membership meaning never varies by representation, and physical padding never enters membership or iteration.
pub enum SelectionRepresentation {
    DenseBitset,
    SparseIndices,
    Runs,
    InlineWord,
}

// ---------------------------------------------------------------------------
// Refusals (role-specific; never one mega-error). Every refusal carries the
// violated law, the typed owner, the offending value's role, and the repair
// direction; the payload bodies close with the guard pass.
// ---------------------------------------------------------------------------

/// Refusals of the pull lane.
pub enum QueryRefusal {
    /// An admitted bound was exhausted before the resolve completed.
    BoundExceeded {
        /* which Work or Result bound, at what consumed value */
    },
    /// A closure-requiring conclusion (absence, negation, exhaustive search, final order, top-k) was demanded over an unclosed source set.
    ClosureRequiredButUnavailable {
        /* the demanded conclusion and the unclosed sources */
    },
    /// The query's frame expectations do not match the bound history.
    FrameExpectationViolated {
        /* expected versus bound frame identity and version */
    },
    /// A parity judgment was requested over an incomparable pairing — different claim, source set, Cut, frame, or profile.
    ParityIncomparable {
        /* the differing binding */
    },
}

/// Refusals of the push lane's advance operation.
pub enum AdvanceRefusal {
    /// The supplied `ViewState` does not belong to the supplied `View` — its id or version does not match the definition.
    ///
    /// The distinctness refusal that exists only because definition and state are separate roles (owner ruling 2026-08-24).
    StateClaimMismatch {
        /* the state's view binding versus the supplied definition's id and version */
    },
    /// The delta's events are outside the claim's source set.
    DeltaSourceMismatch {
        /* the offending source */
    },
    /// The delta does not abut the state's `AppliedCut` — advancing over a gap would silently skip admitted history.
    DeltaNotContiguous {
        /* the state's AppliedCut versus the delta's boundary */
    },
    /// The advance exceeded its declared per-event work or an admitted bound.
    BoundExceeded {
        /* which bound, at what consumed value */
    },
}

/// Refusals of temporal-monitor advance and evaluation.
pub enum MonitorRefusal {
    /// The monitor state does not belong to the supplied claim and version.
    StateClaimMismatch {
        /* the state's claim binding versus the supplied claim */
    },
    /// Settlement was demanded where the horizon cannot close over the claim's source set; the monitor reports incompleteness instead of settling.
    ClosureUnavailable {
        /* the unclosed sources */
    },
    /// An admitted monitor bound was exhausted.
    BoundExceeded {
        /* which bound, at what consumed value */
    },
}

/// Refusals of selection and mask composition.
pub enum SelectionRefusal {
    /// Two masks or columns were composed across row domains whose equality is unproven — including equal-cardinality impostors.
    ///
    /// Settled name; fail-closed here never reports the stronger fact.
    RowDomainEqualityUnproven,
    /// Two masks were composed at different source Cuts.
    CutMismatch {
        /* the two Cut bindings */
    },
    /// A conversion targeted a representation not qualified for this domain's profile.
    RepresentationUnqualified {
        /* the requested representation */
    },
    /// The selection's reported cardinality exceeded its Result-class limit.
    CardinalityExceeded {
        /* the limit and the demanded cardinality */
    },
}

/// Refusals of materialization lifecycle operations.
pub enum MaterializationRefusal {
    /// A generation claims incorporation its evidence cannot prove — new physical bytes masquerading as a newer `AppliedCut`.
    AppliedCutUnproven {
        /* the claimed AppliedCut and the missing evidence */
    },
    /// A stored occurrence failed integrity; it is discarded and rebuilt, never reported as absence and never served as history.
    OccurrenceCorrupt {
        /* the failing occurrence identity */
    },
    /// A lifecycle stage was invoked out of order (publish before validate, activate before publish, reclaim with live readers).
    LifecycleOrderViolated {
        /* the attempted stage and the generation's actual stage */
    },
    /// The operation targeted a retired or superseded generation.
    StaleGeneration {
        /* the targeted generation */
    },
    /// A materialization bound was exhausted.
    BoundExceeded {
        /* which bound, at what consumed value */
    },
}

/// Refusals of the information-release chain.
pub enum ReleaseRefusal {
    /// No applicable grant is installed for the requested extent.
    GrantAbsent,
    /// The request exceeds the installed grant's scope; a derived result never widens the grant that produced it.
    GrantScopeExceeded {
        /* the requested extent versus the grant's scope */
    },
    /// The grant's generation is no longer current.
    GrantGenerationStale,
    /// Resolution was requested ahead of the release chain's order — authorization and exact selection come first; decrypt last, decrypt least.
    ReleaseOrderViolated {
        /* the attempted step and the chain's required predecessor */
    },
}

// ---------------------------------------------------------------------------
// Declared-edge witnesses
// ---------------------------------------------------------------------------

/// The push lane's only lawful input pair: prior derived state plus one bounded admitted delta.
///
/// Declared as a type-level statement that advance consumes accepted events and never foreign or derived substitutes.
pub struct AdmittedDelta<'history> {
    events: &'history [AcceptedEvent],
    /* delta bounds and the contiguity binding to the prior AppliedCut; close with the advance contract */
}
