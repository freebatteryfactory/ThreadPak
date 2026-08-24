# ThreadPak architecture

This document states product law: what ThreadPak defines. Sentences about what the current implementation supports appear only where qualification evidence exists, and a false present-tense support sentence is a broken build.

## One machine

ThreadPak is one coordinate-native semantic dataflow over accepted history. New admitted facts may propagate forward through a shallow, bounded push lane. Requested results may resolve through a deeper, caller-paid pull lane using well-founded recursion under explicit finite bounds. External effects cross through fresh Attempts and typed ports, and their observations re-enter through ordinary admission.

```text
foreign claim / command / physical observation
                     │
                     ▼
             typed validation
                     │
                     ▼
         owner-specific admission
                     │
                     ▼
  accepted coordinate history at exact Cuts
                     │
             one semantic graph
             ╱               ╲
     PUSH: advance        PULL: resolve
     from admitted        from a requested
     delta                result
             ╲               ╱
      values / Fixes / decisions / routes
      event proposals / explicit effect proposals
                     │
                     ▼
               runtime Stitch
                     │
                     ▼
              Bvisor Attempt
                     │
                     ▼
                   Port
                     │
                     ▼
          physical observation
                     │
                     └──── back through admission
```

One semantic dataflow. Two computational lanes. Two membranes — admission into authority, and the external-effect crossing. One durable feedback loop through accepted history.

Admission is not a third computational lane. Bvisor is not a second runtime. A port is not a business model. Transport notifications are awareness — wake, notify, live update — and are never the computational push lane.

## What owns fact

Accepted event history owns domain fact. Role-specific runtime records own only the exact facts they establish — an `AttemptReport` owns one physical effort's observations; an accepted checkpoint advance owns one consumer's right to skip completed logical work; a durably admitted `EffectIntent` owns one logical external commitment. A program's `EffectProposal` is inert data; only REQUEST or PEND admission strengthens it into an `EffectIntent`, exactly as only event admission strengthens an `EventProposal` into an `AcceptedEvent`. Views, indexes, DataBlocks, caches, route plans, and physical plans are derived and rebuildable. When a maintained result disagrees with accepted history, the maintained result is stale, corrupt, incomplete, or wrong — never the history.

Not everything is an event. These roles answer different questions and never substitute for one another:

```text
Command          something requested
Observation      something seen from outside
EventProposal    a proposed fact, not yet authoritative
AcceptedEvent    an immutable fact admitted into history
EffectProposal   a proposed external effect, inert until admitted
EffectIntent     a durably admitted intent to affect the outside world
PortRequest      one physical request
AttemptReport    what one physical Attempt observed
View             derived state
Checkpoint       durable skip authority for one consumer
Receipt          role-specific durable evidence
```

## The two lanes

**Push means forward propagation.** A newly accepted fact advances its dependent computation: prior derived state plus a bounded admitted delta yields the next derived state. Push is shallow, bounded, recursion-free, and parallel-friendly, with declared work per event, no hidden whole-history traversal, and no unbounded fan-out.

**Pull means demand-driven resolution.** A question, destination, or requested result resolves backward through accepted history at exact Cuts. Pull has no universal shallow-depth restriction, but every pull invocation is finitely and explicitly bounded — source set, work, depth, fan-out, memory, result size, and deadline, with a terminal refusal.

**The parity law.** For every result ThreadPak claims to maintain incrementally, the push-maintained result equals pull recomputation at the same claim, source set, exact Cuts, frame, relation versions, configuration, and profile. Parity is the truth boundary between acceleration and authority.

Three cases, never conflated: a semantic maintained result requires parity; an awareness notification carries no result claim; a physical effect or observation is not a derived-view pair at all and answers to its own evidence contract.

## The rails

Cross-cutting laws every owner obeys. They are stated here once and enforced by types, operations, and refusals in the owners — never by a separate subsystem.

1. Accepted-event authority: acceleration never becomes truth.
2. Logical-thread continuity: one intent ties to its events, decisions, Turns, Attempts, receipts, checkpoints, replay, and reconciliation — with no universal ThreadId.
3. Semantic coordinates and exact Cuts: every fact has a typed address; every read names what "now" means.
4. Push-maintained versus pull-recomputed parity.
5. K3 knowledge (`Truth::True / False / Pending`, strong Kleene) versus temporal fate (`Satisfied / Violated / Open`) versus horizon — orthogonal axes, never one enum.
6. Monotone extension versus closure-required finality: proven absence, final order, and top-k require source closure at exact Cuts.
7. Logical Turn versus physical Attempt: replay preserves the Turn; retry mints a fresh Attempt; a response never crosses Attempts.
8. Sync-first semantics with replaceable drivers: one synchronous, bounded, sans-I/O transition protocol; drivers change when and where progress occurs, never what anything means.
9. Live awareness versus durable recovery: a wake can be lost without losing work; authenticated pull plus checkpoint owns recovery.
10. Authority-local exact order (`AuthoritySequence`) versus federation: one exact cut per participating authority, no global sequence, no implied distributed atomicity.
11. Semantic work versus physical scheduling observations.
12. Reference meaning versus qualified optimized plans: an optimized road must agree with the reference road and never acquires authority by being fast.
13. Role-specific authority, bounds, evidence, and outcomes: no universal grant, budget, status, or receipt.
14. Progressive explanation: concise description, typed signature, structured explanation, and complete definitional expansion are four readings of one evaluation.

## Owners are not directories

Four different questions, never conflated:

```text
SEMANTIC OWNER     who owns one meaning, authority, operation, or lifecycle
DEPENDENCY HOME    where code is seated so a wrong dependency cannot compile
PROJECTION         generated implementations, descriptors, codecs, wrappers, docs
MECHANISM          how one qualified realization or its evidence works
```

Semantic owners are earned by unique questions and operations. They are not capped, and they do not automatically become directories. Several owners may lawfully share one dependency home; a role type's declaration seat and its minting authority are separate questions — where a type lives does not move who may create it. Dependency homes follow the compiler-visible graph: actual fields, imports, and Cargo edges, never prose.

A wall survives only when it can answer: what unique semantic question does this owner answer, which operation does it uniquely own, which illegal dependency becomes impossible because the wall exists, and which refusal proves the boundary is real.

## No orphan by distribution

A cross-owner behavior is not seated merely because its concerns are named. Every such behavior must identify:

1. the exact semantic fact;
2. the owner of that fact;
3. the operation that establishes or changes it;
4. the carrier or projection that exposes it;
5. the refusal or nonclaim preventing substitution;
6. its authority chronology — carries a settled ruling, derives from one, supersedes a stale road, or is new closure requiring an explicit mint.

A carrier or wrapper may carry a witness. It may never mint or strengthen one.

## Bounds

Four questions that never share a type:

```text
What is this?                     Value       EventPayload, Coordinate
How much may happen?              Bound       QueryRowLimit, StepBudget
What may this actor do?           Authority   AppendGrant, PortGrant
Why may this result be believed?  Evidence    exact Cut, receipt
```

Bounds classify under seven closed classes — Work, Memory, Result, Output, Effect, Suspension, Time — with owner-specific dimensions. Limits are plain owner-named types living with the operations that consume them; numeric values and paved profiles live in the depot. A budget is affine (move-only, charge-consumes, no widening method anywhere) only where duplicating it would fabricate capacity. Budgets only shrink, deadlines never move later, and authority never widens without a new explicit authority decision.

## Composition and causation

The first built-in semantic join family is `JoinAll`: it preserves every branch's result, refusal, effect intent, outcome posture, explanation, evidence, and semantic work in deterministic branch order. Race-selection and quorum families require real consumers before they exist. A physical race winner never automatically becomes a semantic winner.

Causation keeps three separate relationships: the immediate history predecessor (append integrity), typed bounded multi-parent domain causation (causes already accepted, acyclic for accepted causation), and membership or correlation (never causal proof). Chronology proves no edge; order proves no edge; delivery proves no edge. An unverifiable foreign causal assertion may be admitted as an `UnresolvedCausalClaim` that never counts as domain causation, never closes a traversal, and never establishes completeness. The first profile supports bounded inline causal fan-in; beyond the bound it refuses with a typed result naming an external relation extent as the earned future road, and no such extent is built until a real consumer reaches the bound.

## Checkpoints

Accepted checkpoint-advance records own the authority to skip completed logical work. They live under runtime-owned checkpoint authority regions keyed by process, subscription, or delivery identity; they reference domain Cuts without inhabiting or mutating the observed domain regions. A compact current checkpoint is derived and rebuildable. A checkpoint advance cannot be admitted until every skip prerequisite exists, and an output that committed while the checkpoint lagged remains real — replay plus idempotency and reconciliation handle it, and the checkpoint never lies that work may be skipped.

## Acceptance

The architecture is accepted when two executable witnesses both hold: Witness A proves the complete local machine — ingress custody, accepted history, both lanes with parity, a bounded temporal claim, selection before materialization, a pure program transition through the image gate, one fresh Attempt with honest outcome uncertainty, reconciliation without rewriting, checkpoint recovery after lost wakes, and one logical trace under two physical drivers. Witness B proves authority succession — sealed parents, exactly-covering disjoint children, fresh epochs, preserved identities, route-published-last, and continuity of queries and checkpoints across the split. Witness A is proved through ordered construction cuts, and no cut individually claims ThreadPak.
