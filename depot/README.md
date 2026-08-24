# depot

Data-shaped truth. The depot answers one question: **which exact values, profiles, tags, vectors, and rosters were selected** — stated as data, meaning owned elsewhere.

Every depot entry names its consuming owner. The depot never defines meaning, never computes, and never holds behavior. Git is the depot's history; there is no append-only lifecycle machinery here.

## What the depot holds

- **Limit values and paved default profiles** for owner-named bound types. The types live with the operations that consume them (event, view, program, runtime); the depot holds their selected values (ARCHITECTURE.md — Bounds).
- **Precision profiles** — which exactness, scale, rounding, and quantization a numeric operation was configured with. Numeric code performs the arithmetic; the profile states the selection.
- **Domain-separation tags and wire profile identifiers** — as generated projections of their owning register, never hand-maintained.
- **Golden vectors** — what correct output is for a canonical operation. Core canon computes; the vector states the expected result.
- **Hostile input descriptions and fault scenario descriptions** — each names the exact refusal or recovery behavior it must provoke in its owner.
- **Refusal prose** — the human sentences keyed to typed refusals. The refusal types and their conditions live with their owners; prose adds no variant and no condition.
- **Closed rosters** — the data form of a set an owner declared closed.

## What the depot never holds

Algorithms. Branches. State machines. Runtime-generated evidence. Qualification workflows. Anything that computes, decides, or changes at runtime. A depot file that computes anything is in the wrong home.

## Law, configuration, mechanism, release

ThreadPak law defines each complete lawful algebra. An application or deployment selects a coordinate inside that algebra through a depot profile. A qualified mechanism realizes the selection. A release row promises only what evidence supports. The depot records selections; it never invents an axis, and a profile can never widen the algebra its owner declared.

## Defaults are classified

Every default profile carries exactly one classification, recorded with the profile:

- **asymmetric** — a paved default exists and an explicit override is lawful;
- **symmetric** — equally safe alternatives; no default row exists, the interface selects; the depot may still hold the named selectable profiles;
- **safety-relevant** — the default is the strict or refusing posture.

A default row without a classification is invalid. The classification is carried from its owner's ruling; the depot never assigns it.

## Crossings

Stated per the no-orphan rule (ARCHITECTURE.md — No orphan by distribution): fact, owner, establishing operation, depot projection, substitution refusal, chronology.

| Fact | Owner | Establishing operation | Depot projection | Substitution refusal | Chronology |
|---|---|---|---|---|---|
| A bound's selected value | The bound type's owner (event, view, program, runtime) | The consuming operation's validated construction | Limit value / default profile row | A value outside the owner's declared type refuses at construction; the value never becomes the bound's meaning | Carries ARCHITECTURE.md — Bounds |
| Expected output of a canonical operation | Core canonical-encoding owner | The canonical operation itself | Golden vector | A vector never substitutes for the operation's result; disagreement is evidence against the implementation, and a vector changes only by its owner's act | Carries ARCHITECTURE.md — rail 12 |
| Domain-separation tag / wire profile identifier | The identity register's owner | The register's declaration | Generated tag and identifier tables | A hand-edited projection is invalid; projections regenerate from the register so they cannot drift | Carries the settled register-projection ruling |
| Hostile input / fault scenario | The owner whose refusal it targets | That owner's refusal contract | Description row consumed by the independent Macroonz harness | A hostile row asserts no law; it names the refusal it must provoke | Carries ARCHITECTURE.md — No orphan by distribution |
| Closed roster membership | The declaring owner | The owner's closed declaration | Roster data | A roster edit without its declaring owner's declaration is invalid; no mirror of an owner's roster exists | Carries ARCHITECTURE.md — Owners are not directories |
| Refusal prose | The refusal family's owner | Typed refusal construction | Prose keyed by refusal identity | Prose adds no variant, condition, or meaning | Carries ARCHITECTURE.md — rail 13 |

## Hostile denominator

Each of these must be refused or made impossible, and each has a description row here once its owner's contract lands:

1. A profile value that contradicts its owner's declared bound type.
2. A law sentence smuggled into a data file.
3. A roster edited without its declaring owner.
4. A depot file that computes anything.
5. A hand-maintained mirror of an owner's roster.

Concrete depot files arrive with their consuming owners. No file exists here ahead of the owner that reads it.
