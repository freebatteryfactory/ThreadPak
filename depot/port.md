# depot — port owner rows

Selected facts consumed by the port owner's operations (`port/ops.rs`), passed
in explicitly per `depot/README.md` ("Rows are passed, never fetched"). Every
row carries binding time, change consequence, and status; recovered
contradictions are preserved as separate rows, never averaged. Machine-wide
canon selections (digest family, byte order, domain-tag grammar) are the core
canon owner's rows, cited here, never restated.

## Identity widths (artifact-bound; change = new identity family)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| port.family-id-width | `PortFamilyId` interior | 16 fresh opaque bytes | candidate | old canonical-bytes chapter, Class-D fresh law ("fresh rows carry 16 entropy bytes with no meaning in the bytes") |
| port.request-id-width | `PortRequestId`, `CarrierRequestId`, `QuarantineDispositionRef` interiors | 16 fresh opaque bytes | candidate | same Class-D law |
| port.clock-domain-id-width | `ClockDomainId` interior | 16 fresh opaque bytes; one lineage = one boot of one clock kind | candidate | old temporal algebra T1 ("a fresh occurrence identity, not content") |
| port.registered-id-width | `PortOperationId`, `PortValueRole` interiors | **contradiction preserved**: register law says u16 ("registered ids u16 from the domain-tag register"); the prior draft declared u32 | contradiction — owner map decides at the identity-profile pass | old canonical-bytes chapter conventions vs prior `port/types.rs` |
| port.version-scalar-width | `PortContractVersion`, `ClockProfileVersion`, `PortGrantGeneration`, `QuarantineGrantGeneration` interiors | Class-C scoped u64 scalar, scope in canonical bytes, no cross-scope comparison | candidate | old identity matrix ("version lines are Class C"); Class-C u64 convention |

Consumers: every identity constructor at the guard pass. Nonclaims: a width
proves no role; equal widths never substitute. Falsifier: a value of one role
accepted where another role is required because the bytes fit.

## Boundary byte ceilings (deployment-bound; change = refusal/default change)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| port.request-byte-ceiling | `PortRequestByteLimit` value | withheld — the old book names the bound and states no number | withheld | old bounds law ("lengths, counts, offsets, expansion, and role before allocation") |
| port.response-byte-ceiling | `PortResponseByteLimit` value | withheld | withheld | old outbound-validation law ("response schema/size/identity/freshness/trust posture") |
| port.contract-operation-ceiling | `PortOperationCountLimit` value | withheld | withheld | storage-port roster (~15 operations) is the only recovered magnitude — illustrative, not a default |
| port.contract-role-ceiling | `PortValueRoleCountLimit` value | withheld | withheld | none stated |

Consumers: `declare_contract`, `validate_request`, `validate_response`.
Falsifier: attacker-influenced allocation before the ceiling check.

## Quarantine guardrail ceilings (deployment-bound under a safety-relevant posture)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| port.quarantine-item-ceiling | `QuarantineItemLimit` value | withheld | withheld | ingress paved road: "bounded (size/count/age caps, so quarantine is no denial-of-service surface)" — caps named, unvalued |
| port.quarantine-byte-ceiling | `QuarantineByteLimit` value | withheld | withheld | same |
| port.quarantine-age-ceiling | `QuarantineAgeLimit` value + its clock domain | withheld — a tick count without a domain is not an age | withheld | same; domain requirement from the clock-role law |
| port.quarantine-work-ceiling | `QuarantineWorkLimit` value | withheld | withheld | the fourth guardrail dimension (port README) |

Posture classification: **safety-relevant** — absent a deployment row, the
store refuses rather than defaulting open. The raw-retention window and
per-reason-class capture are deployment tunables under the four guardrails
(recovered: "redacted-diagnostic-by-default … not a product decision").
Falsifier: an unexpired item evicted, or a store admitted past any ceiling.

## Clock profile rows (deployment-bound; change = requalification)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| port.uncertainty-posture | `UncertaintyPosture` default | RefuseUnknown | candidate, **safety-relevant** | old T2 law: "Unknown uncertainty is never zero uncertainty: it is the configured maximum, or refusal" |
| port.uncertainty-maximum | `UncertaintyPosture::DeclaredMaximum` value | withheld | withheld | same law; no number stated |
| port.wall-reading-representation | `WallReading` interior | **contradiction preserved**: T2 says every observation is an interval (earliest/latest under a stated uncertainty model; a point value is the degenerate interval and must say so); the prior draft declared a scalar i128 | contradiction — recorded as README Escalation 1 | old temporal algebra T2 vs prior `port/types.rs` |
| port.monotonic-reading-representation | `MonotonicReading` interior | withheld (prior draft: scalar u128; book states no width) | withheld | old temporal algebra |
| port.resolution-roster / port.regression-roster / port.suspend-roster / port.monotonicity-roster | `DeclaredResolution`, `RegressionBehavior`, `SuspendBehavior`, `MonotonicityClaim` rosters | withheld | withheld | profile facts named ("resolution, monotonicity claim, regression and suspend behavior"), rosters never enumerated |

Consumers: the two observation traits, `validate_request` (deadline domain),
the runtime deadline rebase. Nonclaims: a profile row is a claim roster the
harness qualifies — a compile is not a clock. Falsifier: a cross-domain
comparison, subtraction, or rebase without the explicit fallible conversion.

## Recovery-declaration rosters (artifact-bound; preserved contradictions)

| Row | Value | Status | Source |
|---|---|---|---|
| port.recovery-axis-roster | **three recovered rosters preserved**: (a) the five-posture vocabulary (same-key idempotent, queryable outcome, compensatable, at-least-once, nonreplayable) — the declared `RecoveryContract` shape folds (d)+(e) into `ReplaySafety`; (b) a four-member posture list (idempotency/retry/reconciliation/cancellation) with different membership; (c) a nine-property `EffectRecoveryProfile` (adds duplicate-execution posture, concurrency/lease constraints, external-acknowledgement semantics, evidence retention and freshness, manual-intervention requirements) | contradiction — the current four-field shape stands; the wider rosters widen it only when the runtime effect-admission pass consumes them | old port matrix; ch07 Part III |
| port.family-inventory-floor | fourteen named port families as a floor, not a cap (history inspection, publication, checkpoint authority, mutable authority, artifact, protected-payload, secret-authority, wall observation, monotonic observation, entropy, transport/external effects, compiler services, namespace publication, device/external-tool) | candidate closed roster — two of fourteen declared (`WallObservationPort`, `MonotonicObservationPort`); the rest arrive with their consumers | old port-family matrix ("at least these roles") |
| port.carrier-design-inventory | ten carrier rows (native streams, HTTP/1.1, HTTP/2, HTTP/3, streaming bodies, SSE + reverse lane, WebSocket, WebTransport streams + datagrams, WebTransport fallback, Web Push as wake-to-pull) | candidate design inventory — never a support claim; rows mature independently under Macroonz evidence | old carrier design scope |

Falsifier: a claimed recovery route that does not exist before the
irreversible Attempt; a carrier row treated as a support claim without its
evidence row.
