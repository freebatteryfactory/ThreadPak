# Port owner

This document states product law: what ThreadPak defines. No sentence here claims current implementation support.

## The owner question

> **How does semantic work name and validate an external operation without importing host meaning?**

A port is the explicit typed boundary between ThreadPak semantics and the outside world. Everything beyond it — files, databases, HTTP, browser APIs, workers, human hands — is a mechanism. Mechanisms realize port contracts; they never own meaning, and no carrier type appears in an ordinary ThreadPak signature.

There is no ambient callback. Semantic work never receives a function the host may call back into; it emits one typed request and later receives one validated response. The runtime transition protocol stays synchronous and sans-I/O; the port is where a boundary action leaves it.

## What this owner defines

- **`PortContract`** — the declared boundary of one port family: its closed operation set, request and response roles, refusal families, declared bounds, and deadline participation. Port families are declared by their semantic owners (the event owner declares its storage and quarantine families; programs declare external effect operations); this owner defines the grammar every family declaration uses. There is no universal request envelope and no universal dispatcher — each family's operation set is closed and its own.
- **`PortOperation`** — one operation inside a family: request role, response role, refusal role, and its operation-specific `RecoveryContract`.
- **`PortGrant`** — the admitted authority to attempt operations of one family under one scope. A grant names what it permits and under which authority generation it is current. A path, route, connection, token string, or contract declaration grants nothing; only an admitted grant does. The live installed grant handle is Bvisor's custody, not this owner's.
- **The request/response binding law** — a `PortResponse` is valid only against the exact `PortRequestId`, the exact `AttemptId`, the expected response role, and the current generations it was issued under. Matching bytes are never enough. A response for Attempt A can never satisfy Attempt B; a late response for a dead Attempt may remain authentic physical evidence, but it carries no resume authority.
- **The deadline law** — one absolute operation deadline crosses every adapter, retry, reconnect, suspension, and carrier conversion. Every layer receives the same absolute deadline or a narrower remaining allowance derived from it. No adapter, wrapper, or driver mints fresh patience. Deadline policy and live enforcement are runtime-owned; this owner defines how the carried deadline binds a request.
- **Response validation** — the operation that turns foreign response material into a validated response or a typed refusal: wrong request, wrong Attempt, stale generation, expired deadline, role mismatch, malformed or noncanonical content, duplicate delivery.
- **Recovery contracts** — each operation declares its own recovery posture from the five-posture vocabulary: same-key idempotent, queryable outcome, compensatable, at-least-once, nonreplayable. These are operation-specific declarations binding the exact key roles, outcome-query operations, and compensating operations involved — never one universal posture enum divorced from its bindings. An operation that claims automatic retry, outcome query, or compensation must have that route declared before the irreversible Attempt, not discovered after it.
- **Clock observation ports** — two separate role-specific contracts: wall observation and monotonic observation. One host adapter may implement both; the semantic contracts never merge, and there is no universal clock trait. A raw observation is foreign material: it becomes accepted chronology only through the chronology owner's admission, and it becomes deadline state only through the runtime deadline owner's rebase. A wall observation never impersonates a `CommitPoint`, an `AcceptedHlc`, an `AuthoritySequence` position, or any durable order.
- **The quarantine port** — the boundary through which the ingress owner's quarantine disposition is physically realized. Its four guardrails are law: bounded (count, bytes, age, and work ceilings), expiring with real deletion (key-shred counts only when the quarantine holds its own key scope and the key authority was actually destroyed), access-controlled (explicit audit or debug authority), and never directly re-admittable — quarantined bytes re-enter ThreadPak only as a fresh foreign claim through ordinary ingress validation.

## Transport, wrappers, and awareness

A remote wrapper is a projection over an ordinary ThreadPak operation. It may **carry** witnesses, receipts, checkpoint references, client nonces, reservation tokens, and flow-control credit. It may never **mint or strengthen** any of them. Serialization is not inference; carriage is not authority.

Push transport is bounded awareness. A push payload may carry a semantically correct derived answer, and receiving it still proves nothing about completeness, checkpoint advancement, sender authority, or lost prior signals. Push delivery never carries event truth, capability, admission, or recovery authority; recovery is authenticated pull at exact Cuts plus durable checkpoints.

Carrier equivalence — that the same operation means the same thing over any qualified carrier — is a qualified harness claim established by comparing carrier projections. No runtime operation mints it.

## Crossings

Per the no-orphan rule (fact / owner / operation / carrier / refusal / chronology):

| Fact | Owner | Establishing operation | This owner's role | Substitution refusal | Chronology |
|---|---|---|---|---|---|
| `EffectIntent` | program / runtime | REQUEST or PEND admission | request execution grammar | a port cannot mint, complete, or cancel an intent | carries the effect-membrane law (ARCHITECTURE.md, One machine) |
| Fresh `AttemptId` + live custody | Bvisor | physical admission | every request binds the Attempt | response never crosses Attempts | carries rails §7 |
| Accepted chronology (`AcceptedHlc`) | event chronology | pure chronology admission | supplies `RawWallObservation` | observation ≠ chronology ≠ order ≠ Cut | carries the clock-role ruling (2026-08-24) |
| Live deadline | runtime | deadline rebase from durable policy | supplies `RawMonotonicObservation`; carries the absolute deadline | no layer resets or extends | carries the deadline-never-resets law |
| Quarantine custody + disposition | event ingress | reject / plan quarantine | physically stores under the four guardrails | stored bytes never re-admitted directly | carries the rejected-content custody ruling; guardrails reaffirmed 2026-08-24 |
| Flow-control credit | runtime delivery | grant / consume / replenish / overrun | carried by wrappers | credit is never durable progress | carries the Serve-diaspora seating (2026-08-24) |
| Durable publication | event store contract | storage-family operations | adapter realizes the family | adapter success proves only the exact claim it establishes | carries what-owns-fact (ARCHITECTURE.md) |
| Ingress witnesses (received / validated / admitted) | event ingress | ingress ladder operations | wrappers carry the stage witness | no earlier stage discharges retry | carries the acknowledgment-ladder ruling (2026-08-24) |
| Live grant handle | Bvisor | grant validation + installation | `PortGrant` is the admitted authority record | a serialized grant is data, not custody | derives from role-specific-authority (rails §13) |

## Bounds

Port-family declarations carry their own limits, consumed at admission: `PortRequestByteLimit` (Output class), `PortResponseByteLimit` (Result class). Concurrency and outstanding-request ceilings are physical-admission dimensions enforced by Bvisor. Numeric values and paved profiles live in the depot. No cost or economic dimension is declared until a real consumer earns one.

## Refusals

Port refusals are role-specific and typed: contract-declaration refusals, request pre-flight refusals, response validation refusals, observation refusals, quarantine refusals. Their exact body shapes finalize in the type pass under the shared refusal mechanics. No package-wide error type exists.

## Hostile denominator

Each of these must be structurally impossible or refuse with a typed result, and each earns a harness case:

1. An untrusted response accepted without validation against the exact request, Attempt, role, and generation.
2. A response for one Attempt resuming or satisfying another Attempt, or a replayed duplicate response applied twice.
3. An adapter, retry, reconnect, or carrier conversion resetting or extending the absolute operation deadline.
4. A carrier or host type leaking into an ordinary ThreadPak signature.
5. A wrapper minting or strengthening a witness, receipt, reservation, credit, or grant it was only carrying.
6. A push payload treated as proof of completeness, checkpoint advancement, or sender authority.
7. Quarantined content re-admitted directly, bypassing fresh ingress validation.
8. A wall or monotonic observation impersonating a `CommitPoint`, `AcceptedHlc`, or any durable order.
9. A response accepted after lawful cancellation was observed for its Attempt, outside the operation's declared recovery contract.
10. A grant honored under a stale authority generation.

## Deliberately absent

No universal request envelope, no universal dispatcher, no universal clock trait, no universal recovery enum, no cost dimension, no wrapper types (wrappers are projections, not semantic types), no carrier vocabulary. Each absence is the law working, not a gap.

## Escalations

None. Every contract here carries or derives from a settled ruling; no fork requiring the repository owner's word was encountered.
