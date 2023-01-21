static URL: &str = "https://raw.githubusercontent.com/argoproj/argo-workflows/v3.4.4/api/jsonschema/schema.json";
fn main() -> Result<(), String> {
    let body = reqwest::blocking::get(URL).expect("Failed to get schema from Argo project")
    .text().expect("Failed to get raw text body");
    std::fs::write("schema.json", body).expect("Failed to save to file");
    Ok(())
}
