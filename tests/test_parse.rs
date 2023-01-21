#[test]
fn test_parse_workflow() {
    let body = reqwest::blocking::get(
        "https://raw.githubusercontent.com/argoproj/argo-workflows/v3.4.4/examples/dag-diamond.yaml"
    )
    .unwrap()
    .text()
    .unwrap();

    let target: argo_models::IoArgoprojWorkflowV1Alpha1WorkflowTemplate =
        serde_yaml::from_str(&body).unwrap();
    assert_eq!("Workflow", target.kind.unwrap());
    assert_eq!("diamond", target.spec.entrypoint.unwrap());
    let templates = target.spec.templates.expect("templates should be set");
    assert_eq!(2, templates.len());

    assert_eq!("diamond", templates.get(0).unwrap().name.as_ref().unwrap());
    assert_eq!("echo", templates.get(1).unwrap().name.as_ref().unwrap());
}
