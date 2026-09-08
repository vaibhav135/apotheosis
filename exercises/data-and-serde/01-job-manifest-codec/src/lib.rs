use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A job identifier stored numerically in the domain and represented as a
/// canonical string at the JSON boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(u32);

impl JobId {
    pub fn new(_value: u32) -> Self {
        todo!("construct a JobId")
    }

    pub fn get(self) -> u32 {
        todo!("return the numeric identifier")
    }
}

impl Serialize for JobId {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!("serialize JobId using its canonical wire representation")
    }
}

impl<'de> Deserialize<'de> for JobId {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!("deserialize and validate the JobId wire representation")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobManifest {
    pub job_id: JobId,
    pub name: String,
    pub enabled: bool,
    pub max_retries: u8,
}

pub fn decode_manifest(_input: &str) -> Result<JobManifest, serde_json::Error> {
    todo!("decode a job manifest from JSON")
}

pub fn encode_manifest(_manifest: &JobManifest) -> Result<String, serde_json::Error> {
    todo!("encode a job manifest as compact JSON")
}
