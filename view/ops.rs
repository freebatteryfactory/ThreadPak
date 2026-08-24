//! # view — thin operation signatures
//!
//! Semantic signatures only: exact inputs, outputs, refusals, and bounds.
//! No bodies — realization lands with construction cut A2, and the exact
//! Rust ergonomics may still be machined at the guard pass without changing
//! any contract stated here. Signatures are written in declaration form
//! (trailing semicolon); nothing in this file executes, and nothing claims
//! implementation support.
//!
//! Every operation receives its profile and bounds as explicit typed
//! arguments — no operation reads a depot row, a clock, an environment, or
//! any ambient state (`depot/README.md`, "Rows are passed, never fetched").
//! Affine budgets pass by value and are consumed; plain limits ride inside
//! profiles.
//!
//! `ExactHistoryRead` is the event owner's exact-read surface over accepted
//! history at exact Cuts, declared with the event storage contract; this
//! owner consumes it and never bypasses it.

// ---------------------------------------------------------------------------
// Pull lane
// ---------------------------------------------------------------------------

/// Resolve one demand-driven question against accepted history at exact
/// Cuts.
///
/// Inputs: the resolve profile (row/depth/fan-out selections), the typed
/// query, the event owner's exact-read surface, the exact Cuts that answer
/// "what does now mean", and one affine work budget (consumed; never
/// widened). Output: a `Fix` carrying value, sources, Cuts, completeness,
/// freshness, road, consumed work, and explanation. Refusals:
/// `QueryRefusal` — bound exhaustion, closure demanded over an unclosed
/// source set, frame expectation violated. The row limit binds discovery
/// before decode, join, sort, or materialization; a full page proves no
/// completeness, and an empty page proves no absence beyond the frozen Cut.
pub fn resolve<T>(
    profile: &ViewResolveProfile,
    query: &Query,
    history: &ExactHistoryRead,
    at: &FederationCut,
    budget: QueryWorkBudget,
) -> Result<Fix<T>, QueryRefusal>;

/// Continue one bounded traversal from an exact continuation. The cursor
/// binds the operation it continues; a changed binding refuses as an
/// incompatible continuation — never a silent rebase onto newer history.
/// A cursor is never skip authority and never durable recovery state.
pub fn resolve_continue<T>(
    profile: &ViewResolveProfile,
    cursor: Cursor,
    history: &ExactHistoryRead,
    budget: QueryWorkBudget,
) -> Result<(Fix<T>, Option<Cursor>), QueryRefusal>;

// ---------------------------------------------------------------------------
// Push lane
// ---------------------------------------------------------------------------

/// Advance one maintained result: prior derived state plus one bounded
/// admitted delta yields the next derived state. Shallow, bounded,
/// recursion-free; work is declared per event by the advance profile's
/// formula and accounted in the output. Refusals: `AdvanceRefusal` — a
/// state that does not belong to the claim, a delta outside the claim's
/// sources, a delta that does not abut the state's `AppliedCut`, or an
/// exhausted bound. Producing a `ViewAdvance` publishes nothing and
/// advances no checkpoint.
pub fn advance(
    profile: &ViewAdvanceProfile,
    prior: View,
    delta: &AdmittedDelta<'_>,
) -> Result<ViewAdvance, AdvanceRefusal>;

/// Advance one finite temporal monitor by one bounded admitted delta. At
/// most one newly settled fate per exact claim, latching only forward;
/// knowledge (`Truth`) rides beside fate. A monitor whose horizon cannot
/// close over an incomplete source set reports incompleteness rather than
/// settling (`MonitorRefusal::ClosureUnavailable`).
pub fn advance_monitor(
    profile: &MonitorProfile,
    prior: TemporalMonitorState,
    delta: &AdmittedDelta<'_>,
) -> Result<MonitorAdvance, MonitorRefusal>;

/// Rebuild one maintained result from accepted history by the reference
/// road — the corruption and recovery path. The claim is unchanged; the
/// output carries a fresh `ViewGeneration`. Consumes pull-lane work.
pub fn rebuild(
    profile: &ViewResolveProfile,
    history: &ExactHistoryRead,
    at: &FederationCut,
    budget: QueryWorkBudget,
) -> Result<View, QueryRefusal>;

// ---------------------------------------------------------------------------
// Parity
// ---------------------------------------------------------------------------

/// Judge push-maintained against pull-recomputed at the same claim, source
/// set, exact Cuts, frame, relation versions, configuration, and profile.
/// The reference road is recomputed here — the maintained road never
/// certifies itself, and the two roads share no load-bearing evaluator.
/// Output: `ParityVerdict::Held` with a witness, or `::Diverged` with
/// evidence — on divergence the maintained result loses and is rebuilt.
/// Refusal: `QueryRefusal::ParityIncomparable` when the pairing differs in
/// any held-constant fact — refusing to judge is not a verdict.
pub fn verify_parity(
    profile: &ViewResolveProfile,
    maintained: &View,
    history: &ExactHistoryRead,
    budget: QueryWorkBudget,
) -> Result<ParityVerdict, QueryRefusal>;

// ---------------------------------------------------------------------------
// Selection
// ---------------------------------------------------------------------------

/// Derive exact semantic membership over one row domain at one source Cut.
/// The predicate is application-typed and supplied explicitly; the
/// representation is the profile-selected mechanism and never changes
/// membership meaning. Bounded by the Result-class cardinality limit.
pub fn derive_selection<P>(
    representation: SelectionRepresentation,
    domain: RowDomainId,
    at: Cut,
    predicate: P,
    limit: SelectionCardinalityLimit,
) -> Result<SelectionMask, SelectionRefusal>;

/// Prove two row domains are one population. The only mint of
/// `RowDomainEqualityWitness`; equal cardinality proves nothing here.
/// Refuses (`RowDomainEqualityUnproven`) when the evidence does not close.
pub fn prove_row_domain_equality(
    left: RowDomainId,
    right: RowDomainId,
    history: &ExactHistoryRead,
    budget: QueryWorkBudget,
) -> Result<RowDomainEqualityWitness, SelectionRefusal>;

/// Intersect two masks over proven-equal row domains at the same Cut.
/// Composition without the witness is unrepresentable — the witness
/// parameter is the law, not ceremony.
pub fn intersect_selection(
    left: &SelectionMask,
    right: &SelectionMask,
    equality: &RowDomainEqualityWitness,
) -> Result<SelectionMask, SelectionRefusal>;

/// Union two masks over proven-equal row domains at the same Cut.
pub fn union_selection(
    left: &SelectionMask,
    right: &SelectionMask,
    equality: &RowDomainEqualityWitness,
) -> Result<SelectionMask, SelectionRefusal>;

/// Subtract one mask from another over proven-equal row domains at the same
/// Cut.
pub fn difference_selection(
    left: &SelectionMask,
    right: &SelectionMask,
    equality: &RowDomainEqualityWitness,
) -> Result<SelectionMask, SelectionRefusal>;

/// Complement one mask, bounded by its domain's logical length — physical
/// padding bits can never select nonexistent rows.
pub fn complement_selection(
    mask: &SelectionMask,
) -> Result<SelectionMask, SelectionRefusal>;

/// Convert one mask among qualified representations. Membership meaning is
/// invariant; an unqualified target representation refuses.
pub fn convert_selection(
    mask: &SelectionMask,
    target: SelectionRepresentation,
) -> Result<SelectionMask, SelectionRefusal>;

// ---------------------------------------------------------------------------
// Materialization lifecycle — eight separate operations by law. Stage
// strengthening becomes typestate at realization (the standing roster is a
// recorded realization seam); the semantic distinctions are fixed here.
// ---------------------------------------------------------------------------

/// Pure derivation of one generation's content from accepted history at
/// exact Cuts. Produces a candidate generation; changes no accepted fact.
pub fn derive_materialization(
    profile: &MaterializationProfile,
    role: MaterializationId,
    history: &ExactHistoryRead,
    at: &FederationCut,
    budget: QueryWorkBudget,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// Structural validation of one derived generation's physical material
/// against its descriptor bounds — a reader refuses before allocating.
pub fn validate_materialization(
    profile: &MaterializationProfile,
    generation: &MaterializationGeneration,
) -> Result<(), MaterializationRefusal>;

/// Semantic binding: prove the generation's `AppliedCut` — that the named
/// source Cuts were actually incorporated. New physical bytes never imply
/// a newer `AppliedCut` (`MaterializationRefusal::AppliedCutUnproven`).
pub fn bind_materialization(
    generation: MaterializationGeneration,
    history: &ExactHistoryRead,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// Durably publish one bound generation. A failed publication changes no
/// accepted fact; a published generation is not automatically active.
pub fn publish_materialization(
    generation: MaterializationGeneration,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// Activate one published generation for serving.
pub fn activate_materialization(
    generation: MaterializationGeneration,
) -> Result<MaterializationGeneration, MaterializationRefusal>;

/// Select one active generation as current for its role. Selection is
/// explicit; a newer generation never becomes current by existing.
pub fn select_current_materialization(
    role: MaterializationId,
    generation: MaterializationGenerationId,
) -> Result<(), MaterializationRefusal>;

/// Retire one generation. A superseded generation remains identifiable
/// historical physical evidence.
pub fn retire_materialization(
    generation: MaterializationGeneration,
) -> Result<(), MaterializationRefusal>;

/// Reclaim one retired generation's physical material. Waits for live
/// readers; never rewrites historical evidence.
pub fn reclaim_materialization(
    generation: MaterializationGenerationId,
) -> Result<(), MaterializationRefusal>;

// ---------------------------------------------------------------------------
// Information release
// ---------------------------------------------------------------------------

/// Request physical resolution of protected payload extents — the release
/// chain's final step: grant first, exact lawful selection, skip forbidden
/// extents, verify, then materialize only authorized fields. Decrypt last,
/// decrypt least. The physical crossing is a typed port operation; this
/// owner never receives raw key authority, and a resolution result never
/// widens the grant that produced it. Refusals: `ReleaseRefusal`.
pub fn resolve_protected(
    grant: &ProtectedResolutionGrant,
    mask: &SelectionMask,
    generation: &MaterializationGeneration,
) -> Result<ProtectedResolution, ReleaseRefusal>;
