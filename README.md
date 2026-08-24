# ThreadPak

ThreadPak is an embedded Rust event store and sync-first runtime for coordinate-native applications.

An accepted event is an immutable fact admitted at a typed semantic coordinate inside a named reference frame and under one explicit authority region and epoch. Programs are ordinary Rust values and pure bounded operations that read exact cuts of accepted history and produce event proposals plus explicit effect proposals.

A logical thread ties one command or intent to the events it admits, the views and decisions derived from those events, each logical Turn, each physical Attempt, and the role-specific receipts, checkpoints, retries, replay, and reconciliation that follow.

Queries can recompute exact results from accepted history. Views and subscriptions may maintain those results incrementally. Indexes, DataBlocks, caches, and physical plans accelerate the machine but never replace its authoritative history.

Macroonz removes repetitive Rust. It generates ThreadPak-facing implementations and descriptors, plus harness inputs for tests, mutation, faults, and benchmarks. ThreadPak owns all event, coordinate, program, runtime, port, and receipt semantics. Macroonz participates at build time and test time, never in runtime authority.

**Ordinary surface:** Store · Event · Coordinate · Query · View · Subscription · Program · Port · Receipt

**Expert surface:** Cut · Region · AuthorityEpoch · SelectionMask · DataBlock · ProgramImage · Turn · Attempt · Checkpoint · OutcomeUnknown · Reconciliation
