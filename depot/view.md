# depot — view owner rows

Selected facts consumed by the view owner's operations (`view/ops.rs`), passed
in explicitly per `depot/README.md` ("Rows are passed, never fetched"). Every
row carries binding time, change consequence, and status; recovered
contradictions are preserved as separate rows, never averaged. Machine-wide
canon selections (digest family, byte order, count widths) are the core canon
owner's rows, cited here, never restated.

## Pull-lane bounds (invocation/deployment-bound; change = refusal/default change)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| view.query-row-ceiling | `QueryRowLimit` value | withheld — recorded in the old corpus as `DEFAULT_QUERY_ROWS = …` with the value deliberately unstated; "return ten rows" appears only as an illustrative page teaching that a row cap is not a work bound | withheld | direction bank withheld-value ledger; old bounded-traversal law ("returning ten rows after scanning an unbounded source is not a bounded page operation") |
| view.query-work-denomination | `QueryWorkBudget` denomination and work-dimension roster | withheld — dimensions must be declared before any number means anything | withheld | old work-model law (portable work as a function of the affected input set; never CPU cycles or wall time) |
| view.navigation-depth-ceiling | `NavigationDepth` value | withheld | withheld | old navigation pipeline (names the depth cap, states no number) |
| view.relation-fanout-ceiling | `RelationFanOutLimit` value | withheld | withheld | same |

Consumers: `resolve`, `resolve_continue`, `rebuild`, `verify_parity` via
`ViewResolveProfile`. Nonclaims: a full page proves no completeness; an empty
page proves no absence beyond the frozen Cut. Falsifier:
read-everything-then-return-the-first-N — a limit that binds after decode,
join, sort, or materialization is theater.

## Selection rows (generation-bound for representation; change = new mechanism generation)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| view.mask-representation-roster | `SelectionRepresentation` roster | four qualified representations — DenseBitset, SparseIndices, Runs, InlineWord; conversion among them is a mask operation; membership meaning never varies by representation | candidate mechanism roster | old SelectionMask law, Part III/V ("dense bitset, sparse ordered indices, bounded runs/ranges, small-domain inline words") |
| view.mask-logical-length-width | `SelectionMask` logical-length width | **contradiction preserved**: the old public-surface sketch wrote `len: u64` (explicitly illustrative); the old class-level byte law says counts and lengths are u32, and every declared count bound here is `NonZeroU32` | contradiction — canon profile decides at the identity/canon pass | old ch11 Part V sketch vs old ch10 class-level byte law |
| view.mask-padding-law | representation-profile invariant | padding bits never enter membership, iteration, or complement — complement is bounded by logical length | candidate (law rider on the representation rows) | old SelectionMask law ("unused physical bits cannot select nonexistent rows") |
| view.selection-cardinality-ceiling | `SelectionCardinalityLimit` value | withheld | withheld | none stated |
| view.mask-operation-roster | closed mask-algebra roster consumed by grant admission and `ops.rs` | nine operations — empty/full over a declared length, membership, count, intersection, union, difference, bounded complement, ordered iteration, conversion among qualified representations | candidate closed roster | old SelectionMask operation roster |

Consumers: `derive_selection`, the compose family, `convert_selection`,
`resolve_protected`. Nonclaims: equal physical bit patterns never make
incompatible masks composable. Falsifier: composition across unproven-equal
row domains, mismatched Cuts, or physical padding surfacing as membership.

## Materialization rows (generation-bound; change = new materialization generation)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| view.materialization-byte-ceiling | `MaterializationByteLimit` value | withheld | withheld | old DataBlock chapter names byte ceilings, states no number |
| view.descriptor-bound-families | `MaterializationProfile` descriptor bound fields | five families a descriptor must carry so a reader refuses before allocating or decoding — decode, allocation, component, row, result; values unstated | candidate profile shape; values withheld | old DataBlock descriptor law ("binds every fact needed to reject stale, foreign, wrong-layout, wrong-domain, incomplete, or misinterpreted material") |
| view.datablock-layout-selection | layout/compression/chunking mechanism rows | withheld — old Tile/SIDX layouts and compression choices are quarry, explicitly not carried forward as selections | withheld (legacy mechanisms stay dead; fresh selections arrive with qualification evidence) | old ch11 physical-execution material; final-review disposition ("Tile as universal data object" stays dead) |

Consumers: the eight lifecycle operations via `MaterializationProfile`.
Nonclaims: a descriptor describing itself proves neither source existence nor
derivation correctness. Falsifier: allocation before the descriptor bound
check; new physical bytes presented as a newer `AppliedCut`.

## Subscription and temporal rows (deployment/invocation-bound)

| Row | Role | Value | Status | Source |
|---|---|---|---|---|
| view.subscription-window-ceiling | `SubscriptionWindowLimit` value | withheld | withheld | old broadcast retention/overrun contract — bounded observer memory, no numeric window |
| view.subscription-retention-horizon | a Time-class retention-expiry horizon on the durable subscription relationship | candidate — the recovered eleven-stage subscription lifecycle names "retention expiry" as a stage; no type exists yet and no value is stated | candidate; the type mints with its construction cut, not here | old subscription lifecycle ("declaration/admission … retention expiry, close/drain/terminal") |
| view.temporal-horizon-dimensions | `HorizonDimension` roster | two dimensions — admitted-event count, admitted span; never substituting or converting | ratified by contract (the roster is declared in `view/types.rs`) | current contract; old temporal law ("eventually within 50 events" appears only as an illustrative horizon) |

Consumers: `Subscription`, `advance_monitor` via `MonitorProfile`. Nonclaims:
a retained window is never progress; a horizon is never a truth value.
Falsifier: a slow observer consuming unbounded memory; an "eventually" with
no horizon settling `Violated` over an unclosed source set.

## Recovered rosters held as data (no owner type; resolution route noted)

| Row | Value | Status | Source |
|---|---|---|---|
| view.restricted-query-operations | twelve-operation read-only surface — bounded read/seek/filter/projection/fold/group/aggregate/match/traverse/join/order/page, plus structured explanation; "projection" here is the relational operator, not a noun | candidate closed roster consumed by read-grant admission when the restricted-query surface earns its cut | old restricted-query law ("producer identity, parsing success, a matching digest, or transport security grant no trust") |
| view.membership-view-kinds | six typed membership-view kinds one accepted event may participate in — entity, correlation, business process, effect, subscription, causal | candidate roster; kinds arrive only with real consumers (no universal ThreadId ever) | old logical-threads chapter |
| view.bound-dimension-membership | **contradiction preserved**: two recovered authoring-bounds rosters disagree on membership (one names rows/nodes/groups/matches/windows/recursion-edges/artifacts/deadline; the six-item roster omits several) | contradiction — the current owner map (bounds live with their consuming operations) decides; arithmetic on counts does not | old ch09 Part V vs its own six-item roster |

## Escalation rows

| Row | Value | Status |
|---|---|---|
| view.exact-history-read | `ExactHistoryRead` — the event owner's consumer-facing exact-read surface consumed by every pull-lane operation; declared by the event owner with its storage contract | closed at contract closure (assembly-reconciled) |
| view.definition-state-split | whether `View` lawfully hides ViewDefinition and ViewState | escalated — probe card issued with this packet; the owner rules |
