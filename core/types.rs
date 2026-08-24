//! Core owner — shared-family declarations.
//!
//! Declarations only: no impl blocks, no function bodies, no convenience
//! derives — a derive is a semantic claim minted per type at the guard pass.
//! Families whose law forces no type yet declare nothing here (refusal shapes,
//! the value honesty laws, the identity-class roster): their law lives in the
//! README, and their first forced type arrives with its first consumer —
//! deliberately, per the no-prophecy rule.
//!
//! Where a representation is written `/* closes with … */`, the semantic role
//! is law and the exact shape is owner-derived machining still to land.

// ---------------------------------------------------------------------------
// logic — the shared truth and decision axes
// ---------------------------------------------------------------------------

/// The shared K3 truth axis, strong Kleene. All cells of the truth tables are
/// authored; consumers match exhaustively, and no wildcard arm may quietly
/// turn `Pending` into `False`. `Pending AND False = False`.
///
/// `Pending` is an evidence statement — truth cannot yet be established from
/// what is available. It is not a fate, not a deferral, not a suspension, and
/// not any other owner's "not yet".
pub enum Truth {
    True,
    False,
    Pending,
}

/// The shared decision axis. No conversion exists in either direction between
/// `Decision` and `Truth`; `Defer` is not `Pending` and is not a refusal.
///
/// `Defer` carries the consumer's typed demand — what additional admitted
/// evidence could close the decision (the program owner's
/// `EvidenceRequirement` is the first such type). The demand is inert data:
/// it performs nothing and grants nothing.
pub enum Decision<Demand> {
    Allow,
    Deny,
    Defer(Demand),
}

// ---------------------------------------------------------------------------
// bound — the closed class register and budget mechanics
// ---------------------------------------------------------------------------

/// The settled closed register of bound classes. The class of every concrete
/// bound is declared at its owner, never inferred; concrete bound types live
/// with the operations that consume them, and their numeric values live in
/// the depot.
pub enum BoundClass {
    Work,
    Memory,
    Result,
    Output,
    Effect,
    Suspension,
    Time,
}

// ---------------------------------------------------------------------------
// number-adjacent shared time value roles
//
// Chronology is the event owner's; deadlines are the runtime owner's;
// physical observations are the port owner's. These are the shared value
// roles those owners refuse to conflate.
// ---------------------------------------------------------------------------

/// A signed difference between two time values in one domain. A negative
/// delta is valid evidence — never clamped, never saturated to zero.
/// Not a `Duration`, not a deadline, not chronology.
pub struct TimeDelta {
    /* signed magnitude; domain binding closes with the time-profile pass */
}

/// An unsigned span. Not a `TimeDelta`, not a deadline, not a limit —
/// a `Time`-class bound wraps one where an owner declares a ceiling.
pub struct Duration {
    /* unsigned magnitude; closes with the time-profile pass */
}

// ---------------------------------------------------------------------------
// schema — commitment identities
// ---------------------------------------------------------------------------

/// Identity of one schema commitment. Distinct from any codec, layout, or
/// occurrence identity; equal widths never substitute.
pub struct SchemaId {
    /* closes with the identity profile */
}

/// Version of one schema commitment. A removed field or variant identity is
/// never reused across versions.
pub struct SchemaVersion {
    /* closes with the identity profile */
}

/// Identity of one field within one schema commitment. Field identity
/// survives renames; a rename is documentation, a new identity is a new
/// field.
pub struct FieldId {
    /* closes with the identity profile */
}

// ---------------------------------------------------------------------------
// canon — domain separation
// ---------------------------------------------------------------------------

/// One entry of the domain-tag register. The register is the single source of
/// domain separation; its four projections (derivation context, text prefix,
/// wire role, documentation table) are generated from it and structurally
/// cannot drift. A hand-edited projection is invalid.
pub struct DomainTag {
    /* closes with the canon profile */
}
