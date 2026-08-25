//! Core owner — shared-family declarations.
//!
//! Declarations only: no impl blocks, no function bodies, no convenience derives — a derive is a semantic claim minted per type at the guard pass.
//! Thin operation signatures live in `ops.rs`; the selected values every width, grammar, and mechanism fact below cites live in `depot/core.md` (ratified by the canon-packet mint of 2026-08-24, or carried candidate/qualification/withheld per row status, per the depot row law).
//!
//! Profile types here are ALGEBRA — what may lawfully be selected.
//! A depot row selects one coordinate inside that algebra.
//! Operations receive the exact profile as an explicit argument; nothing here is ambient (`depot/README.md`, "Rows are passed, never fetched").
//!
//! Cross-home names appear as bare names with owner attribution (`ClockDomainId` — port); real imports arrive with the dependency probe.
//! Families whose law forces no type still declare nothing — their first forced type arrives with its first consumer, per the no-prophecy rule.

use core::marker::PhantomData;

// ---------------------------------------------------------------------------
// logic — the shared truth and decision axes
// ---------------------------------------------------------------------------

/// The shared K3 truth axis, strong Kleene.
///
/// All cells of the truth tables are authored; consumers match exhaustively, and no wildcard arm may quietly turn `Pending` into `False`.
/// `Pending AND False = False`.
///
/// `Pending` is an evidence statement — truth cannot yet be established from what is available.
/// It is not a fate, not a deferral, not a suspension, and not any other owner's "not yet".
pub enum Truth {
    True,
    False,
    Pending,
}

/// The shared decision axis.
///
/// No conversion exists in either direction between `Decision` and `Truth`; `Defer` is not `Pending` and is not a refusal.
///
/// `Defer` carries the consumer's typed demand — what additional admitted evidence could close the decision (the program owner's `EvidenceRequirement` is the first such type).
/// The demand is inert data: it performs nothing and grants nothing.
pub enum Decision<Demand> {
    Allow,
    Deny,
    Defer(Demand),
}

// ---------------------------------------------------------------------------
// refusal — anatomy machinery
//
// The four-carry law (violated law, typed owner, offending value's role, repair direction) and the three body shapes remain family law realized per owner — deliberately no universal refusal type.
// What the consumers force here is the identity that keys depot refusal prose.
// ---------------------------------------------------------------------------

/// Identity of one refusal variant, keying the depot's refusal-prose rows.
///
/// A registered refusal is a semantic commitment: new meaning mints a new identity, never reuses a retired one.
/// Prose keyed by this identity adds no variant and no condition.
pub struct RefusalId {
    /* closes with the identity profile; width cites depot/core.md */
}

// ---------------------------------------------------------------------------
// bound — the closed class register, the dimension row shape, and the shared affine mechanic
// ---------------------------------------------------------------------------

/// The settled closed register of bound classes.
///
/// The class of every concrete bound is declared at its owner, never inferred; concrete bound types live with the operations that consume them, and their numeric values live in the depot.
pub enum BoundClass {
    Work,
    Memory,
    Result,
    Output,
    Effect,
    Suspension,
    Time,
}

/// How consuming a bound behaves.
///
/// Affine only where duplicating the value would fabricate capacity; a plain limit is copyable and needs no linear ceremony (`../README.md` §Bounds).
pub enum ChargeLaw {
    /// Charging consumes the value and returns a smaller successor; no widening method exists anywhere.
    Affine,
    /// A copyable declared ceiling consumed by comparison, not by charge.
    PlainLimit,
}

/// The settled default-posture classification every paved profile row carries (D-DEFAULT-1; `depot/README.md` §Defaults are classified).
pub enum DefaultClassification {
    /// A paved default exists and an explicit override is lawful.
    Asymmetric,
    /// Equally safe alternatives; no default row — the interface selects.
    Symmetric,
    /// The default is the strict or refusing posture.
    SafetyRelevant,
}

/// When a selected row's value binds (`depot/README.md` §Binding time).
pub enum BindingTime {
    Artifact,
    Generation,
    Deployment,
    Invocation,
    TestOnly,
}

/// What changes when a row's value changes (`depot/README.md` §Binding time and change consequence).
///
/// A row without a declared consequence is invalid.
pub enum ChangeConsequence {
    RequalificationOnly,
    NewMechanismGeneration,
    NewMaterializationGeneration,
    NewImageIdentity,
    NewCanonicalProfileIdentity,
    NewPersistedFormatVersion,
    CompatibleOverride,
}

/// Identity of one registered bound dimension.
pub struct BoundDimensionId {
    /* closes with the identity profile; width cites depot/core.md */
}

/// Identity of one registered unit.
///
/// Role-distinct from every dimension identity; no implicit conversion crosses units.
pub struct UnitId {
    /* registered id; width cites depot/core.md row registered-id-width */
}

/// One row of the bound-dimension register — the second level beneath `BoundClass` (recovered from the archived register's row shape).
///
/// The numeric value and the owning-family attribution are the depot row's facts per the depot row law; this type binds the typed facts that make the value honest.
pub struct BoundDimension {
    dimension: BoundDimensionId,
    class: BoundClass,
    /// The declared unit — never inferred from the Rust representation.
    unit: UnitId,
    charge: ChargeLaw,
    binding: BindingTime,
    default_classification: DefaultClassification,
    consequence: ChangeConsequence,
}

/// The shared affine budget mechanic.
///
/// INTERNAL machinery: the public API is each owner's named wrapper (`QueryWorkBudget`, `SemanticWorkBudget`, `KnowledgeBudget`, `ReconciliationStepBudget`, `PumpWorkBudget` — the five identical claimants that met this family's extraction bar).
/// Charging consumes the value and returns a smaller successor via `ops::charge`; no widening method exists anywhere.
/// Deliberately neither `Clone` nor `Copy` — duplicating it would fabricate capacity.
///
/// `D` is the owner's dimension marker; the invariant phantom keeps one dimension's budget from ever standing in for another's.
#[must_use]
pub struct Budget<D> {
    /// Magnitude substrate: u64, per depot/core.md row `budget-magnitude` (candidate; matches all five claimants).
    remaining: u64,
    dimension: PhantomData<fn(D) -> D>,
}

/// Typed exhaustion of one budget: which dimension, what was demanded, what remained.
///
/// A terminal typed fact, never a panic.
#[must_use]
pub struct BudgetExhausted<D> {
    demanded: u64,
    remaining: u64,
    dimension: PhantomData<fn(D) -> D>,
}

// ---------------------------------------------------------------------------
// number-adjacent shared time value roles
//
// Chronology is the event owner's; deadlines are the runtime owner's; physical observations are the port owner's.
// These are the shared value roles those owners refuse to conflate.
// ---------------------------------------------------------------------------

/// A signed difference between two time values in one clock domain.
///
/// A negative delta is valid evidence — never clamped, never saturated to zero, and deliberately not authority-capable: it is diagnostic evidence, not a duration, deadline, order, or budget.
///
/// (`ClockDomainId` — port owner. The std `core::time::Duration` has no signed counterpart; this role is the gap it cannot fill.)
pub struct TimeDelta {
    domain: ClockDomainId,
    /// Signed magnitude; substrate i128 per depot/core.md row `time-substrate` (candidate — matches `RawWallObservation.reported`).
    magnitude: i128,
}

// Unsigned spans are `core::time::Duration` (owner ruling 2026-08-24, stdlib-batteries applied): a type is not contaminated by a method (`saturating_sub`) semantic code never calls — ThreadPak semantic operations use only checked operations on it.
// Owner-specific Time-class wrappers (horizons, ceilings, allowances) keep their roles at their owners; no custom generic span type exists here.

// ---------------------------------------------------------------------------
// schema — commitment identities and the shape laws with carriers
// ---------------------------------------------------------------------------

/// Identity of one schema commitment.
///
/// Distinct from any codec, layout, or occurrence identity; equal widths never substitute.
/// Derives from the schema's `SchemaCommitment` under the identity profile.
pub struct SchemaId {
    /* width cites depot/core.md row digest-width-class-ab (ratified) */
}

/// Version of one schema commitment.
///
/// A removed field or variant identity is never reused across versions, and numerically newer never implies compatible.
pub struct SchemaVersion {
    /* Class-C scoped ordinal; width cites depot/core.md row class-c-order-scalar (ratified) */
}

/// Identity of one field within one schema commitment.
///
/// Field identity survives renames; a rename is documentation, a new identity is a new field.
/// Never silently recycled.
pub struct FieldId {
    /* closes with the identity profile; width cites depot/core.md */
}

/// Identity of one variant within one schema commitment.
///
/// Same reuse law as `FieldId`: a removed variant identity is never reused.
pub struct VariantId {
    /* closes with the identity profile; width cites depot/core.md */
}

/// The canonical-byte commitment a `SchemaId` derives from — the schema's semantic meaning as domain-tagged canonical bytes, distinct from any descriptor artifact's digest.
pub struct SchemaCommitment {
    digest: Digest,
}

/// The explicit unknown-member contract every schema declares — never silent (recovered roster, ch10).
///
/// `OpaquePreserved` is monotone extendability (a FUTURE schema may interpret), not pending: there is no future in which THIS schema determines an unknown member, so this never collapses into `Truth::Pending`.
pub enum UnknownMemberPolicy {
    /// Every unknown member refuses.
    Closed,
    /// Skippable or preservable without changing base meaning.
    OptionalExtension,
    /// A reader that does not understand the extension refuses.
    RequiredExtension,
    /// Carried without interpretation and without authority.
    OpaquePreserved,
}

/// Witness that one byte sequence passed bounded schema validation under one exact schema commitment and policy.
///
/// Possession proves the check ran; it proves nothing about domain truth.
#[must_use]
pub struct SchemaAdmission {
    schema: SchemaId,
    version: SchemaVersion,
}

/// The typed refusal family of schema validation.
///
/// Each variant traces to a law sentence of the schema family; the roster closes with the schema profile.
pub enum SchemaValidationRefusal {
    /// A declared bound was exceeded before allocation.
    BoundExceededBeforeAllocation,
    /// Two byte representations decode to one value.
    DuplicateRepresentation,
    /// The bytes admit more than one reading.
    AmbiguousRepresentation,
    /// An unknown member arrived under `UnknownMemberPolicy::Closed` or an ununderstood `RequiredExtension`.
    UnknownRequiredMember,
    /// A removed field or variant identity was reused.
    RemovedIdentityReused,
}

// ---------------------------------------------------------------------------
// identity — the profile algebra
// ---------------------------------------------------------------------------

/// The settled six-class identity classification (roster recovered from the archived matrix; variant spellings are draft, the classification law is settled).
///
/// Every identity in the machine falls into exactly one class, and each class answers the layout and capability questions once — per-class rules such as scope-guarded comparison and no derived `Ord` on scoped values ride the class, never the call site.
pub enum IdentityClass {
    /// Equal meaning ⇒ equal id: domain-tagged digest of normalized meaning; no order; keyed under a `KeyScopeId` when protected.
    SemanticCommitment,
    /// A receipt over exact bytes; proves the byte role its preimage names and nothing more.
    ByteDigest,
    /// Exact order assigned by one writer authority; scope-guarded, no derived `Ord`, no cross-scope comparison.
    AuthorityOrder,
    /// One occurrence among possible equals; a reader parses no structure from it.
    ///
    /// Whether it is minted fresh or derived is the family's `IdentityCreationLaw`, never implied by this class.
    Occurrence,
    /// A composition of referenced identities.
    TypedReference,
    /// Schema-declared within the application namespace.
    ApplicationScope,
}

/// How one identity family's values are minted — the second column of the restored two-column identity law (owner mint 2026-08-24).
///
/// Identity class (what kind of identity) and creation law (how it is minted) are independent columns: `IdentityClass` never implies a creation law.
/// An occurrence may be derived (`TurnId` — replay converges on the same identity) or fresh (a 16-byte opaque id); every identity family's register row carries both columns.
pub enum IdentityCreationLaw {
    /// Deterministically derived from a declared canonical preimage under a registered digest role; replay converges on the same identity.
    DerivedCommitment,
    /// Fresh opaque bytes; uniqueness rides entropy, and no preimage exists.
    FreshOpaque,
    /// Assigned by one authority's own law (a register, a writer, an admission operation) — neither derived nor entropy-fresh.
    AuthorityAssigned,
}

/// Identity of one key scope — the boundary under which keyed commitments and protected material are derived and destroyed.
///
/// Declaration seat only: key custody, installation, and shredding belong to the owners that hold keys (view resolution, port quarantine, host installation), never to this family.
pub struct KeyScopeId {
    /* closes with the identity profile; width cites depot/core.md */
}

/// Key material currently valid under one `KeyScopeId` for **keyed-digest derivation** — this role, not universal secret-authority law.
///
/// The 32-byte width is a mechanism fact of the blake3 keyed mode; a future key kind that is not a keyed-digest key (an encryption key, an external-custody handle) is a distinct role minted with its consumer, never a widening of this one.
/// Declared here so keyed derivation signatures can name it; minted, held, rotated, and destroyed by the host's key custody — never by any core operation.
/// Never `Clone`, never serialized, never formatted or logged (custody law, README §3).
pub struct ScopedKey {
    scope: KeyScopeId,
    /* key bytes; width cites depot/core.md row keyed-digest-key (ratified with the canon packet — 32 bytes, mechanism-forced by keyed mode) */
}

/// Identity of one digest family.
///
/// The family id sits inside every domain-separation tag, so every identity names its family by construction and no ambient "which algorithm?" question exists.
/// Cross-family comparison refuses at the scope guard; migration between families is a named re-digest morphism, never a reinterpretation.
pub struct DigestFamilyId {
    /* registered id; width cites depot/core.md row registered-id-width */
}

/// One digest-family selection: which family, under which realization profile row.
///
/// Realizations of one family (native dispatch, portable Rust, wasm) must produce identical outputs against the depot's golden vectors — swapping realizations is requalification, never a new identity; swapping families is a new canonical profile.
pub struct DigestProfile {
    family: DigestFamilyId,
    /* realization selection cites depot/core.md rows blake3-realization-* (qualification rows — selection is evidence, never taste) */
}

/// The identity-profile algebra: the facts one identity profile binds.
///
/// The selected values are the depot/core.md rows this type provenances — per-class widths, byte order, text form, per-family creation law, and the tag register.
/// No owner hard-codes any of them independently; every width in every home cites its row here.
pub struct IdentityProfile {
    digest: DigestProfile,
    /* per-class width rows, byte-order row, text-form row, and the per-family `IdentityClass` + `IdentityCreationLaw` columns are the depot/core.md selections this profile binds */
}

// ---------------------------------------------------------------------------
// canon — domain separation, preimages, digests
// ---------------------------------------------------------------------------

/// Version of the domain-tag grammar itself (the `<tag-version>` segment).
pub struct TagVersion {
    /* Class-C scoped ordinal; width cites depot/core.md */
}

/// Registered identity of one tag role (the `<role>` segment) — also the wire-role projection's id.
pub struct TagRoleId {
    /* registered id; width cites depot/core.md row registered-id-width */
}

/// Registered identity of one preimage family (the `<family>` segment) — the identity family whose commitments derive under the tag.
///
/// Distinct from the digest family: one names what is being committed, the other names the algorithm committing it.
pub struct PreimageFamilyId {
    /* registered id; width cites depot/core.md row registered-id-width */
}

/// Version of one tag role's preimage contract (the `<role-version>` segment) — bumped when the role's preimage grammar changes.
///
/// A schema version lives inside a schema-family preimage where one exists; it is not a universal axis on every tag — many identity families (Turn, Attempt, checkpoint advances) have no schema.
pub struct RoleVersion {
    /* Class-C scoped ordinal; width cites depot/core.md row class-c-order-scalar (ratified) */
}

/// One entry of the domain-tag register.
///
/// The register is the single source of domain separation; its four projections (derivation context, text prefix, wire role, documentation table) are generated from it and structurally cannot drift.
/// A hand-edited projection is invalid.
///
/// String form (depot/core.md row `domain-tag-grammar`, ratified as retouched 2026-08-24): `threadpak/<tag-version>/<digest-family>/<family>/<role>/<role-version>` — the digest family rides inside the tag, so an identity names its algorithm by construction, and the final segment is the role's own version, never a universal schema version.
/// Non-full-width outputs are distinct registered roles, never ad-hoc truncations.
pub struct DomainTag {
    tag_version: TagVersion,
    digest_family: DigestFamilyId,
    preimage_family: PreimageFamilyId,
    role: TagRoleId,
    role_version: RoleVersion,
}

/// The composed canonical bytes of one declared preimage family, admitted under that family's declared bound.
///
/// The preimage is the identity substrate: the identity derives from it — never the reverse — and binding a preimage supports an integrity or identity claim without independently proving the underlying assertion true.
pub struct CanonicalPreimage {
    /* family binding plus bounded bytes; composition is per-commitment-family law, declared with each owner's commitment row */
}

/// The typed refusal family of preimage admission.
pub enum PreimageRefusal {
    /// The named preimage family is not registered.
    UnknownPreimageFamily,
    /// The bytes exceed the family's declared bound.
    BeyondDeclaredBound,
}

/// One digest value: a receipt over the exact bytes of one preimage under one domain tag.
///
/// Never authority over meaning, never ordered, never parsed.
/// Equality is the only lawful comparison, and constant-time where the role is protected.
pub struct Digest {
    /// Width: depot/core.md row `digest-width-class-ab` (ratified 2026-08-24, 32 bytes, ch10 provenance).
    ///
    /// Other widths are distinct registered roles via XOF, never truncations of this one.
    bytes: [u8; 32],
}

/// The derivation-context projection of one `DomainTag` — the exact context string keyed derivation consumes.
///
/// Generated from the register; hand-authored contexts are invalid.
pub struct DeriveKeyContext {
    /* generated bytes of the tag's string form */
}

/// The canon-profile algebra: byte order, integer posture, text form, framing, and the tag register — the facts whose selected values are the depot/core.md canon rows (big-endian, fixed-width no varints, bech32m text form, `ThreadPakFrameV1` framing — ratified by the canon mint per row status).
///
/// Mechanism selections are profile facts cited from here, never restated per call site.
pub struct CanonProfile {
    identity: IdentityProfile,
    tag_grammar: TagVersion,
    /* byte-order, integer-posture, text-form, and framing selections are the depot/core.md rows this profile binds */
}
