# LLM onboarding and comprehension gate

This protocol allows a capable LLM to enter the repository without receiving private conversation history. Repository documentation is the source of truth.

The gate is not an intelligence contest. It verifies that the agent inspected the project, identified its current state, and understood the constraints that protect the learner.

## When the gate is required

The gate is required before an LLM:

- Creates, changes, or removes an exercise
- Adds tests or new behavioral requirements
- Reviews or rewrites a learner solution
- Changes curriculum structure, philosophy, status, or tooling
- Designs an advanced campaign or Event Horizon milestone

Purely informational questions that require no repository changes do not need a formal gate, though the relevant documents should still be read.

## Mandatory reading order

Always read:

1. `AGENTS.md`
2. `README.md`
3. `docs/philosophy.md`
4. `docs/curriculum.md`
5. `docs/llm-onboarding.md`

Then read the documents required by the task:

6. For exercise creation, changes, tests, or review: `docs/exercise-authoring.md`, the relevant `docs/tracks/<track>.md`, and the complete exercise README, source, and tests
7. For Event Horizon work: `docs/event-horizon.md`
8. For The New Beginning work: `docs/the-new-beginning.md`

The agent must inspect the filesystem rather than assume these files or their status from a user prompt.

## Required gate response

Before editing, respond in this exact conceptual structure. Wording need not match the documentation verbatim.

### 1. Purpose

Explain who writes solutions, what the AI contributes, and what mastery means beyond passing tests.

### 2. Current state

Identify:

- Current Level, Curriculum, and track
- Existing scaffolded exercise
- What the learner is expected to do next
- Whether any solution has been implemented

### 3. Level and curriculum model

Explain the Level-first hierarchy. List Levels 1–5 in order with their engineering responsibilities, identify where Core and Advanced work occur, distinguish Core from Advanced, and define optional specialization. Explain what a Convergence checkpoint is and is not.

### 4. Special designations

Explain The Unknown, Field Quest, and The New Beginning. State which items are and are not Levels.

### 5. Authoring and testing rules

Summarize contract-first design, starter-state policy, visible tests, review-time adversarial tests, implementation constraints, hints, and mastery validation.

### 6. Relevant track boundaries

State the current exercise's objectives, identify concepts reserved for later exercises, and explain whether the surrounding roadmap is approved or provisional. Apply the granularity rule instead of assuming a fixed exercise count.

### 7. Safety and research boundaries

Explain when primary sources, reproducibility, domain review, simulations, or explicit safety limits are required.

The response must also:

- Name all three original Event Horizon paths: **Iron Meridian — The Architecture of Trust**, **Genesis — The Living Equation**, and **The Observer — The Limits of Knowing**.
- State that only one path may be active at a time and completing one completes Event Horizon.
- State that exact charters remain unfrozen until prerequisites are complete and that each active path requires a reviewed charter with domain-appropriate evidence.

### 8. Proposed next action

State one smallest appropriate next action. Surface assumptions and decisions requiring learner approval. Do not perform it yet.

### 9. Forbidden actions

List at least five actions the agent must not take.

### 10. Open questions

Identify genuine ambiguities instead of silently resolving them. State `None` if the requested next action is fully specified.

## Critical invariants

Every item below must be understood. Missing or contradicting any one fails the gate regardless of total score.

1. The learner writes exercise solutions unless explicitly requesting a full implementation.
2. Exercise contracts are discussed and approved before files are generated.
3. The AI critiques and explains before rewriting learner code.
4. The progression is Level 1 — First Light, Level 2 — Ascent, Level 3 — Crucible, Level 4 — Abyss, and Level 5 — Event Horizon.
5. The Unknown applies selectively to Levels 1–3; Levels 4–5 are inherently research-heavy.
6. Field Quest is optional real open-source work, not a Level.
7. Runtime black-box behavior is the default; internal constraints require an explicit learning reason.
8. Passing tests is progress, not mastery; explanation, diagnosis, transfer, and limitation analysis also matter.
9. Planning uses coherent learning units with no fixed exercise count; implementation normally proceeds one approved exercise or milestone at a time.
10. The response satisfies every Event Horizon output requirement stated in section 7; naming all three paths is mandatory rather than implied by saying that three paths exist.
11. The New Beginning is post-Event-Horizon stewardship, not Level 6.
12. Scientific, performance, safety, and research-grade claims cannot be accepted solely because an LLM generated them or ordinary tests pass.
13. The current first track is `serde`; SERDE-01 is scaffolded and intentionally has no solution.
14. The next learner action is to solve SERDE-01 unless the learner explicitly changes direction.
15. Level is primary: Levels 1–2 are Core, Level 3 may be Core or Advanced, Level 4 is Advanced, and Event Horizon is only Level 5. The New Beginning follows the levels.
16. Curriculum distinguishes Core use of established machinery from Advanced investigation or construction beneath it; optional specialization is a separate participation requirement.
17. Convergence is an occasional integration checkpoint for a combined guarantee, not a level, Curriculum value, participation requirement, lifecycle status, or fixed-count requirement.

## Scoring rubric

Score each item as `1` only when the response is materially correct; otherwise score `0`.

### Purpose — 2 points

1. Correctly separates learner and AI responsibilities.
2. Defines mastery beyond green tests.

### Current state — 4 points

3. Identifies the current work as Level 1, Core, and in the `serde` track.
4. Identifies SERDE-01 by name and status.
5. States that its implementation remains `todo!()` scaffolding.
6. Proposes learner implementation or contract-preserving support as the next step.

### Taxonomy — 4 points

7. Lists all five levels correctly and in order.
8. Distinguishes focused application, integration, subsystem design, adversarial research, and grand research; also explains the Level-first Core/Advanced hierarchy and participation requirement.
9. Classifies The Unknown and Field Quest correctly.
10. Classifies The New Beginning correctly.

### Authoring and validation — 5 points

11. Requires contract approval before code generation.
12. Preserves starter scaffolding without a completed solution.
13. Distinguishes visible behavioral tests from review-time adversarial tests.
14. Restricts internal-design enforcement to explicit learning objectives.
15. Includes explanation, changed requirements, and limitations in mastery.

### Scope, research, and safety — 3 points

16. Preserves track boundaries, recognizes provisional roadmap entries, and avoids using a fixed count or stealing later concepts.
17. Requires primary sources and appropriate review for advanced claims.
18. Recognizes portability, reproducibility, and explicit safety boundaries.

### Collaboration — 2 points

19. Proposes one small next action and waits for approval.
20. Surfaces ambiguity rather than making a silent decision.

## Passing rule

The gate passes only when:

- All critical invariants pass, and
- At least 19 of 20 rubric points pass (`95%`).

A score is not valid unless the evaluator cites which response statements satisfy each point. This prevents an unsupported “95% match” claim.

The evaluator may be the learner, a separate capable LLM, or both. For high-impact work, use an independent evaluator rather than relying only on self-scoring.

## Failure and retry

If the gate fails:

1. Identify missed or contradicted invariants.
2. Point the agent to the relevant source documents.
3. Let it reread and submit a new response.
4. Do not permit repository edits until it passes.

The goal is synchronization, not punishment.

## Limits of the gate

Passing proves that an agent can restate the repository contract. It does not prove that it will follow the contract, write correct Rust, understand advanced science, or exercise sound judgment.

All generated work still requires ordinary review, tests, falsification, and domain expertise where applicable.

## Validation status

The current Level-first onboarding protocol requires independent validation before it may be described as validated. Superseded evidence is indexed in [`docs/validation/README.md`](validation/README.md) as a historical audit trail, not as proof about the current rules.
