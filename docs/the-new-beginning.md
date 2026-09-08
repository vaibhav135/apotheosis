# The New Beginning

The New Beginning is the required act of stewardship after completing an Event Horizon path. It is not Level 6 and it is not a victory lap.

The graduate must turn one hard-earned insight into a meaningful problem for a future learner.

## Philosophy

Mastery creates responsibility.

A learner who reaches Event Horizon will have encountered misconceptions, failed designs, incomplete documentation, surprising invariants, and moments where a new mental model changed everything. The New Beginning preserves one of those transformations so another person can encounter it deliberately rather than accidentally.

The governing question is:

> What do you understand now that you wish someone had helped you discover earlier?

The contribution must come from genuine experience. It must not exist merely to appear clever, difficult, dark, or impressive.

## Purpose

The New Beginning should:

- Convert personal insight into transferable learning
- Test whether the graduate can explain as well as implement
- Keep the curriculum responsive to real learner experience
- Reveal gaps the original authors could not predict
- Create a renewable path rather than a permanently frozen syllabus
- Practice responsible technical stewardship

Event Horizon demonstrates engineering and research ability. It does not automatically demonstrate teaching ability, so every contribution requires review and piloting.

## Required contribution

The graduate contributes one complete learning artifact. It may be:

- A new exercise from Level 1 through Level 4
- A trial designated as The Unknown for Levels 1–3
- A Field Quest based on a real open-source need
- An advanced-campaign milestone
- A proposal for another Event Horizon path

A new Event Horizon path is exceptional. It must satisfy the complete multidisciplinary, philosophical, scientific, resource, safety, and validation requirements in [Event Horizon](event-horizon.md). Being very large or technically fashionable is not sufficient.

## Source of the problem

The contribution should arise from at least one of:

- A misconception that survived ordinary tests
- A failure mode that invalidated an apparently correct design
- A compiler error whose deeper lesson was not obvious
- An unsafe invariant that was easy to state incorrectly
- A scientific or mathematical assumption that changed the conclusion
- A performance result invalidated by poor measurement
- A production pattern missing from the curriculum
- An important connection between previously separate disciplines
- A failed Event Horizon approach that produced reusable knowledge
- A limitation discovered in the curriculum itself

## Required rationale

Before creating files, write a proposal answering:

1. Why does this problem deserve to exist?
2. What transferable concept does it teach?
3. What changed in the graduate's own understanding?
4. Why is the selected difficulty appropriate?
5. Which prerequisites are genuinely necessary?
6. What observable evidence would demonstrate learning?
7. How could a learner pass tests without understanding?
8. What changed-requirement task would expose shallow understanding?
9. What safety, accessibility, or scientific-validity risks exist?
10. Why is this better than adding a note or test to an existing exercise?

If the final question has no convincing answer, improve an existing exercise instead of creating a new one.

## Contribution formats

The artifact format follows the contribution type. Do not force non-exercise work into an exercise crate.

### Exercise or The Unknown

An ordinary exercise or The Unknown contribution must include:

```text
exercises/
└── <track>/
    └── <number>-<exercise-name>/
        ├── Cargo.toml
        ├── Cargo.lock
        ├── README.md
        ├── src/
        │   └── lib.rs
        └── tests/
            └── behavior.rs
```

Additional fixtures, binaries, compile-fail suites, benchmarks, simulators, or models should be added only when required by the learning objective.

The exercise README must follow the repository's standard specification and additionally include:

- **Origin:** the insight or failure that inspired the problem
- **Intended transformation:** how the learner's mental model should change
- **Misconception target:** the tempting but incorrect model
- **Transfer task:** a changed requirement used during mastery review
- **Reviewer notes:** likely ambiguity, safety, and validation risks

Do not include a completed implementation in starter code.

### Field Quest

A Field Quest contribution must include:

- Upstream project and issue or need
- Reproduction or motivation
- Proposed patch or review-ready change
- Tests and validation evidence
- Maintainer expectations and compatibility notes
- Review outcome, without requiring upstream acceptance

### Advanced-campaign milestone

An advanced milestone must follow its campaign charter and include its contract, prerequisites, artifacts, validation, safety boundaries, and completion evidence. A Cargo crate is required only when the milestone produces one.

### Event Horizon path proposal

A path proposal belongs under `docs/` and must include the complete Event Horizon charter, coherence argument, resource envelope, safety model, primary-source map, reviewer requirements, and evidence that an advanced campaign cannot represent the same transformation. It does not require an exercise crate before acceptance.

## Difficulty assignment

Assign the lowest level capable of teaching the lesson honestly:

- **Level 1 — First Light:** one focused concept with meaningful edge cases
- **Level 2 — Ascent:** several interacting concepts and realistic error handling
- **Level 3 — Crucible:** reusable subsystem design and defended trade-offs
- **Level 4 — Abyss:** inherently unfamiliar, adversarial, research-heavy engineering
- **Level 5 — Event Horizon:** original multidisciplinary research under a reviewed charter

Difficulty is not determined by code length, obscurity, or how frustrated the author felt.

## The Unknown and Field Quest

These are independent designations:

- **The Unknown:** a Level 1–3 trial that evaluates a structured research process around a major unfamiliar concept
- **Field Quest:** work situated in a real open-source project

The Unknown requires research notes, isolated experiments, a proposed mental model, progressive rescue hints, and a postmortem.

A Field Quest succeeds when the contribution is technically defensible and review-ready. Upstream acceptance cannot be required because maintainers control that outcome.

## Acceptance lifecycle

Every New Beginning contribution remains outside the main curriculum until completing:

```text
Proposed → Reviewed → Piloted → Revised → Accepted
```

The graduate owns the proposal but cannot accept it alone. A designated evaluator confirms lifecycle transitions using evidence from technical reviewers, required domain experts, and the applicable pilot learner or form-specific pilot reviewers. An LLM may assist but cannot be the sole evaluator for scientific, safety-critical, formal, or research-grade claims.

### Proposed

- Rationale is complete.
- Learning or research objective and classification are explicit.
- The contribution-type-specific contract, charter, or review objective exists before implementation-specific validation.
- Prerequisites, risks, and non-goals are documented.

An exercise proposal requires its behavioral contract before tests. A Field Quest requires a reproducible upstream need. An advanced milestone requires a campaign contract. An Event Horizon path proposal requires a complete draft charter.

### Reviewed

At least one reviewer examines:

- Technical correctness
- Pedagogical clarity
- Difficulty calibration
- Test quality
- Possibilities for gaming validation
- Accessibility and resource assumptions
- Safety and ethical boundaries
- Scientific claims where applicable

Domain expertise is mandatory for correctness-critical scientific, financial, security, or formal claims.

### Piloted

Piloting must match the artifact:

- **Exercise or The Unknown:** another learner attempts it without private explanation from the author.
- **Field Quest:** an independent reviewer reproduces the need and reviews or applies the proposed change in a clean checkout.
- **Advanced milestone:** a qualified reviewer runs the smallest useful feasibility or replication milestone defined by its campaign.
- **Event Horizon path proposal:** technical and domain reviewers conduct a charter red team, resource-feasibility review, and bounded prototype or reference-model exercise. They do not need to complete the proposed path.

Where applicable, observe:

- Where the learner misunderstands the contract
- Which hints are opened and when
- Whether difficulty comes from the intended concept
- Whether tests provide useful feedback
- Whether the learner can explain and transfer the result
- Which accidental prerequisites appear
- Whether reviewers can reproduce the claimed need, result, or feasibility without private context

The pilot evaluates the artifact, not the pilot learner or reviewer.

### Revised

The author addresses evidence from review and piloting. Changes and rejected assumptions are summarized as durable conclusions rather than a conversation log.

Substantial revisions may require another pilot.

### Accepted

The contribution joins the curriculum only when:

- Its contract, charter, or contribution objective is precise and fair
- The intended concept, need, or research transformation is coherent
- Form-specific validation supports the claimed outcome
- Required resources are documented and reasonable
- Safety and scientific claims have appropriate review
- A future learner or LLM can maintain it from repository documentation

For exercises, form-specific validation includes tests, mastery gates, and a learner pilot. For Field Quests it includes reproducibility and patch review. For advanced milestones it includes campaign-defined evidence. For Event Horizon path proposals it includes charter review, a bounded feasibility artifact, and all required technical and domain approvals.

## Quality rubric

Reviewers should evaluate each dimension explicitly:

| Dimension | Required question |
|---|---|
| Purpose | Does this teach a transferable concept worth practicing? |
| Coherence | Does every constraint support that purpose? |
| Clarity | Can the learner distinguish requirements, suggestions, and non-goals? |
| Difficulty | Is challenge produced by the intended reasoning rather than accidental confusion? |
| Validation | Can shallow or hard-coded solutions be detected appropriately? |
| Transfer | Can the learner adapt the concept to a changed requirement? |
| Feasibility | Are time, hardware, money, and prerequisites bounded? |
| Reproducibility | Can another person reproduce correctness and performance evidence? |
| Safety | Are harmful, unsafe, or dual-use boundaries explicit? |
| Maintainability | Can dependencies, tests, and documentation survive future changes? |
| Humility | Are limitations and uncertainty stated honestly? |

No aggregate score can excuse a critical failure in correctness, safety, or scientific validity.

## What must not be contributed

Reject a problem that:

- Exists primarily to make someone suffer
- Hides requirements or expects test-author mind reading
- Rewards obscure API memorization
- Combines unrelated disciplines for spectacle
- Claims scientific validity from software tests alone
- Requires expensive infrastructure without a portable learning path
- Depends on live financial, medical, biological, or safety-critical systems
- Includes unreviewed unsafe code as an incidental complication
- Copies another exercise without a distinct learning objective
- Provides a complete starter solution
- Cannot state what evidence would falsify its teaching claim

## Relationship to philosophy review

Before acceptance, apply the complete review protocol in [`philosophy.md`](philosophy.md):

1. Restate the learning claim.
2. Construct the theory of change.
3. Red-team the exercise.
4. Run a premortem.
5. Define falsification evidence.
6. Test coherence and feasibility.
7. Pilot the smallest useful version.
8. Record durable conclusions.

## Completion

The New Beginning is complete when:

- One contribution reaches Accepted status
- The roadmap and status index include it
- Review and pilot evidence are recorded
- The graduate explains what teaching the problem revealed about their own understanding
- The next learner can begin without needing the author's private context

Reaching the end of the curriculum therefore creates a tested beginning for someone else.

## Status

- Philosophy and acceptance lifecycle: agreed
- Graduate contributor: none yet
- Proposed contribution: none
- Accepted contribution: none
