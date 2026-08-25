//! # view — thin operation signatures
//!
//! Semantic signatures only: exact inputs, outputs, refusals, and bounds.
//! Each operation is authored as a Rust function-pointer type alias (`…Fn`) preserving the signature shape the operation must have — no bodies exist here, nothing executes, and nothing claims implementation support.
//! Foreign owner names stand unresolved in this fragment, so this file alone claims neither compilation nor resolved dependency edges; that evidence is the generated contract probe's to produce.
//! Realization lands with construction cut A2, where each operation's implementation is pinned to its declared signature by a Macroonz-generated conformance assertion (`const _: ResolveFn<…> = resolve;` shape); exact Rust ergonomics may still be machined at the guard pass without changing any contract stated here.
//!
//! Every operation receives its profile and bounds as explicit typed arguments — no operation reads a depot row, a clock, an environment, or any ambient state (`depot/README.md`, "Rows are passed, never fetched").
//! Affine budgets pass by value and are consumed; plain limits ride inside profiles.
//!
//! `ExactHistoryRead` is the event owner's exact-read surface over accepted history at exact Cuts, declared with the event storage contract; this owner consumes it and never bypasses it.

// ---------------------------------------------------------------------------
// Pull lane
// ---------------------------------------------------------------------------

/// `resolve` — resolve one demand-driven question against accepted history at exact Cuts.
///
/// Inputs: the resolve profile (row/depth/fan-out selections), the typed query, the event owner's exact-read surface, the exact Cuts that answer "what does now mean", and one affine work budget (consumed; never widened).
/// Output: a `Fix` carrying value, sources, Cuts, completeness, freshness, road, consumed work, and explanation.
/// Refusals: `QueryRefusal` — bound exhaustion, closure demanded over an unclosed source set, frame expectation violated.
/// The row limit binds discovery before decode, join, sort, or materialization; a full page proves no completeness, and an empty page proves no absence beyond the frozen Cut.
pub type ResolveFn<T> = fn(
    profile: &ViewResolveProfile,
    query: &Query,
    history: &ExactHistoryRead,
    at: &FederationCut,
    budget: QueryWorkBudget,
) -> Result<Fix<T>, QueryRefusal>;

/// `resolve_continue` — continue one bounded traversal from an exact continuation.
///
/// The cursor binds the operation it continues; a changed binding refuses as an incompatible continuation — never a silent rebase onto newer history.
/// A cursor is never skip authority and never durable recovery state.
pub type ResolveContinueFn<T> = fn(
    profile: &ViewResolveProfile,
    cursor: Cursor,
    history: &ExactHistoryRead,
    budget: QueryWorkBudget,
) -> Result<(Fix<T>, Option<Cursor>), QueryRefusal>;

// ---------------------------------------------------------------------------
// Push lane
// ---------------------------------------------------------------------------

/// `advance` — advance one maintained state: the `View` definition, its prior `ViewState`, and one bounded admitted delta yield the next state.
///
/// Shallow, bounded, recursion-free; work is declared per event by the advance profile's formula and accounted in the output.
/// The definition and the state arrive as separate arguments — a state that does not belong to the supplied definition refuses (`AdvanceRefusal::StateClaimMismatch`).
/// Further refusals: a delta outside the definition's sources, a delta that does not abut the state's `AppliedCut`, an exhausted bound.
/// Producing a `ViewAdvance` publishes nothing and advances no checkpoint.
pub type AdvanceFn = fn(
    profile: &ViewAdvanceProfile,
    view: &View,
    prior: ViewState,
    delta: &AdmittedDelta<'_>,
) -> Result<ViewAdvance, AdvanceRefusal>;

/// `advance_monitor` — advance one finite temporal monitor by one bounded admitted delta.
///
/// At most one newly settled fate per exact claim, latching only forward; knowledge (`Truth`) rides beside fate.
/// A monitor whose horizon cannot close over an incomplete source set reports incompleteness rather than settling (`MonitorRefusal::ClosureUnavailable`).
pub type AdvanceMonitorFn = fn(
    profile: &MonitorProfile,
    prior: TemporalMonitorState,
    delta: &AdmittedDelta<'_>,
) -> Result<MonitorAdvance, MonitorRefusal>;

/// `rebuild` — rebuild one maintained state from accepted history by the reference road — the corruption and recovery path.
///
/// The `View` definition is unchanged; the output is a fresh `ViewState` carrying a fresh `ViewGeneration`.
/// Consumes pull-lane work.
pub type RebuildFn = fn(
    profile: &ViewResolveProfile,
    view: &View,
    history: &ExactHistoryRead,
    at: &FederationCut,
    budget: QueryWorkBudget,
) -> Result<ViewState, QueryRefusal>;

// ---------------------------------------------------------------------------
// Parity
// ---------------------------------------------------------------------------

/// `verify_parity` — judge push-maintained against pull-recomputed at the same definition, source set, exact Cuts, frame, relation versions, configuration, and profile.
///
/// The judge receives the `View` definition and the maintained `ViewState` as separate arguments and recomputes the reference road here — handing the judge the maintained state as its own reference is unrepresentable, and the two roads share no load-bearing evaluator.
/// Output: `ParityVerdict::Held` with a witness, or `::Diverged` with evidence — on divergence the maintained state loses and is rebuilt.
/// Refusal: `QueryRefusal::ParityIncomparable` when the pairing differs in any held-constant fact — refusing to judge is not a verdict.
pub type VerifyParityFn = fn(
    profile: &ViewResolveProfile,
    view: &View,
    maintained: &ViewState,
    history: &ExactHistoryRead,
    budget: QueryWorkBudget,
) -> Result<ParityVerdict, QueryRefusal>;

// ---------------------------------------------------------------------------
// Selection
// ---------------------------------------------------------------------------

/// `derive_selection` — derive exact semantic membership over one row domain at one source Cut, evaluated against the event owner's exact-read surface — membership is decided over accepted history, never over ambient state.
///
/// The predicate is application-typed and supplied explicitly; the representation is the profile-selected mechanism and never changes membership meaning.
/// Bounded by the Result-class cardinality limit and one affine work budget — evaluating membership over history is Work-class effort like every other history-consuming operation, never free.
pub type DeriveSelectionFn<P> = fn(
    representation: SelectionRepresentation,
    domain: RowDomainId,
    at: Cut,
    history: &ExactHistoryRead,
    predicate: P,
    limit: SelectionCardinalityLimit,
    budget: QueryWorkBudget,
) -> Result<SelectionMask, SelectionRefusal>;

/// `prove_row_domain_equality` — prove two row domains are one population.
///
/// The only mint of `RowDomainEqualityWitness`; equal cardinality proves nothing here.
/// Refuses (`RowDomainEqualityUnproven`) when the evidence does not close.
pub type ProveRowDomainEqualityFn = fn(
    left: RowDomainId,
    right: RowDomainId,
    history: &ExactHistoryRead,
    budget: QueryWorkBudget,
) -> Result<RowDomainEqualityWitness, SelectionRefusal>;

/// `intersect_selection` — intersect two masks over proven-equal row domains at the same Cut.
///
/// Composition without the witness is unrepresentable — the witness parameter is the law, not ceremony.
pub type IntersectSelectionFn = fn(
    left: &SelectionMask,
    right: &SelectionMask,
    equality: &RowDomainEqualityWitness,
) -> Result<SelectionMask, SelectionRefusal>;

/// `union_selection` — union two masks over proven-equal row domains at the same Cut.
pub type UnionSelectionFn = fn(
    left: &SelectionMask,
    right: &SelectionMask,
    equality: &RowDomainEqualityWitness,
) -> Result<SelectionMask, SelectionRefusal>;

/// `difference_selection` — subtract one mask from another over proven-equal row domains at the same Cut.
pub type DifferenceSelectionFn = fn(
    left: &SelectionMask,
    right: &SelectionMask,
    equality: &RowDomainEqualityWitness,
) -> Result<SelectionMask, SelectionRefusal>;

/// `complement_selection` — complement one mask, bounded by its domain's logical length — physical padding bits can never select nonexistent rows.
pub type ComplementSelectionFn = fn(
    mask: &SelectionMask,
) -> Result<SelectionMask, SelectionRefusal>;

/// `convert_selection` — convert one mask among qualified representations.
///
/// Membership meaning is invariant; an unqualified target representation refuses.
pub type ConvertSelectionFn = fn(
    mask: &SelectionMask,
    target: SelectionRepresentation,
) -> Result<SelectionMask, SelectionRefusal>;

// ---------------------------------------------------------------------------
// Materialization lifecycle — eight separate operations by law.
// Stage strengthening becomes typestate at realization (the standing roster is a recorded realization seam; whether persisted/foreign lifecycle state crosses back in as validated dynamic state or in-process affine typestate is part of that same recorded seam, closing at A2).
// The semantic distinctions are fixed here.
// ---------------------------------------------------------------------------

/// `derive_materialization` — pure derivation of one generation's content from accepted history at exact Cuts.
///
/// Produces a candidate generation; changes no accepted fact.
pub type DeriveMaterializationFn = fn(
    profile: &MaterializationProfile,
    role: MaterializationId,
    history: &ExactHistoryRead,
    at: &FederationCut,
    budget: QueryWorkBudget,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// `validate_materialization` — structural validation of one derived generation's physical material against its descriptor bounds — a reader refuses before allocating.
pub type ValidateMaterializationFn = fn(
    profile: &MaterializationProfile,
    generation: &MaterializationGeneration,
) -> Result<(), MaterializationRefusal>;

/// `bind_materialization` — semantic binding: prove the generation's `AppliedCut` — that the named source Cuts were actually incorporated.
///
/// New physical bytes never imply a newer `AppliedCut` (`MaterializationRefusal::AppliedCutUnproven`).
pub type BindMaterializationFn = fn(
    generation: MaterializationGeneration,
    history: &ExactHistoryRead,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// `publish_materialization` — durably publish one bound generation.
///
/// A failed publication changes no accepted fact; a published generation is not automatically active.
pub type PublishMaterializationFn = fn(
    generation: MaterializationGeneration,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// `activate_materialization` — activate one published generation for serving.
pub type ActivateMaterializationFn = fn(
    generation: MaterializationGeneration,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// `select_current_materialization` — select one active generation as current for its role.
///
/// Selection is explicit; a newer generation never becomes current by existing.
pub type SelectCurrentMaterializationFn = fn(
    role: MaterializationId,
    generation: MaterializationGenerationId,
) -> Result<(), MaterializationRefusal>;

/// `retire_materialization` — retire one generation.
///
/// A superseded generation remains identifiable historical physical evidence.
pub type RetireMaterializationFn = fn(
    generation: MaterializationGeneration,
) -> Result<(), MaterializationRefusal>;

/// `reclaim_materialization` — reclaim one retired generation's physical material.
///
/// Waits for live readers; never rewrites historical evidence.
pub type ReclaimMaterializationFn = fn(
    generation: MaterializationGenerationId,
) -> Result<(), MaterializationRefusal>;

// ---------------------------------------------------------------------------
// Information release
// ---------------------------------------------------------------------------

/// `resolve_protected` — request physical resolution of protected payload extents — the release chain's final step: grant first, exact lawful selection, skip forbidden extents, verify, then materialize only authorized fields.
///
/// Decrypt last, decrypt least.
/// The physical crossing is a typed port operation; this owner never receives raw key authority, and a resolution result never widens the grant that produced it.
/// Refusals: `ReleaseRefusal`.
pub type ResolveProtectedFn = fn(
    grant: &ProtectedResolutionGrant,
    mask: &SelectionMask,
    generation: &MaterializationGeneration,
) -> Result<ProtectedResolution, ReleaseRefusal>;
