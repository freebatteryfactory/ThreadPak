# view — the derived-result owner

**Owner question.** What can be derived from accepted history at exact Cuts — and under which authority, completeness, freshness, and bounds may that derivation be believed?

This document states product law: what ThreadPak defines.
No sentence here claims current implementation support.

## The wall

Accepted history is not a derived view.
That single separation is this owner's reason to exist, and it is the machine's honesty contract (carries: `../README.md` — What owns fact; rail 1).

- Every resident of this owner — query results, Fixes, Views and their maintained ViewStates, subscriptions, temporal monitors, selection masks, materializations, DataBlocks — is **derived and rebuildable** from accepted history at exact Cuts.
- No derived resident ever becomes authority.
  When a maintained result disagrees with accepted history, the maintained result is stale, corrupt, incomplete, or wrong — never the history.
  The read-side authority roles declared below (`ReadGrant`, `ProtectedResolutionGrant`) are declaration seats, not derived results; no view operation mints, advances, or widens them.
- The illegal dependency this wall forbids: no event-owner operation may consume a derived result as an input to admission, ordering, or authority.
  Acceleration flows downstream only.
- The refusal that proves the boundary: a derived value presented where accepted authority is required is refused by type — no view type converts into an event-owner authority type, and no view operation mints, advances, or amends a Cut, an `AuthoritySequence` position, or an accepted fact.

## What this owner owns

- **Query** — a demand-driven question against accepted history at exact Cuts.
- **`Fix<T>`** — a derived answer bound to its exact source set and Cuts, carrying completeness and freshness as orthogonal axes.
  A Fix is what can be concluded, never what is authoritatively stored.
- **View** — the durable semantic definition of one maintained derived result: sources, frame, advance law, resolve law, and parity contract.
  Validated once; a definition never advances (owner ruling 2026-08-24 — see Escalation 2, closed).
- **ViewState** — one maintained derived state of one View at one `AppliedCut`: replaced when superseded, rebuilt from accepted history when stale or corrupt.
  The recipe/plate separation is load-bearing: the parity judge receives definition and state separately, so a maintained state can never certify itself.
- **Subscription** — the semantic relationship between a consumer and one View (the definition, never tonight's ViewState), with a bounded retained window.
- **Cursor** — continuation of one traversal or query context.
  A Cursor is never skip authority.
- **Temporal family** — `TemporalClaim`, `TemporalMonitor`, and their evaluation under K3 knowledge, temporal fate, and horizon.
- **Selection** — `RowDomain` and `SelectionMask`: exact semantic membership, decided before expensive or protected work.
- **Materialization family** — `Materialization` (named derived role), `MaterializationGeneration`, `DataBlock`, occurrence, and `AppliedCut`.
- **Read-side authority declarations** — `ReadGrant` and `ProtectedResolutionGrant` are declared here.
  Declaration seat is not minting authority: grants are installed by the host's authority machinery, never minted by any view operation (carries: `../README.md` — Owners are not directories).
- **Read-side bounds** — the owner-named limits listed under Bounds below.

## The two lanes live here

(carries: `../README.md` — The two lanes; rails 4, 12)

**Advance (push lane).** Prior derived state plus a bounded admitted delta yields the next derived state.
Advance is shallow, bounded, recursion-free, and parallel-friendly, with declared work per event, no hidden whole-history traversal, and no unbounded fan-out.

**Resolve (pull lane).** A requested result resolves backward through accepted history at exact Cuts.
Resolve may use well-founded recursion, deep traversal, reconstruction, explanation, and counterexample derivation.
It has no universal shallow-depth restriction, and every invocation is finitely and explicitly bounded, with a terminal refusal.

**The parity contract.** For every result this owner claims to maintain incrementally, the push-maintained result equals pull recomputation at the same claim, source set, exact Cuts, frame, relation versions, configuration, and profile.
Parity applies to exactly one of the three root cases: a semantic maintained result requires parity; an awareness notification carries no result claim and is not this owner's resident (it is runtime delivery); a physical effect or observation is not a derived-view pair at all.

The two roads must remain structurally independent where parity is judged: the maintained road and the reference recomputation road may share declarations, but never one shared load-bearing evaluator whose common defect would certify itself.
The View/ViewState split carries this structurally — the judge takes the definition and the maintained state as separate arguments and recomputes the reference from the definition, so handing it the plate twice is unrepresentable.
Macroonz, as the published generation and harness dependency, may generate plumbing for both roads, project this owner's profile and operation indexes, and independently pressure the pair; it owns no view semantics.

## Operations, profiles, and rows

The thin semantic signatures live in `ops.rs`; every operation receives its profile and bounds as explicit typed arguments, and the selected values are rows in `depot/view.md` (`depot/README.md` — "Rows are passed, never fetched").
The pull-lane operations consume the event owner's exact-read surface over accepted history (`ExactHistoryRead`, declared by the event owner with its storage contract).

| Operation | Consumes | Refuses with |
| --- | --- | --- |
| `resolve`, `resolve_continue`, `rebuild` | `ViewResolveProfile`, affine `QueryWorkBudget` | `QueryRefusal` |
| `advance` | `ViewAdvanceProfile`, the `View` definition, prior `ViewState`, `AdmittedDelta` | `AdvanceRefusal` |
| `advance_monitor` | `MonitorProfile`, prior `TemporalMonitorState`, `AdmittedDelta` | `MonitorRefusal` |
| `verify_parity` | `ViewResolveProfile`, the `View` definition, the maintained `ViewState`, affine `QueryWorkBudget` | `QueryRefusal` (verdicts are `ParityVerdict::Held / Diverged`) |
| `derive_selection`, `prove_row_domain_equality`, the compose family, `convert_selection` | `SelectionRepresentation`, `ExactHistoryRead` (membership is decided over accepted history), `SelectionCardinalityLimit`, `RowDomainEqualityWitness` | `SelectionRefusal` |
| the eight materialization lifecycle operations | `MaterializationProfile` | `MaterializationRefusal` |
| `resolve_protected` | `ProtectedResolutionGrant`, lawful `SelectionMask` | `ReleaseRefusal` |

## Temporal claims

(carries: `../README.md` — rails 5, 6)

Three orthogonal axes, never one enum:

- **K3 knowledge** — `Truth::True / False / Pending`, strong Kleene.
  Pending is an evidence statement: truth cannot yet be established from what is available.
- **Temporal fate** — `Satisfied / Violated / Open`.
  Open means lawful future history can still decide either way.
  Fate latches: `Open → Satisfied` and `Open → Violated`, never back.
- **Horizon** — an explicit bound riding beside the axes.
  "Eventually within N" carries N as a bound, not as a fourth truth value.

**Claim-relative finality.** A settled fate is stable only while claim identity, claim version, source set, horizon, and frame semantics remain the same.
Changing any of these creates a new claim; it never reopens the old fate.
A later contradictory fact produces a new evaluation at a newer Cut and never edits the result that was lawful under the old evidence.

**Monitors are push-lane citizens.** A bounded temporal claim compiles to a finite monitor advanced per accepted event.
Deep explanation, exact rederivation, closure proof, and counterexample construction are pull work.
Persisted monitor state is derived and rebuildable: it binds claim identity and version, source set, exact `AppliedCut`, and monitor generation, and when stale or corrupt it is recomputed from accepted history — never trusted.

**Monotone extension versus closure-required finality.** A monotone result may extend as new facts arrive.
Proven absence, negation, exhaustive search, final order, and top-k require closure over the named source set at exact Cuts.
"Nothing has arrived yet" never becomes "nothing exists."
A claim whose horizon cannot close over an incomplete source set reports its incompleteness rather than settling.

## Selection and information release

**RowDomain.** A row domain names exactly which population a mask, column, or materialization speaks about.
Equal cardinality is never equality of domains.

**SelectionMask.** A mask is exact semantic membership over one row domain at one source Cut.
Its physical representation may vary; its membership meaning may not.
Masks compose only across proven-equal row domains at the same Cut; composition over unproven equality refuses (`RowDomainEqualityUnproven`).
Fail-closed is not permission to report a stronger fact: a mask never flattens `Truth::Pending` into `False`.

**The release chain.** Authorization comes first and improves performance rather than being appended after it:

```text
ReadGrant / ProtectedResolutionGrant
→ exact lawful SelectionMask
→ skip forbidden blocks and rows
→ approximate candidate acceleration
→ exact verification against accepted history
→ late materialization of only authorized fields and extents
```

Decrypt last, decrypt least: protected material is resolved only after selection and verification, only under the applicable grant, and resolution cost scales with what is returned, not what is stored.
Physical resolution of a protected payload crosses through the port owner; no view operation ever receives raw secret authority.

**Approximate candidates.** An approximate index or navigator may propose candidates only.
It never establishes truth, absence, causation, authority, completeness, or order.
Exact verification, or an honest incomplete result, remains necessary.

**Protected-index nonclaim.** ThreadPak ships no first-party protected-index family.
This is a reversible standing bar, not a configuration axis and not a permanent impossibility claim: if such a family is ever admitted, it enters candidate-only, verify-always, key-scoped, and fail-closed (carries: owner ruling 2026-07-29; supersedes any text listing protected-index leakage as live configuration).

## Materializations and DataBlocks

Four distinct identities, never conflated:

```text
Materialization              the named derived role and contract
MaterializationGeneration    one realization generation of that role
DataBlock                    one bounded physical component under a generation
Occurrence                   one exact stored or in-memory realization
```

**Lifecycle stages are separate operations:** pure derive ≠ structural validation ≠ semantic binding ≠ publish ≠ activate ≠ select-as-current ≠ retire ≠ reclaim.
A failed publication changes no accepted fact.
A published block is not automatically active.
A superseded block remains identifiable historical physical evidence.
Reclamation waits for live readers and never rewrites historical evidence.

**`AppliedCut`.** An AppliedCut states exactly which authoritative source Cuts a derived generation incorporated.
It is never a storage snapshot identifier: new physical bytes do not imply a newer AppliedCut, and a newer AppliedCut requires proof that newer source Cuts were actually incorporated.
`CommitPoint`, `AppliedCut`, and checkpoint remain three different facts (carries: `../README.md` — What owns fact, Checkpoints).

**Corruption law.** A corrupt derived artifact is discarded and rebuilt from accepted history.
It is never reported as absence, and it never impersonates history.
A DataBlock describing itself proves neither source existence nor derivation correctness; layout and device hints are untrusted claims.

## Bounds owned here

Plain owner-named limits; numeric values and paved profiles live in the depot (carries: `../README.md` — Bounds).

```text
QueryRowLimit               Result
QueryWorkBudget             Work      affine: charging consumes, nothing widens
NavigationDepth             Work
RelationFanOutLimit         Work
SelectionCardinalityLimit   Result
MaterializationByteLimit    Memory
TemporalHorizon             Time
SubscriptionWindowLimit     Memory
```

## Crossings

Each crossing is stated per the no-orphan rule: fact, owner, establishing operation, carrier, substitution refusal, chronology.

1. **Exact Cuts, accepted events, frames, coordinates.**
   Fact: what is accepted, where, in what order, through which Cut.
   Owner: event.
   Established by: event admission and Cut freezing.
   Carried here as: immutable inputs to every advance and resolve.
   Refusal: no view operation mints, advances, or amends any of them.
   Chronology: carries `../README.md` — What owns fact; rails 1, 3, 10.

2. **Subscription checkpoints.**
   Fact: a subscriber's durable right to skip completed logical work.
   Owner: runtime (checkpoint authority regions keyed by subscription identity).
   Established by: accepted checkpoint-advance admission.
   Carried here as: a reference a Subscription may hold.
   Refusal: no view type carries skip authority; a Cursor, a wake, or a delivered update never advances a checkpoint.
   Chronology: carries `../README.md` — Checkpoints (ruling of 2026-08-24).

3. **Wake and live delivery.**
   Fact: awareness that work may exist.
   Owner: runtime delivery.
   Established by: delivery mechanics.
   Carried here as: nothing — a wake is not a view resident and carries no result claim.
   Refusal: a lost wake loses no work; recovery is authenticated pull at exact Cuts plus checkpoint.
   Chronology: carries `../README.md` — rails 8, 9; The two lanes (three cases).

4. **Protected-payload resolution.**
   Fact: one protected extent physically resolved under authority.
   Owner: port (physical crossing) under event/view custody law.
   Established by: a typed port operation under `ProtectedResolutionGrant`.
   Carried here as: the release chain's final step.
   Refusal: view code never receives raw key authority, and a resolution result never widens the grant that produced it.
   Chronology: derives from the release chain above and the protected-payload custody law.

5. **Semantic work accounting.**
   Fact: declared portable work consumed by advance and resolve.
   Owner: this owner declares its work dimensions; physical scheduling observations belong to runtime and mechanisms.
   Refusal: a stopwatch is never a semantic budget.
   Chronology: carries `../README.md` — rail 11.

## Hostile denominator

The falsifiers any realization of this owner must be pressured with.
Each names the lie it kills.

1. **Push/pull disagreement** — the maintained road and the reference road diverge at the same claim, source set, Cuts, frame, and profile; parity must fail loudly, and the maintained result must lose.
   *Killed by:* `verify_parity` → `ParityVerdict::Diverged`, then `rebuild`.
2. **Stale `AppliedCut`** — a generation claims incorporation it cannot prove; new bytes masquerade as new knowledge.
   *Killed by:* `bind_materialization` → `MaterializationRefusal::AppliedCutUnproven`.
3. **Equal-cardinality wrong RowDomain** — a mask or column transplanted onto a different population of the same size; must refuse, never compose.
   *Killed by:* the compose family requiring `RowDomainEqualityWitness`; `SelectionRefusal::RowDomainEqualityUnproven`.
4. **Approximate candidate claiming truth** — a candidate set reported as an exact result, an absence, or an order.
   *Killed by:* `ApproximateCandidateSet` having no conversion to any mask or result type — verification or an honest incomplete result are its only consumers.
5. **Forbidden field materialized before authorization** — any payload or extent touched ahead of the lawful mask.
   *Killed by:* `resolve_protected` → `ReleaseRefusal::ReleaseOrderViolated` / `GrantAbsent` / `GrantScopeExceeded`.
6. **Corrupt materialization impersonating history** — a damaged derived artifact reported as absence or as fact instead of being discarded and rebuilt.
   *Killed by:* `MaterializationRefusal::OccurrenceCorrupt`, then rebuild.
7. **Unbounded-liveness claim settling without closure** — an "eventually" with no horizon reported `Violated` (or `Satisfied`-by-silence) over an unclosed source set.
   *Killed by:* `TemporalHorizon` required by `TemporalClaim` construction; `MonitorRefusal::ClosureUnavailable`.
8. **Mask flattening `Pending` into `False`** — fail-closed misused to report the stronger fact.
   *Killed by:* construction — membership derivation carries `Truth` per claim law and no arm converts `Pending`.
9. **Cursor or wake presented as progress** — either accepted where checkpoint authority is required.
   *Killed by:* construction — `Cursor` has no conversion to any checkpoint role; wakes are not view residents.
10. **Derived value crossing the wall** — any view resident consumed by admission, ordering, or authority.
    *Killed by:* construction — no view type converts into an event-owner authority type (this section's refusal law).

## Refusal families

Role-specific, never one mega-error: `QueryRefusal`, `AdvanceRefusal`, `SelectionRefusal`, `MonitorRefusal`, `MaterializationRefusal`, `ReleaseRefusal`.
The reachable rosters are declared in `types.rs`; every variant's payload carries the violated law, the typed owner, the offending value's role, and the repair direction, with the exact payload bodies closing at the guard pass.
`RowDomainEqualityUnproven` is a settled member of `SelectionRefusal`.
Fake totality is a defect: no roster carries an arm no public input can reach.

## Escalations — open seams recorded, not closed

These are the repository owner's calls; nothing below is decided by this document.

1. **`Fix<T>` exact public shape.** The role and its axes are law; the exact Rust spelling and public field surface remain open per the coordinates ruling that fixed the semantics while leaving names open.
2. **View definition/state split (cross-owner seam — closed by owner ruling 2026-08-24).** The probe closed all six earning questions and the owner ruled: split, spelled `View` (the durable definition — the recipe) and `ViewState` (one maintained state at one `AppliedCut` — the plate), with `MaterializationGeneration` remaining the physical realization.
   `Projection` stays unminted; the lowercase word remains the root's generated-artifact category and the relational operator sense.
   `AdvanceRefusal::StateClaimMismatch` is the distinctness refusal, and the former definition-binding seams in `types.rs` (`View`, `Subscription`) are closed.
   This entry stays as the record of the seam, no longer open.
3. **Temporal claim starter set.** Which claim families exist first (always, never, bounded eventually, bounded until, or a smaller set).
4. **Cross-source temporal finality postures.** Which claims may settle over independently frozen federation Cuts and which require causally constrained or coordinated Cuts.
5. **Monitor fast-start.** Recompute-only first, or optional derived monitor checkpoints with exact invalidation keys.
6. **`ExactHistoryRead` (cross-owner seam — closed).** Every pull-lane operation consumes the event owner's exact-read surface over accepted history.
   The event owner seated the declaration with its storage contract at contract closure; this entry stays as the record of the seam, no longer open.
7. **Subscription retention horizon.** The recovered subscription lifecycle names a retention-expiry stage; no Time-class horizon type exists yet (`depot/view.md`, `view.subscription-retention-horizon`).
   The type mints with its construction cut, not here.
