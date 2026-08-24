# depot — core rows (identity, canon, shared substrates)

Selected facts consumed by the core owner's profile algebra (`core/types.rs`
`IdentityProfile`, `CanonProfile`, `DomainTag`, `Budget`). Rows are
selections-as-data consumed through explicit typed profile arguments — a row
swap changes behavior with zero code edits, and no operation ever fetches a
row (`depot/README.md`, "Rows are passed, never fetched").

Status is per row: **ratified** (the canon-packet mint, 2026-08-24),
**candidate** (recovered, awaiting an owner act), **qualification**
(evidence-selected mechanism — selection is evidence, never taste),
**withheld** (decision recorded as unmade — the depot never invents the
selection). Contradictory recovered values are preserved as separate rows,
never averaged.

Authority classes: `owner-ruled` (the repository owner's word), `recovered`
(archived selection under the seed ruling), `evidence` (matches an
already-committed declaration or a live mechanism fact), `withheld`
(decision recorded as unmade).

## Identity and canon selections

| Row | Value | Source | Authority | Status | Binding | Consequence | Consumers | Nonclaim / Falsifier |
|---|---|---|---|---|---|---|---|---|
| `digest-family` | blake3-256 | ch10 "day-one candidate family is blake3-256"; owner blake3-global ruling 2026-08-24 | owner-ruled + recovered | **ratified (2026-08-24)** | artifact | new canonical profile identity | every commitment/identity derivation (`ops.rs DeriveDigestFn`) | a digest proves only its preimage's byte role; family swap = re-digest morphism with receipts, never reinterpretation |
| `digest-width-class-ab` | 32 bytes | ch10 "Class A/B digest width is 32 bytes"; matches committed `[u8;32]` fields in program | recovered + evidence | **ratified (2026-08-24)** | artifact | new profile identity | `Digest`, `SchemaId`, program/event commitments | equal width never makes roles substitutable; truncation = new registered role, falsifier: an ad-hoc truncated digest sharing a tag |
| `fresh-opaque-width-class-d` | 16 bytes | ch10 / Part VII "a reader of a generated-opaque id sees 16 opaque bytes" | recovered | **ratified (2026-08-24)** | artifact | new identity family | fresh-opaque occurrence-class identities across owners | a reader parses no structure; falsifier: structure parsed from the 16 bytes |
| `identity-creation-law-column` | every identity family's register row carries two independent columns — `IdentityClass` and `IdentityCreationLaw` (derived-commitment / fresh-opaque / authority-assigned); class never implies creation law. A derived occurrence derives through its registered digest role (32-byte class); a fresh occurrence is 16 opaque bytes | archived two-column identity law, restored at the mint | owner-ruled + recovered | **ratified (2026-08-24)** | artifact | new canonical profile identity | `IdentityProfile`, every identity family's register row | creation law says how a value is minted, never what the identity proves; falsifier: a family whose class is read as implying its minting (e.g. "Occurrence therefore fresh") |
| `class-c-order-scalar` | u64 + scope binding in canonical bytes | ch10 "one u64-plus-scope layout, the roles distinguished by domain tag" | recovered | **ratified (2026-08-24)** | artifact | compatibility change | `AuthoritySequence`, `CommitPoint`, version/generation ordinals | no cross-scope comparison; falsifier: two scopes' ordinals compared as integers |
| `byte-order` | big-endian, uniform | ch10 "multi-byte integers are big-endian" | recovered | **ratified (2026-08-24)** | artifact | new persisted-format version | canon encoders | host endianness is never a wire fact |
| `integer-posture` | fixed-width, no varints | ch10 | recovered | **ratified (2026-08-24)** | artifact | new persisted-format version | canon encoders, frame grammar | falsifier: a varint accepted where the row says fixed |
| `domain-tag-grammar` | `threadpak/<tag-version>/<digest-family>/<family>/<role>/<role-version>` | ch10 tag form, retouched at the mint: the final segment is the role's own version, never a universal schema version — a schema version lives inside a schema-family preimage where one exists; Turn, Attempt, and checkpoint families have no schema | owner-ruled + recovered | **ratified as retouched (2026-08-24)** | artifact | new canonical profile identity | `DomainTag`, `DeriveKeyContextFn` | digest family rides inside the tag — no ambient algorithm question; falsifier: a hand-authored context string |
| `tag-projections` | four generated: derive-key context, text prefix, wire role, docs table | ch10 "one register, four projections" (generator role = Macroonz) | recovered | **ratified (2026-08-24)** | artifact | requalification only | register tooling | a hand-edited projection is invalid |
| `registered-id-width` | u16 **per closed wire registry** (domain-tag register, operation discriminants, closed enum wire roles) — never a global Rust identity width; Rust interiors stay opaque newtypes | ch10 "registered ids u16", scoped at the mint | owner-ruled + recovered | **ratified as scoped (2026-08-24)** — prior u16-vs-u32 contradiction **disposed**: the earlier u32 was an in-memory representation answer, the u16 a wire-discriminant answer; different questions compose | artifact | new persisted-format version | `TagRoleId`, `DigestFamilyId`, `PreimageFamilyId`, wire enums | unknown registered id refuses, never defaults; a registry needing a wider ceiling declares its own width row |
| `enum-wire-encoding` | registered u16, refuse-on-unknown | ch10 "enums registered-u16 refuse-on-unknown" | recovered | **ratified (2026-08-24)** | artifact | new persisted-format version | every closed enum crossing canonical bytes | an unknown discriminant is a refusal, never a default arm |
| `text-form` | role prefix + case-insensitive base32 + strict role-covering bech32m; mixed case refuses | ch10 "One text-form scheme" | recovered | **ratified (2026-08-24)** | artifact | new canonical profile identity | human identity rendering (tag text-prefix projection) | text form covers its role — a string minted for one role cannot validate as another; falsifier: mixed-case string accepted |
| `frame-magic` | 4 B ASCII `TPAK` — the magic of the first shared frame profile `ThreadPakFrameV1`. A byte family adopts the frame when its owner proves it serves that family — never a law over every binary file (event logs, images, DataBlocks, and checkpoints may earn different framing) | ch10 frame grammar, scoped at the mint | owner-ruled + recovered | **ratified as scoped (2026-08-24)** | artifact | new persisted-format version | `ThreadPakFrameV1` adopters (future storage/framing pass) | a suffix or magic grants no meaning, authenticity, or support |
| `frame-role-width` / `frame-profile-version-width` | u16 / u16 | ch10 frame grammar | recovered | **ratified within `ThreadPakFrameV1` (2026-08-24)** | artifact | new persisted-format version | `ThreadPakFrameV1` grammar | unknown role or version is a typed refusal |
| `non-full-width-outputs` | distinct registered roles via XOF, never truncations | ch10 | recovered | **ratified (2026-08-24)** | artifact | new profile identity | XOF-width roles | truncation changes the collision claim, so it must change the name |
| `keyed-digest-key` | 32 bytes | blake3 keyed mode takes exactly a 32-byte key — mechanism-forced (batteries report, crate 1.8.7) | evidence | **ratified with the packet (2026-08-24)** | artifact | new canonical profile identity | `ScopedKey`, `DeriveKeyedDigestFn` | the width is a mechanism fact of the keyed mode, never by itself a strength claim; falsifier: key material cloned, logged, serialized, or outliving its scope |

## blake3 modes and realization profiles

| Row | Value | Source | Authority | Status | Binding | Consequence | Consumers | Nonclaim / Falsifier |
|---|---|---|---|---|---|---|---|---|
| `blake3-mode-per-role` | `hash` = public commitments; `keyed_hash` = keyed scope-bound fingerprints (KeyScope-bound, never public unkeyed digests of low-entropy input); `derive_key` = KDF under the tag's generated context; XOF = non-32-byte registered roles | ch10 + cleanroom delta + settled ingress oracle law | owner-ruled + recovered | **ratified (2026-08-24)** | artifact | new canonical profile identity | `DeriveDigestFn`, `DeriveKeyedDigestFn`, `DeriveKeyContextFn` | blake3 is never password hashing, encryption, random identity minting, semantic idempotency identity, ordering, canonical encoding, or a human-copy checksum |
| `blake3-realization-native-dispatch` | optimized C/asm + runtime CPU dispatch (crate default sans `pure`) | batteries report (crate 1.8.7) | evidence | **qualification — paved native profile**; selection is evidence, never taste | deployment | requalification only | native builds | C/asm + `cc` build-dep enter the TCB; identical outputs against the golden vectors required |
| `blake3-realization-portable-rust` | `pure` feature — single portable Rust routine | batteries report | evidence | **qualification — strict first-party / reference profile** | deployment | requalification only | reference road, strict rows | slower; same outputs, same vectors |
| `blake3-realization-wasm-portable` / `-wasm-simd` | portable wasm path / `wasm32_simd` feature | batteries report | evidence | **qualification** | deployment | requalification only | wasm target | an optimized road agrees with the reference road and never acquires authority by being fast (rail 12); realization swap = requalification, never a new identity |
| `blake3-feature-reconciliation` | manifest must declare `default-features = false` (deny row `allow = []` forbids default `std`); per-realization-profile feature rows replace the flat empty set at manifest time | batteries report vs `deny.toml` | evidence | **qualification — resolves at the dependency probe / manifest pass** | deployment | requalification only | future `Cargo.toml` | the current flat ban simultaneously forbids `pure`, making C/asm dispatch the un-chosen default — preserved as the tension the profile rows resolve |
| `blake3-wrapper-ownership` | one canon-owner wrapper; zero direct `blake3::` calls outside it; no public generic digest API (`digest`/`traits-preview` stay off) | owner blake3-global ruling + DRY/SSOT law | owner-ruled | **ratified (2026-08-24)** | artifact | requalification only | all derivation ops | a hash function reaches code as a profile fact, never a type parameter |

## Refusal authoring shapes — recovered roster, closed from the banked census

ThreadPak authoring law: refusals are ThreadPak's product. Generic
generation machinery (Macroonz) **consumes** this classification to generate
constructors, diagnostics, inspection, and refusal fixtures — it never owns
it, and no public runtime shape value ever flows through the machine. The
census counts below describe the archived population and are carried as
provenance only, never as claims about this repository.

| Row | Value | Status | Source |
|---|---|---|---|
| `refusal-shape-single-cause` | `SingleCause` — exactly one mutually exclusive cause, or an explicit deterministic cause-selection rule; realized as an enum | recovered roster member | banked authoring-denominator census (34 families, all enums) |
| `refusal-shape-issue-collection` | `IssueCollection` — a bounded, nonempty, canonical collection of independently established issues, carrying its completion or early-stop posture; no invented primary issue; realized as a one-field struct | recovered roster member | banked census (27 families, all one-field structs) |
| `refusal-shape-inseparable-pair` | `InseparablePair` — two facts jointly constituting one refusal, neither an optional secondary cause; realized as a two-field struct | recovered roster member | banked census (1 family: `LineageRefusal`) |

The shape ↔ Rust-kind correspondence was total in the census — nothing in
the archived population contradicted it.

## Shared substrates

| Row | Value | Source | Authority | Status | Binding | Consequence | Consumers | Nonclaim / Falsifier |
|---|---|---|---|---|---|---|---|---|
| `budget-magnitude` | u64 | five identical committed claimants (`remaining: u64` ×3, `NonZeroU64` ×2) | evidence | candidate | artifact | requalification only | `Budget<D>` mechanic | budgets only shrink; falsifier: any widening route |
| `time-substrate` | wall delta i128 signed (`TimeDelta`); monotonic reading u128 unsigned (port observation substrate) | committed port observation substrate (91514f6) | evidence | candidate | artifact | new persisted-format version | `TimeDelta`; port `RawMonotonicObservation` | a negative delta is evidence, never clamped. Unsigned semantic spans are `core::time::Duration` (owner ruling 2026-08-24): its representation is std's own, not a depot selection, and semantic code uses only checked operations on it |

## Withheld rows — recorded as unmade, the depot never invents these

| Row | What is withheld | Source |
|---|---|---|
| `zeroize-mechanism` | Which physical key-erasure mechanism realizes the closed custody law (`core/README.md` §3): zeroizing storage and drop behavior per in-process realization profile; opaque handle + backend destruction contract for external custody. The `zeroize` crate/feature composition selects at the manifest/profile pass (the deny row currently forbids the feature — the tension the manifest rows resolve). The custody **law** is closed; only the mechanism selection is withheld. | batteries report; port/event shred law; owner mint 2026-08-24 |
| `public-digest-entropy-floor` | The numeric entropy floor below which a public digest of protected plaintext is prohibited ("low-entropy" never quantified). | ch13 Part V |
| `kdf-parameters` | Key-derivation parameters beyond the tag context (lifetime, rotation cadence). | ch13 Part V |
| `identifier-unicode-pin` | Unicode 17.0.0 XID identifier profile — recovered, but its consumer (a text-admission surface) does not exist yet; enters with that consumer. | ch09 Part II |
| `numeric-authority-families` | The nine exact authority-capable numeric families (`ExactInteger` … `TypedMargin`), each a closed single-cause constructor family, and the six-operator interval comparison returning `Truth`. Recovered algebra; each family enters with its first consumer. | ch06 (archive) |
| `schema-shape-axes` | `FieldCardinality` (3) / `Nullability` (2) / `DefaultPolicy` (2) rosters beside the minted `UnknownMemberPolicy` (4); enter with the schema-profile pass. | ch10 Act I/V |

## Preserved contradictions — the owner map decides, never averaging

| Contradiction | Rows | Disposition status |
|---|---|---|
| Registered-id width | ch10 says u16; port had declared u32 ordinals | **disposed (2026-08-24)** — different questions compose: u32 was an in-memory representation answer, u16 the per-closed-registry wire width; Rust interiors stay opaque newtypes |
| Frame length widths | general frame length u32 vs some component lengths u64 | open — per-row consumer rationale required |
| Bound classes five vs seven | book Law 6 five; archive seven; direction bank "no ontology" | **disposed** — seven-class register + owner-named types + no ministry (nine-owners ruling 2026-08-24, later ruling wins) |
| `CommitKnowledge` | book's three knowledge axes include a three-valued commit-knowledge axis; the new repo declares two (`Truth`, `OutcomeKnowledge`) | open — seats with the event storage pass if its consumer materializes |
| Wire bound widths | `NavigationDepth(NonZeroU32)` vs recovered wire field u16 | open — wire width is codec-profile, in-memory bound is owner law; likely composes |
