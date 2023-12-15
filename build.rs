use std::{env, fs, path::Path};
use typify::{TypeSpace, TypeSpacePatch, TypeSpaceSettings};

/// The URL for the Argo JSON schema at a fixed version
static URL: &str =
    "https://raw.githubusercontent.com/argoproj/argo-workflows/v3.5.2/api/jsonschema/schema.json";

fn main() -> Result<(), String> {
        let body = reqwest::blocking::get(URL)
        .expect("Failed to get schema from Argo project")
        .text()
        .expect("Failed to get raw text body");
    std::fs::write("schema.json", body.clone()).expect("Failed to save to file");
    let schema = serde_json::from_str::<schemars::schema::RootSchema>(&body).unwrap();

    let mut settings = TypeSpaceSettings::default();
    settings.with_struct_builder(true);

    for k in schema.definitions.keys() { // .filter(|n| n.contains("io.argoproj.workflow")) {
        if k.contains("io.argoproj") {
            let type_name = k.split(".").last();
            let mut nn = k
                .replace("IoArgoproj", "")
                .replace("V1alpha1", "");
            nn = nn.split(".").last().unwrap().to_string();
            settings.with_patch(
                format!("IoArgoprojWorkflowV1alpha1{}", type_name.unwrap_or("")),
                TypeSpacePatch::default()
                    .with_rename(nn.clone()),
            );
            settings.with_patch(
                format!("IoArgoprojEventsV1alpha1{}", type_name.unwrap_or("")),
                TypeSpacePatch::default()
                    .with_rename(format!("Events{}", nn.clone())),
            );
        }
    }

    let mut type_space = TypeSpace::new(&settings);
    type_space.add_root_schema(schema).expect("Couldn't add root schema");

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
            //Custom change so we produce block doc strings correctly due to http:// usage
            .replace("http://", "http")
            .replace("https://", "https")
    );

    let mut out_file = Path::new(&format!("{}", env::var("OUT_DIR").unwrap())).to_path_buf();
    out_file.push("argo.rs");
    fs::write(out_file, contents).unwrap();
    Ok(())
}
