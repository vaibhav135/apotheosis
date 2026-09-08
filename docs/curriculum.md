# Curriculum architecture and status

This is the canonical source for Apotheosis's stages, difficulty model, workflow, progression, and roadmap status. Educational principles live in [`philosophy.md`](philosophy.md); exercise-construction rules live in [`exercise-authoring.md`](exercise-authoring.md).

## Documentation ownership

Each subject has one canonical owner:

| Subject | Canonical source |
|---|---|
| Human introduction and immediate next action | `README.md` |
| Educational philosophy and granularity | `docs/philosophy.md` |
| Curriculum stages, lifecycle, aggregate status, and roadmap | `docs/curriculum.md` |
| Agent startup behavior | `AGENTS.md` |
| Gate structure and scoring | `docs/llm-onboarding.md` |
| Exercise construction, testing, and status authority | `docs/exercise-authoring.md` |
| Track-local concept ownership and status | `docs/tracks/<track>.md` |
| Approved exercise behavior | The exercise's own `README.md` |
| Event Horizon | `docs/event-horizon.md` |
| The New Beginning | `docs/the-new-beginning.md` |

Other documents may summarize a rule for their audience but must link to its canonical owner instead of redefining it. Gate invariants are intentional checksums of canonical rules and must be updated and revalidated when those rules change.

If summaries and canonical sources conflict, report the conflict and stop for learner clarification; do not silently choose one. Current status summaries in the human README, onboarding gate, track specification, and this document must be updated together, with this document owning aggregate status and each exercise README owning its local lifecycle status.

## Curriculum journey

Apotheosis has four broad stages:

1. **Core curriculum:** practical Rust, common libraries, and difficult language features
2. **Advanced campaigns:** databases, compilers, distributed systems, performance, formal methods, and security
3. **Event Horizon:** one original Level 5 multidisciplinary path
4. **The New Beginning:** one reviewed and piloted contribution for a future learner

The journey is adaptive. Progress depends on demonstrated mastery rather than a fixed exercise count.

## Difficulty levels

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

The Unknown is a selective trial for Levels 1–3, not a difficulty level. It intentionally introduces a central unfamiliar domain and evaluates the research process:

```text
vocabulary → primary sources → experiments → mental model → design → implementation → postmortem
```

Levels 4–5 do not use the label because independent research is already inherent.

### Field Quest

Field Quest is optional real open-source work at any level. It is not a difficulty. Upstream acceptance cannot be required; success means producing a calibrated, technically defensible, review-ready contribution.

### The New Beginning

The New Beginning is post-Event-Horizon stewardship, not Level 6. The graduate creates, reviews, pilots, revises, and contributes one meaningful problem. See [`the-new-beginning.md`](the-new-beginning.md).

“The Void” is atmospheric language for the experience beyond ordinary exercises. It is not a difficulty, trial, stage, or lifecycle state.

## Core curriculum

The initial coverage direction includes:

1. Data formats and `serde`
2. Errors, APIs, and type-driven design
3. Iterators, files, parsing, and zero-copy processing
4. CLI applications and observability
5. Async Rust, networking, and services
6. Threads, synchronization, channels, and atomics
7. Advanced traits, lifetimes, macros, and compile-time design
8. Unsafe Rust, memory, FFI, and sound abstractions

These are planning areas, not equal-sized boxes. Concepts should move into the context where they are learned most naturally.

The detailed Rust coverage map includes:

- Smart pointers and interior mutability
- Closures and the `Fn`, `FnMut`, and `FnOnce` traits
- Advanced trait design, associated types, and generic associated types
- Trait objects, object safety, and dynamic dispatch
- Generics, `impl Trait`, monomorphization, and static dispatch
- Typestate and type-driven API design
- `Send`, `Sync`, threads, channels, locks, and atomics
- Futures, `Pin`, cancellation, async lifetimes, and backpressure
- Declarative and procedural macros
- Unsafe Rust, raw pointers, aliasing, provenance, and memory layout
- FFI, zero-copy techniques, allocation behavior, and performance trade-offs

## Current learning unit

The first active unit is [Reliable data boundaries with `serde`](tracks/data-and-serde.md).

| ID | Difficulty | Exercise | Status |
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
- LLM onboarding and comprehension gate: operational
- Exercise-authoring specification and template: documented
- First active track: reliable data boundaries with `serde`
- SERDE-01: scaffolded with no learner solution
- Later Serde entries: provisional
- Advanced campaigns: conceptual only
- Event Horizon: three paths documented; none active
- The New Beginning: required after Event Horizon
