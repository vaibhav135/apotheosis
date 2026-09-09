# LLM Onboarding Validation: 11dfc43

## Purpose And Date

- **Purpose:** Record an auditable repository-only validation of the LLM onboarding and comprehension-gate protocol at the tested revision.
- **Date:** 2026-09-08
- **Tested commit:** `11dfc4352bd6fd9cdf5c948ddeddc879a1856514`

## Repository And Test Conditions

- Repository: `/Users/vaibhavbisht/Documents/projects/apotheosis`
- The agent was introduced as a fresh agent with only the repository path and no prior conversation context.
- The working tree was verified clean and at the tested commit before the gate response was produced.
- The agent inspected the repository rather than relying on status claims from the prompt.
- Mandatory onboarding documents were read, including `AGENTS.md`, `README.md`, `docs/philosophy.md`, `docs/curriculum.md`, and `docs/llm-onboarding.md`.
- Exercise-specific documents were also read: `docs/exercise-authoring.md`, `docs/tracks/data-and-serde.md`, and the complete SERDE-01 README, starter source, tests, and manifest.
- After the first evaluation failure, the agent re-read `docs/event-horizon.md`, the gate requirements, and the relevant Event Horizon, lifecycle, deterministic-validation, and unsafe-code rules in `docs/curriculum.md`.
- No repository files were modified before the gate passed.
- This was a comprehension-protocol test. It did not execute or validate the SERDE-01 Rust implementation, whose starter tests are intentionally expected to fail while learner-owned `todo!()` scaffolding remains.
- The successful response was evaluated by an independent parent agent against all 14 critical invariants and all 20 rubric points.

## Retry Record

The first gate response failed critical invariant 10 because it did not name all three original Event Horizon paths, explicitly state that only one path may be active at a time, and state the reviewed-charter and domain-appropriate-evidence requirements. The agent re-read the relevant canonical documents and submitted the corrected response below. Independent parent-agent evaluation reported that the corrected response passed all 14 critical invariants and all 20 rubric points.

## Successful Gate Response

### 1. Purpose

Apotheosis is a learner-driven Rust curriculum. The learner writes exercise solutions unless they explicitly request a full implementation. The AI discusses and scaffolds exercises, provides progressive guidance, runs tests, reviews work, and maintains curriculum documentation. It critiques and explains before rewriting learner code.

Passing tests is progress, not mastery. Mastery also requires explaining concepts, diagnosing broken variants, adapting designs to changed requirements, defending trade-offs, and identifying limitations and failure boundaries.

### 2. Current State

- **Tested commit SHA:** `11dfc4352bd6fd9cdf5c948ddeddc879a1856514`
- The inspected working tree is clean and at that committed revision.
- The current stage is the **Core curriculum**.
- The first active track is **Reliable data boundaries with `serde`**.
- **SERDE-01: Job Manifest Codec** is **Scaffolded** and is the only Serde exercise with an approved complete contract.
- Its learner-owned implementation remains six `todo!()` sections in `src/lib.rs`; no learner solution has been implemented.
- The learner is expected to implement SERDE-01, run its tests, formatting check, and Clippy, and then submit it for review.
- SERDE-02 through SERDE-06 are provisional planning hypotheses.
- Advanced campaigns are conceptual, Event Horizon prerequisites are incomplete, and no Event Horizon path is active.

Canonical ownership is explicit:

- `README.md` owns the human introduction and immediate next action.
- `docs/philosophy.md` owns educational philosophy and learning-unit granularity.
- `docs/curriculum.md` owns stages, lifecycle, aggregate status, and roadmap.
- `AGENTS.md` owns agent startup behavior.
- `docs/llm-onboarding.md` owns the gate structure and evaluation rules.
- `docs/exercise-authoring.md` owns exercise construction, testing, and status authority.
- Track documents own track-local concept boundaries and status.
- Each exercise README owns its approved behavior and local lifecycle status.
- `docs/event-horizon.md` owns Event Horizon rules.
- `docs/the-new-beginning.md` owns The New Beginning.

Only the learner or designated evaluator confirms roadmap promotions and lifecycle transitions. An LLM may record one only when explicitly designated as evaluator or after confirmation. Exercise, track, and curriculum status must be updated consistently.

### 3. Difficulty Model

1. **Level 1 — First Light:** applies one focused mechanism in a meaningful problem with limited architecture, useful edge cases, and at least one real design decision.
2. **Level 2 — Ascent:** integrates multiple concepts in a realistic task and requires local API, data-structure, and error-handling decisions.
3. **Level 3 — Crucible:** requires a reusable production-like subsystem, public contract design, important resource and failure behavior, and defended architectural trade-offs.
4. **Level 4 — Abyss:** is bounded, adversarial, inherently research-heavy engineering involving profound unfamiliarity, interacting guarantees, and failure analysis.
5. **Level 5 — Event Horizon:** is original multidisciplinary engineering research under a reviewed charter, with explicit guarantees, scope, threats, trusted boundaries, safety, and domain-appropriate evidence.

Difficulty is determined by ambiguity, design responsibility, interacting guarantees, and failure modes, not code length or exercise count.

### 4. Special Designations

- **The Unknown** is a selective research-process trial available only at Levels 1–3. It is not a difficulty level. Levels 4–5 do not use the label because research is inherent at those levels.
- **Field Quest** is optional real open-source work at any level. It is not a difficulty, and upstream acceptance is not required; success is a calibrated, defensible, review-ready contribution.
- **The New Beginning** is post-Event-Horizon stewardship in which the graduate creates, reviews, pilots, revises, and contributes a meaningful problem for a future learner. It is not Level 6.
- Event Horizon has exactly three original paths:
  - **Iron Meridian — The Architecture of Trust**
  - **Genesis — The Living Equation**
  - **The Observer — The Limits of Knowing**
- Only one Event Horizon path may be active at a time. Completing one completes Event Horizon; switching requires a formal Exit Review.
- Before implementation, the selected path requires a project charter that survives the philosophy review protocol. Charter freezing and final completion require a designated evaluator using evidence from independent technical reviewers and every required domain expert. An LLM cannot be the sole evaluator of research-grade, scientific, safety, or performance claims.

### 5. Authoring and Testing Rules

- Exercise creation is contract-first. The scenario, governing question, objectives, API, inputs and outputs, behavior, edge cases, errors, constraints, non-goals, dependencies, platform assumptions, test strategy, hints, mastery variation, and granularity decision must be discussed and approved before files are generated.
- Contract approval and implementation must not be combined into one silent action.
- Starter code supplies only the signatures and structural scaffolding needed to make the task unambiguous. Learner-owned work remains as `todo!()`, and a completed solution is not included unless explicitly requested.
- Starter crates should normally compile while visible tests fail at runtime for the intended reason. Intentional compile-fail exercises must say so explicitly.
- Visible tests in `tests/behavior.rs` use only documented public behavior and API. They demonstrate a small representative set of success and failure cases without becoming an implementation recipe.
- Review-time adversarial tests in `tests/adversarial.rs` may probe malformed input, boundaries, panic resistance, scheduling, resource limits, and interactions already implied by the contract. They must not introduce undocumented requirements.
- Runtime black-box behavior is the default. Internal implementation constraints are allowed only when the technique is an explicit learning objective.
- Hints progress from naming the concept and authoritative sources through decomposition and relevant APIs. Structural pseudocode or partial scaffolding is reserved for final rescue.
- The AI reviews and explains findings before rewriting learner code, allowing the learner to attempt fixes first.
- Mastery requires behavioral and adversarial validation, explanation, diagnosis, changed-requirement transfer, defended decisions, and limitation analysis.
- `Provisional` is a roadmap planning label, not exercise metadata or a lifecycle status. It means the idea is unapproved and its identity, position, level, scope, or existence may change.

### 6. Relevant Track Boundaries

SERDE-01 teaches:

- Serde derives for the outer `JobManifest`.
- Manual `Serialize` and `Deserialize` implementations for `JobId`.
- Separation of the numeric domain representation from its canonical string wire representation.
- Validation of exactly `job_` followed by eight lowercase hexadecimal digits.
- Invariant enforcement during deserialization.
- Returning errors rather than panicking on invalid external data.
- How a derived outer implementation invokes custom trait implementations on an inner type.

SERDE-01 does not teach strict unknown-field rejection, aliases, advanced structural-versus-semantic error modeling, tagged enums, deliberate missing-versus-`null` semantics, inconsistent legacy representations, private wire-type normalization, NDJSON streaming, schema migrations, borrowed deserialization, async I/O, or implementation of a complete Serde format. Those concepts belong to later exercises or tracks and must not become hidden SERDE-01 requirements.

Only SERDE-01 has an approved complete contract. SERDE-02 through SERDE-06 remain provisional and may be merged, split, moved, reordered, renamed, or removed after learner feedback and granularity review.

There is no fixed exercise count. Work should be:

- Kept together when interaction is the lesson or the guarantee exists only at the integration boundary.
- Split when concepts have independent feedback loops, obscure one another, or have different prerequisites and failure modes.
- Moved when another practical context teaches the concept more naturally.
- Removed when it exists only to manufacture density.

Implementation normally proceeds one approved exercise or milestone at a time.

### 7. Safety and Research Boundaries

Correctness-critical claims must trace to authoritative primary sources. Scientific, research-grade, performance, security, biological, financial, unsafe, safety-critical, and dual-use work requires explicit scope, non-goals, threats, trusted boundaries, safety limits, falsification criteria, and appropriate independent or domain review. Such claims cannot be accepted merely because an LLM generated them or ordinary tests pass.

Correctness tests must be deterministic and portable by default. Unavoidable nondeterminism requires learner approval, controlled seeds or bounded tolerances, reproducibility measures, and documented limitations. Unsupported environments require a portable reference implementation or simulator path.

Performance claims require pinned workloads, documented hardware, distributions and uncertainty rather than unsupported averages, resource limits, and reproducible methodology. Ordinary CI cannot establish environment-specific performance claims.

Simulation is evidence about the declared model, not automatic evidence of scientific truth. Scientific work requires reference models, numerical-error and uncertainty analysis, provenance, reproducible experiments, and review by the relevant domain experts.

Unsafe code must be explicitly permitted and justified by the learning objective. Every exercise permitting it must document its safety invariants and explain why they hold. Tests, Miri, fuzzing, model checking, or simulation can support but cannot replace the safety argument.

Event Horizon work must remain within each path’s explicit safety boundary:

- Iron Meridian is an educational simulation that never connects to live financial venues or controls real funds.
- Genesis uses public, non-sensitive, non-hazardous computational models and must not design pathogens, prescribe treatment, direct wet-lab work, or claim truth beyond validated model boundaries.
- The Observer must not claim physical hardware or quantum advantage without evidence, and must retain a bounded conventional simulation path.

Each Event Horizon path requires a reviewed charter and domain-appropriate evidence, including independent technical reviewers and every required domain expert. Only one path may be active at a time.

### 8. Proposed Next Action

After the evaluator confirms this gate, the learner should begin the approved SERDE-01 solution by implementing the first learner-owned function, `JobId::new`, without changing the public API or contract.

This assumes the learner has not explicitly changed direction. Choosing SERDE-02, altering SERDE-01’s contract, or requesting an AI-written full implementation would require explicit learner approval. I will not perform the proposed action before gate confirmation.

### 9. Forbidden Actions

- Do not modify repository files before the gate is confirmed.
- Do not provide or insert a completed learner solution unless explicitly requested.
- Do not create SERDE-02 under the current direction.
- Do not treat provisional roadmap ideas as approved exercises or lifecycle statuses.
- Do not confirm or record status transitions without learner or designated-evaluator authority.
- Do not invent unresolved behavior, public APIs, error contracts, dependencies, or implementation constraints.
- Do not silently change an approved public API after learner work begins.
- Do not add adversarial tests that impose undocumented behavior.
- Do not steal concepts reserved for later exercises or tracks.
- Do not enforce implementation internals without an explicit learning reason.
- Do not rewrite learner code before reviewing, explaining, and allowing a learner fix attempt.
- Do not use nondeterministic correctness tests without approval and reproducibility controls.
- Do not accept scientific, performance, safety, unsafe-code, or research claims solely from ordinary tests or LLM output.
- Do not activate multiple Event Horizon paths simultaneously.
- Do not freeze an Event Horizon charter or claim completion without designated evaluation and required independent domain evidence.
- Do not silently choose between conflicting canonical documents.

### 10. Open Questions

None.

## Independent Rubric Evaluation

| Point | Status | Citation and evidence |
|---:|:---:|---|
| 1 | Pass | Section 1 separates learner solution ownership from AI scaffolding, testing, review, and maintenance. |
| 2 | Pass | Section 1 defines mastery through explanation, diagnosis, adaptation, trade-offs, and limitations beyond passing tests. |
| 3 | Pass | Section 2 identifies the Core curriculum and the active reliable-data-boundaries-with-`serde` track. |
| 4 | Pass | Section 2 identifies SERDE-01, Job Manifest Codec, as Scaffolded. |
| 5 | Pass | Section 2 states that six learner-owned `todo!()` sections remain and no learner solution exists. |
| 6 | Pass | Sections 2 and 8 identify learner implementation of SERDE-01 as the next step. |
| 7 | Pass | Section 3 lists Levels 1 through 5 in order with their canonical names. |
| 8 | Pass | Section 3 distinguishes focused application, concept integration, reusable subsystem design, adversarial research engineering, and original multidisciplinary research. |
| 9 | Pass | Section 4 classifies The Unknown as a selective Levels 1–3 trial and Field Quest as optional open-source work; neither is a difficulty. |
| 10 | Pass | Section 4 classifies The New Beginning as post-Event-Horizon stewardship rather than Level 6. |
| 11 | Pass | Section 5 requires discussion and approval of the complete exercise contract before files are generated. |
| 12 | Pass | Section 5 preserves learner-owned `todo!()` scaffolding and excludes a completed solution unless explicitly requested. |
| 13 | Pass | Section 5 distinguishes visible behavioral tests from review-time, contract-preserving adversarial tests. |
| 14 | Pass | Section 5 states that runtime black-box behavior is the default and internal constraints need an explicit learning objective. |
| 15 | Pass | Sections 1 and 5 require explanation, diagnosis, changed-requirement transfer, trade-off defense, and limitation analysis for mastery. |
| 16 | Pass | Section 6 preserves SERDE-01 boundaries, identifies later concepts, treats SERDE-02 through SERDE-06 as provisional, and applies the no-fixed-count granularity rule. |
| 17 | Pass | Section 7 requires authoritative primary sources and appropriate independent or domain review for advanced claims. |
| 18 | Pass | Section 7 requires deterministic and portable correctness by default, reproducibility controls, simulator or reference paths, and explicit safety boundaries. |
| 19 | Pass | Section 8 proposes one smallest next action and explicitly waits for evaluator confirmation. |
| 20 | Pass | Sections 8 and 10 surface the operative assumption and state that there are no remaining open questions rather than silently selecting a different direction. |

## Critical Invariant Evaluation

| Invariant | Status | Citation and evidence |
|---:|:---:|---|
| 1 | Pass | Section 1 states that the learner writes exercise solutions unless explicitly requesting a full implementation. |
| 2 | Pass | Section 5 requires exercise contracts to be discussed and approved before files are generated. |
| 3 | Pass | Sections 1 and 5 require the AI to critique and explain before rewriting learner code. |
| 4 | Pass | Section 3 lists Level 1 — First Light, Level 2 — Ascent, Level 3 — Crucible, Level 4 — Abyss, and Level 5 — Event Horizon in order. |
| 5 | Pass | Section 4 limits The Unknown to Levels 1–3 and states that Levels 4–5 are inherently research-heavy. |
| 6 | Pass | Section 4 identifies Field Quest as optional real open-source work and not a difficulty. |
| 7 | Pass | Section 5 makes runtime black-box behavior the default and permits internal constraints only for an explicit learning objective. |
| 8 | Pass | Sections 1 and 5 state that passing tests is not mastery and require explanation, diagnosis, transfer, and limitation analysis. |
| 9 | Pass | Section 6 rejects a fixed exercise count, applies coherent-unit granularity, and requires one approved exercise or milestone at a time. |
| 10 | Pass | Sections 4 and 7 name Iron Meridian — The Architecture of Trust, Genesis — The Living Equation, and The Observer — The Limits of Knowing; state that only one path may be active; and require reviewed charters and domain-appropriate evidence. |
| 11 | Pass | Section 4 identifies The New Beginning as post-Event-Horizon stewardship and not Level 6. |
| 12 | Pass | Section 7 rejects accepting scientific, performance, safety, or research-grade claims solely from LLM output or ordinary tests. |
| 13 | Pass | Section 2 identifies `serde` as the first active track and SERDE-01 as intentionally scaffolded without a learner solution. |
| 14 | Pass | Sections 2 and 8 state that the learner’s next action is to solve SERDE-01 unless the learner explicitly changes direction. |

## Limitations

- This validation shows that one fresh agent could inspect the repository at the tested commit and produce a response satisfying the documented gate after one targeted retry.
- It does not prove that the same agent or another agent will follow the repository contract in future work.
- It does not establish Rust implementation correctness, exercise quality, learner mastery, scientific validity, safety, performance, or sound engineering judgment.
- It does not permanently validate later repository revisions. Material changes to onboarding, canonical ownership, taxonomy, authoring policy, status authority, track boundaries, deterministic-validation policy, unsafe-code policy, or Event Horizon rules require a new repository-only validation.
- The evaluation depends on the cited repository documentation and the independent parent agent’s reported judgment. High-impact work still requires ordinary review, testing, falsification, reproducibility, and relevant domain expertise.
