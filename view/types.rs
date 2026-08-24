//! # view — role-graph declarations
//!
//! The derived-result owner's nouns. This file declares roles and the law each
//! role carries; behavior lives in this owner's thin operations, and nothing in
//! this file is authority — every resident is derived from accepted history at
//! exact Cuts and is rebuildable (see `README.md`, "The wall").
//!
//! The `use` declarations below are this owner's declared dependency edges:
//! view depends on core vocabulary and on the event owner's authority nouns,
//! and on nothing else. No event-owner or runtime-owner code may depend on
//! these types for admission, ordering, or authority.
//!
//! Declarations only. Fields appear where product law pins them; interiors
//! marked as seams are the realization pass's to close under this contract.

use crate::core::logic::Truth;
use crate::event::types::{AcceptedEvent, Cut, FederationCut, ReferenceFrameId, SourceSetId};

use core::num::{NonZeroU32, NonZeroU64};

// ---------------------------------------------------------------------------
// Identities
// ---------------------------------------------------------------------------

/// Names exactly which population a mask, column, or materialization speaks
/// about. Equal cardinality is never equality of domains; two `RowDomainId`s
/// are comparable only by identity, never by size or by byte-shape.
pub struct RowDomainId {
    // seam: canonical-byte identity per the identity owner's class rules
}

/// The named derived role and contract — which question a materialization
/// answers. One of four never-conflated identities (role, generation, block,
/// occurrence).
pub struct MaterializationId {
    // seam: canonical-byte identity
}

/// One realization generation of a materialization role. A new generation is
/// a new physical realization claim; it never renames the role.
pub struct MaterializationGenerationId {
    // seam: canonical-byte identity
}

/// One bounded physical component under a materialization generation.
pub struct DataBlockId {
    // seam: canonical-byte identity
}

/// One exact stored or in-memory realization of a DataBlock.
pub struct OccurrenceId {
    // seam: canonical-byte identity
}

/// Identity of one temporal claim. A settled fate binds to exactly one
/// (`TemporalClaimId`, `TemporalClaimVersion`) pair: changing the claim,
/// its version, its horizon, or its source set creates a NEW claim and never
/// reopens an old fate.
pub struct TemporalClaimId {
    // seam: canonical-byte identity
}

/// Version of a temporal claim's semantics. Versions separate claims; they do
/// not amend settled evaluations.
pub struct TemporalClaimVersion {
    // seam: version representation per the identity owner's class rules
}

/// Identity of one subscription relationship. Declaration seat only: the
/// durable checkpoint keyed by this identity is runtime-owned authority, and
/// nothing in this owner may advance it.
pub struct SubscriptionId {
    // seam: canonical-byte identity
}

// ---------------------------------------------------------------------------
// Derived answers
// ---------------------------------------------------------------------------

/// A demand-driven question against accepted history at exact Cuts.
/// A query names its source set, its Cuts, its frame expectations, and its
/// bounds; every read names what "now" means.
pub struct Query {
    // seam: question body, source and Cut requirements, bound set
}

/// A derived answer: what can be concluded from accepted history at exact
/// Cuts — never what is authoritatively stored. Shape, completeness, and
/// freshness are orthogonal axes and never collapse into one another or into
/// the value.
///
/// The exact public field surface is a recorded open seam (Escalation 1 in
/// `README.md`); the axes below are law.
pub struct Fix<T> {
    value: T,
    sources: SourceSetId,
    at: FederationCut,
    completeness: Completeness,
    freshness: Freshness,
    // seam: provenance, consumed work, and explanation bindings
}

/// This owner's completeness roster (evicted from any universal enum: each
/// axis family keeps an owner-specific roster). Incompleteness names what is
/// missing; it is never silently dropped, and "nothing arrived yet" never
/// becomes "nothing exists".
pub enum Completeness {
    /// The named source set was completely observed through the bound Cuts.
    Complete,
    /// One or more named sources were not completely observed. Closure-
    /// requiring conclusions (absence, negation, exhaustive search, final
    /// order, top-k) are unavailable in this state.
    Incomplete {
        // seam: named missing/unclosed sources
    },
}

/// This owner's freshness roster: how the answer's Cuts relate to what was
/// requested. Freshness is evidence about staleness, never permission to
/// substitute a newer or older result.
pub struct Freshness {
    // seam: requested-versus-bound Cut relationship roster
}

/// A maintained derived result — the push lane's resident. Rebuildable from
/// accepted history by the reference road at the same claim, source set,
/// Cuts, frame, and profile; parity with that road is required, and on
/// disagreement the maintained result loses.
///
/// (`Projection` is deliberately unminted pending the View/Projection naming
/// call — Escalation 2 in `README.md`.)
pub struct View {
    // seam: maintained-state binding: role, source set, AppliedCut, generation
}

/// The semantic relationship between one consumer and one maintained result,
/// with a bounded retained window. A subscription's durable skip authority is
/// its runtime-owned checkpoint; a delivered update, a wake, and a retained
/// window are never progress.
pub struct Subscription {
    // seam: subscriber binding, subject binding, window bound
}

/// Continuation of one traversal or query context. A cursor continues work;
/// it is never skip authority, never a checkpoint, and never durable recovery
/// state.
pub struct Cursor {
    // seam: traversal-context binding
}

// ---------------------------------------------------------------------------
// Temporal claims
// ---------------------------------------------------------------------------

/// One bounded temporal claim over accepted history: identity + version +
/// source set + frame + horizon + claim body. Which claim families exist
/// first is a recorded open seam (Escalation 3 in `README.md`).
pub struct TemporalClaim {
    id: TemporalClaimId,
    version: TemporalClaimVersion,
    sources: SourceSetId,
    frame: ReferenceFrameId,
    horizon: TemporalHorizon,
    // seam: claim body (the temporal proposition itself)
}

/// Temporal fate — orthogonal to K3 knowledge (`Truth`) and to horizon;
/// the three never merge into one enum. Fate latches: `Open → Satisfied` and
/// `Open → Violated`, never back, for the same exact claim.
pub enum TemporalFate {
    /// Lawful future extensions of history can still decide either way.
    Open,
    /// No lawful future extension can unsatisfy the claim.
    Satisfied,
    /// No lawful future extension can repair the claim.
    Violated,
}

/// A finite push-lane monitor's persisted state: derived and rebuildable,
/// never temporal truth. Binds the exact claim, its source set, the exact
/// `AppliedCut` incorporated, and a monitor generation; when stale or corrupt
/// it is recomputed from accepted history, never trusted.
pub struct TemporalMonitorState {
    claim: TemporalClaimId,
    claim_version: TemporalClaimVersion,
    sources: SourceSetId,
    applied: AppliedCut,
    // seam: monitor generation and bounded state body
}

/// One monitor advance outcome: the next state and, at most once per exact
/// claim, a newly settled fate. Knowledge (`Truth`) rides beside fate and is
/// evaluated per claim law — `Truth::Pending` is an evidence statement, not a
/// fate.
pub struct MonitorAdvance {
    next: TemporalMonitorState,
    settled: Option<TemporalFate>,
    knowledge: Truth,
    // seam: declared-work accounting binding
}

// ---------------------------------------------------------------------------
// Selection and release
// ---------------------------------------------------------------------------

/// Exact semantic membership over one row domain at one source Cut. The
/// physical representation may vary; membership meaning may not. Masks
/// compose only across proven-equal row domains at the same Cut; composition
/// over unproven equality refuses (`SelectionRefusal::RowDomainEqualityUnproven`).
/// A mask never flattens `Truth::Pending` into `False`: fail-closed is not
/// permission to report the stronger fact.
pub struct SelectionMask {
    row_domain: RowDomainId,
    at: Cut,
    // seam: private membership representation
}

/// Read authority over selected information. Declared here; minted only by
/// the host's authority installation, never by any view operation, and never
/// widened by any result derived under it.
pub struct ReadGrant {
    // seam: scope, attenuation, generation, expiry bindings
}

/// Authority to request physical resolution of protected payload extents.
/// The physical crossing is a typed port operation; view code never receives
/// raw key authority.
pub struct ProtectedResolutionGrant {
    // seam: key-scope, extent-scope, generation bindings
}

// ---------------------------------------------------------------------------
// Materializations
// ---------------------------------------------------------------------------

/// Which authoritative source Cuts a derived generation incorporated.
/// Never a storage snapshot identifier: new physical bytes do not imply a
/// newer `AppliedCut`, and a newer `AppliedCut` requires proof that newer
/// source Cuts were actually incorporated. `CommitPoint`, `AppliedCut`, and
/// checkpoint remain three different facts.
pub struct AppliedCut {
    sources: SourceSetId,
    incorporated: FederationCut,
}

/// One realization generation of a materialization role: the binding among
/// role, generation, source set, and incorporated Cuts. Lifecycle stages
/// (derive, validate, bind, publish, activate, select, retire, reclaim) are
/// separate operations; the standing roster for those stages is a realization
/// seam under this contract.
pub struct MaterializationGeneration {
    role: MaterializationId,
    generation: MaterializationGenerationId,
    applied: AppliedCut,
    // seam: derivation-profile and evidence bindings
}

/// One bounded physical component under a generation: derived acceleration,
/// never authority. Describing itself proves neither source existence nor
/// derivation correctness; layout and device hints are untrusted claims.
/// A corrupt block is discarded and rebuilt — never reported as absence.
pub struct DataBlock {
    generation: MaterializationGenerationId,
    block: DataBlockId,
    row_domain: RowDomainId,
    // seam: column/extent descriptor bindings (grouping lawful, omission not)
}

// ---------------------------------------------------------------------------
// Bounds owned here (numeric values and paved profiles live in the depot)
// ---------------------------------------------------------------------------

/// Result-class limit: maximum rows one query may return.
pub struct QueryRowLimit(NonZeroU32);

/// Work-class affine budget for one resolve invocation: charging consumes it
/// and returns a smaller successor; no widening operation exists anywhere.
/// Deliberately neither `Clone` nor `Copy` — duplicating it would fabricate
/// capacity.
#[must_use]
pub struct QueryWorkBudget(NonZeroU64);

/// Work-class limit: maximum navigation depth one resolve may traverse.
pub struct NavigationDepth(NonZeroU32);

/// Work-class limit: maximum relation fan-out one traversal step may follow.
pub struct RelationFanOutLimit(NonZeroU32);

/// Result-class limit: maximum selected-membership cardinality one selection
/// may report.
pub struct SelectionCardinalityLimit(NonZeroU32);

/// Memory-class limit: maximum bytes one materialization pass may retain.
pub struct MaterializationByteLimit(NonZeroU64);

/// Time-class bound: the explicit horizon a bounded temporal claim carries.
/// A horizon is a bound riding beside K3 and fate — never a truth value.
pub struct TemporalHorizon {
    // seam: horizon dimension (admitted-event count or admitted duration)
}

/// Memory-class limit: maximum retained window one subscription may hold.
pub struct SubscriptionWindowLimit(NonZeroU32);

// ---------------------------------------------------------------------------
// Refusals (role-specific; never one mega-error)
// ---------------------------------------------------------------------------

/// Refusals of the pull lane. Every refusal names the violated law, the typed
/// owner, the offending value's role, and the repair direction.
pub enum QueryRefusal {
    // seam: roster closed by the realization pass under this contract
}

/// Refusals of the push lane's advance operation.
pub enum AdvanceRefusal {
    // seam
}

/// Refusals of selection and mask composition.
pub enum SelectionRefusal {
    /// Two masks or columns were composed across row domains whose equality
    /// is unproven — including equal-cardinality impostors. Settled name;
    /// fail-closed here never reports the stronger fact.
    RowDomainEqualityUnproven,
    // seam: remaining roster
}

/// Refusals of temporal-monitor advance and evaluation.
pub enum MonitorRefusal {
    // seam
}

/// Refusals of materialization lifecycle operations.
pub enum MaterializationRefusal {
    // seam
}

/// Refusals of the information-release chain.
pub enum ReleaseRefusal {
    // seam
}

// ---------------------------------------------------------------------------
// Declared-edge witnesses
// ---------------------------------------------------------------------------

/// The push lane's only lawful input pair: prior derived state plus one
/// bounded admitted delta. Declared as a type-level statement that advance
/// consumes accepted events and never foreign or derived substitutes.
pub struct AdmittedDelta<'history> {
    events: &'history [AcceptedEvent],
    // seam: delta bounds and source binding
}
