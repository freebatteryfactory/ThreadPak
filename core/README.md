# Core

This document is the owner contract for the `core` home.
It states product law: what ThreadPak defines.
No sentence here claims current implementation support.
Root law: `ARCHITECTURE.md`.

The home's question:

> **Which semantic families are genuinely shared vocabulary — explainable without saying append, event history, query, Fix, Turn, Attempt, checkpoint, or port?**

That sentence is the admission test, and it is brutal on purpose.
A concept that needs any of those words to explain itself belongs with the owner that uses them.
Even a concept that passes the test enters only with a concrete consumer — core is shared vocabulary, not a retirement home for abstract nouns.

## Co-seated semantic families

Per `ARCHITECTURE.md` §Owners are not directories, one dependency home may seat several owner families.
Core seats eight, each separately named, none answering another's question:

| Family | Unique question |
| --- | --- |
| refusal | What shape does a typed refusal take, and what must every refusal carry? |
| logic | What are the shared truth and decision axes? |
| identity | What makes an identity lawful, and what may never be done with one? |
| value | What honesty laws bind every semantic value surface? |
| schema | What is a schema commitment, distinct from codec, layout, and bytes? |
| number | What exact numeric behavior is shared? |
| bound | What are the bound classes and the budget mechanics? |
| canon | What are canonical bytes, and how is domain separation kept honest? |

Basic time value roles are seated with `number`-adjacent value law below; chronology belongs to event, deadlines to runtime, and physical observations to port — core holds only the value roles those owners share.

**What core must never grow:** `CoreOperation`, `CoreStatus`, `CoreReceipt`, `CoreContext`, `UniversalIdentity`, `UniversalGrant`, `UniversalBudget`, or any universal envelope.
There is deliberately **no `core::authority`**: concrete grants live with their owners (event, view, port, runtime), and a shared attenuation algebra may be extracted only after at least two real grant families prove *identical* behavior — identical mechanics, not similar prose.

---

## 1. Refusal

Error **is** refusal — one vocabulary, same folds, no separate exception religion.
Every refusal family is owner-local; no package-wide mega-error exists anywhere in ThreadPak.

Every realized refusal carries four things: the violated law, the typed owner, the offending value's role, and the repair direction.
Refusal prose (the human sentences) lives in the depot keyed by refusal identity (`RefusalId`, declared here) and adds no variant and no condition.
A registered refusal identity is a semantic commitment: new meaning mints a new identity, never reuses a retired one.

A refusal family takes one of three body shapes, chosen by how its checks relate — each shape exists to kill a named lie.
The shape is chosen per family at its owner; this family owns the shape vocabulary and the anatomy law, and deliberately declares no universal refusal type.
The roster, recovered whole from the banked authoring census (`depot/core.md` §Refusal authoring shapes): **SingleCause** — exactly one mutually exclusive cause, or an explicit deterministic cause-selection rule; realized as an enum.
**IssueCollection** — a bounded, nonempty, canonical collection of independently established issues, carrying its completion or early-stop posture, with no invented primary issue; realized as a one-field struct.
**InseparablePair** — two facts jointly constituting one refusal, neither an optional secondary cause; realized as a two-field struct.
This is ThreadPak authoring law: generation machinery consumes the classification, and no public runtime shape value ever flows through the machine.

Fake totality is a defect: a refusal arm no public input can reach is a lie in the roster, not caution.

## 2. Logic

**Truth** is the shared K3 axis: `True`, `False`, `Pending`, with strong Kleene behavior.
`Pending AND False = False`; `Pending OR True = True`; `NOT Pending = Pending`.
All cells of the truth tables are authored, and consumers match exhaustively — no wildcard arm may quietly turn `Pending` into `False`.
`Pending` is an evidence statement: truth cannot yet be established from what is available.

**Decision** is the shared decision axis: `Allow`, `Deny`, `Defer(demand)`.
`Defer` names what additional admitted evidence could close the decision; the demand payload is the consumer's type (the program owner's `EvidenceRequirement` is the first), carried generically so this family depends on no upper owner.
There is **no conversion in either direction** between `Truth` and `Decision`.
`Defer` is not `Pending` and is not a refusal.

Only knowledge axes say "not yet," and each is owner-specific: `Truth::Pending` here, `OutcomeKnowledge` at runtime, commit knowledge with its owner.
No other enum grows a pending-shaped variant, and no universal status exists (`ARCHITECTURE.md` rail 13; the not-yet non-collapse table is the runtime contract's).

## 3. Identity

- Identities are role-branded: no bare integers or naked byte arrays cross a semantic surface, and equal widths never make two roles substitutable.
- A hash is a receipt over bytes, never authority over meaning.
- Readers never parse structure out of an opaque identity.
- Canonical bytes are the preimage; the identity derives from them — never the reverse.
- Identity versions are per preimage family: one family's bump must never rename identities under another, and the profile itself carries no version.
- A derived (computed) identity is admitted only where it earns a convergence consumer **and** preimage custody.
  Where no preimage exists, the design answer is fresh opaque bytes — never a parsed or composed identity.
- Declaration seat is not minting authority: where an identity type is declared says nothing about who may create one.

The settled identity-class matrix (six classes, with per-class capability rules such as scope-guarded comparison and no derived `Ord` on scoped values) is law.
The identity law is two-column (restored 2026-08-24): identity class says what kind of identity, and `IdentityCreationLaw` says how the family mints it — derived commitment, fresh opaque, or authority-assigned.
The class never implies the creation law: `TurnId` is a derived occurrence, a fresh opaque id is a fresh one, and every family's register row carries both columns.
The algebra is declared here — `IdentityClass`, `IdentityCreationLaw`, `DigestProfile`, `IdentityProfile`, `KeyScopeId` in `types.rs` — and its selected values (per-class widths, byte order, digest family, text form, per-family creation law) are the `depot/core.md` rows, ratified by the canon-packet mint (2026-08-24) or carried per row status.
Every width in every home cites its row; no owner hard-codes one independently.
Keyed derivation is scope-bound: `KeyScopeId` is declared here as a seat only — key custody, rotation, and destruction belong to the owners that hold keys.

Key-custody law (closed 2026-08-24): owned key material is never `Clone`, never serialized, never formatted or logged; copies are minimized; scope and generation are explicit; destroying a scope's key revokes the relevant live authority, and a later protected resolution over a shredded scope reports the shredded outcome honestly — never a fabricated success.
Zeroing one buffer never claims every historical copy vanished: zeroization is defense-in-depth, never proof of global erasure.
The physical mechanism (zeroizing storage and drop behavior for in-process key bytes; an opaque handle with a backend destruction contract for external custody) is selected per key-custody realization row at the manifest pass.

## 4. Value

The honesty laws every semantic value surface obeys:

- **No bools on semantic surfaces.**
  A boolean answers no semantic question; the two arms get named types or named variants.
- **No null.**
  Foreign absence is classified once at decode into a typed, owner-specific absence role and never travels as a hole.
- **Absence never means Unit, and a measured zero is not absence** — an observed zero remains a genuine measurement.
- **Owner-specific absence rosters, never one universal enum.**
  Unavailable, not-yet-observed, first-run, and withheld are different facts with different owners.
- **"Unsupported" is only ever an answer.**
  A request never pre-weakens itself to dodge a refusal.
- A function's result never claims knowledge its inputs don't carry.

## 5. Schema

A schema is a semantic commitment, distinct from the codec that encodes it, the layout that stores it, and the exact occurrence bytes of one artifact.
Rust struct layout is not a wire format, and serde-style attributes are not semantic identity.

Law: bounded validation before allocation; duplicate and ambiguous representations refuse; unknown-field policy is explicit per schema, never silent (`UnknownMemberPolicy`, four closed postures — `OpaquePreserved` is monotone extendability, never `Pending`); a removed field or variant identity (`FieldId`, `VariantId`) is never reused; schema identity and version are declared facts derived from the schema's `SchemaCommitment`, and compatibility is stated per horizon (API, persisted history, schema/codec, image, checkpoint, protocol, receipt) by the owner making the claim.
Validation is one operation (`ops.rs check_schema`) returning a `SchemaAdmission` witness or a typed refusal — the witness proves the check ran, never domain truth.

## 6. Number

Numbers are behavior, not depot data: exact arithmetic, checked operations, and explicit conversions are code seated here; precision profiles, allowed scales, rounding-mode selections, and golden vectors are depot facts.

Law: no silent overflow, wrap, or saturation — arithmetic refuses or widens by declaration; rounding, scale, units, and aggregation order are explicit; exceptional values are typed, never sentinel numbers.
Estimates, intervals-as-uncertainty, and information-loss honesty belong to the program owner's knowledge family — this family supplies the exact substrate they bind.

Shared time value roles are declared here: a signed `TimeDelta` is not an unsigned span, and neither is a deadline, a chronology value, or an observation.
A negative delta is valid evidence, never clamped.
The generic unsigned span is `core::time::Duration` (owner ruling 2026-08-24, stdlib-batteries applied): a type is not contaminated by a method (`saturating_sub`) our semantic operations never call — ThreadPak semantic code uses only checked operations on it, and owner-specific `Time`-class wrappers (horizons, ceilings, allowances) keep their roles at their owners.
No custom generic span type exists.

## 7. Bound

The two-level register, settled: **`BoundClass`** — `Work`, `Memory`, `Result`, `Output`, `Effect`, `Suspension`, `Time` — crossed with owner-specific **bound dimensions**.
The class of every bound is declared at its owner, never inferred.

Budget mechanics: an affine budget is consumed by charging and returns a smaller successor; **no widening method exists anywhere**.
A bound is affine only where duplicating it would fabricate capacity; a plain limit (a page size, a byte ceiling) is copyable and needs no linear ceremony.
The one shared charge mechanic is declared here (`Budget<D>` + `ops.rs charge` + `BudgetExhausted<D>`) — its extraction bar was met by five owner families claiming literally identical behavior — and it stays internal machinery: the public API remains each owner's named wrapper, never a universal budget.
Concrete bound types live with the operations that consume them; numeric values and paved profiles live in the depot; this family owns the classes and the mechanics only.

The register's second level is declared: `BoundDimension` — the row shape binding each registered dimension's owner, class, unit, charge law, binding time, default classification (D-DEFAULT-1), and change consequence.
The numeric value is the depot row the shape provenances.

The four axes never share a type: value (what is it), bound (how much may happen), authority (what may this actor do), evidence (why believe the result).

## 8. Canon

Canonical bytes are the identity substrate: the canonical semantic preimage is computed here, and the depot's golden vectors say what correct output is — the encoder computes, the vector testifies.

Law: one canonical representation per value role — a second representation for the same value refuses at decode; domain separation rides the domain-tag register (`DomainTag`: tag version, digest family, preimage family, role, role version — the final segment is the role's own version, never a universal schema version), whose projections (derivation context, text prefix, wire role, documentation table) are generated from the one register and structurally cannot drift; the digest family rides **inside** the tag, so every identity names its algorithm by construction and no ambient "which algorithm?" question exists; binary forms are for the store, text forms exist only at the human boundary and cover their role; mechanism selections (hash function, text encoding, endianness, widths, realization profiles) are profile facts bound by this family and cited from the `depot/core.md` rows — not restated per call site.
Derivation is per-role under a registered tag through `ops.rs` (`admit_preimage`, `derive_digest`, `derive_keyed_digest`, `derive_key_context`); there is deliberately no public "hash these bytes" API, and keyed fingerprints are always scope-bound — never public unkeyed digests of low-entropy input.

---

## Bounds

Core declares no owner-local limits of its own: bound *mechanics* live here; every concrete limit lives with the operation that consumes it.
If a core family ever needs a limit, it declares it like any other owner.

## Crossings

Per `ARCHITECTURE.md` §No orphan by distribution — fact, owner, establishing operation, carrier, substitution refusal, chronology:

1. **Defer's demand payload.**
   Fact: what evidence could close a decision.
   Owner: the deferring consumer (first: program knowledge's `EvidenceRequirement`).
   Operation: the consumer's evaluation.
   Carrier: `Decision::Defer` generically.
   Refusal: the demand performs nothing and grants nothing; core never names consumer types.
   Chronology: carries the knowledge-seat ruling (2026-08-24).
2. **Not-yet non-collapse.**
   Fact: the many distinct "not yet" states.
   Owner: each state's owner (runtime holds the table).
   Operation: each owner's declarations.
   Carrier: this family supplies only `Truth::Pending` and `Defer`.
   Refusal: no generic pending status exists.
   Chronology: carries `ARCHITECTURE.md` rail 13.
3. **Grants.**
   Fact: role-specific authority.
   Owner: event, view, port, runtime — never core.
   Operation: each owner's installation.
   Carrier: none here.
   Refusal: no `core::authority` exists until two real grant families prove identical behavior.
   Chronology: carries the owner-endorsed deletion of core/authority (2026-08-24).
4. **Refusal prose, golden vectors, tag projections, precision profiles.**
   Fact: selected data.
   Owner: the declaring family here; the depot projects.
   Operation: the owner's declaration; depot rows regenerate.
   Refusal: a depot row adds no variant, condition, or meaning; hand-edited projections are invalid.
   Chronology: carries the depot contract.
5. **Identity profile.**
   Fact: exact widths, preimage grammar, the six-class roster, and the per-family creation law.
   Owner: the identity family — the algebra in `types.rs`, the selected values as `depot/core.md` rows.
   Operation: this contract's declarations plus the owner's row acts.
   Carrier: every other home cites its row, never mirrors it.
   Refusal: no other file restates the matrix or hard-codes a width.
   Chronology: carries the settled identity-matrix rulings, the owner's seed ruling, and the canon-packet mint (all 2026-08-24); remaining candidate/withheld rows stay per row status, machining is owner-derived.

## Hostile denominator

Each must be unrepresentable or refuse with a typed result:

1. A bool on a semantic surface; a null or sentinel crossing a decode boundary unclassified.
2. A non-exhaustive match quietly turning `Pending` into `False`; any `Truth` ↔ `Decision` conversion.
3. A bare integer or naked byte array used as an identity; structure parsed out of an opaque identity; a hash treated as authority.
4. A derived `Ord`, `Hash`, or comparison on a scope-bound value outside its proven scope.
5. A budget acquiring a widening method, `Clone`, or `Copy`; a limit's class inferred instead of declared.
6. Unchecked arithmetic; silent rounding; a sentinel number standing for an exceptional value.
7. A second lawful byte representation for one value role; a hand-edited register projection.
8. An unknown field silently accepted, or a removed field identity reused.
9. Any universal status, receipt, grant, budget, or envelope appearing in this home.
10. `core::authority` resurrected without two proven-identical grant families.

## Escalations

Open seams recorded, not closed — the repository owner's calls:

1. **Withheld depot rows stay withheld.**
   The `depot/core.md` withheld rows (public-digest entropy floor, KDF parameters, Unicode identifier pin, numeric authority families, schema shape axes) record unmade decisions; each closes with its consumer or an owner statement.
2. **Key-custody mechanism.**
   The semantic custody law is closed (§3); the physical mechanism selection (zeroizing storage / `zeroize` feature composition per realization profile) lands with the manifest pass — recorded as the `zeroize-mechanism` withheld row.

Owner-derived machining recorded: `IdentityClass` and `IdentityCreationLaw` variant spellings are draft (classification law settled, names revisable); `PreimageFamilyId` and `RoleVersion` are draft spellings forced by the minted tag grammar; the exact `Decision` demand-payload spelling (generic parameter versus another shape) is a draft-spelling note, not law.

Closed by the 2026-08-24 mints: the canon packet ratified with four retouches (role-version tag segment; per-closed-registry u16; `ThreadPakFrameV1`-scoped `TPAK`; restored `IdentityCreationLaw` column); the `Duration` collision resolved to `core::time::Duration`; the three refusal shapes recovered from the banked census; the zeroize semantic custody law closed with the mechanism deferred to manifest.
