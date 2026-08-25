# depot — program owner rows

Selected facts consumed by the program owner's operations.
Every row follows `depot/README.md`: passed as explicit arguments, never fetched; classified before entry; binding time and change consequence declared; contradictory recovered values preserved as separate rows, never averaged.
Source citations name the archived corpus ("the book") this machine's values were recovered from; the book is quarry, never authority — a row's status says what its value is worth now.

## Packaging

- **Row: packaging roster.** Role: how a `ProgramImage` carries its components.
  Value: closed roster `SelfContained · ImmutableBound · Hybrid`.
  Authority class: owner ruling (D-IMG-2, carried).
  Status: **ratified**.
  Binding time: artifact.
  Change consequence: new image-format identity.
  Consumers: `Packaging`, `validate_image_closure`.
  Nonclaims: a packaging road changes no semantic or execution commitment.
  Falsifier: an image admitting a component road outside the roster.
- **Row: packaging default.** Value: `SelfContained`.
  Classification: **asymmetric** — paved default, lawful override to either other road.
  Authority class: owner ruling (D-IMG-2: best for offline verification, regulated/air-gapped deployment, agent handoff, reproducibility).
  Status: **ratified**.
  Binding time: artifact.
  Change consequence of overriding: none semantic; requalification of the packaging road only.

## Bound rosters — preserved contradiction, disposed

- **Row: bound classes, book Law 6.** Value: five — work, memory, result, effect, suspension.
  Source: book ch01 §2 Law 6.
  Status: **superseded quarry** (no Output, no Time).
- **Row: bound classes, book image roster.** Value: six — work/memory/result/artifact/effect/suspension ("artifact" where the settled register says Output; no Time).
  Source: book ch06 Part II.
  Status: **superseded quarry**.
- **Row: bound classes, settled.** Value: seven — Work, Memory, Result, Output, Effect, Suspension, Time.
  Authority class: owner-endorsed consolidation (2026-08-24); later ruling wins.
  Status: **ratified**; the register lives at core `BoundClass`, cited here for the contradiction ledger only.
  Resolution route: chronology filter, no owner action open.

## Work

- **Row: portable work-dimension register.** Role: the closed roster `WorkDimensionId` keys.
  Value (candidate, from book ch06 Part VII, nine clusters): semantic operations and recursive edges · decoded bytes and validated values · rows/groups/matches/joins/traversal steps · definition and kernel calls · memory and active frames · result and artifact bytes · effects and publication intents · suspended frames and responses · explanation and evidence construction.
  Authority class: owner-derived (book evidence).
  Status: **candidate** — exact roster and per-dimension units close with the work-profile pass.
  Binding time: artifact.
  Change consequence: new work-model version; declared formulas re-validate.
  Consumers: `WorkTerm`, `ConsumedWorkTerm`, `SemanticWorkFormula`.
  Nonclaims: backend instruction count, wall/CPU time, allocations, cache misses, and scheduler counters are mechanism diagnostics, never dimensions.
  Falsifier: an optimized realization charging fewer units for identical semantic work.
- **Row: work composition law.** Value: sequential composition sums; choice takes the maximum lawful branch bound, never the sum; capability-requirement union is never grant union.
  Source: book ch06 Part I.
  Status: **candidate** (doc-carried on `SemanticWorkFormula`).
  Binding time: artifact.
  Falsifier: a choice composition charged as a sum.
- **Row: fold/unfold charging shape.** Value: a fold's bound is |structure| × per-node bound; an unfold's bound is the fuel handed to it (the affine budget).
  Source: book ch06 Part IV.
  Status: **candidate**.
  Consumers: `SemanticWorkFormula`, `SemanticWorkBudget`.

## Recursion

- **Row: measure algebra.** Value: bounded naturals and lexicographic tuples of them under an admitted well-founded order; never an arbitrary callback.
  Source: book ch06 Part IV.
  Status: **candidate** — algebra closes with the measure profile.
  Binding time: artifact.
  Consumers: `DecreasingMeasure`, `construct_program`.
  Falsifier: a measure value that admits an infinite descending chain.
- **Row: interleaved-lane closure roster.** Value: eight clauses — effect count and order, capabilities, recursion and continuation depth, captured bytes, suspensions and responses, work/memory/output, deadline, recovery posture.
  Source: book ch06 Part IV.
  Status: **ratified shape** — realized as the `RecursionWitness` field roster.
  Falsifier: a REQUEST or PEND crossing while any clause is open.
- **Row: second-lock law.** Value: runtime metering stays active after static admission; tail position is never a termination proof.
  Source: book ch06 Part IV.
  Status: **ratified shape** (carried by the witness docs and the runtime meter).

## Knowledge

- **Row: loss-binding roster, Law 3 form.** Value: six bindings — discarded distinctions, policy, reversibility/entropy posture, disclosure, explanation, evidence (input/output claims listed as retained).
  Source: book ch01 §2 Law 3.
  Status: **composed** — see next row.
- **Row: loss-binding roster, §4 form.** Value: eight bindings — the six above plus input claim and output claim moved into the declared list.
  Source: book ch01 §4.
  Status: **composed** — the two rosters are one obligation at two granularities; the superset is realized as `InformationLossCrossing`'s field roster.
  Preserved as two rows because the counts differ in one document; resolution route: different-granularity composition, no owner action open.
- **Row: loss-kind roster.** Value: `ExactToInterval · ExactToDistribution · EstimateToEstimate`.
  Source: book ch06 Part VI.
  Status: **candidate**, seeded as `LossKind`.
  Falsifier: a silent exact-to-estimate collapse reaching a claim without a crossing.
- **Row: estimate families (application-side guidance).** Value: exact, interval, distribution — three role-distinct application estimate shapes; no bare point-estimate accessor.
  Source: book ch06 Part VI.
  Status: **guidance** — estimates are application-owned (witness A3); ThreadPak mints no estimate type until two concrete application families prove the abstraction.
  Consumers: none in this owner by design.
- **Row: requirement-composition terminals.** Value (candidate): six — ConclusivelySatisfied, ConclusivelyRejected, Unresolved, Invalid, SourceIncomplete, ProofUnavailable; composition never turns Unresolved or SourceIncomplete into rejection.
  Source: book ch06 Part VI.
  Status: **deferred candidate** — no composition operation is declared yet; the enum enters with its first consumer at a construction cut.
  Nonclaims: recording this row builds nothing.

## Image identities

- **Row: image version identities.** Value (candidate): image-family format version, Execution Form version, semantic-kernel version — realized as `ImageFormatVersion`, `ExecutionFormVersion`, `SemanticKernelVersion`.
  The book names a fourth ("image profile version") whose distinction from the format version is **unresolved**; preserved here, not minted.
  Source: book ch06 Part II/IX (nine role-distinct identities; four seated as content identities in types, three as versions, ApplicationImage reference dead).
  Binding time: artifact.
  Change consequence: per identity — semantic, execution, image-bytes, and kernel compatibility do not move together.

## Numeric values — all withheld

The book dialed no numbers for this owner's limits; every count below is **withheld — the repository owner selects**, classification pending per value.
Binding time: deployment or invocation per row; change consequence: refusal-boundary change only, never a semantic change.

- `RecursionDepthLimit` — withheld.
- `EventProposalLimit` — withheld.
- `EffectProposalLimit` — withheld.
- `ResultValueLimit` (bytes) — withheld.
- `SuspensionLimit` — withheld.
- `MemoryByteLimit` — withheld.
- `OutputByteLimit` — withheld.
- `ImageByteLimit` / `DecodeDepthLimit` / `ComponentCountLimit` — withheld.
- `SemanticWorkBudget` / `KnowledgeBudget` initial values — withheld.

## Cross-owner notes

- The book's third knowledge axis (`CommitKnowledge`, beside `Truth` and `OutcomeKnowledge`) has no seat in this owner; its seat question belongs to the event/core packets.
- Rounding modes, quantization postures, and interval decision tables are the core number family's rows, not this owner's; the program knowledge types only carry the crossings that cite them.
