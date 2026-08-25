# Runtime

This document is the runtime owner contract.
It states product law.
Sentences about what any implementation supports appear only where qualification evidence exists.

Three semantic owners are co-seated here.
They answer different questions and never blur:

```text
Logical runtime   What does one observation mean for the continuing logical operation?
PakVM             What does this admitted program compute next?
Bvisor            May this exact physical step happen here and now, with these
                  grants, reservations, ports, clock domains, and deadlines?
```

Co-seating is a dependency-home fact, not a merger of meaning.
No type, operation, or refusal in this home may serve two of these owners at once.

## Turn and Stitch

A **Turn** is one logical transition: selected runnable logical work, frozen typed inputs at exact Cuts, one admitted program invocation, a semantic result or refusal, event proposals, effect proposals, and a checkpoint consequence.
The logical-book entry is two append-only records with one typed join: the `Turn` record freezes the invocation half (operation, process and generation, inputs, invocation, bounds, recovery posture, checkpoint consequence), and the `TurnConclusion` record binds that Turn's identity to what it concluded (semantic conclusion, event proposals, effect proposals, work, explanation).
Neither record edits the other; their accepted composition is the complete entry.
`TurnId` is a derived identity over the Turn's identity-bearing inputs; replay reconstructs the same Turn where lawful, and a changed identity input is a different Turn.
Four lineage identities never substitute: `LogicalOperationId` (the application-meaningful unit), `TurnId` (one logical transition), `EffectIntentId` (one admitted external intent), `AttemptId` (one physical effort).

Stitch consumes one member of the typed-observation family `StitchStimulus`.
The family's roster closes with the stimulus-algebra seam; a wake is awareness and is never a member.

**Stitch** is the runtime transition — the pure operation that advances one logical process:

```text
prior admitted state
+ one typed observation
+ explicit context (the Turn being realized, the process contract,
  the deadline policy)
→ StitchAdvance (next state + TurnConclusion: semantic conclusion,
  event proposals, effect proposals, work, explanation — joined to
  the Turn's identity)
| StitchRefusal
```

Stitch begins as a plain pure function.
No trait exists until two real, substitutable transition providers exist.
The vocabulary wall is absolute: **tick** is the clock's tick, **step** is PakVM's internal movement, **Turn** is the logical transition identity, **Stitch** is the runtime transition, **Attempt** is one physical effort.

Stitch produces event *proposals*.
Only event admission makes them accepted facts; the runtime assigns no `AuthoritySequence` and publishes no domain history.

## REQUEST and PEND

Two operation postures for external work:

```text
REQUEST   admit one program-produced EffectProposal as a durable EffectIntent
          and return without waiting for physical completion

PEND      admit the same proposal as a durable EffectIntent, drive one
          immediate bounded Attempt, and observe within explicit bounds
```

PEND never means wait-forever, retry-until-green, or hide-a-Future-inside-the-semantics.
An effect admitted before a later semantic refusal remains admitted and receipted; there is no imaginary rollback.
The spelling is PEND — nothing waits; a suspension is explicit typed state, not a blocked thread of meaning.

## The many faces of "not yet"

These are distinct facts.
Collapsing any two is a defect:

| Word | Actual meaning | Owner |
| --- | --- | --- |
| `PEND` | Drive one immediate bounded physical Attempt | runtime |
| `Poll::Pending` | A host Future adapter made no physical progress this poll | driver adapter |
| `Truth::Pending` | Available evidence establishes neither true nor false | core logic |
| fate `Open` | Future accepted history can still decide the temporal claim | view |
| `Defer` | A decision requires more admitted evidence | core logic |
| Deferred input | An admitted input cannot run in the current process state | runtime |
| Outstanding effect | A durable EffectIntent has no terminal outcome knowledge | runtime |
| `OutcomeUnknown` | An external consequence may exist; evidence cannot establish it | runtime |
| Backpressured | Capacity is presently unavailable under one delivery contract | runtime |

No generic `Pending` status exists anywhere in this home.

## Completion

A `Completion` is one operation–observer relationship.
It resolves at most once, with the strongest honest terminal observation available at that boundary.
It is not the external outcome, not the live continuation, not a receipt, not a checkpoint, and not retry authority.
A `Completion` that resolved `OutcomeUnknown` is never mutated by later evidence; later evidence produces new facts through reconciliation, and the old observation stays truthful.

Observer abandonment — a dropped Future, a closed channel, a caller that stopped waiting — cancels nothing by itself.
Cancellation is a separately authorized request.

## Deferred input

A lawfully admitted input that cannot yet run is retained as explicit bounded state: the input identity, why it is deferred, the exact state or evidence that could unblock it, its age and absolute deadline, its collation position, and its share of the deferred-input bounds.
When it can no longer become admissible it receives a terminal refusal.
Reconsidering the same input under unchanged state and unchanged evidence is no progress, and the runtime must recognize no progress rather than spin.
No unbounded selective-receive mailbox exists.

## Drivers, wakes, and delivery

ThreadPak defines one synchronous, bounded, sans-I/O transition protocol.
Blocking calls, cooperative pumps, dedicated threads, Rust Futures, browser Promises, workers, async runtimes, and deterministic simulation are drivers of that one protocol.
A driver changes when and where progress occurs.
It may never change accepted events, semantic results, Turn identity, EffectIntent identity, declared result order, semantic work, checkpoint meaning, or explanation — the logical trace is invariant under lawful schedule variation.

Driver law:

- One drive call owns one runtime lane; same-lane reentrancy is unavailable by default. Host work happens outside the transition; responses re-enter through a new explicit drive call.
- The wake protocol closes the check-to-park race: check readiness, register interest, check again, park only if still not ready. Wake tokens are generation-bound; a stale generation's wake is not runnable work.
- A wake may be lost, duplicated, or coalesced. A lost wake increases latency and never loses work: the durable mailbox of a persistent process is accepted history selected by its input contract plus its durable checkpoint. Queues, channels, and notifications are acceleration.
- Close, drain, shutdown, observer abandonment, cancellation request, cancellation observation, and deadline expiry are separate transitions with separate meanings. A profile states which of them it performs; none is implied by a channel library's defaults.

**Backpressure is totally accounted.**
Every retained layer reachable through an operation — queued inputs and bytes, retained results, live continuations, outstanding port requests, active Attempts, pending observers, retained subscription windows, driver and carrier buffers, retry backlog, checkpoint lag — is bounded, proven borrowed, or the profile is unsupported.
Backpressure may not migrate into an unaccounted adapter.

**Fairness and liveness are qualified profile claims**, never consequences of a channel dependency.
A claim names its population, fairness unit, horizon, load assumptions, backpressure posture, and evidence route.

## Deadlines

One absolute operation deadline crosses every adapter, retry, reconnect, suspension, worker migration, and fresh Attempt.
No layer mints fresh patience.
The deadline is three objects, never one:

```text
DeadlinePolicy            durable semantic commitment
ConsumedBudgetEvidence    persisted named observations of spent allowance
LiveDeadline              process-local enforcement state in one monotonic
                          clock domain; never serialized; dies with the process
```

Rebasing derives a `LiveDeadline` from policy, one admitted monotonic observation (the port owner's validated enclosure — raw readings never cross this boundary), and consumed evidence.
Remaining allowance never grows without a new explicit authority decision.

## Checkpoint authority

Accepted checkpoint-advance records own the authority to skip completed logical work.
Law of this owner:

- Checkpoint records live in **runtime-owned checkpoint authority regions**, keyed by a role-branded `CheckpointSubject` — process, subscription, or delivery identity as typed variants, so a process checkpoint can never substitute for a subscription checkpoint merely because the identities share a width. They reference domain Cuts and never inhabit or mutate observed domain regions.
- One writer per checkpoint region and epoch. This means one active semantic authority — never one thread, task, or file per process.
- A `CheckpointAdvanceProposal` is admissible only when every skip prerequisite exists: expected predecessor, current generation, required output and publication receipts, and every outstanding effect and reconciliation obligation accounted. Advancing before prerequisites is unrepresentable or refuses.
- `CurrentCheckpoint` is a compact derived image, rebuildable from accepted advances. It is never the authority.
- Output committed while the checkpoint lagged stays real: replay may occur, idempotency and reconciliation absorb it, and the checkpoint never lies that work may be skipped.
- `CheckpointAdvanceGrant` is the role-specific authority to admit an advance. No other grant substitutes.

A cursor, HLC value, wake, session, delivery index, or derived fast-start image never advances a checkpoint.

## Replay, retry, and the two books

The runtime keeps two books and never lets one edit the other:

```text
Logical book    operation, Turn, frozen Cuts, semantic result, EffectIntent,
                checkpoint meaning, retry legality, reconciliation posture

Physical book   Attempt, grants, reservations, requests and responses,
                exits, deadline and resource observations, commit knowledge
```

Replay preserves the logical Turn where lawful and always mints fresh Attempts.
Physical failure never erases accepted logical intent.
Physical success never becomes domain success.
Retry legality is a logical-book decision made by the runtime under the operation's recovery contract — never by Bvisor, never by a supervisor merely observing a death, never by a reconnecting carrier.

## Runtime record publication

This owner's durable records — admitted `EffectIntent`s, accepted checkpoint advances, sealed `AttemptReport`s where a profile requires durability, and reconciliation records — publish through a **runtime-declared storage port family**, stated in the port owner's contract grammar exactly as the event owner declares its own storage family.
An admitted intent's proposal publishes with it: the proposal's canonical bytes — naming its port operation, contract version, and request-value commitment — remain durably reachable through this family, so the exact physical request is realizable from the durable record alone, and `EffectProposalCommitment` proves the correspondence.
The behavioral contract mirrors the event storage law: append against an expected predecessor, exact read-back of the published records, crash recovery bounded by the committed boundary, and idempotent reopen.
One qualified physical adapter may realize this family and the event owner's family on one backend; the semantic owners never merge, and an adapter's success claim never substitutes for a receipt it did not establish.
Nothing here publishes domain history: these records reference domain Cuts and never inhabit domain regions.

## Reconciliation

Reconciliation is a pure bounded conclusion over append-only evidence: the effect's recovery contract, the durable EffectIntent, every known AttemptReport, acknowledgments, outcome-query evidence, and current policy, yielding a current lawful conclusion and next action.
Whether reconciliation is owed and how it concluded are two facts that never share one enum: `ReconciliationLifecycle` answers *whether* (not required, outstanding, complete), and `ReconciliationDisposition` — carried only inside completion — answers *how*, so a disposition-without-completion is unrepresentable.
It never rewrites an earlier Attempt, observation, or external event; later evidence supersedes the conclusion while the historical record stands.
Lawful next actions include observing, waiting, one fresh lawful Attempt, a separately admitted compensating effect, an authorized human decision, accepting a durable partial outcome where the contract permits, or terminating unresolved.
Repeated `OutcomeUnknown` is a stable honest result — inconvenience is not permission to relabel it failure or to retry without legality.

## JoinAll

The first built-in semantic join family.
`JoinAll` preserves every branch's result or refusal, effect intents, outcome posture, explanation, evidence, and semantic work, collated in deterministic branch order.
Cancellation before and after child admission, mixed terminal and `OutcomeUnknown` branches, and budget and deadline partitioning are explicit in the family's contract.
A physical race winner never automatically becomes a semantic winner.
Race-selection and quorum families do not exist until real consumers earn them.

## PakVM

PakVM is the closed, bounded, synchronous, safe-Rust value machine that executes admitted programs.
It answers one question — what does this admitted program compute next — and owns nothing else.

- **Closed value algebra.** `VmValue` admits exact scalars, role-specific identities and references, records and variants, bounded collections, exact numeric roles, admitted address and cut values, immutable image constants, bounded continuation captures, and opaque typed handles to host-owned resources. It never admits `Any`, host objects, raw pointers, callbacks, Futures, sockets, ambient capabilities, or serialized live authority.
- **Explicit memory roles.** Immutable image region, bounded per-Turn arena, bounded scratch, an explicit frame stack, minimal capture records, and live handles owned outside ordinary values. Every allocation and retained value charges an exact bound; a result limit checked only after unbounded work is not a bound.
- **Synchronous stepping to one boundary.** A step continues, returns a value, refuses, produces a publication intent, produces one typed port request plus one bounded suspension, or exhausts an admitted semantic budget. PakVM never awaits the host and never performs I/O.
- **One-shot continuations.** A suspension is bounded, request-bound, Attempt-bound, generation-bound, deadline-bound, and not `Clone`. It is resumed exactly once, terminated exactly once, or abandoned and sealed as physical evidence. Process death destroys it; recovery reconstructs the durable Turn and mints a fresh Attempt — a dead Attempt's stack is never deserialized back to life.
- **Portable semantic work.** The meter charges operator applications, rows considered, edges traversed, values decoded, and result construction — never CPU cycles, wall time, polls, wakes, or scheduler quanta. Two hosts may schedule differently and preserve one logical trace and one work account.
- **Validation is not PakVM's job.** PakVM receives only an `ExecutableProgramImage` — a value the program owner's image gate has already strengthened; private construction makes an unvalidated image unrepresentable here. PakVM refuses only execution-state integrity violations: corrupt live state, a wrong continuation response, budget exhaustion, an impossible operator state.

## Bvisor

Bvisor is the physical-admission membrane around one admitted logical invocation.
It is not an OS, hypervisor, policy engine, scheduler, retry engine, or second runtime.
Containment is semantic and OS-free; physical isolation is an optional layered defense selected by profile.

- **AdmissionPlan.** Physical admission closes the validated image and operation, the Turn relationship, exact source cuts and generations, required ports and grants, authority generations and revocation posture, semantic and physical bounds, target profile, clock domains, the absolute deadline, reservation dimensions, and report requirements.
- **All-or-release acquisition.** Reservations are affine physical custody: Attempt-bound, not `Clone`, released exactly once, never reusable by a retry. Partial acquisition releases everything acquired and refuses; no partially admitted Attempt and no leaked reservation exists.
- **No Attempt before admission.** A failure before minting is an admission refusal — no `AttemptId` exists and no report implies one. A mechanism-start failure after minting consumes the live Attempt, releases its custody, and seals a terminal `AttemptReport` recording the start failure; the runtime alone decides whether a fresh Attempt is lawful.
- **Attempt custody is typestate.** Planned invocation, admitted Attempt, running Attempt, live suspended Attempt, terminal Attempt: each transition consumes the prior live value. A persisted description of an Attempt's state is data; decoding it never resurrects live custody. Live custody and one-shot authority types are never `Clone`, never serialized, and never accidentally `Send` or `Sync` — an auto-derived crossing would move live authority between workers without an admission decision, so custody interiors are chosen at the guard pass to make that crossing unrepresentable.
- **Response binding.** Every port request binds the AttemptId, request identity, port family and operation, expected response family, grant identity and generation, remaining bounds, the absolute deadline, and the continuation identity. A response must match every correctness-bearing coordinate; matching bytes are not enough. Response authority is one-shot. A late response for a dead Attempt may remain authentic physical evidence; it resumes nothing.
- **AttemptReport.** Sealed physical facts only: identity and lineage, selected mechanism profile, installed grants, reservation and consumption evidence, requests and validated responses, cancellation and deadline observations, exits, commit knowledge where a backend establishes it. It never claims domain success, retry legality, compensation, checkpoint advancement, or semantic correctness.
- **Cancellation.** Bvisor may attempt physical cancellation under the selected profile and reports what it observed. A cancellation observation does not prove the external consequence never occurred.

## Bounds and profiles

Owner-local bounds, consumed by the operations that declare them; numeric values and paved profiles live in `depot/runtime.md`, and every operation receives its profile or budget as an explicit argument — never through an ambient lookup (`depot/README.md`, "Rows are passed, never fetched"): `AttemptLimit`, `ReservationLimit`, `PortRequestLimit`, `DeferredInputLimit`, `DeferredInputByteLimit`, `DeferredInputAgeLimit`, `DeferredInputReconsiderationBudget`, `CheckpointLagLimit`, `PumpWorkBudget`, `ReconciliationStepBudget`, `JoinBranchLimit`, and the PakVM four (`FrameLimit`, `ValueByteLimit`, `ScratchByteLimit`, `ContinuationByteLimit`).
The response byte ceiling is the port owner's `PortResponseByteLimit` — declared with the port contract, consumed here inside response binding; this owner declares no twin.

The profile algebras this owner declares — `DriverProfile`, `CheckpointStorageProfile`, `ReconciliationProfile`, `EffectAdmissionProfile`, `PanicContainmentProfile` — state the lawful configuration axes; depot rows select coordinates inside them.
A profile selects within an operation's declared contracts and never widens one.

## Crossings

Per the no-orphan rule, each cross-owner behavior names its fact, owner, establishing operation, carrier, substitution refusal, and chronology.

**ExecutableProgramImage.**
Fact: this image's Execution Form realizes its Semantic Form under closed validation.
Owner: program.
Establishing operation: the program owner's image gate, including its structurally independent lowering-agreement road.
Carrier: the runtime passes the strengthened value; PakVM consumes it.
Substitution refusal: an unvalidated or merely decoded image is unrepresentable as PakVM input, and no runtime or Bvisor admission re-decides image semantics.
Chronology: carries the three-gate ruling of 2026-08-24; derives from ARCHITECTURE.md "Owners are not directories".

**Accepted history — AcceptedEvent, Cut, AuthoritySequence, authority regions.**
Fact: what became accepted, where, in what exact order, through which durable cut.
Owner: event.
Establishing operation: event admission and publication.
Carrier: the runtime freezes Turn inputs at exact Cuts and its checkpoint records reference domain Cuts.
Substitution refusal: the runtime never assigns order, never publishes domain events, and a checkpoint record never inhabits a domain region.
Chronology: carries ARCHITECTURE.md "What owns fact" and "Checkpoints".

**Subscription semantics and parity.**
Fact: what one subscription means and that push-maintained results equal pull recomputation at the same Cut.
Owner: view.
Establishing operation: view advancement and recomputation.
Carrier: runtime delivery moves updates, wakes, credit, and checkpoints.
Substitution refusal: delivery progress, wake receipt, and credit state never impersonate parity, completeness, or checkpoint advancement.
Chronology: carries ARCHITECTURE.md rails 4 and 9.

**Port contracts and clock observations.**
Fact: the typed boundary grammar for external operations and physical time.
Owner: port.
Establishing operation: port contract declaration; host adapters realize it.
Carrier: Bvisor binds requests to Attempts and validates responses; the deadline owner consumes admitted monotonic observations (the port owner's validated enclosures — raw readings never cross); wall observations belong to event chronology, not to this home.
Substitution refusal: no ambient clock exists anywhere in this home, and a response satisfying different correctness coordinates refuses.
Chronology: carries the role-specific clock-contract ruling of 2026-08-24; derives from ARCHITECTURE.md rail 8.

**EffectProposal and EffectIntent.**
Fact: one durably admitted intent to affect the outside world.
Meaning owner: program — a transition produces the inert `EffectProposal` that declares the effect.
Record owner: this home — REQUEST or PEND admission consumes the proposal and mints the durable `EffectIntent`, and this owner holds its publication contract and the outstanding relationship; Bvisor realizes it through fresh Attempts; reconciliation concludes it.
Substitution refusal: a proposal is not an admitted intent; neither runtime nor Bvisor mints or edits effect *meaning*; an admitted intent survives any later semantic refusal.
Chronology: carries ARCHITECTURE.md "What owns fact" and the effect proposal/admission split (owner-ruled 2026-08-24).

**Runtime record publication.**
Fact: durable acceptance of this owner's records (admitted intents, checkpoint advances, sealed reports where required, reconciliation records).
Owner: this home (contract); a qualified store realizes it.
Establishing operation: publication of what admission accepted, through the runtime-declared storage port family.
Carrier: that family, stated in the port owner's contract grammar.
Substitution refusal: a storage mechanism's success claim never substitutes for a receipt it did not establish, and publication of a runtime record never touches domain history or its order.
Chronology: derives from the event owner's storage-publication pattern and the checkpoint ruling (2026-08-24).

Generated implementations and harness descriptors may realize plumbing declared by this contract; Macroonz participates at build time and test time, never in runtime authority.

## Hostile denominator

Each of the following is unrepresentable by construction or refuses with a named refusal; the harness plants each one and must observe the refusal:

1. A late response resuming a fresh Attempt.
2. A disconnect treated as cancellation.
3. A timeout treated as proof of nonexecution.
4. An `AttemptId` reused by retry.
5. A reservation surviving a failed admission.
6. A checkpoint advancing before its prerequisites exist.
7. A wake or notification impersonating durable progress.
8. A driver schedule changing semantic work or the logical trace.
9. A continuation resumed twice.
10. A replay changing the logical result of an unchanged Turn.
11. A physical success reported as domain success.

## Escalations

Genuine forks that only the repository owner rules; nothing below is decided by this contract:

1. **PEND's stopping rule.** Whether PEND observes until the first external boundary, one validated response, one semantic terminal, or a selected bounded combination.
2. **Portable continuation profiles.** Whether any semantic continuation state may cross process or worker boundaries; live same-Attempt state remains non-portable regardless.
3. **Panic containment profiles.** Which posture ships where: caught unwind as a physical observation, abort-and-fresh-Attempt, or an isolated-worker profile — each with its honest nonclaims.
4. **The paved driver surface.** Which adapters are first-party paved road (blocking, pump, Completion handle, Future, Promise, worker) versus host-side, and which one greets a new user first.
5. **Quiescence vocabulary.** The exact public distinction among temporarily quiescent, backpressured, deferred, oscillating, starved, and irrecoverably stuck.

## Open realization seams

Owner-derived contract work, not taste votes, and not closed here: the stimulus-algebra roster (`StitchStimulus` is seated; its member roster closes here — the `stitch` signature is declared in `ops.rs` as the `StitchFn` alias, realized as a pure free function until two real providers exist), the exact `VmValue` representation and operator inventory, wake and reservation mechanisms per target profile, and the checkpoint storage profile's mechanism selection.
Each lands with its first construction cut and its own falsifiers.
