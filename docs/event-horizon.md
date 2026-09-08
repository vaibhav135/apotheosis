# Event Horizon

Event Horizon is Level 5 and the ultimate difficulty in Practical Rust Mastery. It is where guided exercises become original, cross-disciplinary engineering research.

This document records the durable philosophy, rules, and original project portfolio. It is a specification, not a transcript and not yet a complete implementation plan.

## Purpose

The universe of computing is too large for any person or curriculum to master. Event Horizon offers a tiny but honest encounter with that scale.

Its purpose is to demonstrate the ability to:

- Enter unfamiliar technical and scientific domains without pretending certainty
- Build accurate mental models from primary sources and experiments
- Integrate several disciplines around a coherent system guarantee
- Distinguish software correctness from mathematical or scientific validity
- Design reproducible evidence for correctness, safety, and performance
- Produce a small but defensible original contribution
- Explain limitations, trusted boundaries, uncertainty, and unresolved questions

Completion does not create an “absolute programmer.” It demonstrates intellectual humility, independent research ability, and engineering judgment at the boundary of current knowledge.

## Governing principles

The repository's curriculum constitution applies in full. Event Horizon especially requires:

1. Learning over suffering
2. Transformation over completion
3. Coherence over breadth
4. Evidence over confidence
5. Bounded guarantees over infinite ambition
6. Scientific humility over impressive claims
7. Reproducibility before performance claims
8. Safety and ethics before technical spectacle

Difficulty must come from making demanding guarantees hold together. It must never come from hidden requirements, arbitrary scale, unavailable information, or deliberately hostile instruction.

## What Event Horizon is not

Event Horizon is not:

- A single oversized coding exercise
- A checklist containing every difficult academic subject
- A requirement to solve a famous open mathematical problem
- An excuse to combine unrelated technologies
- A test of access to expensive hardware
- A claim that a simulation establishes scientific truth
- A contest in enduring frustration
- A system intended for real financial, biological, medical, or safety-critical deployment

The exact integrated system may have no existing implementation. Its underlying theories, bounded guarantees, and validation methods must nevertheless be sufficiently established for meaningful progress and review.

## Entry requirements

Before Event Horizon, the learner should complete the relevant core tracks and advanced campaigns in:

- Advanced Rust, unsafe code, and API design
- Storage and database correctness
- Compilers, interpreters, and runtimes
- Distributed systems and deterministic simulation
- Performance engineering and computer architecture
- Mathematical reasoning and formal methods
- Security and adversarial engineering

Readiness is demonstrated through mastery gates, not exercise count alone.

## Opening milestone

# Death of Ego — Define the Machine You Claim to Build

All three paths share this opening milestone.

The phrase “death of ego” means abandoning the expectation of already knowing. It does not mean destroying confidence or connecting personal worth to difficulty.

Before selecting a path or writing optimized code, the learner prepares a research dossier for all three paths containing:

- A knowledge and assumption inventory
- Unknown concepts and required prerequisite study
- Existing systems, papers, specifications, and reference implementations
- Mathematical and scientific foundations
- Hardware, time, financial, and computational resource requirements
- Ethical, safety, and dual-use boundaries
- Plausible guarantees and explicit non-goals
- Candidate validation and falsification methods
- Potential original contributions
- Questions that may remain genuinely unresolved

The learner must then answer:

> What exactly are you promising, and what evidence would convince a hostile but fair reviewer that the promise holds?

## Path selection and commitment

There are three original Event Horizon paths. Only one may be active at a time.

Completing one path completes Event Horizon. Another path may be attempted afterward, but completing all three is not required.

Selection creates a written Commitment Document containing:

- Why this path was selected
- The learner's current preparation and knowledge gaps
- The proposed bounded contribution
- Expected resources and risks
- Reviewers or expertise that will be needed
- Initial completion and abandonment criteria

The commitment is serious but not irreversible. Switching paths requires a formal Exit Review documenting:

- What became infeasible or invalid
- Which assumptions failed
- Evidence supporting the decision to stop
- Knowledge and artifacts produced
- What would be done differently on the next path

This prevents casual switching without rewarding destructive sunk-cost behavior.

## Common engineering spine

Every path must meaningfully involve:

- Advanced Rust and explicit unsafe-code reasoning where applicable
- Algorithms, data structures, concurrency, and resource ownership
- A language, compiler, interpreter, planner, or equivalent programmable abstraction
- Durable state, recovery, provenance, or reproducible checkpoints
- Parallel or distributed execution where it serves the central guarantee
- Security boundaries and hostile-input handling
- Mathematics, probability, statistics, and numerical reasoning
- Formal models, reference implementations, or differential validation
- Performance measurement grounded in physical hardware
- Reproducible experiments and documented uncertainty
- Ethical limitations and responsible-use constraints

Every included discipline must answer:

> Which essential guarantee would become impossible or dishonest without this discipline?

If there is no convincing answer, that component does not belong.

## Required project charter

Before implementation, the selected path must define:

1. A minimum research question
2. A bounded supported subset
3. Required guarantees
4. Explicit non-goals
5. Failure, threat, and safety models
6. Mathematical and scientific assumptions
7. Reference models and trusted boundaries
8. Hardware, time, money, and compute budgets
9. Correctness, scientific-validity, and performance methodologies
10. The required original contribution
11. External-review requirements
12. Completion, failure, and abandonment criteria

The charter must survive the repository's philosophy review protocol before it is frozen.

## Common validation requirements

Depending on the path, evidence should include:

- Deliberately slow reference implementations
- Differential and metamorphic testing
- Property-based testing and fuzzing
- Deterministic simulation and fault injection
- Model checking and bounded verification
- Miri and unsafe-code review
- Numerical-error and uncertainty analysis
- Reproducible seeded experiments
- Performance distributions rather than averages alone
- Documented hardware and workload details
- Independent technical and domain review

A negative result may count only when it is reproducible, independently reviewed, and establishes something meaningful about the system or its guarantees. Abandonment without evidence is not completion.

## Path 1: Iron Meridian — The Architecture of Trust

### Technical objective

Build a deterministic, programmable, replicated, low-latency transaction engine using a simulated financial exchange as its adversarial world.

### Philosophical question

> How can an imperfect physical machine make a trustworthy promise about time, order, ownership, and value?

### Disciplines

- Systems programming and operating systems
- Databases, transactions, and crash recovery
- Distributed systems and replicated state machines
- Compiler and runtime construction
- Networking and binary protocols
- Computer architecture, electronics, clocks, and storage physics
- Discrete mathematics, fixed-point arithmetic, and formal methods
- Probability, statistics, queueing theory, and experimental design
- Economics, market structure, fairness, and adversarial behavior
- Security, accountability, and incident reconstruction

### Candidate system responsibilities

- Binary order-entry and sequenced market-data protocols
- Price-time-priority matching
- A typed language for deterministic pre-trade risk rules
- Parser, type checker, typed IR, bytecode compiler, verifier, and interpreter
- Optional differential execution through a Cranelift JIT
- Fixed-point accounting with explicit overflow guarantees
- Write-ahead logging, checksums, snapshots, and crash recovery
- Static three-node replication, failover, and fencing
- Idempotent retries and explicit acknowledgment boundaries
- Deterministic replay and replica state hashing
- Bounded queues, backpressure, arenas, and buffer pools
- Allocation-free warmed critical paths
- Tail-latency measurement without coordinated omission
- Deterministic simulation of networks, disks, clocks, and processes
- Time-travel diagnostics explaining every execution or rejection

### Central guarantee

> Every acknowledged command survives every permitted crash schedule, appears exactly once in the logical history, produces identical decisions after replay, preserves all market and risk invariants, and remains within explicitly measured resource and latency bounds.

### Safety boundary

Iron Meridian is strictly an educational simulation. It uses synthetic or recorded data, never connects to a live financial venue, and never controls real funds.

### Possible original contribution

- Interpreter/JIT translation validation
- New deterministic crash and replay testing techniques
- Unified simulation of storage, network, clock, and latency faults
- New latency-versus-durability strategies
- Explainable time-travel debugging for deterministic systems

## Path 2: Genesis — The Living Equation

### Technical objective

Build a programmable, reproducible, multiscale biological simulation and inference engine for a carefully bounded class of cellular systems.

### Philosophical question

> How can a machine distinguish what a model predicts from what living reality actually permits us to claim?

### Disciplines

- Systems programming and scientific computing
- Compiler and domain-specific-language design
- Molecular and cellular biology
- Chemistry and thermodynamics
- Reaction-diffusion physics
- Differential equations and stochastic processes
- Probability, statistics, Bayesian inference, and experimental design
- Numerical analysis and uncertainty quantification
- Parallel, distributed, and GPU computing
- Reproducible science and scientific ethics

### Candidate system responsibilities

- A typed language for biochemical reaction networks and cellular compartments
- Dimensional analysis and unit-safe model validation
- Compilation into deterministic simulation plans
- Ordinary-differential-equation and stochastic simulation backends
- A bounded spatial reaction-diffusion backend
- Deterministic random-stream management across parallel workers
- Parameter estimation against published, non-sensitive datasets
- Sensitivity and uncertainty analysis
- Checkpointing, recovery, experiment provenance, and replay
- Differential comparison against trusted reference solvers
- Explicit reporting of numerical error, model assumptions, and identifiability limits

### Central guarantee

> Every reported result identifies its model assumptions, numerical error, stochastic uncertainty, parameter sensitivity, provenance, and reproducibility boundary; equivalent supported executions agree within declared and scientifically justified tolerances.

### Safety boundary

Genesis is entirely computational. It uses public, non-sensitive, non-hazardous models and datasets. It must not design pathogens, prescribe medical treatment, direct wet-lab work, or claim biological truth beyond validated model boundaries.

### Possible original contribution

- A type system for unit-safe, composable biological models
- Reproducible parallel stochastic simulation
- Cross-solver translation validation
- Better provenance and uncertainty representation
- Adaptive simulation techniques with defensible error bounds

## Path 3: The Observer — The Limits of Knowing

### Technical objective

Build a verified quantum-classical language, compiler, simulator portfolio, and bounded runtime.

### Philosophical question

> What can a program promise when observation, probability, and information define the boundary of what can be known?

### Disciplines

- Systems programming
- Quantum mechanics
- Linear algebra, complex numbers, and numerical analysis
- Probability, statistics, and information theory
- Compiler construction and intermediate representations
- Linear type systems and formal logic
- Verification and program equivalence
- Parallel and GPU computing
- Quantum noise and error-correcting codes
- Hardware topology, scheduling, and reproducible experimentation

### Candidate system responsibilities

- A language for qubits, classical values, gates, measurement, and bounded control
- A linear type system enforcing resource use and no-cloning constraints
- A typed quantum IR
- Circuit optimization and translation validation
- Gate decomposition, qubit mapping, and scheduling
- A deliberately slow reference interpreter
- State-vector and stabilizer simulators
- Optional tensor-network and parallel/GPU backends
- Reproducible noisy-device simulation
- Bounded quantum-error-correction experiments
- Statistical comparison of observable output distributions
- Explicit numerical-error, approximation, and resource reporting

### Central guarantee

> Every accepted program is type-safe under the supported quantum model; compiler transformations preserve its observable behavior within declared mathematical and numerical bounds; and independent execution backends agree within statistically justified tolerances.

### Safety and honesty boundary

The Observer does not claim to build physical quantum hardware or demonstrate quantum advantage without evidence. Hardware access is optional; reference simulation and bounded validation must remain possible on documented conventional resources.

### Possible original contribution

- Quantum linear types integrated with Rust-inspired ownership
- Translation validation for circuit optimizations
- Deterministic replay of hybrid quantum-classical programs
- Better numerical-error accounting across simulators
- Verified topology mapping, scheduling, or error-correction transformations

## Portfolio balance

The three paths represent different encounters:

- **Iron Meridian:** truth and promises across time, competition, and failure
- **Genesis:** emergence and inference in living systems
- **The Observer:** information, measurement, and the limits of certainty

All three share the engineering spine. None attempts to include every discipline. A fourth path should be accepted only if it offers a distinct philosophical transformation, coherent technical necessity, defensible validation, realistic resources, and domain expertise that cannot be represented adequately by an advanced campaign.

## Milestone discipline

No path may be generated as one giant task. After Death of Ego, it must be decomposed into independently reviewable milestones following this broad order:

1. Charter, definitions, and primary-source study
2. Mathematical or scientific model
3. Deliberately slow reference implementation
4. Test generators and falsification strategy
5. Minimal end-to-end system
6. Durability, distribution, or reproducibility mechanisms
7. Adversarial and failure validation
8. Performance engineering
9. Original contribution
10. Independent review, replication, and final defense

Optimization must not precede a defensible reference model and measurement methodology.

## Completion

Event Horizon is complete when:

- The frozen charter's required guarantees are satisfied or a qualifying research result revises them
- The original contribution is explicit and independently reviewable
- Correctness, scientific, safety, and performance claims have appropriate evidence
- Important limitations and trusted boundaries are documented
- Experiments are reproducible within the declared resource envelope
- Domain-appropriate external review is complete
- The learner can defend the work and explain failed approaches
- The learner proceeds to [The New Beginning](the-new-beginning.md)

## Status

- Philosophy and common rules: agreed
- Original portfolio size: three paths
- Path names and directions: agreed
- Exact project charters: intentionally not frozen
- Active path: none
- Entry prerequisites: not yet completed
