use qsharp_bridge::{qasm::{qasm2, qasm2_expression, QasmGenerationOptions, QasmResetBehavior}};

#[test]
fn test_qasm_entanglement() {
    let source = std::fs::read_to_string("tests/assets/entanglement.qs").unwrap();
    let generation_options = QasmGenerationOptions {
        include_qelib: true,
        reset_behavior: QasmResetBehavior::Supported,
    };
    let result = qasm2(&source, generation_options).unwrap();
    let expected = r####"OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
creg c[2];
h q[0];
cx q[0], q[1];
measure q[0] -> c[0];
reset q[0];
measure q[1] -> c[1];
reset q[1];
"####;
    assert_eq!(result, expected);
}

#[test]
fn test_qasm_entanglement_no_reset() {
    let source = std::fs::read_to_string("tests/assets/entanglement_noreset.qs").unwrap();
    let generation_options = QasmGenerationOptions {
        include_qelib: false,
        reset_behavior: QasmResetBehavior::Ignored,
    };
    let result = qasm2(&source, generation_options).unwrap();
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
    let generation_options = QasmGenerationOptions {
        include_qelib: false,
        reset_behavior: QasmResetBehavior::Ignored,
    };
    let result = qasm2_expression("{ operation Foo() : Result { use q = Qubit(); let r = M(q); r }; Foo() }", generation_options).unwrap();
    let expected = r####"OPENQASM 2.0;
qreg q[1];
creg c[1];
measure q[0] -> c[0];
"####;
    assert_eq!(result, expected);
}