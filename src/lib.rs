use serde_derive::{Deserialize, Serialize};

schemafy::schemafy!(
    root: Schema
    "schema.json"
);

pub type WorkflowSpec = IoArgoprojWorkflowV1Alpha1WorkflowSpec;
