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

1. `AGENTS.md`
2. `README.md`
3. `docs/llm-onboarding.md`
4. `docs/exercise-authoring.md`
5. The relevant `docs/tracks/<track>.md`
6. The relevant exercise README, source, and tests
7. `docs/event-horizon.md` or `docs/the-new-beginning.md` when that work is relevant

The agent must inspect the filesystem rather than assume these files or their status from a user prompt.

## Required gate response

Before editing, respond in this exact conceptual structure. Wording need not match the documentation verbatim.

### 1. Purpose

Explain who writes solutions, what the AI contributes, and what mastery means beyond passing tests.

### 2. Current state

Identify:

- Current stage and track
- Existing scaffolded exercise
- What the learner is expected to do next
- Whether any solution has been implemented

### 3. Difficulty model

List Levels 1–5 in order and distinguish their engineering responsibilities.

### 4. Special designations

Explain The Unknown, Field Quest, and The New Beginning. State what is and is not a difficulty level.

### 5. Authoring and testing rules

Summarize contract-first design, starter-state policy, visible tests, review-time adversarial tests, implementation constraints, hints, and mastery validation.

### 6. Relevant track boundaries

State the current exercise's objectives, identify concepts reserved for later exercises, and explain whether the surrounding roadmap is approved or provisional. Apply the granularity rule instead of assuming a fixed exercise count.

### 7. Safety and research boundaries

Explain when primary sources, reproducibility, domain review, simulations, or explicit safety limits are required.

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
4. Difficulty is Level 1 — First Light, Level 2 — Ascent, Level 3 — Crucible, Level 4 — Abyss, and Level 5 — Event Horizon.
5. The Unknown applies selectively to Levels 1–3; Levels 4–5 are inherently research-heavy.
6. Field Quest is optional real open-source work, not a difficulty.
7. Runtime black-box behavior is the default; internal constraints require an explicit learning reason.
8. Passing tests is progress, not mastery; explanation, diagnosis, transfer, and limitation analysis also matter.
9. Planning uses coherent learning units with no fixed exercise count; implementation normally proceeds one approved exercise or milestone at a time.
10. Event Horizon has three original paths, only one active at a time, and requires reviewed charters and domain-appropriate evidence.
11. The New Beginning is post-Event-Horizon stewardship, not Level 6.
12. Scientific, performance, safety, and research-grade claims cannot be accepted solely because an LLM generated them or ordinary tests pass.
13. The current first track is `serde`; SERDE-01 is scaffolded and intentionally has no solution.
14. The next learner action is to solve SERDE-01 unless the learner explicitly changes direction.

## Scoring rubric

Score each item as `1` only when the response is materially correct; otherwise score `0`.

### Purpose — 2 points

1. Correctly separates learner and AI responsibilities.
2. Defines mastery beyond green tests.

### Current state — 4 points

3. Identifies the core curriculum and `serde` track.
4. Identifies SERDE-01 by name and status.
5. States that its implementation remains `todo!()` scaffolding.
6. Proposes learner implementation or contract-preserving support as the next step.

### Taxonomy — 4 points

7. Lists all five levels correctly and in order.
8. Distinguishes focused application, integration, subsystem design, adversarial research, and grand research.
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

## Protocol validation

The onboarding protocol was blind-retested on 2026-09-08 after adopting adaptive learning-unit granularity. The fresh agent received only the repository path and no conversation history.

Independent evaluation found:

- All 14 critical invariants satisfied
- 20 of 20 rubric points satisfied
- Correct identification of SERDE-01 as scaffolded and unsolved
- Correct identification of learner implementation as the next action
- Correct preservation of later `serde` concept boundaries
- Correct rejection of fixed exercise counts
- Correct identification of later `serde` entries as provisional planning hypotheses
- Correct explanation of when work should remain integrated or be split
- Correct refusal to edit before gate confirmation

This validates repository-only onboarding for the current core state. It does not permanently validate future revisions. Repeat the blind pilot after material changes to onboarding rules, taxonomy, authoring policy, or curriculum structure.
