# Apotheosis

> From First Light, through the Event Horizon, to a New Beginning.

Apotheosis is a learner-driven Rust curriculum for people who know the language fundamentals and want to become capable, independent systems engineers.

It combines practical work with common Rust libraries, difficult language features, testing, system design, mathematics, and eventually original cross-disciplinary research.

> You do not emerge knowing everything. You emerge afraid of nothing you do not yet know.

## Who this is for

You should already be comfortable with:

- Ownership and borrowing
- Structs and enums
- Traits and `impl` blocks
- `Option` and `Result`
- Pattern matching
- Cargo and ordinary Rust tests

No previous experience with the ecosystem library used by a starting track is assumed unless its prerequisites say otherwise.

If these prerequisites are shaky, pause and ask for a focused recovery unit rather than struggling through unrelated gaps. Exercises have no target completion time; understanding and transfer matter more than speed.

## Setup

Install Rust through [`rustup`](https://rustup.rs/) and run commands from the repository root. The checked-in `rust-toolchain.toml` selects the required compiler and components automatically.

If you have not cloned the repository yet:

```bash
git clone https://github.com/vaibhav135/apotheosis.git
cd apotheosis
```

Confirm the environment:

```bash
rustc --version
cargo --version
```

## Start here

The current exercise is:

### [SERDE-01: Job Manifest Codec](exercises/data-and-serde/01-job-manifest-codec/README.md)

- **Level:** Level 1 — First Light
- **Curriculum:** Core
- **Participation:** Required

You will use Serde derives for an outer type while manually implementing `Serialize` and `Deserialize` for a validated `JobId` newtype.

1. Read the complete exercise README.
2. Inspect `src/lib.rs` and `tests/behavior.rs`.
3. Implement the `todo!()` sections yourself.
4. Run the checks below.
5. Ask your reviewer or repository-connected AI to inspect `src/lib.rs` before attempting the exercise's [mastery review](exercises/data-and-serde/01-job-manifest-codec/README.md#mastery-review). A branch, commit, or direct local review is sufficient; no external submission platform is required.

```bash
cargo test --manifest-path exercises/data-and-serde/01-job-manifest-codec/Cargo.toml
cargo fmt --manifest-path exercises/data-and-serde/01-job-manifest-codec/Cargo.toml --check
cargo clippy --manifest-path exercises/data-and-serde/01-job-manifest-codec/Cargo.toml -- -D warnings
```

The starter tests intentionally fail until you implement the exercise.

## How learning works

Each exercise follows this cycle:

```text
approved contract
      ↓
learner implementation
      ↓
behavioral and adversarial testing
      ↓
code and design review
      ↓
changed-requirement mastery task
      ↓
explanation and limitation analysis
```

Passing tests is progress, not mastery. You should also be able to explain the concept, diagnose a broken variation, adapt your design, and identify its limits.

The learner writes solutions. AI support may scaffold, test, explain, and review, but does not provide completed implementations unless explicitly requested.

Review criteria come from [`docs/exercise-authoring.md`](docs/exercise-authoring.md). The reviewer reports evidence and findings; the learner or designated evaluator confirms status transitions.

## Levels

| Level | Name | What changes |
|---:|---|---|
| 1 | **First Light** | Learn one focused mechanism through meaningful application. |
| 2 | **Ascent** | Combine concepts and make local engineering decisions. |
| 3 | **Crucible** | Design reusable subsystems and defend trade-offs. |
| 4 | **Abyss** | Enter bounded, adversarial, research-heavy engineering. |
| 5 | **Event Horizon** | Produce original multidisciplinary work under a reviewed charter. |

A Level describes the difficulty and engineering responsibility of the current work, not a permanent rank attached to the learner. It is shaped by unfamiliarity, interacting guarantees, design responsibility, failure modes, and the evidence required for correctness—not code length or suffering.

## Special parts of the journey

### The Unknown

A selective Level 1–3 trial where researching an unfamiliar domain is part of the work. It is not another Level.

### Field Quest

Optional, calibrated work in a real open-source project. Upstream acceptance is not required; producing defensible, review-ready work is.

### Event Horizon

The final level offers three original paths:

- **Iron Meridian — The Architecture of Trust**
- **Genesis — The Living Equation**
- **The Observer — The Limits of Knowing**

One path is active at a time, and completing one completes Event Horizon. Exact charters remain deliberately unfrozen until the prerequisite work is complete. See [`docs/event-horizon.md`](docs/event-horizon.md).

### The New Beginning

After Event Horizon, the graduate turns one hard-earned insight into a reviewed and piloted problem for a future learner. See [`docs/the-new-beginning.md`](docs/the-new-beginning.md).

## Curriculum structure

Level is the primary progression. Curriculum identifies the nature of work within Levels 1–4:

| Level | Curriculum or path |
|---|---|
| Level 1 — First Light | Core |
| Level 2 — Ascent | Core |
| Level 3 — Crucible | Core or Advanced |
| Level 4 — Abyss | Advanced |
| Level 5 — Event Horizon | One Event Horizon path |
| After Level 5 | The New Beginning |

**Core** builds production systems using established machinery. **Advanced** builds, investigates, or proves that machinery through focused campaigns. Event Horizon is only Level 5, and The New Beginning is stewardship after the levels.

The cumulative Core capability areas cover:

- Validated data and error design
- Files, text, binary data, and streaming I/O
- CLI applications, configuration, observability, and process boundaries
- Reusable APIs and Cargo project architecture
- Database use, migrations, and transactions
- Threads, synchronization, and bounded work
- Async networking, cancellation, backpressure, and services
- Compatibility, testing, release maintenance, and basic performance evidence
- Unsafe Rust and safe abstraction boundaries

Security, deterministic testing, documentation, portability, observability, dependency stewardship, and responsible measurement develop throughout Core rather than appearing only once.

Advanced campaigns later cover storage engines, compiler and runtime internals, distributed guarantees, formal verification, advanced security engineering, and hardware-level performance.

This is not a rigid checklist. The structure follows the knowledge: concepts may be combined, split, moved, or removed according to what creates the clearest and most authentic learning unit.

### How curriculum labels fit together

For ordinary Levels 1–4 work, read the metadata in this order:

- **Level** identifies the difficulty and engineering responsibility of the current work.
- **Curriculum** identifies Core or Advanced work.
- **Participation** identifies whether the work is required or an optional specialization for a learner's path.

For example, Level 3 work may be Core or Advanced; within either Curriculum, Participation is separately Required or Optional specialization. Level 5 instead follows its Event Horizon path charter.

An occasional **Convergence checkpoint** integrates previous capabilities to establish a guarantee none can provide alone. It is not a level, Curriculum value, or required exercise count.

See the definitive hierarchy and cumulative Core capability areas in [`docs/curriculum.md`](docs/curriculum.md#level-first-curriculum-structure).

## Core principles

- Learning over suffering
- Transformation over completion
- Transfer over test-passing
- Coherence over breadth
- Evidence over confidence
- Primary sources over invented authority
- Safety and ethics before spectacle
- Reproducibility before performance claims
- Stewardship after mastery

Read the complete constitution and review method in [`docs/philosophy.md`](docs/philosophy.md).

## Documentation map

### AI-agent entrypoint

AI agents must begin with [`AGENTS.md`](AGENTS.md), follow its mandatory reading order, and pass the comprehension gate before modifying the repository. This human README is intentionally not the complete agent specification.

| Audience or subject | Canonical document |
|---|---|
| Human introduction and next action | This README |
| Educational philosophy and constitution | [`docs/philosophy.md`](docs/philosophy.md) |
| Levels, curriculum classification, progression, and status | [`docs/curriculum.md`](docs/curriculum.md) |
| AI-agent startup rules | [`AGENTS.md`](AGENTS.md) |
| LLM comprehension gate | [`docs/llm-onboarding.md`](docs/llm-onboarding.md) |
| Exercise authoring and testing | [`docs/exercise-authoring.md`](docs/exercise-authoring.md) |
| Serde track boundaries | [`docs/tracks/data-and-serde.md`](docs/tracks/data-and-serde.md) |
| Event Horizon | [`docs/event-horizon.md`](docs/event-horizon.md) |
| The New Beginning | [`docs/the-new-beginning.md`](docs/the-new-beginning.md) |
| Exercise README template | [`templates/exercise-readme.md`](templates/exercise-readme.md) |

Each subject has one canonical home. Other documents should summarize and link rather than duplicate full rules.

## Current status

- SERDE-01 is scaffolded and has no learner solution.
- Later Serde ideas are provisional and may move into more natural integration tracks.
- Advanced campaigns are conceptual only.
- No Event Horizon path is active.

## License

This project is available under the [MIT License](LICENSE).
