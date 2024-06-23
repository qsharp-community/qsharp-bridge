use qsharp_bridge::sim::{run_qs, run_qs_shots};

#[test]
fn test_hello() {
    let source = std::fs::read_to_string("tests/assets/hello.qs").unwrap();
    let result = run_qs(&source).unwrap();

    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0], "Hello");

    assert_eq!(result.qubit_count, 0);
    assert_eq!(result.states.len(), 0);

    assert!(result.result.is_some());
    assert_eq!(result.result, Some("()".into()));
}

#[test]
fn test_hello_shots() {
    let source = std::fs::read_to_string("tests/assets/hello.qs").unwrap();
    let result = run_qs_shots(&source, 100).unwrap();

    assert_eq!(result.len(), 100);
    for res in &result {
        assert_eq!(res.messages.len(), 1);
        assert_eq!(res.messages[0], "Hello");
    
        assert_eq!(res.qubit_count, 0);
        assert_eq!(res.states.len(), 0);
    
        assert!(res.result.is_some());
        assert_eq!(res.result, Some("()".into()));
    }
}

#[test]
fn test_entanglement() {
    let source = std::fs::read_to_string("tests/assets/entanglement.qs").unwrap();
    let result = run_qs(&source).unwrap();

    assert_eq!(result.messages.len(), 0);
    assert_eq!(result.qubit_count, 2);
    assert_eq!(result.states.len(), 2);

    assert!(result.result.is_some());

    let result_str = result.result.unwrap();
    assert!(result_str == "(One, One)" || result_str == "(Zero, Zero)", "Unexpected result: {}", result_str);
}

#[test]
fn test_entanglement_shots() {
    let source = std::fs::read_to_string("tests/assets/entanglement.qs").unwrap();
    let result = run_qs_shots(&source, 100).unwrap();

    assert_eq!(result.len(), 100);

    for res in &result {
        assert_eq!(res.messages.len(), 0);
        assert_eq!(res.qubit_count, 2);
        assert_eq!(res.states.len(), 2);

        assert!(res.result.is_some());

        assert!(res.result == Some("(One, One)".into()) || res.result == Some("(Zero, Zero)".into()));
    }
}

#[test]
fn test_teleportation() {
    let source = std::fs::read_to_string("tests/assets/teleportation.qs").unwrap();
    let result = run_qs(&source).unwrap();

    assert_eq!(result.messages.len(), 1);
    assert_eq!(result.messages[0], "Teleported: true");

    assert_eq!(result.qubit_count, 0);
    assert_eq!(result.states.len(), 0);

    assert_eq!(result.result, Some("true".into()));
}

#[test]
fn test_teleportation_shots() {
    let source = std::fs::read_to_string("tests/assets/teleportation.qs").unwrap();
    let result = run_qs_shots(&source, 100).unwrap();

    assert_eq!(result.len(), 100);

    for inner_result in &result {

        assert_eq!(inner_result.messages.len(), 1);
        assert_eq!(inner_result.messages[0], "Teleported: true");

        assert_eq!(inner_result.qubit_count, 0);
        assert_eq!(inner_result.states.len(), 0);

        assert_eq!(inner_result.result, Some("true".into()));
    }
}