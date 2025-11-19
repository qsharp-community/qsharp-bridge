use qsc::{
    LanguageFeatures, PackageType, SourceMap, interpret::{CircuitEntryPoint, CircuitGenerationMethod, Interpreter}, target::Profile
};
use expect_test::expect;
use qsc_circuit::TracerConfig;
use qsharp_bridge::quantikz;

fn interpreter_with_circuit_trace(code: &str, profile: Profile) -> Interpreter {
    let sources = SourceMap::new([("test.qs".into(), code.into())], None);
    let (std_id, store) = qsc::compile::package_store_with_stdlib(profile.into());
    Interpreter::with_circuit_trace(
        sources,
        PackageType::Exe,
        profile.into(),
        LanguageFeatures::default(),
        store,
        &[(std_id, None)],
        Default::default(),
    )
    .expect("interpreter creation should succeed")
}

fn get_quantikz(
    interpreter: &mut Interpreter,
    entry: CircuitEntryPoint,
    method: CircuitGenerationMethod,
    config: TracerConfig,
) -> String {
    let circuit = interpreter
        .circuit(entry, method, config)
        .expect("circuit generation should succeed");

    quantikz::circuit_to_quantikz(&circuit)
}

#[test]
fn quantikz_one_gate() {
    let mut interpreter = interpreter_with_circuit_trace(
        r"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use q = Qubit();
                    H(q);
                    M(q);
                }
            }
        ",
        Profile::Unrestricted,
    );

    let tex = get_quantikz(
        &mut interpreter,
        CircuitEntryPoint::EntryPoint,
        CircuitGenerationMethod::ClassicalEval,
        TracerConfig::default(),
    );

    expect![[r#"
        \begin{quantikz}
        \lstick{\ket{0}_{0}} & \gate{H} & \qw & \qw \\
         & \cw & \meter{} & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_toffoli() {
    let mut interpreter = interpreter_with_circuit_trace(
        r"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use q = Qubit[3];
                    CCNOT(q[0], q[1], q[2]);
                }
            }
        ",
        Profile::Unrestricted,
    );

    let tex = get_quantikz(
        &mut interpreter,
        CircuitEntryPoint::EntryPoint,
        CircuitGenerationMethod::ClassicalEval,
        TracerConfig::default(),
    );

    expect![[r#"
        \begin{quantikz}
        \lstick{\ket{0}_{0}} & \ctrl{2} & \qw \\
        \lstick{\ket{0}_{1}} & \ctrl{1} & \qw \\
        \lstick{\ket{0}_{2}} & \targ{} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_swap_gate() {
    let mut interpreter = interpreter_with_circuit_trace(
        r"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use q = Qubit[2];
                    SWAP(q[0], q[1]);
                }
            }
        ",
        Profile::Unrestricted,
    );

    let tex = get_quantikz(
        &mut interpreter,
        CircuitEntryPoint::EntryPoint,
        CircuitGenerationMethod::ClassicalEval,
        TracerConfig::default(),
    );

    expect![[r#"
        \begin{quantikz}
        \lstick{\ket{0}_{0}} & \swap{2} & \qw \\
         & \cw & \cw \\
        \lstick{\ket{0}_{1}} & \targX{} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_complex_sample() {
    let mut interpreter = interpreter_with_circuit_trace(
        r#"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use (q0, q1, q2) = (Qubit(), Qubit(), Qubit());
                    H(q2);
                    Controlled S([q1], q2);
                    Controlled T([q0], q2);
                    H(q1);
                    Controlled S([q0], q1);
                    H(q0);
                    SWAP(q0, q2);
                    let r0 = M(q0);
                    let r1 = M(q1);
                    let r2 = M(q2);
                }
            }
        "#,
        Profile::Unrestricted,
    );

    let tex = get_quantikz(
        &mut interpreter,
        CircuitEntryPoint::EntryPoint,
        CircuitGenerationMethod::ClassicalEval,
        TracerConfig::default(),
    );

    expect![[r#"
        \begin{quantikz}
        \lstick{\ket{0}_{0}} & \gate{R_z(0.3927)} & \qw & \qw & \qw & \qw & \qw & \ctrl{4} & \qw & \ctrl{4} & \gate{T} & \qw & \ctrl{2} & \qw & \ctrl{2} & \gate{H} & \swap{4} & \qw & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw \\
        \lstick{\ket{0}_{1}} & \gate{T} & \qw & \ctrl{2} & \qw & \ctrl{2} & \qw & \qw & \qw & \qw & \gate{H} & \gate{T} & \targ{} & \gate{T^\dagger} & \targ{} & \qw & \qw & \qw & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw \\
        \lstick{\ket{0}_{2}} & \gate{H} & \gate{T} & \targ{} & \gate{T^\dagger} & \targ{} & \gate{R_z(0.3927)} & \targ{} & \gate{R_z(-0.3927)} & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \targX{} & \qw & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw \\
        \end{quantikz}
    "#]].assert_eq(&tex);
}

#[test]
fn quantikz_rotation_circuit() {
    let mut interpreter = interpreter_with_circuit_trace(
        r#"
            namespace Test {
                open Microsoft.Quantum.Math;
                @EntryPoint()
                operation Main() : Unit {
                    use (q0, q1) = (Qubit(), Qubit());
                    X(q0);
                    X(q1);
                    H(q0);
                    CNOT(q0, q1);
                    Rz(2.0 * PI() / 3.0, q1);
                    H(q0);
                    H(q1);
                    M(q0);
                    M(q1);
                }
            }
        "#,
        Profile::Unrestricted,
    );

    let tex = get_quantikz(
        &mut interpreter,
        CircuitEntryPoint::EntryPoint,
        CircuitGenerationMethod::ClassicalEval,
        TracerConfig::default(),
    );

    expect![[r#"
        \begin{quantikz}
        \lstick{\ket{0}_{0}} & \gate{X} & \gate{H} & \ctrl{2} & \gate{H} & \qw & \qw & \qw \\
         & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{1}} & \gate{X} & \qw & \targ{} & \gate{R_z(2.0944)} & \gate{H} & \qw & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_cat_state() {
    let mut interpreter = interpreter_with_circuit_trace(
        r"
            namespace Test {
                import Std.Measurement.*;
                @EntryPoint()
                operation Main() : Result[] {
                    use qubits = Qubit[8];

                    // apply Hadamard gate to the first qubit
                    H(qubits[0]);

                    // apply CNOT gates to create entanglement
                    for qubit in qubits[1..Length(qubits) - 1] {
                        CNOT(qubits[0], qubit);
                    }

                    // return measurement results
                    MResetEachZ(qubits)
                }
            }
        ",
        Profile::Unrestricted,
    );

    let tex = get_quantikz(
        &mut interpreter,
        CircuitEntryPoint::EntryPoint,
        CircuitGenerationMethod::ClassicalEval,
        TracerConfig::default(),
    );

    expect![[r#"
        \begin{quantikz}
        \lstick{\ket{0}_{0}} & \gate{H} & \ctrl{2} & \ctrl{4} & \ctrl{6} & \ctrl{8} & \ctrl{10} & \ctrl{12} & \ctrl{14} & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{1}} & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{2}} & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{3}} & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{4}} & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{5}} & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{6}} & \qw & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \lstick{\ket{0}_{7}} & \qw & \qw & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \gate{\ket{0}} & \qw \\
         & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \cw & \meter{} & \cw & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}