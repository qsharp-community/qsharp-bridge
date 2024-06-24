use qsharp_bridge::sim::circuit;

#[test]
fn test_empty() {
    let source = std::fs::read_to_string("tests/assets/hello.qs").unwrap();
    let circuit = circuit(&source).unwrap();
    assert_eq!(0, circuit.qubits.len());
}

#[test]
fn test_entanglement() {
    let source = std::fs::read_to_string("tests/assets/entanglement.qs").unwrap();
    let circuit = circuit(&source).unwrap();
    assert_eq!(2, circuit.qubits.len());
    assert_eq!(6, circuit.operations.len());
    assert_eq!("H", circuit.operations[0].gate);
    assert_eq!("X", circuit.operations[1].gate);
    assert_eq!("Measure", circuit.operations[2].gate);
    assert_eq!("|0〉", circuit.operations[3].gate);
    assert_eq!("Measure", circuit.operations[4].gate);
    assert_eq!("|0〉", circuit.operations[5].gate);
}