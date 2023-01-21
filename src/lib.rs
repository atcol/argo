//! The Argo CD, Events and Workflows objects auto generated from its schema.
use serde_derive::{Deserialize, Serialize};

schemafy::schemafy!(
    root: Schema
    "schema.json"
);

pub type WorkflowSpec = IoArgoprojWorkflowV1Alpha1WorkflowSpec;
