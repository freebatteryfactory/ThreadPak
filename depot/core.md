# depot — core rows (identity, canon, shared substrates)

Selected facts consumed by the core owner's profile algebra (`core/types.rs`
`IdentityProfile`, `CanonProfile`, `DomainTag`, `Budget`). Every row is
**CANDIDATE** unless its Status says otherwise: recovered from the archived
corpus under the seed ruling, carried here with provenance for owner
ratification. A **withheld** row records an unmade decision — the depot never
invents the selection. Contradictory recovered values are preserved as
separate rows, never averaged.

Authority classes: `owner-ruled` (the repository owner's word), `recovered`
(archived selection under the seed ruling), `evidence` (matches an
already-committed declaration), `withheld` (decision recorded as unmade).

## Identity and canon selections

| Row | Value | Source | Authority | Status | Binding | Consequence | Consumers | Nonclaim / Falsifier |
|---|---|---|---|---|---|---|---|---|
| `digest-family` | blake3-256 | ch10 "day-one candidate family is blake3-256"; owner blake3-global ruling 2026-08-24 | owner-ruled + recovered | candidate | artifact | new canonical profile identity | every commitment/identity derivation (`ops.rs derive_digest`) | a digest proves only its preimage's byte role; family swap = re-digest morphism with receipts, never reinterpretation |
| `digest-width-class-ab` | 32 bytes | ch10 "Class A/B digest width is 32 bytes"; matches committed `[u8;32]` fields in program | recovered + evidence | candidate | artifact | new profile identity | `Digest`, `SchemaId`, program/event commitments | equal width never makes roles substitutable; truncation = new registered role, falsifier: an ad-hoc truncated digest sharing a tag |
| `fresh-opaque-width-class-d` | 16 bytes | ch10 / Part VII "a reader of a generated-opaque id sees 16 opaque bytes" | recovered | candidate | artifact | new identity family | occurrence-class identities across owners | a reader parses no structure; falsifier: structure parsed from the 16 bytes |
| `class-c-order-scalar` | u64 + scope binding in canonical bytes | ch10 "one u64-plus-scope layout, the roles distinguished by domain tag" | recovered | candidate | artifact | compatibility change | `AuthoritySequence`, `CommitPoint`, version/generation ordinals | no cross-scope comparison; falsifier: two scopes' ordinals compared as integers |
| `byte-order` | big-endian, uniform | ch10 "multi-byte integers are big-endian" | recovered | candidate | artifact | new persisted-format version | canon encoders | host endianness is never a wire fact |
| `integer-posture` | fixed-width, no varints | ch10 | recovered | candidate | artifact | new persisted-format version | canon encoders, frame grammar | falsifier: a varint accepted where the row says fixed |
| `domain-tag-grammar` | `threadpak/<tag-version>/<family>/<role>/<schema-version>` | ch10 tag form | recovered | candidate | artifact | new canonical profile identity | `DomainTag`, `derive_key_context` | family rides inside the tag — no ambient algorithm question; falsifier: a hand-authored context string |
| `tag-projections` | four generated: derive-key context, text prefix, wire role, docs table | ch10 "one register, four projections" (generator role = Macroonz) | recovered | candidate | artifact | requalification only | register tooling | a hand-edited projection is invalid |
| `registered-id-width` | u16 | ch10 "registered ids u16" | recovered | candidate — **contradiction preserved**: port previously declared u32 ordinals | artifact | new persisted-format version | `TagRoleId`, `DigestFamilyId`, wire enums | unknown registered id refuses, never defaults |
| `enum-wire-encoding` | registered u16, refuse-on-unknown | ch10 "enums registered-u16 refuse-on-unknown" | recovered | candidate | artifact | new persisted-format version | every closed enum crossing canonical bytes | an unknown discriminant is a refusal, never a default arm |
| `text-form` | role prefix + case-insensitive base32 + strict role-covering bech32m; mixed case refuses | ch10 "One text-form scheme" | recovered | candidate | artifact | new canonical profile identity | human identity rendering (tag text-prefix projection) | text form covers its role — a string minted for one role cannot validate as another; falsifier: mixed-case string accepted |
| `frame-magic` | 4 B ASCII `TPAK`, one magic for every binary file | ch10 frame grammar | recovered | candidate | artifact | new persisted-format version | frame grammar (future storage/framing pass) | a suffix or magic grants no meaning, authenticity, or support |
| `frame-role-width` / `frame-profile-version-width` | u16 / u16 | ch10 frame grammar | recovered | candidate | artifact | new persisted-format version | frame grammar | unknown role or version is a typed refusal |
| `non-full-width-outputs` | distinct registered roles via XOF, never truncations | ch10 | recovered | candidate | artifact | new profile identity | XOF-width roles | truncation changes the collision claim, so it must change the name |

## blake3 modes and realization profiles

| Row | Value | Source | Authority | Status | Binding | Consequence | Consumers | Nonclaim / Falsifier |
|---|---|---|---|---|---|---|---|---|
| `blake3-mode-per-role` | `hash` = public commitments; `keyed_hash` = keyed scope-bound fingerprints (KeyScope-bound, never public unkeyed digests of low-entropy input); `derive_key` = KDF under the tag's generated context; XOF = non-32-byte registered roles | ch10 + cleanroom delta + settled ingress oracle law | recovered | candidate | artifact | new canonical profile identity | `derive_digest`, `derive_keyed_digest`, `derive_key_context` | blake3 is never password hashing, encryption, random identity minting, semantic idempotency identity, ordering, canonical encoding, or a human-copy checksum |
| `blake3-realization-native-dispatch` | optimized C/asm + runtime CPU dispatch (crate default sans `pure`) | batteries report (crate 1.8.7) | evidence | candidate — paved native profile | deployment | requalification only | native builds | C/asm + `cc` build-dep enter the TCB; identical outputs against the golden vectors required |
| `blake3-realization-portable-rust` | `pure` feature — single portable Rust routine | batteries report | evidence | candidate — strict first-party / reference profile | deployment | requalification only | reference road, strict rows | slower; same outputs, same vectors |
| `blake3-realization-wasm-portable` / `-wasm-simd` | portable wasm path / `wasm32_simd` feature | batteries report | evidence | candidate | deployment | requalification only | wasm target | an optimized road agrees with the reference road and never acquires authority by being fast (rail 12) |
| `blake3-feature-reconciliation` | manifest must declare `default-features = false` (deny row `allow = []` forbids default `std`); per-realization-profile feature rows replace the flat empty set at manifest time | batteries report vs `deny.toml` | evidence | candidate — resolves at the dependency probe | deployment | requalification only | future `Cargo.toml` | the current flat ban simultaneously forbids `pure`, making C/asm dispatch the un-chosen default — preserved as the tension the profile rows resolve |
| `blake3-wrapper-ownership` | one canon-owner wrapper; zero direct `blake3::` calls outside it; no public generic digest API (`digest`/`traits-preview` stay off) | owner blake3-global ruling + DRY/SSOT law | owner-ruled | candidate | artifact | requalification only | all derivation ops | a hash function reaches code as a profile fact, never a type parameter |

## Shared substrates

| Row | Value | Source | Authority | Status | Binding | Consequence | Consumers | Nonclaim / Falsifier |
|---|---|---|---|---|---|---|---|---|
| `budget-magnitude` | u64 | five identical committed claimants (`remaining: u64` ×3, `NonZeroU64` ×2) | evidence | candidate | artifact | requalification only | `Budget<D>` mechanic | budgets only shrink; falsifier: any widening route |
| `time-substrate` | wall delta i128 signed / monotonic span u128 unsigned | committed port observation substrate (91514f6) | evidence | candidate | artifact | new persisted-format version | `TimeDelta`, `Duration` | a negative delta is evidence, never clamped; std `Duration`'s zero-saturating subtraction is exactly the forbidden behavior |

## Withheld rows — recorded as unmade, the depot never invents these

| Row | What is withheld | Source |
|---|---|---|
| `refusal-shape-roster` | The three refusal body shapes' names and definitions ("three refusal family shapes" carried forward with no roster). Needs one full read of the archived results/evidence chapter or an owner statement before the guard pass. | direction bank ~L11482 |
| `zeroize-posture` | Whether keyed-material memory zeroization enters the canon profile (key-shred custody claims need a memory story; the deny row currently forbids the `zeroize` feature). | batteries report; port/event shred law |
| `public-digest-entropy-floor` | The numeric entropy floor below which a public digest of protected plaintext is prohibited ("low-entropy" never quantified). | ch13 Part V |
| `kdf-parameters` | Key-derivation parameters beyond the tag context (lifetime, rotation cadence). | ch13 Part V |
| `identifier-unicode-pin` | Unicode 17.0.0 XID identifier profile — recovered, but its consumer (a text-admission surface) does not exist yet; enters with that consumer. | ch09 Part II |
| `numeric-authority-families` | The nine exact authority-capable numeric families (`ExactInteger` … `TypedMargin`), each a closed single-cause constructor family, and the six-operator interval comparison returning `Truth`. Recovered algebra; each family enters with its first consumer. | ch06 (archive) |
| `schema-shape-axes` | `FieldCardinality` (3) / `Nullability` (2) / `DefaultPolicy` (2) rosters beside the minted `UnknownMemberPolicy` (4); enter with the schema-profile pass. | ch10 Act I/V |

## Preserved contradictions — the owner map decides, never averaging

| Contradiction | Rows | Disposition status |
|---|---|---|
| Registered-id width | ch10 says u16; port had declared u32 ordinals | open — resolves in the identity-profile ratification |
| Frame length widths | general frame length u32 vs some component lengths u64 | open — per-row consumer rationale required |
| Bound classes five vs seven | book Law 6 five; archive seven; direction bank "no ontology" | **disposed** — seven-class register + owner-named types + no ministry (nine-owners ruling 2026-08-24, later ruling wins) |
| `CommitKnowledge` | book's three knowledge axes include a three-valued commit-knowledge axis; the new repo declares two (`Truth`, `OutcomeKnowledge`) | open — seats with the event storage pass if its consumer materializes |
| Wire bound widths | `NavigationDepth(NonZeroU32)` vs recovered wire field u16 | open — wire width is codec-profile, in-memory bound is owner law; likely composes |
