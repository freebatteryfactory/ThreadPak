# program — the Program owner

**Owner question:** what bounded, pure Rust computation does the application mean?

This contract is product law for the Program owner and its co-seated Knowledge owner.
It states what ThreadPak defines.
No sentence here claims current implementation support.
Root law: `ARCHITECTURE.md`.

## Scope

The Program owner owns executable meaning: the authoring pipeline from declaration to executable image, the transition algebra a program produces, the effectful-recursion contract, all ProgramImage validation, the semantic work formula, the static least-authority requirements projection, and neutral inspection of every stage.
The co-seated Knowledge owner owns the honesty of application-derived claims: model bindings, assumptions, dependence, calibration, conditioning, information-loss crossings, and evidence requirements.

The Program owner does not own: VM stepping (runtime, PakVM), invocation admission (runtime and Bvisor), the REQUEST and PEND operations (runtime), domain-fact admission (event), derived-view evaluation (view), or the port boundary grammar (port).
Each crossing is stated below under the no-orphan rule.

There is no text language, no parser, no source grammar, no formatter, no language service, and no self-hosting anywhere in this owner.
Programs are ordinary Rust values and pure bounded operations.
`ProgramImage` exists only because PakVM requires one closed executable representation, and it is produced from Rust-owned programs.

## The authoring pipeline

One owner, four stages.
The stages are not separate architectural homes.

```text
ProgramDescriptor
    ↓ checked construction
Program
    ↓ semantic lowering
ExecutionForm
    ↓ packaging
ProgramImage
```

- A `ProgramDescriptor` is authored data: operations, inputs, outputs, source and Cut requirements, effect posture, bounds, refusal families, and explanation relationships.
- A `Program` is the checked semantic form.
  Checked construction refuses a descriptor that is structurally open, unbounded, or self-contradictory (`ConstructionRefusal`, composing the gate's own typed bodies — no fourth refusal vocabulary).
- An `ExecutionForm` is the portable execution representation: explicit operators, control flow, value movement, frame layout, captures, charging points, boundary-request points, and continuation layout.
- A `ProgramImage` binds the semantic commitment and the execution commitment with the operation table, schema closure, required port profiles, bounds, entrypoints, and packaging.

The execution representation never becomes semantic truth by being executable.
The semantic commitment and the execution commitment remain distinct commitments inside one artifact.

**Packaging (owner-ruled, carried).** Three lawful roads carry an image's components — `SelfContained` (everything inline), `ImmutableBound` (components referenced by exact content digest from an immutable store), and `Hybrid` — all satisfying the same dual-form closure and standalone-inspection requirement.
**`SelfContained` is the paved-road default** (asymmetric classification: a lawful override selects another road; the depot row records the selection).
An unresolvable or digest-mismatched component reference refuses the image.

## Transition: what one evaluation produces

The paved operation shape is a pure bounded function (its signature is declared in `ops.rs` as the function-type alias `DecideFn`, with the body landing at construction cut A3; profiles arrive as explicit arguments, never through ambient lookup):

```rust
pub fn decide(
    profile: &EvaluationProfile,
    inputs: &DecisionInputs,
    input: OperationInput,
    budget: &mut SemanticWorkBudget,
) -> Result<Transition, DecisionRefusal>;
```

`DecisionInputs` is the frozen typed input bundle — the source set and the exact Cuts every input was frozen at, binding the view-owned Fix values and admitted observations the application supplies.
(Draft spelling; it replaces the earlier illustrative `Snapshot`, which no owner declared.)

A `Transition` carries event proposals, effect proposals, and an optional immediate result, together with explanation and consumed-work accounting.
A program evaluation:

- reads only explicit typed inputs frozen at exact Cuts — no ambient clock, no ambient store, no host callback, no hidden effect;
- proposes events; it never accepts them — event admission is the only domain-fact admission and belongs to the event owner;
- proposes effects; it never performs or admits them — REQUEST and PEND admission mints the durable, runtime-owned `EffectIntent`, and realization belongs to the runtime, Bvisor, and port owners;
- refuses with a typed `DecisionRefusal` when its inputs, bounds, or laws cannot be satisfied.

**An effect proposal is complete.** `EffectProposal` names the port family, the named port operation and the exact contract version it is declared under, the typed request role, the canonical request-value commitment, and carries the typed request value itself — inert inside the `Transition`.
The durable `EffectIntent` references the proposal by that commitment, and the runtime's publication contract keeps the proposal durably reachable, so the exact physical request is realizable from the intent without reconstruction.
Recovery posture and duplication-recognition identity bind through the named operation's declared `RecoveryContract` — cited, never mirrored.

If a program needs physical time or external evidence, it either receives an already-admitted observation as explicit input or produces a typed request.
There is no ambient fallback.

## Operation postures

A program operation declares one posture.
The posture states what the operation means; the runtime owner owns the REQUEST and PEND operations themselves.

```text
ASK       pure evaluation over supplied immutable inputs at exact Cuts
DO        admit local event-publication intent once applicable requirements close
REQUEST   admit the effect proposal as a durable EffectIntent, return without waiting
PEND      admit the same proposal durably and drive one immediate bounded Attempt
```

PEND is the settled spelling; nothing waits.
A posture declaration performs nothing by itself.

## Effectful recursion: two roads

Well-founded recursion is lawful.
It meets the external world in exactly two ways.
(These are recursion roads, not the machine's two computational lanes — the divided highway of `ARCHITECTURE.md` is a different distinction.)

**Atomic planning — the paved road.** The recursion builds one bounded `EffectBatch` of effect proposals as data and crosses no external boundary while recursing.
The complete batch is admitted afterward and executed later.
If the recursion refuses at any depth, nothing external has happened.

**Interleaved effects.** The recursion crosses REQUEST or PEND boundaries while evaluation continues — each crossed proposal becomes one durable `EffectIntent`.
This is lawful only when the recursion witness closes: maximum effect count, effect ordering, capabilities, semantic work, memory, output, suspension depth, continuation state, the absolute deadline, and the recovery posture.

The hard law of the interleaved lane: **an effect admitted before a later recursive refusal remains admitted and receipted.**
There is no rollback of external reality because a deeper frame later refused.

## Gate 1: ProgramImage validation

The Program owner owns all ProgramImage validation.
The gate is one staged strengthening chain applied identically to every image:

```text
untrusted image bytes or locally built image
    → bounded decode                              (DecodedProgramImage)
    → structural closure
    → Semantic Form validation
    → recursion and bound validation              (SemanticImage)
    → independent lowering agreement
    → Execution Form validation                   (AgreementCheckedImage)
    → effect, capability, source, and profile closure
    → ExecutableProgramImage
```

The ladder is affine typestate: each stage is a sealed constructor consuming the prior value and returning the stronger type or a typed `ImageRefusal`.
Each stage's input carries what that stage judges — bounded decode yields the complete untrusted image record inside `DecodedProgramImage` (decode proves shape, never truth), and every declared roster in it remains unvalidated until its stage strengthens it.
From the executable image onward, states are durable and crash-recoverable and belong to the runtime owner's enum-plus-record custody, not compile-time typestate.

**One owner, two judgment roads.** The Program owner owns the Semantic-to-Execution lowering law and consumes its agreement result, but the production lowerer and the independent agreement route must not share load-bearing lowering or verdict logic.
Every locally or externally produced image crosses the same agreement gate.

These may be shared between the two roads: the public Semantic Form and Execution Form declarations, declared operator identities, canonical primitive definitions, the named lowering contract, and the decoded immutable inputs.

These may never be one shared route: the lowering dispatch, the node-to-operator visitor, control-flow reconstruction, capture calculation, recursion-witness translation, effect and capability propagation, work and bound formula lowering, continuation placement, expected-result generation, and the final agreement predicate.

**Three verdicts, never collapsed:**

```text
AgreementEstablished       the independent route established the lowering relation
DisagreementEstablished    an independently checked relation does not hold — red evidence
AgreementNotEstablished    the required independent route could not run to a verdict —
                           missing required evidence, not a defect finding
```

`ImageRefusal::LoweringMismatch` is the refusal proving the Semantic Form versus Execution Form wall.

**No local bypass.** "We just generated it ourselves" is not evidence.
A locally built image and a foreign decoded image cross the same gate.
`ExecutableProgramImage` has private construction; the only road to one runs through the complete gate.
PakVM receives only an `ExecutableProgramImage` — the wall is private fields and constructors, not a comment.

## Knowledge: the co-seated honesty owner

**Owner question:** under which accepted facts, Cuts, model, estimator, assumptions, dependence posture, calibration, and information-loss crossings is this derived claim lawful?

The application owns its concrete models and vocabulary — its eligibility models, risk estimators, domain assumptions, and estimate types are ordinary application Rust.
ThreadPak owns the relationships that keep them honest:

- model identity and version, and the estimator contract;
- the exact accepted inputs and Cuts a claim was derived from;
- the assumption set and the dependence posture;
- calibration evidence connecting model output to observed outcomes;
- conditioning: prior estimate plus one admitted observation plus assumptions yields an immutable successor bound to a new Cut, or a typed refusal — never a mutation of the prior claim;
- information-loss crossings: every lossy crossing (quantize, redact, summarize, truncate, sample, aggregate, interval-to-point) states what it discarded.
  No surprise midpoint, no silent zero, no distribution silently becoming a point estimate, and no bare `confidence: f64` standing in for any of this;
- `EvidenceRequirement` as a value: a typed description of what additional admitted evidence could close a decision.
  It is consumed by the shared logic axis `Decision::Defer(EvidenceRequirement)`.
  The requirement performs nothing; the application's acquisition policy later chooses REQUEST or PEND, a source, a deadline, and authority.

Application content stays application vocabulary throughout: premises, dependence declarations, loss policies, and evidence questions are carried as **schema-bound statements** (`SchemaBoundStatement` — a schema commitment plus canonical statement commitment), so ThreadPak binds without interpreting.
A declared lossy crossing carries its kind (`LossKind`: exact-to-interval, exact-to-distribution, estimate-to-estimate), both claims, the selecting policy, exactly what was discarded, its reversibility posture, its disclosure statement, and its evidence Cut.

There is no universal `KnowledgeEnvelope`, and no universal `Estimate<T>` until two concrete application families prove that abstraction — estimate types themselves are application Rust (the acceptance witness binds application-owned estimates through these relationships).
Names are module-scoped: this owner's `Calibration` is semantic model calibration and shares nothing with any physical-resource calibration another owner declares.

The knowledge operations are `bind_claim`, `condition`, and `declare_loss` (`ops.rs`), each consuming the affine `KnowledgeBudget` and refusing with `KnowledgeRefusal`.

## Bounds

Owner-local bound types, classified under the seven closed classes of root law; numeric values and paved profiles live in the depot.

```text
SemanticWorkBudget     Work        affine — charging consumes, no widening exists
RecursionDepthLimit    Work        copyable limit
DecodeDepthLimit       Work        copyable limit
EventProposalLimit     Output      copyable limit
OutputByteLimit        Output      copyable limit
EffectProposalLimit    Effect      copyable limit
ResultValueLimit       Result      copyable limit
SuspensionLimit        Suspension  copyable limit
MemoryByteLimit        Memory      copyable limit
ImageByteLimit         Memory      copyable limit
ComponentCountLimit    Memory      copyable limit
KnowledgeBudget        Work        affine — charging consumes, no widening exists
```

`DeclaredBounds` is the complete roster one descriptor or image carries.
The semantic work formula is declared, portable work as a function of the affected input set — never CPU cycles, wall time, or scheduler observations; its terms scale with dimensions of the shared portable work-dimension register (`WorkDimensionId`, a depot closed roster), sequential composition sums, and choice takes the maximum lawful branch bound.
A result limit checked only after unbounded work is not a bound.

**Profiles.** The owner-declared configuration algebra: `EvaluationProfile` (the selected bound roster one decide runs under), `LoweringProfile` (Execution Form grammar and semantic-kernel inventory), `ImageDecodeProfile` (the decode bound algebra).
Operations receive the selected profile as an explicit argument; the selected rows live in `depot/program.md`, and nothing is fetched ambiently.

## Least authority and inspection

Every Program yields a static `RequirementsProjection`: which event sources it reads, which exact Cuts it requires, which frames and relations it traverses, which ports it may request, which capabilities it requires, which bound classes it consumes, which protected fields may cross, and its suspension ceiling.
Recovery postures are **cited, never mirrored**: the projection references port operations, and each operation's declared `RecoveryContract` carries its posture — restating postures here would create a mirror that drifts.
The projection is not a grant and proves nothing about runtime behavior; Bvisor compares it against installed authority and observed crossings.

Every stage of this owner is inspectable without execution, and every evaluation answers the progressive-explanation rail: concise description, typed semantic signature, structured explanation from the same evaluation, and complete definitional expansion.
A short description may omit detail; it may never contradict the expansion.

## Crossings

Each entry follows the no-orphan rule: fact, owner, establishing operation, carrier, substitution refusal, chronology.

| Fact | Owner | Establishing operation | Carrier | Substitution refusal | Chronology |
|---|---|---|---|---|---|
| What an admitted image computes next | runtime (PakVM) | VM stepping over `ExecutableProgramImage` plus an admitted invocation | `ExecutableProgramImage`, Program-minted | PakVM accepts only `ExecutableProgramImage`; private construction forbids unvalidated images; PakVM owns execution-state integrity, never image meaning | Carries owner ruling (2026-08-24, three-gate image law) |
| Whether one invocation may run now | runtime and Bvisor | invocation admission: runtime binds Turn, inputs, Cuts, generation, semantic bounds, recovery posture; Bvisor binds grants, ports, reservations, clock domains, deadline, fresh Attempt | admission request referencing an `ExecutableProgramImage` | an executable image is not an admitted invocation | Carries owner ruling (2026-08-24, three-gate image law) |
| External effect realization | runtime (REQUEST and PEND admission mint the durable `EffectIntent`), Bvisor, port | durable intent admission, fresh Attempt, typed port crossing | `EffectProposal` and `EffectBatch`, Program-declared | a proposal performs nothing; a Transition never executes or admits an effect | Derives from sync-first rail (`ARCHITECTURE.md`, "The rails" 8) |
| Domain-fact acceptance | event | the one event admission operation at an expected Cut | `EventProposal` (event-owned noun), carried in the Transition | a Transition proposes and never accepts; there is no second admission primitive anywhere | Carries owner ruling (2026-08-24, single-admission law) |
| Derived inputs at exact Cuts | view | pull recomputation or push maintenance under the parity law | typed `DecisionInputs` binding view-owned Fix inputs, consumed by `decide` | a program reads explicit frozen inputs only; no ambient reads | Carries root law (`ARCHITECTURE.md`, "The two lanes") |
| Application effect operations | port (grammar), application (declaration) | port contract declaration | typed port request and response families | the port grammar owns the boundary; no port family is declared inside this owner | Carries root law (`ARCHITECTURE.md`, "One machine") |
| Evidence acquisition after Defer | runtime drives; port acquires; event admits | acquisition policy selects REQUEST or PEND; observation re-enters through ordinary admission at a new Cut | `EvidenceRequirement` value inside `Decision::Defer` | the requirement performs no effect and grants nothing | Carries owner ruling (2026-08-24, Knowledge seat) |
| Generated realization | Macroonz (published toolchain dependency) | build-time generation of descriptors, plumbing, and harness pressure — including independent pressure on both Gate 1 roads | generated Rust and descriptors | Macroonz never owns meaning; every generated implementation is replaceable by hand-written Rust with identical ThreadPak meaning; no generated route both produces a material claim and its sole expected answer | Carries root law (`ARCHITECTURE.md`, "Owners are not directories") |

## Refusal families

Three owner-local families, each with a typed owner, the violated law, the offending value, and the repair direction:

- `DecisionRefusal` — a transition could not be lawfully produced: unsatisfied input requirement, bound exhaustion, recursion-witness violation, posture violation.
- `ImageRefusal` — the gate refused an image: bounded-decode failure, structural openness, semantic-form violation, recursion or bound violation, `LoweringMismatch`, execution-form violation, closure failure, or agreement-not-established.
- `KnowledgeRefusal` — a conditioning or binding could not be lawfully produced: stale Cut, missing assumption closure, undeclared information loss, calibration mismatch, budget exhaustion.

`ConstructionRefusal` composes the same typed bodies for checked construction — no fourth vocabulary.
Every refusal body is a typed role (a locator, a closed clause roster, or an exact measured fact); no refusal carries an opaque byte payload, and refusal prose lives in the depot keyed by refusal identity, adding no variant and no condition.

## Hostile denominator

The falsifiers this owner's contract must survive.
Each names a defect the gate, the types, or the harness must catch:

1. Semantic Form and Execution Form disagree — `LoweringMismatch`, never a silent pass.
2. A shared lowerer/checker defect — one route used for both production and judgment must be structurally impossible, and planted disagreements must be detected.
3. An unbounded recursive path — refused at checked construction or recursion validation, never discovered at runtime.
4. Missing effect closure — an image whose declared effects do not close refuses at the gate.
5. A host callback hidden inside a program — unrepresentable in the closed value model; any representation attempt refuses.
6. A foreign image bypassing agreement — no road to `ExecutableProgramImage` except the gate.
7. A locally built image bypassing agreement — same gate, no shortcut.
8. Effect rollback after refusal — an admitted effect surviving a later recursive refusal must remain admitted and receipted.
9. An `EvidenceRequirement` performing an effect — the requirement is inert data; any effectful requirement is a contract violation.
10. A lossy crossing hiding what it discarded — undeclared information loss is a `KnowledgeRefusal`, never a silent narrowing.

## Draft-name notes

`CandidateExecutionForm`, `AgreementEstablished`, `DisagreementEstablished`, `AgreementNotEstablished`, `LoweringMismatch`, `DecisionInputs`, `SemanticImage`, and `AgreementCheckedImage` carry the ruled meanings under draft spellings minted in this type pass; renaming any of them is a taste decision that changes no law.

## Escalations

Recorded, not closed — owner-derived machining, no taste fork open:

1. **Measure algebra profile.** `DecreasingMeasure`'s exact representation (bounded naturals and lexicographic tuples, recovered from the archived corpus) closes with its own profile pass.
2. **Work-dimension register.** The exact roster and per-dimension units behind `WorkDimensionId` close with the work-profile pass; nine recovered candidate clusters are recorded in `depot/program.md`.
3. **Requirement-composition terminals.** A recovered six-terminal composition outcome family for `EvidenceRequirement` sits in `depot/program.md` as a deferred candidate; it enters with its first consumer at a construction cut, not before.
4. **Image profile version.** The archived corpus names an image-profile version distinct from the image-format version; the distinction is unresolved and preserved in `depot/program.md`, and no fourth version identity is minted until it earns one.
