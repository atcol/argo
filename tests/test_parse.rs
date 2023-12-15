#[test]
fn test_parse_workflow() {
    let body = reqwest::blocking::get(
        "https://raw.githubusercontent.com/argoproj/argo-workflows/v3.5.2/examples/dag-diamond.yaml"
    )
    .unwrap()
    .text()
    .unwrap();

    let target: argo_crds::argo::WorkflowTemplate =
        serde_yaml::from_str(&body).unwrap();
    assert_eq!("Workflow", target.kind.unwrap());
    assert_eq!("diamond", target.spec.entrypoint.unwrap());
    assert_eq!(2, target.spec.templates.len());
    assert_eq!("diamond", target.spec.templates.get(0).unwrap().name.as_ref().unwrap());
    assert_eq!("echo", target.spec.templates.get(1).unwrap().name.as_ref().unwrap());
}
