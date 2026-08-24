//! Core owner — thin semantic operation signatures.
//!
//! Signature declarations only: no bodies, no traits, no executor, no I/O.
//! Each signature states inputs, outputs, refusals, and bounds; bodies land
//! with the construction cuts. Every operation receives its profile as an
//! explicit argument — nothing reads an ambient register, per
//! `depot/README.md` "Rows are passed, never fetched".
//!
//! Deliberately absent: a universal "hash these bytes" API (derivation is
//! per-role, under a registered tag), a universal conversion operation
//! (checked width and role conversions ride `TryFrom` per consumer, and a
//! failed conversion is the consumer's typed refusal), and any operation
//! minting authority, identity scope, or key material.

/// Charge one affine budget. Consumes the budget and returns the smaller
/// successor, or the typed exhaustion naming the dimension, the demand, and
/// what remained. The one shared budget mechanic behind every owner-named
/// wrapper; no widening operation exists anywhere.
pub fn charge<D>(
    budget: Budget<D>,
    amount: u64,
) -> Result<Budget<D>, BudgetExhausted<D>>;

/// Admit candidate bytes as the canonical preimage of one declared family.
/// Bounded before allocation by the family's declared bound row; refuses an
/// unregistered family. The returned preimage is the identity substrate its
/// family's commitments derive from.
pub fn admit_preimage(
    profile: &CanonProfile,
    family: &DomainTag,
    bytes: &[u8],
) -> Result<CanonicalPreimage, PreimageRefusal>;

/// Derive the digest of one admitted preimage under one registered tag.
/// Total over an admitted preimage — every refusal already happened at
/// `admit_preimage`. The result is a receipt over exact bytes: it proves
/// the byte role its tag names and nothing more, and it is never authority.
pub fn derive_digest(
    profile: &IdentityProfile,
    tag: &DomainTag,
    preimage: &CanonicalPreimage,
) -> Digest;

/// Derive the keyed digest of one admitted preimage under one registered
/// tag and one scope-bound key — the keyed, scope-bound fingerprint the
/// ingress and protected-payload laws demand. Never a public unkeyed digest
/// of low-entropy input; the key is consumed by reference and never copied,
/// logged, or serialized by this operation.
pub fn derive_keyed_digest(
    profile: &IdentityProfile,
    tag: &DomainTag,
    key: &ScopedKey,
    preimage: &CanonicalPreimage,
) -> Digest;

/// Project the derivation context of one registered tag — the generated
/// context string keyed derivation consumes. A projection of the register
/// row; hand-authored contexts are invalid, and the other three projections
/// (text prefix, wire role, documentation table) are build-time generation,
/// not operations.
pub fn derive_key_context(
    profile: &CanonProfile,
    tag: &DomainTag,
) -> DeriveKeyContext;

/// Validate candidate bytes against one exact schema commitment under its
/// declared unknown-member policy. Bounded before allocation; duplicate and
/// ambiguous representations refuse; a removed identity never returns. The
/// witness proves the check ran under exactly this schema and version — it
/// proves nothing about domain truth, authority, or acceptance.
pub fn check_schema(
    schema: &SchemaCommitment,
    schema_id: SchemaId,
    version: SchemaVersion,
    policy: UnknownMemberPolicy,
    bytes: &[u8],
) -> Result<SchemaAdmission, SchemaValidationRefusal>;
