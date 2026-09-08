use serde_01_job_manifest_codec::{JobId, JobManifest, decode_manifest, encode_manifest};

#[test]
fn job_id_serializes_to_the_canonical_wire_format() {
    let encoded = serde_json::to_string(&JobId::new(42)).expect("JobId should serialize");

    assert_eq!(encoded, r#""job_0000002a""#);
}

#[test]
fn job_id_deserializes_from_the_canonical_wire_format() {
    let id: JobId =
        serde_json::from_str(r#""job_deadbeef""#).expect("valid JobId should deserialize");

    assert_eq!(id.get(), 0xdead_beef);
}

#[test]
fn a_complete_manifest_round_trips() {
    let original = JobManifest {
        job_id: JobId::new(42),
        name: "thumbnail-generator".to_owned(),
        enabled: true,
        max_retries: 3,
    };

    let encoded = encode_manifest(&original).expect("manifest should serialize");
    let decoded = decode_manifest(&encoded).expect("serialized manifest should deserialize");

    assert_eq!(decoded, original);
    assert!(!encoded.contains('\n'), "encoding should use compact JSON");
}

#[test]
fn a_malformed_identifier_is_rejected() {
    let result = serde_json::from_str::<JobId>(r#""job_42""#);

    assert!(result.is_err());
}

#[test]
fn a_json_number_is_not_a_job_identifier() {
    let result = serde_json::from_str::<JobId>("42");

    assert!(result.is_err());
}
