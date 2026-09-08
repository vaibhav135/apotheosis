# Apotheosis

> From First Light, through the Event Horizon, to a New Beginning.

Apotheosis is a practical, exercise-driven Rust curriculum for someone who already understands the language fundamentals, including ownership, borrowing, traits, and `impl` blocks.

The goal is not merely to pass small syntax puzzles. The goal is to become capable of building production-style Rust software, understanding difficult Rust code, diagnosing compiler errors, and designing safe, idiomatic abstractions.

The name describes transformation, not omniscience. Completion does not place anyone above evidence, correction, or continued learning. It should produce someone capable of entering unfamiliar domains with humility, learning rigorously, building carefully, and allowing the work to speak for itself.

> You do not emerge knowing everything. You emerge afraid of nothing you do not yet know.

## Navigation

- [LLM entrypoint](#llm-entrypoint)
- [Learning outcomes](#learning-outcomes)
- [Teaching philosophy](#teaching-philosophy)
- [Curriculum constitution](#curriculum-constitution)
- [Philosophy review protocol](#philosophy-review-protocol)
- [Difficulty levels](#difficulty-levels)
- [Philosophical naming and reflection](#philosophical-naming-and-reflection)
- [The Unknown](#the-unknown)
- [Pinnacle philosophy](#pinnacle-philosophy)
- [Event Horizon paths](#event-horizon-paths)
- [The New Beginning](#the-new-beginning)
- [Curriculum organization](#curriculum-organization)
- [Repository and exercise structure](#repository-structure)
- [Solution and review workflow](#solution-and-review-workflow)
- [Validation and completion](#validation-methods)
- [Mastery and adaptive progression](#mastery-and-adaptive-progression)
- [Starting primary sources](#starting-primary-sources)
- [Instructions for future LLMs](#instructions-for-future-llms)
- [Current status](#current-status)

## LLM entrypoint

An AI agent receiving this repository without conversation history must begin with [`AGENTS.md`](AGENTS.md) and pass the semantic gate in [`docs/llm-onboarding.md`](docs/llm-onboarding.md) before modifying the repository.

Exercise authors must also read [`docs/exercise-authoring.md`](docs/exercise-authoring.md), the relevant specification under [`docs/tracks/`](docs/tracks/), and every file belonging to the exercise being changed.

## Learning outcomes

The curriculum must develop both of these areas together:

1. **Applied Rust:** using common crates and techniques to solve realistic problems.
2. **Hard Rust:** understanding the language features and design constraints that make non-trivial Rust programs safe and correct.

Completing the curriculum should provide practical experience with:

- Serialization and data formats
- Command-line applications
- Error design and recovery
- File, iterator, and stream processing
- HTTP clients, servers, and external APIs
- Async Rust and cancellation
- Concurrency and synchronization
- Testing, mocking, and observability
- Advanced borrowing and lifetimes
- Smart pointers and interior mutability
- Closures and function traits
- Advanced traits, associated types, and generic associated types
- Trait objects, object safety, and dynamic dispatch
- Generics, `impl Trait`, and static dispatch
- Typestate and type-driven API design
- `Send`, `Sync`, channels, locks, and atomics
- Futures, `Pin`, async lifetimes, and backpressure
- Declarative and procedural macros
- Unsafe Rust, raw pointers, memory layout, and sound abstractions
- FFI, zero-copy techniques, and performance trade-offs
- Mathematical reasoning about correctness, probability, latency, and resource bounds
- The physical and hardware realities beneath software abstractions

This list is a coverage map, not a promise to cover every crate or every corner of Rust. New topics should be added when they teach transferable, production-relevant skills.

## Teaching philosophy

- Exercises use realistic scenarios rather than isolated syntax trivia.
- The learner writes the implementation.
- Tests primarily validate observable behavior rather than prescribing internals.
- Implementation constraints are permitted when a particular technique is the learning objective.
- Difficult topics may require design explanations, compile-time checks, benchmarks, or safety arguments in addition to passing runtime tests.
- Exercise batches are deliberately small so future work can respond to the learner's progress.
- The curriculum favors actively maintained, commonly used crates but teaches concepts rather than crate-specific trivia.
- Introductory exercises remain challenging: they isolate a new concept but still require validation, edge-case reasoning, and meaningful design decisions.
- Every ecosystem track begins with challenging first-use exercises before progressing into production usage, failure handling, integration, and advanced design.
- Advanced work must be difficult because of authentic guarantees, not hidden requirements, obscure trivia, or intentionally hostile instructions.

## Curriculum constitution

These principles outrank dramatic naming, exercise count, and attachment to any particular roadmap:

1. **Learning over suffering:** frustration is not evidence of progress.
2. **Transformation over completion:** changed reasoning matters more than finishing a checklist.
3. **Transfer over test-passing:** mastery must survive changed requirements and unfamiliar variations.
4. **Coherence over breadth:** every concept and discipline must support a real learning objective or system guarantee.
5. **Evidence over confidence:** correctness and performance claims require reproducible support.
6. **Bounded guarantees over infinite ambition:** difficult work still needs explicit scope, non-goals, and completion conditions.
7. **Primary sources over invented authority:** correctness-critical claims must trace to authoritative material.
8. **Scientific humility over impressive claims:** software correctness does not automatically establish scientific truth.
9. **Commitment without destructive sunk costs:** changing direction requires evidence and reflection, but must remain possible.
10. **Safety and ethics before technical spectacle:** unsafe, financial, biological, security, and dual-use work requires explicit boundaries.
11. **Reproducibility before performance claims:** measurements must document workloads, environments, uncertainty, and limitations.
12. **Stewardship after mastery:** advanced learners improve the path for those who follow.

## Philosophy review protocol

The curriculum itself must remain challengeable. Before a major roadmap expansion, after each completed track, and before freezing any Event Horizon specification, perform this review:

1. **Restate the claim:** what ability is this work supposed to produce?
2. **Construct the theory of change:** connect activities to observable learning outcomes.
3. **Red-team it:** search for ambiguity, artificial difficulty, unsafe incentives, inaccessible requirements, and ways to game validation.
4. **Run a premortem:** assume the track or project failed educationally and identify plausible causes.
5. **Define falsification evidence:** state what observations would prove the current design ineffective.
6. **Test coherence:** require every crate, concept, scientific domain, and operational constraint to justify its presence.
7. **Test feasibility:** document time, hardware, money, prerequisite knowledge, and external-review requirements.
8. **Pilot before scaling:** validate the smallest useful version before generating a large campaign.
9. **Record the conclusion:** update this README with durable decisions, rejected assumptions, and changed policies rather than conversational history.

Warning signs that require revision include:

- Learners pass tests but cannot solve related variations.
- Difficulty creates confusion without improving reasoning.
- A discipline cannot be connected to a concrete guarantee.
- Scientific claims cannot be independently validated.
- Completion depends primarily on expensive hardware or services.
- The roadmap grows faster than the learner's demonstrated readiness.
- Names and spectacle become more important than technical substance.
- New exercises repeatedly require undocumented assumptions.

## Difficulty levels

Difficulty is determined by ambiguity, interacting concepts, design responsibility, failure modes, and performance constraints—not simply by lines of code.

### Level 1 — First Light

Introduces one crate or technique in a focused but non-trivial problem. First Light still requires validation, edge-case reasoning, and at least one meaningful design decision.

- One primary concept
- Clear input and expected output
- Focused but meaningful edge cases
- Progressive API and concept hints
- Usually one function or module

Example: derive serialization for a job manifest while manually implementing `Serialize` and `Deserialize` for a validated identifier newtype.

### Level 2 — Ascent

Combines multiple skills in a realistic task.

- Two or three interacting concepts
- Meaningful error handling
- Several edge cases
- Some data-structure and API decisions
- Conceptual rather than step-by-step hints

Example: process newline-delimited JSON asynchronously, tolerate malformed records, and calculate statistics.

### Level 3 — Crucible

Requires the learner to design a reusable, production-like component.

- Multiple interacting components
- Public API or trait design
- Important failure behavior
- Mocks, unusual inputs, or resource constraints
- Small, optional hints
- A short design explanation

Example: implement an async paginated API client with typed errors, retries, and configurable timeouts.

### Level 4 — Abyss

Models a research-heavy real-world problem where naive solutions are observably incorrect or unsafe. Independent investigation is inherent at this level.

- Complex ownership, concurrency, or async behavior
- Cancellation and partial failures
- Performance or memory constraints
- Adversarial validation
- Hints describe failure modes rather than solutions
- A required design explanation

Example: build a bounded concurrent ingestion pipeline that preserves ordering, applies backpressure, respects cancellation, and shuts down cleanly.

### Level 5 — Event Horizon

Event Horizon is the ultimate difficulty: an open-ended grand engineering challenge integrating several deep disciplines into one coherent system.

At this level, the learner helps define:

- What correctness means
- Which guarantees are both valuable and achievable
- The system's failure and threat models
- The architecture and trusted boundaries
- How correctness, safety, and performance will be validated
- Which trade-offs are acceptable and how they will be measured

Requirements may evolve after design review, and discoveries in later phases may invalidate earlier architecture. Difficulty must come from making demanding guarantees hold simultaneously—not hidden tests, arbitrary scope, impossible targets, or undocumented assumptions.

## Philosophical naming and reflection

Major exercises from Level 3 onward should pair an evocative philosophical title with a precise technical subtitle:

```text
<Philosophical title> — <Technical subtitle>
```

The title must express the lesson's intended transformation rather than merely sound dark or difficult. The subtitle must keep the task technically clear and searchable.

Each such exercise should state:

- Its technical objective
- Its philosophical intent
- The expected change in the learner's mental model
- Reflection questions to answer after implementation

Examples include **The Weight of Time — Build a Fault-Tolerant Logical Clock**, **Many Worlds — Explore Concurrent Interleavings**, and **A Promise Written in Stone — Design Crash-Consistent Storage**.

The opening Event Horizon milestone is reserved as **Death of Ego — Define the Machine You Claim to Build**. Before optimized implementation, the learner must inventory known facts and assumptions, define the system's guarantees and invariants, identify unknown domains, construct reference models, and accept the scale of what remains to be learned.

“Death of ego” means abandoning the expectation of already knowing—not destroying confidence or tying personal worth to difficulty. The intended progression is humility, investigation, understanding, and responsible action.

## The Unknown

The Unknown is a selective trial format for Levels 1–3. It introduces an unfamiliar technical domain whose concepts and architecture must be researched before implementation can begin.

- The first task is learning the vocabulary and constructing a mental model
- Requirements are precise, but no tutorial or implementation recipe is supplied
- Research notes, small experiments, and a proposed architecture precede the implementation
- Progressive rescue hints move from concept names to documentation, decomposition, API anchors, and finally partial scaffolding
- Completion includes a postmortem describing failed assumptions and newly acquired understanding

Examples include building a minimal async runtime, implementing a Serde data format, creating a procedural macro with span-aware diagnostics, or designing a safe cross-language ownership boundary.

The Unknown is not enabled or disabled on an existing exercise. A problem is deliberately designed as an Unknown trial and declares that status in its README. It describes epistemic difficulty—the challenge of entering unfamiliar territory—rather than implementation size alone.

Levels 4 and 5 do not use this label because independent research and profound unfamiliarity are already part of their definitions. The Unknown should remain rare—approximately 3–5 exercises across the complete curriculum—so it remains a meaningful test of learning how to learn.

## Pinnacle philosophy

The universe of computing is too large for any curriculum—or any person—to master completely. Event Horizon exists to provide a tiny but honest taste of that scale.

Completion does not confer the title of “absolute programmer.” Instead, it demonstrates a more defensible form of mastery: the ability to enter unfamiliar domains, construct accurate mental models, integrate knowledge across disciplines, test assumptions, and defend a complex system under adversarial review.

Engineering remains central, but the final challenge should draw meaningfully from the following areas where they support the system.

### Mathematics

- Logic, invariants, and proof-oriented reasoning
- Discrete mathematics, graphs, state machines, and partial orders
- Probability, statistics, and experimental design
- Queueing theory and latency distributions
- Fixed-point arithmetic, numerical error, and overflow analysis
- Optimization, cost models, and resource allocation
- Information theory, encoding, checksums, and compression trade-offs
- Formal models of concurrency, consistency, and program refinement

### Science and physical computing

- Clocks, oscillator drift, synchronization, and timestamp uncertainty
- Signals, noise, encoding, and error detection
- CPU pipelines, caches, branch prediction, and memory hierarchies
- Networks, NICs, interrupts, polling, and kernel bypass
- NUMA, PCIe, storage media, and durability boundaries
- Power, thermal behavior, frequency scaling, and system jitter
- The distinction between an abstract machine and the physical machine executing it

### Engineering disciplines

- Programming languages, compilers, interpreters, and runtimes
- Databases, transactions, storage engines, and crash recovery
- Operating systems, networking, and hardware-aware programming
- Distributed systems, replication, consensus, and fault tolerance
- Security, sandboxing, unsafe-code auditing, and hostile-input handling
- Low-latency design, observability, benchmarking, and performance diagnosis
- Testing through simulation, model checking, fuzzing, differential execution, and fault injection

The project must not include disciplines merely to make the list longer. Every component must support a real system guarantee.

## Event Horizon paths

The complete philosophy, shared engineering spine, path requirements, and validation rules are maintained in [`docs/event-horizon.md`](docs/event-horizon.md).

All paths begin with **Death of Ego — Define the Machine You Claim to Build**. The learner studies all three, then commits to one active path:

| Path | Philosophical focus | Technical world |
|---|---|---|
| **Iron Meridian — The Architecture of Trust** | Promises across time, competition, and failure | Low-latency systems, databases, compilers, distributed state, hardware, statistics, and economics |
| **Genesis — The Living Equation** | Emergence, inference, and scientific humility | Biology, chemistry, physics, simulation, numerical methods, statistics, compilers, and parallel systems |
| **The Observer — The Limits of Knowing** | Information, measurement, and uncertainty | Quantum mechanics, mathematics, compilers, verification, simulation, and high-performance computing |

Completing one path completes Event Horizon; completing all three is not required. A path may be changed only through a formal Exit Review. Every path requires a bounded charter, common engineering foundations, original work, reproducible evidence, safety boundaries, and domain-appropriate external review.

“The Void” is atmospheric language for the experience beyond ordinary exercises, not another level or mode.

## The New Beginning

The New Beginning is the required epilogue after completing Event Horizon. It is not another difficulty level.

Its complete philosophy, contribution format, quality rubric, and acceptance lifecycle are maintained in [`docs/the-new-beginning.md`](docs/the-new-beginning.md).

Every learner who completes the grand project must leave behind one meaningful problem for a future learner. The problem should arise from a misconception, blind spot, failed design, or profound insight encountered during the curriculum—not from a desire to manufacture arbitrary difficulty.

The graduate must:

1. Explain why the problem deserves to exist and what transferable concept it teaches.
2. Assign an appropriate track, prerequisite chain, and difficulty from Level 1 through Level 5. Another Event Horizon problem must satisfy the complete multidisciplinary standard rather than merely being very large.
3. Decide whether it qualifies as The Unknown or a Field Quest.
4. Write a precise behavioral contract, starter scaffold, progressive hints, and validation strategy.
5. Include tests without including a completed starter solution.
6. Have the exercise reviewed for fairness, correctness, scope, and educational value.
7. Pilot it with another learner and revise it using observed confusion and failure modes.
8. Update the curriculum roadmap so the next learner can discover it.

The New Beginning makes the curriculum generative: reaching its end creates a thoughtful beginning for someone else.

A New Beginning exercise uses a separate acceptance lifecycle:

```text
Proposed → Reviewed → Piloted → Revised → Accepted
```

It remains outside the main curriculum until completing this lifecycle. Event Horizon completion demonstrates engineering mastery, not automatic mastery of teaching or curriculum design.

## Curriculum organization

The curriculum has two preparatory stages, the Level 5 Event Horizon project, and The New Beginning epilogue.

### Stage 1: Core curriculum

The initial roadmap should contain approximately **32 core exercises across 8 coherent tracks**. This is a starting scope, not a fixed limit.

Core tracks include:

1. Data formats and `serde`
2. Errors, APIs, and type-driven design
3. Iterators, files, parsing, and zero-copy processing
4. CLI applications and observability
5. Async Rust, networking, and services
6. Threads, synchronization, channels, and atomics
7. Advanced traits, lifetimes, macros, and compile-time design
8. Unsafe Rust, memory, FFI, and sound abstractions

This stage develops practical Rust fluency, familiarity with common libraries, and command of the language's difficult features.

#### First track: reliable data boundaries with `serde`

The first planned batch introduces `serde` without reducing fundamentals to trivial derive-only tasks:

| ID | Difficulty | Exercise | Primary concepts | Status |
|---|---|---|---|---|
| SERDE-01 | Level 1 — First Light | Job Manifest Codec | Derives plus manual `Serialize` and `Deserialize` for a validated newtype | Scaffolded |
| SERDE-02 | Level 1 — First Light | Reliable Configuration | Renaming, defaults, optional fields, unknown fields, and semantic validation | Planned |
| SERDE-03 | Level 1 — First Light | API Events | Nested structures, tagged enums, missing versus null, and wire names | Planned |
| SERDE-04 | Level 2 — Ascent | Legacy Data Normalizer | Untagged representations, custom fields, domain conversion, and typed errors | Planned |
| SERDE-05 | Level 2 — Ascent | Streaming Records | NDJSON, incremental I/O, per-record failures, and bounded memory | Planned |
| SERDE-06 | Level 3 — Crucible | Versioned Protocol | Schema evolution, migrations, compatibility, and API design | Planned |

Borrowed and zero-copy deserialization belongs in the later parsing and lifetime track, after ordinary deserialization is understood.

None of the first six exercises is designated as The Unknown. Because this is the learner's first substantial use of `serde`, the track should teach the library through difficult, progressively scaffolded work before testing independent discovery.

### Stage 2: Advanced campaigns

The core curriculum alone is not sufficient preparation for Event Horizon. Advanced campaigns must cover:

1. Storage engines, transactions, and database correctness
2. Compilers, interpreters, virtual machines, and runtimes
3. Distributed systems, replication, consensus, and deterministic simulation
4. Performance engineering, computer architecture, and low latency
5. Mathematical reasoning, formal methods, model checking, and verification
6. Security, sandboxing, unsafe-code auditing, and hostile-input engineering

Advanced campaigns are generated incrementally after the core. Their exercise count is determined by demonstrated readiness and coverage rather than a fixed quota. They should emphasize Levels 2–4. Selected Level 1–3 exercises may be designed as The Unknown.

### Stage 3: Event Horizon

After the prerequisite campaigns and Death of Ego, the learner selects one of the three original Level 5 paths documented in [`docs/event-horizon.md`](docs/event-horizon.md). Its frozen charter and milestone plan must reflect what was learned during the preceding stages.

### Stage 4: The New Beginning

After Event Horizon, the learner contributes one meaningful new exercise under the requirements defined above.

Generate only **4–6 related exercises at a time**. Do not generate the entire roadmap in one pass. Later batches should account for weaknesses and misunderstandings discovered during solution reviews.

Tracks do not need equal numbers of exercises or one exercise at every difficulty. Coverage and learning progression take priority over symmetry.

## Repository structure

Each exercise is an independent Cargo crate so its dependencies and incomplete implementation do not break unrelated exercises.

```text
exercises/
└── <track>/
    └── <number>-<exercise-name>/
        ├── Cargo.toml
        ├── README.md
        ├── src/
        │   └── lib.rs
        └── tests/
            └── behavior.rs
```

An exercise may use a binary, multiple modules, fixtures, benchmarks, or compile-fail tests when required by its objective. These additions should be intentional rather than boilerplate.

## Exercise specification

The canonical authoring rules and metadata schema are defined in [`docs/exercise-authoring.md`](docs/exercise-authoring.md). New exercise READMEs must begin from [`templates/exercise-readme.md`](templates/exercise-readme.md).

At minimum, every exercise README must contain:

1. **Canonical metadata:** ID, track, stage, difficulty, trial, Field Quest, status, and prerequisites
2. **Learning objectives**
3. **Scenario** explaining why the problem matters in real software
4. **Task and public API**
5. **Required and error behavior** expressed as observable acceptance criteria
6. **Constraints and explicit non-goals**
7. **Examples and commands**
8. **Visible and review-time test scope**
9. **Progressive hints**, preferably in collapsible `<details>` blocks
10. **Mastery review and primary sources**

Problem statements must distinguish required behavior from suggestions. They must not secretly require a specific internal design unless that design is explicitly part of the lesson.

## Starter state

- Provide necessary public function signatures, types, or traits.
- Leave implementation work to the learner, normally with `todo!()`.
- Prefer a crate that compiles but has failing tests so feedback is easy to run.
- Exercises specifically about compiler behavior may intentionally fail to compile.
- Do not include completed solutions in starter code.

## Solution and review workflow

1. Read the exercise README and visible tests.
2. Implement a solution without receiving a completed reference implementation.
3. Run the exercise's formatting, linting, and testing commands.
4. Submit the implementation for review.
5. Review the code for correctness, idiomatic Rust, error handling, and the exercise's learning objective.
6. Add broader black-box or adversarial tests when useful.
7. Let the learner attempt identified fixes before rewriting their implementation.

A few visible tests should demonstrate the contract. Additional tests added during review should focus on edge cases and externally observable behavior rather than duplicating the learner's implementation.

For The Unknown, research notes and tiny isolated experiments come before the full design. For Event Horizon work, the specification, reference model, failure model, and validation strategy must be reviewed before optimized implementation begins.

## Optional field quests

Real open-source work is valuable at every level and must not be treated as a difficulty category by itself. An optional field quest may accompany any track:

- Level 1 — First Light: documentation, examples, or missing tests
- Level 2 — Ascent: reproduce and fix a contained bug
- Level 3 — Crucible: implement a feature or meaningful optimization
- Level 4 — Abyss: repair a subsystem-level issue
- The Unknown: research and contribute within an unfamiliar subsystem
- Level 5 — Event Horizon: publish specifications, components, benchmarks, findings, or substantial upstream work for external review

Issue labels do not reliably indicate actual difficulty, so every field quest must be calibrated after inspecting the codebase and maintainer expectations. Upstream acceptance cannot be required because it is outside the learner's control; success means producing a technically defensible, review-quality contribution.

## Validation methods

Runtime black-box tests are the default, but they are not sufficient for every Rust concept. Exercises may also use:

- Compile-fail and compile-pass tests, such as with `trybuild`
- Explicit trait-bound or API constraints
- Property-based tests
- Mock servers, clocks, or external dependencies
- Timeouts and bounded-concurrency checks
- Benchmarks or allocation constraints
- Short design explanations
- Written safety invariants for unsafe code

Tests must be deterministic and must not depend on live external services unless an exercise explicitly teaches integration with such a service and provides a reliable local substitute.

Correctness and performance validation must be separated:

- **Correctness gate:** portable and deterministic wherever practical
- **Performance gate:** executed on documented reference hardware with a pinned workload
- **Systems extension:** may explicitly require Linux, `perf`, AF_XDP, DPDK, specialized NICs, or bare-metal access

Exercises developed on unsupported hardware must still provide a portable simulator or reference path. Environment-specific performance claims must never be inferred from ordinary CI results.

Advanced exercises should prefer primary sources such as official crate documentation, language references, protocol specifications, implementation papers, and maintainer-authored material. Secondary tutorials may supplement but must not replace authoritative sources for correctness-critical claims.

## Toolchain and dependency policy

- Stable Rust and Edition 2024 are the default.
- A repository-level `rust-toolchain.toml` will pin the exact toolchain once exercise code is introduced.
- Nightly Rust is allowed only when an exercise explicitly requires an unstable compiler feature or tool.
- Every independent exercise crate should commit its `Cargo.lock` for reproducibility.
- Dependencies should be actively maintained and pinned through the lockfile.
- Dependency upgrades must be deliberate, tested, and recorded when they change an exercise's API or behavior.

## Definition of completion

Unless an exercise states otherwise, completion requires:

```bash
cargo test --manifest-path exercises/<track>/<exercise>/Cargo.toml
cargo fmt --manifest-path exercises/<track>/<exercise>/Cargo.toml --check
cargo clippy --manifest-path exercises/<track>/<exercise>/Cargo.toml -- -D warnings
```

It also requires:

- All documented behavior is implemented.
- There are no unexplained panics or uses of unsafe code.
- Levels 3 and 4 include a short explanation of important design decisions.
- The Unknown includes research notes, preliminary experiments, and a postmortem.
- Event Horizon milestones define and satisfy their own reviewed correctness, safety, and performance gates.
- Any unsafe exercise documents its safety invariants and explains why they hold.
- Constraints tied to the learning objective are satisfied even if another implementation could produce the same output.

## Mastery and adaptive progression

Passing tests marks implementation progress, not automatic mastery. A topic is mastered when the learner can:

- Pass behavioral and adversarial validation
- Explain the central concepts in their own words
- Defend important design choices and trade-offs
- Diagnose at least one intentionally broken variation
- Adapt the solution to a meaningful changed requirement
- Identify limitations and failure boundaries in the implementation

Progress should adapt to demonstrated understanding:

- Add a focused recovery exercise when a misconception remains.
- Add a mutation or extension when tests pass but the design is fragile.
- Reduce repetition when the learner demonstrates command of a concept.
- Increase constraints when an exercise is substantially easier than its assigned difficulty.
- Never advance solely because a fixed number of exercises was completed.

Every track and advanced campaign must define three bounded exit points:

- **Minimum mastery:** the smallest defensible set of transferable skills
- **Recommended mastery:** the intended preparation for dependent tracks
- **Research-depth mastery:** optional deeper work for specialization

This ensures that pausing before Event Horizon still represents a meaningful accomplishment and prevents an unbounded roadmap from invalidating earlier progress.

Exercise status should use this progression:

```text
Planned → Scaffolded → In Progress → Tests Pass → Reviewed → Mastered
```

## Adding exercises

Before adding an exercise:

1. Identify a gap in the coverage map or a weakness observed during review.
2. State the transferable concept being taught.
3. Choose the lowest difficulty that can teach it meaningfully.
4. Create an independent crate following the standard structure.
5. Write the behavioral contract before writing tests.
6. Add a small set of visible tests without implementing the solution.
7. Confirm that the task is realistic, focused, deterministic, and completable.
8. Update the curriculum status in this README.

Prefer exercises that:

- Reflect common production work
- Reveal meaningful Rust-specific trade-offs
- Have testable behavior
- Teach reusable reasoning
- Use actively maintained dependencies

Avoid exercises that:

- Test obscure API memorization
- Add complexity unrelated to the learning goal
- Depend unnecessarily on network access or machine-specific state
- Prescribe an internal implementation without explaining why
- Combine so many new ideas that the source of difficulty becomes unclear

## Starting primary sources

This list is a starting research map, not a fixed syllabus. Exercise-specific READMEs should cite the exact authoritative material they rely on.

- [The Rust Reference](https://doc.rust-lang.org/reference/)
- [Rust Compiler Development Guide](https://rustc-dev-guide.rust-lang.org/)
- [Serde](https://serde.rs/)
- [Tokio documentation](https://tokio.rs/)
- [Miri](https://github.com/rust-lang/miri)
- [Loom](https://github.com/tokio-rs/loom)
- [Kani Rust Verifier](https://model-checking.github.io/kani/)
- [SQLite: Atomic Commit](https://sqlite.org/atomiccommit.html)
- [FoundationDB: Simulation and Testing](https://apple.github.io/foundationdb/testing.html)
- [TigerBeetle: Safety](https://docs.tigerbeetle.com/concepts/safety/)
- [Raft paper](https://www.usenix.org/conference/atc14/technical-sessions/presentation/ongaro)
- [LLVM Language Reference](https://llvm.org/docs/LangRef.html)
- [Cranelift IR](https://docs.rs/cranelift-codegen/latest/cranelift_codegen/ir/)
- [Jane Street: How to Build an Exchange](https://www.janestreet.com/tech-talks/building-an-exchange/)
- [Jane Street: System Jitter and Where to Find It](https://www.janestreet.com/tech-talks/system-jitter-and-where-to-find-it/)
- [Nasdaq TotalView-ITCH specification](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/NQTVITCHspecification.pdf)

## Instructions for future LLMs

When extending this repository:

- Treat this README as the curriculum contract.
- Do not generate a large exercise set without first discussing the next track with the learner.
- Generate only 4–6 exercises in a coherent batch.
- Explain proposed learning objectives, sequencing, dependencies, and difficulty before creating files.
- Scaffold problem statements, starter signatures, and tests; do not provide finished solutions unless explicitly requested.
- Keep decisions visible and avoid unrelated repository changes.
- During review, critique and explain before rewriting the learner's code.
- Update this README with the final curriculum status and durable decisions, not a transcript of the discussion.
- If a new convention conflicts with this document, discuss and record the agreed change before applying it broadly.
- Do not dilute The Unknown, Abyss, or Event Horizon into ordinary large coding assignments. Preserve their research, specification, validation, and adversarial-review requirements.
- Do not claim that a finite curriculum produces universal mastery. Emphasize transferable reasoning and the ability to learn unfamiliar systems.
- Treat The Unknown and Field Quest as special designations, not additional difficulty levels.
- Preserve The New Beginning as a required act of stewardship after Event Horizon.
- Apply the curriculum constitution and philosophy review protocol before major expansions; do not defend an existing plan merely because it is already documented.
- Require domain-appropriate external review before accepting scientific or research-grade claims.

## Current status

- Curriculum constitution and top-level contract: complete
- Curriculum constitution and red-team protocol: complete
- LLM onboarding and 95% comprehension gate: complete
- Repository-only blind onboarding pilot: passed at 20/20 with all critical invariants
- Exercise-authoring specification and template: complete
- Exercise crates: SERDE-01 scaffolded; no solution implemented
- First track in progress: reliable data boundaries with `serde`
- Data-and-Serde track specification: complete
- Initial roadmap target: approximately 32 exercises across 8 tracks
- Advanced roadmap: six prerequisite campaigns with counts determined by learner readiness
- The Unknown target: approximately 3–5 selected exercises across Levels 1–3
- Event Horizon: three original Level 5 paths documented; none selected or started
- The New Beginning: required after Event Horizon
