# Philosophy and curriculum constitution

This is the canonical source for Apotheosis's educational philosophy. The human-facing [`README.md`](../README.md) summarizes it; agent and authoring documents apply it.

## Meaning of Apotheosis

> From First Light, through the Event Horizon, to a New Beginning.

Apotheosis describes transformation, not omniscience. No curriculum makes an absolute programmer, and no engineer becomes exempt from evidence, correction, or continued learning.

The intended graduate can enter an unfamiliar domain with humility, build an accurate mental model, investigate rigorously, engineer carefully, and allow reproducible work to speak for itself.

> You do not emerge knowing everything. You emerge afraid of nothing you do not yet know.

## Purpose

Apotheosis is for someone who already understands Rust fundamentals such as ownership, borrowing, traits, and `impl` blocks. It develops two forms of mastery together:

1. **Applied Rust:** using common crates and techniques to solve realistic problems.
2. **Hard Rust:** understanding difficult language features and designing safe, idiomatic abstractions.

The curriculum should eventually provide practical experience with:

- Serialization, data formats, files, parsing, and streams
- CLI applications, HTTP, services, observability, and testing
- Error design, recovery, and type-driven APIs
- Async Rust, cancellation, concurrency, synchronization, and atomics
- Advanced borrowing, lifetimes, closures, traits, associated types, and dynamic dispatch
- Futures, `Pin`, macros, unsafe Rust, memory layout, FFI, and zero-copy techniques
- Performance, hardware, mathematical reasoning, and reproducible experimentation
- Databases, compilers, distributed systems, formal methods, and security in advanced work

This is a coverage direction, not a promise to enumerate every crate or corner of Rust.

## Teaching philosophy

- The learner writes exercise solutions.
- The AI discusses, scaffolds, tests, reviews, and helps maintain the curriculum.
- Exercises use realistic scenarios rather than isolated syntax trivia.
- Runtime tests validate observable behavior by default.
- Internal implementation constraints are allowed only when the technique is the lesson.
- Passing tests is progress, not mastery.
- Difficult work may require explanations, compile-time checks, properties, simulation, benchmarks, model checking, or safety arguments.
- Common libraries receive enough focused introduction to establish a mental model, then reappear naturally in integrated work.
- Advanced difficulty comes from authentic guarantees, not hidden requirements, obscure trivia, or hostile instruction.
- Durable documentation records conclusions rather than conversation transcripts.

## Curriculum constitution

These principles outrank dramatic naming, exercise count, and attachment to any roadmap:

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

## Learning-unit granularity

> The structure follows the knowledge—not the other way around.

There is no mandatory number of exercises in a unit, track, or campaign. Plan one coherent learning unit at a time, then implement and review one approved exercise or milestone at a time.

Every exercise should have one governing question. It may contain several objectives when their interaction is necessary to answer that question. Density comes from deeper validation, review, transfer, and reflection—not unrelated features.

### Split work when

- A concept can be learned and validated independently.
- Multiple unfamiliar ideas obscure the source of difficulty.
- Feedback would arrive too late.
- Parts have different prerequisites or failure modes.
- Each part can produce a meaningful result.

### Keep work together when

- The interaction is the lesson.
- Splitting would create artificial toy tasks.
- The guarantee exists only at the integration boundary.
- Architectural trade-offs cannot be understood independently.

Granularity decisions must consider cognitive load, prerequisites, authenticity, testability, feedback speed, transfer, motivation, resources, and safety.

## Difficulty and transformation

Difficulty is determined by ambiguity, interacting concepts, design responsibility, failure modes, and guarantees—not code length.

- **Level 1 — First Light:** illuminate one focused mechanism through meaningful application.
- **Level 2 — Ascent:** combine concepts and make local engineering decisions.
- **Level 3 — Crucible:** design a reusable subsystem and defend trade-offs.
- **Level 4 — Abyss:** enter bounded but adversarial research engineering.
- **Level 5 — Event Horizon:** produce original multidisciplinary work under a reviewed charter.

Major Level 3–5 work may pair an evocative philosophical title with a precise technical subtitle. The title must express the intended transformation rather than merely sound dark.

Each such problem should state:

- Technical objective
- Philosophical intent
- Expected change in the learner's mental model
- Reflection questions

The opening Event Horizon milestone is **Death of Ego — Define the Machine You Claim to Build**. “Death of ego” means abandoning the expectation of already knowing, not destroying confidence or tying personal worth to difficulty.

## Intellectual humility at the pinnacle

The universe of computing is too large for any person or curriculum to master. Event Horizon provides a tiny but honest encounter with that scale.

Engineering remains central, while mathematics and science appear where they support genuine guarantees:

- Logic, invariants, graphs, partial orders, probability, statistics, queueing, numerical error, optimization, and information theory
- Clocks, signals, noise, CPU pipelines, caches, networks, storage, power, thermals, and other physical limits beneath abstractions
- Compilers, databases, operating systems, distributed systems, security, low latency, simulation, verification, and fault injection

No discipline belongs merely to make a project look impressive. Event Horizon's detailed philosophy is in [`event-horizon.md`](event-horizon.md).

## Philosophy review protocol

Before a major roadmap expansion, after each completed track, and before freezing an Event Horizon charter:

1. **Restate the claim:** what ability should this work produce?
2. **Construct the theory of change:** connect activity to observable learning.
3. **Red-team it:** attack ambiguity, artificial difficulty, unsafe incentives, inaccessible requirements, and gameable validation.
4. **Run a premortem:** assume educational failure and identify plausible causes.
5. **Define falsification evidence:** state what observations would prove the design ineffective.
6. **Test coherence:** require every concept and constraint to justify its presence.
7. **Test feasibility:** document time, hardware, money, prerequisites, and review needs.
8. **Pilot before scaling:** validate the smallest useful version.
9. **Record conclusions:** update canonical documentation, not a conversation log.

## Warning signs

Revise the curriculum when:

- Learners pass tests but cannot solve related variations.
- Difficulty creates confusion without improving reasoning.
- A discipline cannot be connected to a concrete guarantee.
- Scientific claims cannot be independently validated.
- Completion primarily depends on expensive hardware or services.
- The roadmap grows faster than demonstrated readiness.
- Names and spectacle become more important than technical substance.
- Exercises repeatedly depend on undocumented assumptions.

## Falsifiable standard

Apotheosis succeeds only if learners become more capable of solving changed and unfamiliar problems. Exercise count, time spent, emotional struggle, and green tests are insufficient proxies.

The philosophy itself remains open to revision when evidence contradicts it.
