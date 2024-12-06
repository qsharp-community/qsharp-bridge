use qsharp_bridge::sim::{qasm2, qasm2_expression};

#[test]
fn test_qasm_entanglement() {
    let source = std::fs::read_to_string("tests/assets/entanglement_noreset.qs").unwrap();
    let result = qasm2(&source).unwrap();
    let expected = r####"OPENQASM 2.0;
qreg q[2];
creg c[2];
h q[0];
cx q[0], q[1];
measure q[0] -> c[0];
measure q[1] -> c[1];
"####;
    assert_eq!(result, expected);
}

#[test]
fn test_qasm_expression() {
    let result = qasm2_expression("{ operation Foo() : Result { use q = Qubit(); let r = M(q); r }; Foo() }").unwrap();
    let expected = r####"OPENQASM 2.0;
qreg q[1];
creg c[1];
measure q[0] -> c[0];
"####;
    assert_eq!(result, expected);
}