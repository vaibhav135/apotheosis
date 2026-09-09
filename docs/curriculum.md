# Curriculum architecture and status

This is the canonical source for Apotheosis's levels, curriculum classification, workflow, progression, and roadmap status. Educational principles live in [`philosophy.md`](philosophy.md); exercise-construction rules live in [`exercise-authoring.md`](exercise-authoring.md).

## Documentation ownership

Each subject has one canonical owner:

| Subject | Canonical source |
|---|---|
| Human introduction and immediate next action | `README.md` |
| Educational philosophy and granularity | `docs/philosophy.md` |
| Levels, curriculum classification, lifecycle, aggregate status, and roadmap | `docs/curriculum.md` |
| Agent startup behavior | `AGENTS.md` |
| Gate structure and scoring | `docs/llm-onboarding.md` |
| Exercise construction, testing, and status authority | `docs/exercise-authoring.md` |
| Track-local concept ownership and status | `docs/tracks/<track>.md` |
| Approved exercise behavior | The exercise's own `README.md` |
| Event Horizon | `docs/event-horizon.md` |
| The New Beginning | `docs/the-new-beginning.md` |

Other documents may summarize a rule for their audience but must link to its canonical owner instead of redefining it. Gate invariants are intentional checksums of canonical rules and must be updated and revalidated when those rules change.

If summaries and canonical sources conflict, report the conflict and stop for learner clarification; do not silently choose one. Current status summaries in the human README, onboarding gate, track specification, and this document must be updated together, with this document owning aggregate status and each exercise README owning its local lifecycle status.

## Level-first curriculum structure

**Level is the primary progression.** It describes the difficulty and engineering responsibility of the current exercise or milestone, not a permanent rank attached to the learner. Apotheosis remains adaptive: progress depends on demonstrated mastery rather than a fixed exercise count.

Within Levels 1–4, **Curriculum** describes the nature of the work:

- **Core:** build, test, release, and operate production systems using established Rust and ecosystem abstractions. Core requires understanding contracts, failure modes, and limitations; it is not a shallow library tour.
- **Advanced:** build, investigate, or establish stronger guarantees for machinery that Core learned to use. Advanced work is organized into campaigns such as storage engines, runtimes, compilers, distributed systems, verification, security, and hardware-level performance.

Level 3 is the deliberate overlap: both a demanding production subsystem and an introductory machinery-building campaign may require Crucible-level responsibility.

| Level | Curriculum or path |
|---|---|
| Level 1 — First Light | Core |
| Level 2 — Ascent | Core |
| Level 3 — Crucible | Core or Advanced |
| Level 4 — Abyss | Advanced |
| Level 5 — Event Horizon | One Event Horizon path |
| After Level 5 | The New Beginning |

Event Horizon is only Level 5, not a second Curriculum value. Its three paths and charter rules are defined in [`event-horizon.md`](event-horizon.md). The New Beginning is stewardship after the levels, not Level 6.

The shortest Core-versus-Advanced test is:

> Core uses established machinery responsibly. Advanced builds, investigates, or proves the machinery itself.

For example, using database migrations and transactions is Core; implementing write-ahead logging and crash recovery is Advanced. Using Tokio cancellation and backpressure is Core; implementing an executor or reactor is Advanced.

### Participation requirement

- **Required:** part of the mastery gate for the learner's current path.
- **Optional specialization:** production-relevant depth selected because it serves the learner's goals or project. It is not a level, a Curriculum value, or a synonym for easy work, and it does not automatically block progression.

For example, every Core learner needs byte-versus-text literacy, while locale-aware Unicode segmentation may be an optional Core specialization. Every Core learner needs unsafe-code literacy, while substantial direct FFI work may be an optional Core specialization. An optional specialization may still be Level 2 or Level 3.

Field Quest remains a separate optional designation for real open-source work. The Unknown remains a selective research trial for Levels 1–3. Neither changes an item's Level, Curriculum, or Participation value.

### Convergence checkpoint

A **Convergence checkpoint** is an occasional integration project or milestone that demonstrates a guarantee no preceding capability can establish alone. It is not a level, Curriculum value, participation requirement, lifecycle status, or mandatory checkpoint after a fixed number of exercises.

Convergence follows coherent capability clusters. Add one only when integration is itself the lesson; omit it when focused work already provides the necessary evidence.

## Levels

### Level 1 — First Light

A focused but non-trivial problem introducing one primary mechanism.

- Clear behavior and limited architecture
- Meaningful validation and edge cases
- At least one meaningful design decision
- Progressive concept and API hints

### Level 2 — Ascent

Several skills interact in a realistic task.

- Meaningful error handling
- Local API and data-structure choices
- Conceptual rather than procedural hints

### Level 3 — Crucible

A reusable production-like subsystem requiring defended design.

- Public API or trait design
- Multiple valid architectures
- Important failure and resource behavior
- Required design explanation

### Level 4 — Abyss

A bounded, adversarial, inherently research-heavy problem.

- Profound unfamiliarity
- Concurrency, safety, cancellation, performance, or other interacting guarantees
- Adversarial validation and failure analysis
- Sparse hints focused on failure modes

### Level 5 — Event Horizon

Original multidisciplinary engineering research under a reviewed project charter.

- The learner helps define correctness and scope.
- Reference models, trusted boundaries, threats, safety, and evidence are explicit.
- Scientific and performance claims require appropriate review.
- Requirements may evolve when research invalidates assumptions.

See [`event-horizon.md`](event-horizon.md) for the three original paths and complete rules.

## Special designations

### The Unknown

The Unknown is a selective trial for Levels 1–3, not another Level. It intentionally introduces a central unfamiliar domain and evaluates the research process:

```text
vocabulary → primary sources → experiments → mental model → design → implementation → postmortem
```

Levels 4–5 do not use the label because independent research is already inherent.

### Field Quest

Field Quest is optional real open-source work at any Level. It is not a Level itself. Upstream acceptance cannot be required; success means producing a calibrated, technically defensible, review-ready contribution.

### The New Beginning

The New Beginning is post-Event-Horizon stewardship, not Level 6. The graduate creates, reviews, pilots, revises, and contributes one meaningful problem. See [`the-new-beginning.md`](the-new-beginning.md).

“The Void” is atmospheric language for the experience beyond ordinary exercises. It is not a level, Curriculum value, trial, or lifecycle state.

## Core curriculum

Core develops production fluency through the ordered capability areas below. An area is a dependency and mastery boundary, not a fixed batch: it may require one exercise, several exercises, focused recovery work, or no new exercise when the learner already demonstrates transfer.

Capability areas use plain technical names because they organize the curriculum rather than name individual major problems. Major Level 3–5 problems still follow the philosophical-title rules in [`philosophy.md`](philosophy.md#difficulty-and-transformation).

### 1. Validated data and error design

**Governing question:** How should invalid external information become explicit domain meaning or a useful error?

Capabilities include:

- Wire representations, domain representations, validation, and canonicalization
- Syntax and structural decoding versus semantic validation
- Newtypes and type-driven invariants
- Structured library and domain errors, application context, and source chains
- `From`, `TryFrom`, and deliberate numeric conversions
- Integer widths, ranges, overflow policies, and basic floating-point failure cases
- Behavioral, adversarial, and property-based validation of meaningful invariants

SERDE-01 is the approved starting exercise for this capability area. Later Serde concepts remain provisional and may move to the area where their interaction is most authentic.

### 2. Files, text, binary data, and streaming I/O

**Governing question:** How can imperfect external data be processed incrementally without losing location, correctness, portability, or resource bounds?

Capabilities include:

- Bytes versus UTF-8 text, Unicode scalar values, and grapheme awareness
- Explicit rejecting versus lossy text conversion
- Fixed-width binary representation, endianness, and message framing
- `Read`, `Write`, `BufRead`, buffering, partial operations, flushing, and EOF
- `Path`, `PathBuf`, `OsStr`, `OsString`, and environment portability
- Iterators, parser composition, normalization, and bounded streaming
- RAII for non-memory resources, `Drop`, drop order, and explicit fallible cleanup
- Recoverable errors versus panics, unwind versus abort, and basic panic safety

Provisional legacy-normalization and synchronous-streaming Serde work fit naturally here when their contracts are approved.

#### Convergence 1: Reliable ingestion boundary

> Every accepted record becomes a valid domain value; every rejected record receives deterministic location-aware diagnostics; working memory is bounded independently of total input size.

Evidence should include malformed and large generated inputs, deterministic ordering, panic resistance, and an explicit stop-or-continue policy. Wall-clock timing is not proof of bounded memory.

### 3. CLI applications, configuration, and observability

**Governing question:** What turns correct library behavior into a program another person can run, operate, and diagnose?

Capabilities include:

- Typed arguments, subcommands, help, version, and validation
- Configuration sources, precedence, defaults, semantic validation, and secret separation
- Stable exit behavior and stdout-versus-stderr contracts
- Structured logs, traces, basic metrics, and bounded label cardinality
- Process spawning without shell interpolation
- Child-process ownership, output bounds, termination, waiting, and cleanup
- Deterministic integration tests with temporary resources and controlled dependencies
- Cargo packages, crates, targets, dependency kinds, profiles, and release-mode behavior

Provisional reliable-configuration Serde work may supply the file-decoding layer, but environment merging and application precedence remain integration concerns rather than hidden additions to a Serde contract.

#### Convergence 2: Failure-aware batch application

> Under documented filesystem and platform assumptions, invalid configuration or input cannot produce false success or an undocumented partial result, and an operator can determine what failed.

### 4. Reusable APIs and Cargo project architecture

**Governing question:** How can proven behavior become reusable without hiding ownership, failure, or compatibility consequences?

Capabilities include:

- Closures and the `Fn`, `FnMut`, and `FnOnce` traits
- Generics, `impl Trait`, monomorphization, and static dispatch
- Trait objects, dyn compatibility, and dynamic-dispatch trade-offs
- Associated types, selected generic associated types, coherence, and auto traits
- Lifetimes and deliberate borrowed-versus-owned API forms
- Smart pointers, interior mutability, and selected typestate
- Standard trait interoperability and public error design
- Rustdoc, doctests, compile-fail guarantees, and API limitation documentation
- Cargo workspaces, lockfiles, resolver behavior, features, supported combinations, editions, MSRV awareness, and SemVer

Abstractions should be extracted from behavior already built in earlier capability areas rather than invented to manufacture trait complexity.

### 5. Database use and transactions

**Governing question:** How can several state changes succeed or fail as one application-level operation?

Capabilities include:

- Using one mature relational database abstraction
- Parameterized queries, typed rows, schema constraints, and connection ownership
- Versioned migrations and an explicit deployment policy
- Transaction commit, rollback, and short transaction scope
- Expected constraint, contention, and availability failures
- Deterministic integration tests against the selected database when backend semantics matter

Core uses database guarantees responsibly. Storage engines, page layouts, write-ahead logging, crash recovery, MVCC, isolation proofs, replication, and distributed transactions belong to Advanced campaigns.

### 6. Threads, synchronization, and bounded work

**Governing question:** How can work proceed concurrently while remaining bounded, owned, and accountable?

Capabilities include:

- `Send`, `Sync`, ownership transfer, and scoped threads
- Channels, mutexes, read-write locks, and shared-state trade-offs
- Bounded queues, admission policy, and overload behavior
- Worker ownership, shutdown protocols, and failure propagation
- Result ordering and accounting guarantees
- Atomic vocabulary and safe local uses, including when a lock or channel is preferable

#### Convergence 3: Bounded worker system

> Queued and active work are bounded; every accepted item is accounted for according to a documented shutdown policy; promised result ordering is preserved.

Lock-free data structures and nontrivial memory-ordering proofs belong to Advanced campaigns.

### 7. Async networking and services

**Governing question:** How can a network service remain correct when clients, dependencies, tasks, and shutdown stop at different times?

Capabilities include:

- Established async runtime and HTTP client/server abstractions
- Task ownership, joining, failure observation, and blocking-work boundaries
- Timeouts, cooperative cancellation, and graceful shutdown
- Bounded admission, channels, request bodies, and backpressure
- Explicit retry and idempotency policies
- Stable request/response errors and trusted data boundaries
- Request tracing, health versus readiness, and secret-safe telemetry
- Controlled local service substitutes and layered integration tests
- Enough `Future` and `Pin` understanding to use and diagnose established APIs safely

Provisional tagged-event and missing-versus-`null` Serde concepts fit naturally at this API boundary. Network transport remains outside the Serde-specific contract.

#### Convergence 4: Bounded cancellation-aware service

> Under a declared shutdown and dependency model, the service bounds admitted work, propagates cancellation and timeouts, returns stable boundary errors, protects secrets, and leaves no unowned background work.

Executor, reactor, scheduler, protocol-stack, and manual pin-projection internals belong to Advanced campaigns.

### 8. Compatibility and release maintenance

**Governing question:** How can deployed interfaces and artifacts evolve without corrupting current domain meaning or surprising their users?

Capabilities include:

- Private version-specific wire models and explicit migration into one current domain model
- Compatibility, canonical-output, deprecation, and public-error policies
- Dependency updates, advisories, licenses, lockfile discipline, and feature matrices
- CI for formatting, linting, tests, documentation, supported toolchains, and declared platforms
- Release artifacts, smoke tests, operational documentation, and rollback expectations
- Basic benchmarking and profiling with representative release-mode workloads
- Claims limited by documented environment, uncertainty, and measurement scope

Provisional versioned-protocol Serde work fits naturally here when its contract is approved.

#### Convergence 5: Evolvable production system

> Supported historical inputs preserve their declared meaning, generated output follows one canonical version, and the released artifact can be built, tested, operated, upgraded, and diagnosed under documented assumptions.

### 9. Unsafe Rust and safe abstraction boundaries

**Governing question:** How can a narrow unsafe or foreign operation support a safe public abstraction?

Capabilities include:

- Reading unsafe APIs and identifying the obligations their callers must uphold
- Layout, validity, aliasing, lifetime, ownership, and thread-safety reasoning
- Keeping unsafe operations narrow and exposing a safe public contract
- One bounded unsafe or foreign wrapper with a written safety argument
- Panic and unwind considerations at trusted boundaries
- Miri or other appropriate supporting validation, with explicit recognition that tools do not replace the safety proof

Allocators, complex unsafe containers, deep provenance research, substantial FFI systems, and hardware-specific optimization belong to Advanced campaigns.

### Cross-cutting Core disciplines

The following mature throughout Core rather than appearing once at the end:

- Security and hostile-input/resource bounds
- Deterministic testing and reproducibility
- Documentation and limitation analysis
- Portability and explicit platform assumptions
- Observability and secret-safe diagnostics
- Cargo, dependency, and release stewardship
- Measurement before optimization

### Optional Core specializations

Optional specialization follows learner goals and project needs. Examples include:

- Crate publishing, advanced packaging, build scripts, and native dependency discovery
- Substantial direct FFI, `no_std`, and embedded systems
- Procedural-macro authoring
- Unicode normalization, segmentation, collation, and locale-aware text
- Decimal, fixed-point, arbitrary-precision, or units-aware arithmetic
- Multiple database backends or specialized network protocols
- OpenTelemetry and advanced telemetry pipelines
- Authentication protocols, cloud secret managers, and platform service integration
- Sustained fuzzing campaigns and specialized profiling

Using ecosystem and procedural macros is Core; authoring a substantial procedural macro is optional. Unsafe-code literacy and one narrow safe-wrapper argument are Core; broad platform-specific FFI is optional.

## Current learning unit

The first active unit is [Reliable data boundaries with `serde`](tracks/data-and-serde.md).

| ID | Level | Exercise | Status |
|---|---|---|---|
| SERDE-01 | Level 1 — First Light | Job Manifest Codec | Scaffolded |
| SERDE-02 | Level 1 — First Light | Reliable Configuration | Provisional |
| SERDE-03 | Level 1 — First Light | API Events | Provisional |
| SERDE-04 | Level 2 — Ascent | Legacy Data Normalizer | Provisional |
| SERDE-05 | Level 2 — Ascent | Streaming Records | Provisional |
| SERDE-06 | Level 3 — Crucible | Versioned Protocol | Provisional |

Only SERDE-01 has an approved complete contract. Later entries are hypotheses that may be merged, moved, split, reordered, or removed after granularity review and learner feedback.

The current learner action is to implement SERDE-01. Do not scaffold SERDE-02 unless the learner explicitly changes direction or SERDE-01 has been reviewed.

## Advanced campaigns

The core alone is not sufficient preparation for Event Horizon. Advanced work must eventually address:

1. Storage engines, transactions, and database correctness
2. Compilers, interpreters, virtual machines, and runtimes
3. Distributed systems, replication, consensus, and deterministic simulation
4. Performance engineering, computer architecture, and low latency
5. Mathematical reasoning, formal methods, model checking, and verification
6. Security, sandboxing, unsafe-code auditing, and hostile-input engineering

Campaigns are authored only when prerequisites and readiness are demonstrated. Each defines its own primary-source map, validation methods, safety requirements, and exit gates.

## Event Horizon

After the prerequisite campaigns and **Death of Ego**, the learner chooses one active path:

| Path | Focus |
|---|---|
| Iron Meridian — The Architecture of Trust | Low-latency systems, databases, compilers, distribution, hardware, statistics, and economics |
| Genesis — The Living Equation | Biology, chemistry, physics, simulation, numerical methods, statistics, compilers, and parallel systems |
| The Observer — The Limits of Knowing | Quantum mechanics, mathematics, compilers, verification, simulation, and high-performance computing |

Completing one path completes Event Horizon. Switching requires a formal Exit Review. Exact charters remain intentionally unfrozen until prerequisites are complete.

## Learning-unit planning

There is no mandatory number of exercises per unit, track, or campaign.

- Plan one coherent learning unit at a time.
- Implement and review one approved exercise or milestone at a time.
- Give each exercise one governing question.
- Split when concepts need independent feedback or obscure one another.
- Keep together when their interaction is the lesson.
- Move concepts into a better practical context when appropriate.
- Remove goals that exist only to manufacture density.

The canonical granularity philosophy is in [`philosophy.md`](philosophy.md); authoring decisions are governed by [`exercise-authoring.md`](exercise-authoring.md).

## Exercise lifecycle

```text
Planned → Scaffolded → In Progress → Tests Pass → Reviewed → Mastered
```

- **Planned:** broad objective and sequence position agreed; contract may be incomplete
- **Scaffolded:** approved contract, starter code, visible tests, and compile validation exist
- **In Progress:** learner implementation has begun
- **Tests Pass:** current tests, formatting, and Clippy pass
- **Reviewed:** correctness and code-quality findings are resolved
- **Mastered:** explanation, diagnosis, transfer, and limitation checks pass

`Provisional` is a roadmap planning label, not an exercise lifecycle status. It indicates that an idea has not yet reached Planned.

- **Provisional:** an unapproved hypothesis; its ID, position, level, scope, and existence may change.
- **Planned:** the learner or designated evaluator has approved its broad objective and sequence position; its exact contract may remain incomplete.

Only the learner or designated evaluator promotes a Provisional item to Planned. The same authority confirms later lifecycle transitions after their entry criteria are verified. An LLM may record the transition only when designated as evaluator or after explicit confirmation.

## Solution and review workflow

1. Read the exercise contract and visible tests.
2. Implement without receiving a completed reference solution.
3. Run formatting, linting, and tests.
4. Submit the implementation for review.
5. Review correctness, idiomatic Rust, error handling, and the learning objective.
6. Add contract-preserving adversarial tests when useful.
7. Let the learner attempt identified fixes before rewriting.
8. Run the mastery gate and update status.

## Validation

Runtime black-box behavior is the default. Depending on the stated lesson, validation may also use:

- Compile-pass and compile-fail tests
- Structured error and trait-bound checks
- Property-based tests and fuzzing
- Mock services, clocks, and deterministic simulation
- Timeouts and bounded-concurrency checks
- Benchmarks and allocation constraints
- Miri, Loom, Kani, and other specialized tools
- Design explanations and documented unsafe-code safety invariants
- Scientific reference models and uncertainty analysis

Correctness tests must be deterministic by default and portable wherever practical. Any unavoidable nondeterminism requires approval, controlled seeds or bounded tolerance, reproducibility measures, and documented limits. Hardware-specific performance gates must document reference hardware, pinned workloads, uncertainty, and limitations. Unsupported environments require a portable reference or simulator path. Never infer environment-specific performance claims from ordinary CI.

## Definition of exercise completion

Unless an exercise states otherwise:

```bash
cargo test --manifest-path exercises/<track>/<exercise>/Cargo.toml
cargo fmt --manifest-path exercises/<track>/<exercise>/Cargo.toml --check
cargo clippy --manifest-path exercises/<track>/<exercise>/Cargo.toml -- -D warnings
```

Completion also requires documented behavior, no unexplained panics or unsafe code, any level-specific artifacts, and compliance with explicit learning constraints. Every exercise that permits unsafe code must document its safety invariants and explain why they hold.

## Mastery and adaptive progression

Mastery requires the learner to:

- Pass behavioral and adversarial validation
- Explain central concepts in their own words
- Defend important choices and trade-offs
- Diagnose an intentionally broken variation
- Adapt to a meaningful changed requirement
- Identify limitations and failure boundaries

Add recovery work when misconceptions remain, mutations when designs are fragile, and stronger constraints when work is materially easier than calibrated. Reduce repetition when understanding is demonstrated. Never advance solely because a count was reached.

Every track or campaign defines:

- **Minimum mastery:** smallest defensible transferable skill set
- **Recommended mastery:** intended preparation for dependent work
- **Research-depth mastery:** optional specialization

## Repository structure

Each ordinary exercise is an independent Cargo crate:

```text
exercises/
└── <track>/
    └── <number>-<exercise-name>/
        ├── Cargo.toml
        ├── Cargo.lock
        ├── README.md
        ├── src/
        └── tests/
```

Detailed metadata, testing, hints, artifacts, and LLM decision boundaries are defined in [`exercise-authoring.md`](exercise-authoring.md).

## Toolchain and dependency policy

- Stable Rust and Edition 2024 are the default.
- `rust-toolchain.toml` pins the repository toolchain.
- Nightly requires an explicit exercise need.
- Independent exercise crates commit `Cargo.lock`.
- Dependencies must serve the lesson. Upgrades must be deliberate and tested, and changes to APIs or observable behavior must be recorded.

## Starting primary sources

- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)
- [Serde](https://serde.rs/)
- [Tokio](https://tokio.rs/)
- [Miri](https://github.com/rust-lang/miri)
- [Loom](https://github.com/tokio-rs/loom)
- [Kani](https://model-checking.github.io/kani/)
- [SQLite: Atomic Commit](https://sqlite.org/atomiccommit.html)
- [FoundationDB: Simulation and Testing](https://apple.github.io/foundationdb/testing.html)
- [TigerBeetle: Safety](https://docs.tigerbeetle.com/concepts/safety/)
- [Raft paper](https://www.usenix.org/conference/atc14/technical-sessions/presentation/ongaro)
- [LLVM Language Reference](https://llvm.org/docs/LangRef.html)
- [Cranelift IR](https://docs.rs/cranelift-codegen/latest/cranelift_codegen/ir/)
- [Jane Street: How to Build an Exchange](https://www.janestreet.com/tech-talks/building-an-exchange/)
- [Jane Street: System Jitter and Where to Find It](https://www.janestreet.com/tech-talks/system-jitter-and-where-to-find-it/)
- [Nasdaq TotalView-ITCH specification](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHspecification.pdf)

Exercise and campaign documents cite the exact authoritative subset they use.

## Current status

- Philosophy and curriculum constitution: documented
- LLM onboarding and comprehension gate: Level-first revision documented; independent revalidation pending
- Exercise-authoring specification and template: documented
- First active track: reliable data boundaries with `serde`
- SERDE-01: scaffolded with no learner solution
- Later Serde entries: provisional
- Advanced campaigns: conceptual only
- Event Horizon: three paths documented; none active
- The New Beginning: required after Event Horizon
