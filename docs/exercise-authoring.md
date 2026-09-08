# Exercise authoring specification

This document defines how exercises are proposed, scaffolded, tested, reviewed, and maintained. It converts the top-level philosophy into concrete authoring rules.

Use [`../templates/exercise-readme.md`](../templates/exercise-readme.md) for every new exercise.

## Authoring sequence

1. Read the top-level README and relevant track specification.
2. Identify one curriculum gap or approved roadmap item.
3. Propose the contract, public API, dependencies, constraints, and test strategy.
4. Explain assumptions and alternatives.
5. Wait for learner approval.
6. Create starter code, visible tests, hints, and documentation without implementing the solution.
7. Verify formatting, linting, and test compilation.
8. Let the learner implement and run tests.
9. Review before rewriting.
10. Add contract-preserving adversarial tests where useful.
11. Run the mastery gate and update status.

Do not batch contract approval and implementation into one silent action.

## Mandatory metadata

Every exercise README begins with these fields:

```text
ID: <TRACK-NUMBER>
Track: <canonical track name>
Stage: Core | Advanced | Event Horizon
Difficulty: Level 1 — First Light | Level 2 — Ascent | Level 3 — Crucible | Level 4 — Abyss | Level 5 — Event Horizon
Trial: None | The Unknown
Field Quest: Yes | No
Status: Planned | Scaffolded | In Progress | Tests Pass | Reviewed | Mastered
Prerequisites: None | <comma-separated exercise IDs or named gates>
```

Rules:

- `The Unknown` is valid only for Levels 1–3.
- Levels 4–5 are inherently research-heavy and use `Trial: None`.
- Field Quest is independent of difficulty and trial.
- IDs remain stable after publication.
- Difficulty changes require a documented review, not only a renamed label.

## Status transitions

### Planned

- Roadmap entry exists.
- Broad objective and place in the sequence are agreed.
- Exact contract may remain undecided.

### Scaffolded

- Contract is approved and documented.
- Starter code and visible tests exist.
- Formatting, Clippy, and test compilation succeed.
- Learner implementation remains incomplete.

### In Progress

- The learner has begun implementation.

### Tests Pass

- Current behavioral and adversarial tests pass.
- Formatting and Clippy pass.
- Mastery review has not necessarily occurred.

### Reviewed

- Correctness and code-quality review is complete.
- Identified revisions are resolved.
- Required design, research, or safety notes exist.

### Mastered

- The learner explains the concept.
- The learner diagnoses a broken variation.
- The learner adapts the solution to a changed requirement.
- The learner identifies limitations and failure boundaries.

Status changes must be reflected in the exercise README and top-level roadmap.

## Contract-first design

Before tests or starter code, decide and obtain approval for:

- Scenario and learning objective
- Inputs, outputs, and public API
- Required behavior and edge cases
- Error categories and stability expectations
- Relevant resource and performance limits
- Required implementation constraints
- Explicit non-goals
- Visible-test scope
- Later adversarial-test scope
- Hint progression
- Mastery variation
- Dependencies and platform assumptions

The contract must answer relevant boundary questions such as:

- Missing versus `null`
- Duplicate fields or keys
- Unknown fields or variants
- Casing, whitespace, Unicode, and normalization
- Overflow, malformed values, and trailing input
- Ordering and determinism
- Panic behavior
- Input size and resource bounds

Do not invent answers to irrelevant boundary questions merely to lengthen the problem.

## Public API policy

- Tests may depend only on documented public behavior and API.
- A public signature is part of the contract once approved.
- Do not change it silently after the learner starts.
- Keep representation private when encapsulation is part of the lesson.
- Expose constructors and accessors needed for behavioral testing.
- Avoid getters, wrappers, or traits that do not serve the exercise.

## Starter-code policy

- Provide signatures and structural scaffolding required to make the task unambiguous.
- Use `todo!()` for learner-owned implementation.
- Prefix unused starter parameters with `_` so Clippy can pass before implementation.
- Prefer a starter crate that compiles and whose tests fail at runtime.
- Intentional compile-fail exercises must clearly say so.
- Boilerplate unrelated to the lesson may be provided when doing so does not solve the task.
- Never include a completed learner solution unless explicitly requested.

## Test lifecycle

### Visible behavioral tests

Initial tests live in:

```text
tests/behavior.rs
```

They should:

- Demonstrate representative contract behavior
- Include at least one failure case when errors matter
- Use only documented public API
- Avoid revealing an implementation recipe
- Compile before the learner solution exists

There is no fixed test count. Use the smallest set that communicates the contract without making the test file a disguised solution.

### Review-time adversarial tests

After the learner's initial solution, broader tests may be added to:

```text
tests/adversarial.rs
```

They may explore malformed input, boundaries, panic resistance, scheduling, resource limits, and interactions already implied by the contract.

Adversarial tests must not introduce undocumented behavior. If a new requirement is valuable, discuss and approve a contract change before using it to reject the solution.

Committed adversarial tests remain visible. Temporary reviewer experiments are permitted, but any experiment used as lasting completion evidence must be reproducible and documented.

### Other validation

Use compile tests, properties, fuzzing, benchmarks, Miri, Loom, Kani, simulation, or scientific reference models only when appropriate to the lesson. Document commands, assumptions, and limits.

## Error-contract policy

- Prefer stable public error variants when callers need to distinguish failure categories.
- Test exact variants or structured fields when they are part of the contract.
- Do not test complete third-party error wording.
- Require useful error context through documented categories or stable substrings only when necessary.
- Distinguish structural decoding errors from semantic/domain validation when that distinction is the lesson.
- Invalid external input should return an error rather than panic unless panic behavior is explicitly the subject.

## Hints

Hints are progressive and collapsible:

1. Name the concept.
2. Point to authoritative documentation.
3. Suggest decomposition.
4. Identify relevant APIs.
5. Reveal a likely broken assumption.
6. Provide structural pseudocode or partial scaffolding only as a final rescue.

Opening a hint is not failure. Hints should teach rather than merely reveal syntax.

## Mastery artifacts

Create a `learner/` directory only when an artifact becomes necessary:

```text
learner/
├── design.md       # required for Levels 3–5
├── mastery.md      # explanation, diagnosis, transfer, and limitations
├── research.md     # required for The Unknown and research-grade work
└── postmortem.md   # required for The Unknown and Event Horizon milestones
```

Do not generate completed learner artifacts. Exercise READMEs provide prompts; the learner creates the responses.

## Difficulty calibration

- **Level 1 — First Light:** one focused concept, meaningful validation, little architectural freedom
- **Level 2 — Ascent:** interacting concepts, realistic errors, and local design choices
- **Level 3 — Crucible:** reusable subsystem design, public contracts, and defended trade-offs
- **Level 4 — Abyss:** bounded but adversarial research engineering with profound unfamiliarity
- **Level 5 — Event Horizon:** original multidisciplinary work under a reviewed charter

Level 3–5 exercises use philosophical titles only when the title describes the intended transformation. Always pair one with a precise technical subtitle.

## Track boundaries

- Follow the relevant track specification.
- Do not teach a concept reserved for a later exercise as a hidden requirement.
- Repetition is allowed only when the concept appears in a meaningfully different interaction.
- Each track defines minimum, recommended, and research-depth exit gates.
- Cross-track prerequisites must name an exercise ID or mastery gate.

## Dependencies and portability

- Stable Rust and Edition 2024 are the default.
- Each exercise is an independent crate with a committed lockfile.
- Add only dependencies that serve the learning objective.
- Prefer maintained crates and authoritative documentation.
- Correctness tests should be deterministic and portable wherever practical.
- Separate hardware-specific performance gates from portable correctness gates.
- Live external services require an approved reason and a reliable local substitute.

## Review checklist

Before marking an exercise Scaffolded, verify:

- Metadata is complete and canonical.
- Contract and non-goals are explicit.
- Public API matches tests and prose.
- Starter code does not contain the solution.
- Visible tests compile and fail for the intended reason.
- Formatting and Clippy pass with warnings denied.
- Dependencies and lockfile are present.
- Hints progress from concepts to APIs without giving away the solution immediately.
- Primary sources are cited.
- Track boundaries are preserved.
- README roadmap status is updated.

Before marking Mastered, verify:

- Behavioral and adversarial validation passes.
- Review findings are resolved.
- Required learner artifacts exist.
- Explanation, diagnosis, transfer, and limitation checks pass.
- Status is updated consistently.

## LLM decision boundaries

An LLM may independently:

- Point out ambiguity
- Propose alternatives and trade-offs
- Add tests for already documented behavior after review
- Suggest focused recovery work
- Update status after verified completion

An LLM must obtain approval before:

- Choosing an unresolved behavior
- Changing public API or error contracts
- Adding dependencies
- Moving concepts between exercises
- Adding implementation constraints
- Changing difficulty or prerequisites
- Expanding scope or introducing a new track
- Rewriting learner code

When uncertain, stop and ask. Silence is not approval.
