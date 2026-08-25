# depot — runtime rows

Selected facts consumed by the runtime owner (`runtime/types.rs`, `runtime/ops.rs`).
Data only; meaning lives in `runtime/README.md`.
Every row follows `depot/README.md`: passed as an explicit argument, never fetched.

Status vocabulary: **ratified** (owner law, seated), **candidate** (recovered with rationale, awaiting ratification), **withheld** (the selection is deliberately unmade — the depot never invents it), **contradiction** (competing recovered values preserved as separate rows; the owner map decides, never averaging).

## Limits and budgets

All numeric values below are **withheld** unless a value is shown: the old corpus named the bound families and refused to fabricate numbers, and this depot does the same.
Binding time: deployment unless noted; change consequence: refusal/default change under requalification.

| Row | Consuming role | Value | Status | Authority | Notes (nonclaim / falsifier) |
| --- | --- | --- | --- | --- | --- |
| runtime.attempt-limit | `AttemptLimit` · `admit_attempt` | — | withheld | owner-derived | caps live Attempts per scope; never retry legality / hostile: admission beyond cap must refuse |
| runtime.reservation-limit | `ReservationLimit` · `admit_attempt` | — | withheld | owner-derived | / hostile: partial acquisition must release all |
| runtime.port-request-limit | `PortRequestLimit` | — | withheld | owner-derived | per-Attempt outstanding requests |
| runtime.port-response-byte-limit | port-owned `PortResponseByteLimit` (declaration seat: `depot/port.md`; cited, never restated) | — | withheld | port owner | consumed here inside response binding; refuses before allocation |
| runtime.deferred-input-count | `DeferredInputLimit` · `stitch` | — | withheld | owner-derived | / hostile: retention beyond count refuses, never silently drops |
| runtime.deferred-input-bytes | `DeferredInputByteLimit` | — | withheld | owner-derived | |
| runtime.deferred-input-age | `DeferredInputAgeLimit` | — | withheld | owner-derived | Time class; expiry is a typed terminal refusal, never silent loss |
| runtime.deferred-reconsideration-work | `DeferredInputReconsiderationBudget` | — | withheld | owner-derived | reconsidering unchanged input is no progress / hostile: spin under unchanged state |
| runtime.checkpoint-lag-limit | `CheckpointLagLimit` | — | withheld | owner-derived | lag posture applies at the bound; lag is never permission to skip |
| runtime.pump-max-work | `PumpWorkBudget` · `drive` | — | withheld | recovered (bank: "all three numbers withheld") | invocation-bound |
| runtime.pump-max-transitions | *(no owning type — candidate declaration)* | — | withheld | recovered | the bank names it beside max-work; declaring the bound type awaits its consumer |
| runtime.pump-max-boundary-actions | *(no owning type — candidate declaration)* | — | withheld | recovered | as above |
| runtime.join-branch-limit | `JoinBranchLimit` · `collate_join` | — | withheld | owner-derived | |
| runtime.reconciliation-step-budget | `ReconciliationStepBudget` · `reconcile` | — | withheld | owner-derived | budget exhaustion is a typed refusal |
| runtime.vm-frame-limit | `FrameLimit` | — | withheld | owner-derived | |
| runtime.vm-value-byte-limit | `ValueByteLimit` | — | withheld | owner-derived | |
| runtime.vm-scratch-byte-limit | `ScratchByteLimit` | — | withheld | owner-derived | per-Turn scratch |
| runtime.vm-continuation-byte-limit | `ContinuationByteLimit` | — | withheld | owner-derived | |

## Widths

Artifact-bound; changing one is a new persisted-format version.

| Row | Consuming role | Value | Status | Authority | Notes |
| --- | --- | --- | --- | --- | --- |
| runtime.generation-width | `ProcessGeneration`, `WakeGeneration` | u64 | candidate | recovered (bank, wake-generation law) | generation staleness discrimination; falsifier: a stale-generation wake accepted as runnable work |
| runtime.checkpoint-epoch-width | `CheckpointEpoch` | u64 | candidate | recovered | |
| runtime.join-branch-index-width | `JoinBranchId` | u32 | candidate | owner-derived | declared order, never completion order |

## Defaults (classified per depot law)

| Row | Consuming role | Value | Classification | Status | Authority |
| --- | --- | --- | --- | --- | --- |
| runtime.deadline-policy-shape | `DeadlinePolicy` | `DurationBudget` paved; `WallAnchoredWithTolerance` expressible, never default | asymmetric | candidate | recovered (ch04 deadline split) |
| runtime.deadline-pending-narrowing | deadline checks | undetermined comparison narrows to "budget exhausted unless proven otherwise" | safety-relevant | candidate | recovered (ch01 T3) |
| runtime.deadline-lost-provenance | `DeadlineRefusal::LostProvenance` | refusal, never optimistic rebase | safety-relevant | ratified | seated as law (ch07 recovered) |
| runtime.reentrancy-posture | `DriverProfile` | non-reentrant by default | safety-relevant | candidate | recovered (bank) |

## Closed rosters (data form of owner-declared sets)

| Row | Set | Value | Status | Authority |
| --- | --- | --- | --- | --- |
| runtime.reconciliation-next-actions | `ReconciliationNextAction` | Observe / Wait / OneFreshLawfulAttempt / ProposeCompensation / AwaitAuthorizedDecision / AcceptDurablePartialOutcome / TerminateUnresolved | ratified | seated as owner enum |
| runtime.reconciliation-dispositions | `ReconciliationDisposition` | Reconciled / CompensationProposed / ManualInterventionRequired / AutomaticActionRefused | ratified | seated as owner enum (ch07 recovered) |
| runtime.drive-outcomes | `DriveOutcome` | MadeProgress / NeedsHostAction / Quiescent / Terminal | ratified | seated (bank exact match) |
| runtime.wake-degradations | wake-source posture in `DriverProfile` | coalesce / late / duplicate / lost | candidate | recovered (ch07) |
| runtime.turn-phase-roster | *(no phase type — candidate declaration)* | fourteen phases, `Runnable` initial, `ReconciliationComplete` terminal | candidate | recovered (ch07 + sketch); the phase type awaits the construction cut that consumes it |
| runtime.driver-invariant-facts | harness pressure rows | ten driver-invariant facts (identity, ordering, one-shot binding, cancellation meaning, deadline, checkpoint ordering, recovery, work, receipts, traces) | candidate | recovered (ch07) |
| runtime.wake-mechanism-candidates | wake realization | Waker / condvar / eventfd / browser callback / channel notification / worker postMessage / host polling — none selected | withheld | evidence-selected mechanism |

## Contradictions preserved (owner map decides; never averaged)

| Row | Subject | Competing recovered values | Sources |
| --- | --- | --- | --- |
| runtime.backpressure-dimensions.a | total-accounting dimension roster | 10 dimensions | bank ~2749 |
| runtime.backpressure-dimensions.b | 〃 | 14 (adds driver/carrier/browser-worker buffers, host actions, retries) | bank ~7303 |
| runtime.backpressure-dimensions.c | 〃 | ~18 (adds live continuations, pending observers, broadcast window, unacknowledged updates, handoff queue, retry backlog, materialization lag) | bank ~9187 |
| runtime.driver-model-roster.a | driver models | 6 (direct-blocking, cooperative-pump, thread, Future, Promise, deterministic-drive) | ch03 |
| runtime.driver-model-roster.b | 〃 | 9 (adds browser worker, ecosystem async-runtime, embedded/custom; deterministic named separately) | ch07 |
| runtime.fairness-claim-bindings.a | fields one fairness claim binds | 6 | bank ~868 |
| runtime.fairness-claim-bindings.b | 〃 | 8 | bank ~7337 / ~9252 |
| runtime.turnid-preimage.a | `TurnId` preimage | frozen six-field preimage | ch01 §5 |
| runtime.turnid-preimage.b | 〃 | "at least" seven components (process contract+coordinate, generation, source set+cuts, operation, application generation, partition epoch) | ch07 |
| runtime.cancellation-fact-roster.a | cancellation-adjacent facts | 8 (ch07 roster) | ch07 |
| runtime.cancellation-fact-roster.b | 〃 | 7, with shutdown carrying 6 profile-selected meanings | bank ~9024 |

Capacity numbers inside every backpressure row are withheld in every source.
The `TurnId` preimage rows are artifact-bound: ratifying one is a new identity-family decision, and the preimage roster then lives here, cited by the identity profile — never restated in code comments.
