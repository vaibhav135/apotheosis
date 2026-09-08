# SERDE-01: Job Manifest Codec

- **ID:** `SERDE-01`
- **Track:** Reliable data boundaries with `serde`
- **Stage:** Core
- **Difficulty:** Level 1 — First Light
- **Trial:** None
- **Field Quest:** No
- **Status:** Scaffolded
- **Prerequisites:** Rust fundamentals

## Learning objectives

- Use Serde derives for ordinary structures
- Implement `Serialize` and `Deserialize` manually for a domain newtype
- Separate an internal representation from its wire representation
- Enforce domain invariants during deserialization
- Return useful errors instead of panicking
- Understand how a derived outer implementation invokes a custom inner implementation

## Scenario

An external job system represents identifiers as fixed-format JSON strings:

```json
{
  "job_id": "job_0000002a",
  "name": "thumbnail-generator",
  "enabled": true,
  "max_retries": 3
}
```

Inside the application, an identifier is numeric:

```rust
pub struct JobId(u32);
```

The wire representation and domain representation deliberately differ.

## Task

Complete the functions and trait implementations in `src/lib.rs`.

You must implement:

- `JobId::new`
- `JobId::get`
- `Serialize` for `JobId`
- `Deserialize` for `JobId`
- `decode_manifest`
- `encode_manifest`

`JobManifest` already derives Serde's traits.

## Public API

The approved public API is defined in `src/lib.rs` and consists of:

- `JobId` with `new` and `get`
- Manual `Serialize` and `Deserialize` implementations for `JobId`
- `JobManifest` with public data fields
- `decode_manifest`
- `encode_manifest`

Do not rename or remove these items while solving the exercise.

## Job identifier format

A valid wire representation contains `job_` followed by exactly eight lowercase hexadecimal digits:

```text
job_<eight lowercase hexadecimal digits>
```

Valid examples:

```text
job_00000000
job_0000002a
job_deadbeef
job_ffffffff
```

Invalid examples:

```text
job_42
job_0000002A
JOB_0000002a
task_0000002a
job_0000000g
job_000000000
job_-000002a
```

Serialization must always produce the canonical lowercase representation.

## Required behavior

1. `JobId::new(42)` serializes as `"job_0000002a"`.
2. `"job_deadbeef"` deserializes to the corresponding numeric value.
3. Complete manifests serialize and deserialize correctly.
4. Valid manifests survive a serialization round trip.
5. Malformed identifiers return errors.
6. JSON numbers, booleans, arrays, objects, and `null` are not accepted as job IDs.
7. Missing required manifest fields return errors.
8. Invalid input never causes a panic.
9. Validation errors communicate the expected identifier format.
10. Encoding produces compact JSON.
11. Every `u32` has exactly one canonical serialized representation.

Unknown manifest fields use Serde's default behavior in this exercise. Strict configuration handling belongs in SERDE-02.

## Error behavior

- Invalid JSON and invalid job IDs return `serde_json::Error` through the documented API.
- Job ID validation errors should communicate the expected canonical format.
- Consumers must not depend on the complete wording of third-party errors.
- Invalid external input must not panic.

## Constraints

- Manually implement `Serialize` and `Deserialize` for `JobId`.
- Do not replace those implementations with Serde conversion attributes.
- Let Serde parse complete JSON documents; do not manually rewrite or parse the surrounding JSON.
- Do not use `unwrap`, `expect`, or deliberate panics in library code.
- Use only `serde`, `serde_json`, and the standard library.
- Consumers must not need to match the complete wording of a Serde error.

## Non-goals

- Rejecting unknown manifest fields
- Supporting legacy or alternate identifier formats
- Semantic validation of manifest fields other than `job_id`
- Borrowed or zero-copy deserialization
- Implementing a complete Serde data format
- Async, streaming, or multi-format input

## Commands

Run the exercise tests:

```bash
cargo test --manifest-path exercises/data-and-serde/01-job-manifest-codec/Cargo.toml
```

Run formatting and lint checks from the repository root:

```bash
cargo fmt --manifest-path exercises/data-and-serde/01-job-manifest-codec/Cargo.toml --check
cargo clippy --manifest-path exercises/data-and-serde/01-job-manifest-codec/Cargo.toml -- -D warnings
```

## Visible-test scope

The starter tests cover:

- One canonical serialization
- One canonical deserialization
- A complete manifest round trip
- One malformed identifier
- One incorrect JSON value type

## Review-time adversarial scope

After the initial solution, review tests may cover:

- Every documented format boundary
- Lowercase canonicalization and maximum `u32` values
- Wrong JSON types and missing manifest fields
- Malformed ASCII and Unicode input
- Panic resistance

These tests may not introduce legacy aliases, strict unknown-field behavior, or concepts reserved for later exercises.

## Progressive hints

Open only as much help as you need.

<details>
<summary>Hint 1 — Representation</summary>

Keep the numeric domain value separate from the string used at the JSON boundary.

</details>

<details>
<summary>Hint 2 — Traits</summary>

Read the signatures of `Serialize::serialize` and `Deserialize::deserialize`. Your implementation receives a format-independent serializer or deserializer.

</details>

<details>
<summary>Hint 3 — Producing the string</summary>

A serializer can serialize an already formatted string. Look for a suitable method on the `Serializer` trait.

</details>

<details>
<summary>Hint 4 — Receiving the string</summary>

`String::deserialize(deserializer)` can let Serde perform JSON string extraction before you validate the result.

</details>

<details>
<summary>Hint 5 — Validation</summary>

`strip_prefix` and `u32::from_str_radix` may help. Remember that successful hexadecimal parsing alone does not prove that the input used the exact canonical length and casing.

</details>

<details>
<summary>Hint 6 — Validation errors</summary>

`serde::de::Error::custom` can convert your validation failure into the deserializer's error type.

</details>

## Mastery review

Passing tests is not the final mastery gate. After implementation, be prepared to:

1. Explain how derived `JobManifest::deserialize` reaches your custom `JobId` implementation.
2. Diagnose an intentionally broken implementation that accepts non-canonical values.
3. Adapt the decoder to accept an uppercase legacy ID while continuing to serialize canonical lowercase IDs.
4. Explain why accepted input and generated output do not always require identical representations.

Do not implement the legacy variation until the initial solution has been reviewed.

## Primary sources

- [Serde overview](https://serde.rs/)
- [Using derive](https://serde.rs/derive.html)
- [Implementing `Serialize`](https://serde.rs/impl-serialize.html)
- [Implementing `Deserialize`](https://serde.rs/impl-deserialize.html)
- [`serde_json`](https://docs.rs/serde_json/)

## Authoring notes

These notes constrain future maintainers and do not prescribe the learner's implementation.

- **Reserved concepts:** strict unknown-field configuration, tagged enums, legacy alternatives, streaming, borrowed deserialization, and full custom data formats belong to later exercises.
- **Difficulty rationale:** one primary new system is introduced, but the custom newtype, canonical representation, and malformed-input requirements prevent a trivial derive-only solution.
- **Expected failure modes:** accepting uppercase or wrong-length IDs, serializing the numeric value directly, panicking on malformed input, parsing complete JSON manually, or using unstable full error strings in tests.
- **Approval record:** the learner approved the contract before scaffolding.
