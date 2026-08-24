# Depot — event owner rows

Selected and recovered facts consumed by the event owner's operations
(`event/ops.rs`), per the depot row law (`depot/README.md`): rows are passed
into operations as fields of the profiles in `event/types.rs`, never fetched.
A row records a selection or its deliberate absence; a **withheld** row means
the owner has not selected a value and nothing may invent one. Contradictory
recovered values are preserved as separate rows, never averaged.

Authority classes below: **ruling** (owner-ruled law), **recovered** (mined
from the archived corpus with provenance), **owner-derived** (forced by
declared law, value open).

## Admission (profile: `EventAdmissionProfile`)

| Row | Consumed as | Value | Status | Binding | Change consequence | Source / provenance |
| --- | --- | --- | --- | --- | --- | --- |
| event.max-event-bytes | `EventByteLimit` | — | withheld | deployment | refusal/default change; no old-history rewrite | Book withheld `MAX_EVENT_BYTES` deliberately; ch10 frames it as the semantic capacity profile checked before decode ("anything larger is a content region by construction"). Classification: asymmetric (paved default + lawful override) once selected. |
| event.frame-length-field-width | wire grammar (canon packet) | u32 | candidate | artifact | new persisted-format version | ch10 shared frame grammar: "`length (u32)` … the u32 length is the **physical** bound; the semantic bound is the capacity profile." Preserved contradiction: `EventByteLimit` is declared `NonZeroU64` — wider than the physical field. The canon profile pass reconciles; neither number is averaged. |
| event.max-batch-events | `BatchEventLimit` | — | withheld | deployment | refusal/default change | Book bounds batches ("bounded accepted batch, one contiguous membership") and names no number. Group commit requires stable idempotency identity per member before grouping (ch07). |
| event.causal-parent-limit | `CausalParentLimit` | — | withheld | deployment | refusal change; beyond-bound refusal names the external relation extent (owner-ruled 2026-08-24) | Book: "a finite fan-in bound … `[deferred: exact fan-in bound value — evidence/config-selected]`" — an explicitly recorded unmade decision. |
| event.unresolved-claim-limit | `UnresolvedCausalClaimLimit` | — | withheld | deployment | refusal change | Owner-derived: admitted rosters are bounded; no book number exists. |
| event.federation-source-limit | `FederationSourceLimit` | — | withheld · cap ruled kept (owner 2026-08-24) | deployment | refusal change | The cap-versus-no-cap fork is disposed: the cap stays as a typed admission bound — every actual operation is finitely bounded, and no eternal product-wide source count is implied. The numeric value remains withheld, profile-selected. Provenance preserved: the book states no cap anywhere ("one exact durable cut per store and never invents a global commit sequence" — cardinality law, no cap). |
| event.recovery-scan-budget | `RecoveryScanBudget` (`RecoveryProfile`) | — | withheld | deployment | recovery-behavior change | Book bounds recovery work, names no number. Recovery boundary law (committed-boundary-bounded) is ratified separately below. |

Family nonclaims: a limit value is never the bound's meaning; a value outside
the declared type refuses at construction. Family falsifiers: an oversize
admission accepted; a bound checked only after allocation ("a bound checked
after a collection grew is theater" — ch07); unknown size not charged at the
strict maximum.

Ratified admission-side law rows (safety-relevant classification):

| Row | Law | Status | Source |
| --- | --- | --- | --- |
| event.unknown-size-charging | Unknown size charges the strict maximum or refuses. | ratified · safety-relevant | ch07 Part VI, fail-closed metering default. |
| event.allocation-order | validate → charge against the capacity profile → reserve fallibly → only then admit data. Behavior seated with the operations; recorded here as the charging discipline every byte row is consumed under. | ratified | ch07 Part VI. |

## Chronology (policy: `ChronologyPolicy`)

| Row | Consumed as | Value | Status | Binding | Change consequence | Source / provenance |
| --- | --- | --- | --- | --- | --- | --- |
| event.hlc-component-widths | `SourceHlc` / `AcceptedHlc` interiors | physical u64 · logical u32 | **ratified** (owner 2026-08-24) | artifact (history profile) | new canonical/identity profile; persisted-history compatibility change | ch10 Time: "{physical u64, logical u32}"; rationale: "logical overflow refuses (u32 is the smallest width whose overflow can only mean broken clock physics, never legitimate load)." Contradiction disposed by the ruling: the direction bank withheld the widths at every occurrence ("Exact HLC component widths … require a bounded decision"); the owner ratified the ch10 pair as chronology encoding profile V1. These bytes become canonical only when every companion row below binds. |
| event.hlc-physical-epoch | chronology profile V1 | — | withheld | artifact | new canonical/identity profile | Companion of the ratified widths: the epoch the physical component counts from. No value stated anywhere; nothing may invent one. |
| event.hlc-physical-unit | chronology profile V1 | — | withheld | artifact | new canonical/identity profile | Companion of the ratified widths: the unit/resolution of the physical component. No value stated anywhere. |
| event.hlc-wall-interpretation | chronology profile V1 | — | withheld | artifact | new canonical/identity profile | Companion of the ratified widths: how an admitted wall-observation enclosure maps into the physical component (which bound, under which posture). Closes with the chronology-profile pass. |
| event.hlc-logical-increment | chronology profile V1 | — | withheld | artifact | new canonical/identity profile | Companion of the ratified widths: the logical-counter increment rule. Overflow behavior is already ratified law (`event.overflow-posture`); the increment rule itself is unstated. |
| event.chronology-summary-counter-width | `ChronologySummary` interior | u32 | candidate | artifact | same as widths row | ch10 Time: "counters u32; overflow refuses, never wraps." |
| event.skew-ceiling | `SkewCeiling` | — | withheld | deployment | refusal change | Named at every occurrence ("maximum accepted drift / skew ceiling"), number withheld everywhere (book and direction bank). |
| event.regression-posture | `ChronologyPolicy` roster | — | withheld | deployment | refusal change | Roster closes with the chronology-profile pass. |
| event.future-posture | `ChronologyPolicy` roster | — | withheld | deployment | refusal change | Excessive-future values are preserved and classified, never clamped (ratified law); the classification roster closes with the profile pass. |
| event.overflow-posture | `ChronologyRefusal::LogicalCounterOverflow` | typed refusal — no wrap, no saturation, no clamping, prior accepted state intact | ratified · safety-relevant | artifact | none (law) | Direction bank: "Overflow does not justify inventing chronology." |
| event.merge-refusal-roster | `ChronologyMergeRefusal` | single cause: `ProfileMismatch` | candidate (seated) | artifact | new profile identity | Archive ch04: merge totality clause is the roster — "total over validated same-profile summaries names exactly one guard"; no overflow cause exists for componentwise max. Seated as its own family, distinct from admission refusals. |

Family nonclaims: chronology proves no causation, order, completeness, or
checkpoint progress. Family falsifiers: wraparound on overflow; clamped
future values; a merge consulting wall time; cross-profile merge passing.

## Reservation and ingress (profile: `ReservationProfile`)

All numeric values below are **withheld**: the reservation family was closed
as law in the ingress ruling (2026-08-24) with every tunable left marked
open. Binding time deployment; change consequence refusal/default change;
consumers `reserve_ingress` and `admit_claim`.

| Row | Consumed as | Status |
| --- | --- | --- |
| event.reservation-count | `ReservationCountLimit` | withheld |
| event.reservation-bytes | `ReservationByteLimit` | withheld |
| event.reservations-per-principal | `ReservationsPerPrincipalLimit` | withheld |
| event.reservations-per-tenant | `ReservationsPerTenantLimit` | withheld |
| event.reservations-per-operation | `ReservationsPerOperationLimit` | withheld |
| event.reservation-creation-rate | `ReservationCreationRateLimit` | withheld |
| event.reservation-lookup-work | `ReservationLookupWorkBudget` | withheld |
| event.reservation-create-work | `ReservationCreateWorkBudget` | withheld |
| event.client-nonce-bytes | `ClientNonceByteLimit` | withheld |
| event.reservation-token-bytes | `ReservationTokenByteLimit` | withheld |
| event.active-reservation-age | `ActiveReservationAgeLimit` | withheld |
| event.conflict-evidence | `ConflictEvidenceLimit` | withheld |
| event.token-usability-horizon | `TokenUsabilityHorizon` | withheld |
| event.duplicate-recognition-horizon | `DuplicateRecognitionHorizon` | withheld |

Ratified ingress law rows (safety-relevant classification):

| Row | Law | Source |
| --- | --- | --- |
| event.horizon-ordering | The duplicate-recognition horizon never closes before the token-usability horizon; guarded construction refuses the inversion. | Owner-ruled lifecycle closure 2026-08-24. |
| event.no-eviction-into-duplicate | Capacity exhaustion is a typed refusal; an unexpired reservation is never evicted into an ambiguous duplicate. | Owner-ruled 2026-08-24. |
| event.ingress-queue-arity | Ingress queue capacity is three-dimensional — items AND bytes AND weight — charged from validated data; deceptive size reports refuse. | ch11 ingress paved road (arity ratified; values withheld). |

Family nonclaims: a reservation token is no grant, no admission, no Attempt,
no checkpoint, and no proof of truth. Family falsifiers: duplicate Reserve
minting a second token; expiry silently converting a retry into a fresh
intent; an unauthenticated caller receiving an unbounded durable-state mint.
