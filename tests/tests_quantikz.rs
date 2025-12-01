use expect_test::expect;
use qsharp_bridge::quantikz::{quantikz, quantikz_operation};

#[test]
fn quantikz_one_gate() {
    let tex = quantikz(
        r"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use q = Qubit();
                    H(q);
                    M(q);
                }
            }
        "
    ).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{H} & \meter{} & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_operation_one_gate() {
    let source = r"
        namespace Test {
            operation Main() : Unit {
                use q = Qubit();
                H(q);
                M(q);
            }
        }
    ";
    let tex = quantikz_operation("Test.Main", source).expect("quantikz generation should succeed");
    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{H} & \meter{} & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_toffoli() {
    let tex = quantikz(
        r"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use q = Qubit[3];
                    CCNOT(q[0], q[1], q[2]);
                }
            }
        "
    ).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \ctrl{2} & \qw \\
        \lstick{$\ket{0}_{1}$} & \ctrl{1} & \qw \\
        \lstick{$\ket{0}_{2}$} & \targ{} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_operation_toffoli() {
    let source =
        r"
            namespace Test {
                operation Main() : Unit {
                    use q = Qubit[3];
                    CCNOT(q[0], q[1], q[2]);
                }
            }
        ";

    let tex = quantikz_operation("Test.Main", source).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \ctrl{2} & \qw \\
        \lstick{$\ket{0}_{1}$} & \ctrl{1} & \qw \\
        \lstick{$\ket{0}_{2}$} & \targ{} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_swap_gate() {
    let tex = quantikz(
        r"
            namespace Test {
                @EntryPoint()
                operation Main() : Unit {
                    use q = Qubit[2];
                    SWAP(q[0], q[1]);
                }
            }
        "
    ).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \swap{1} & \qw \\
        \lstick{$\ket{0}_{1}$} & \targX{} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_operation_swap_gate() {
    let source =
        r"
            namespace Test {
                operation Main() : Unit {
                    use q = Qubit[2];
                    SWAP(q[0], q[1]);
                }
            }
        ";

    let tex = quantikz_operation("Test.Main", source).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \swap{1} & \qw \\
        \lstick{$\ket{0}_{1}$} & \targX{} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_complex_sample() {
    let tex = quantikz(
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
        "#
    ).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{R_z(0.3927)} & \qw & \qw & \qw & \qw & \qw & \ctrl{2} & \qw & \ctrl{2} & \gate{T} & \qw & \ctrl{1} & \qw & \ctrl{1} & \gate{H} & \swap{2} & \meter{} & \cw \\
        \lstick{$\ket{0}_{1}$} & \gate{T} & \qw & \ctrl{1} & \qw & \ctrl{1} & \qw & \qw & \qw & \qw & \gate{H} & \gate{T} & \targ{} & \gate{T^\dagger} & \targ{} & \qw & \qw & \meter{} & \cw \\
        \lstick{$\ket{0}_{2}$} & \gate{H} & \gate{T} & \targ{} & \gate{T^\dagger} & \targ{} & \gate{R_z(0.3927)} & \targ{} & \gate{R_z(-0.3927)} & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \targX{} & \meter{} & \cw \\
        \end{quantikz}
    "#]].assert_eq(&tex);
}

#[test]
fn quantikz_operation_complex_sample() {
    let source = r#"
        namespace Test {
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
    "#;

    let tex = quantikz_operation("Test.Main", source).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{R_z(0.3927)} & \qw & \qw & \qw & \qw & \qw & \ctrl{2} & \qw & \ctrl{2} & \gate{T} & \qw & \ctrl{1} & \qw & \ctrl{1} & \gate{H} & \swap{2} & \meter{} & \cw \\
        \lstick{$\ket{0}_{1}$} & \gate{T} & \qw & \ctrl{1} & \qw & \ctrl{1} & \qw & \qw & \qw & \qw & \gate{H} & \gate{T} & \targ{} & \gate{T^\dagger} & \targ{} & \qw & \qw & \meter{} & \cw \\
        \lstick{$\ket{0}_{2}$} & \gate{H} & \gate{T} & \targ{} & \gate{T^\dagger} & \targ{} & \gate{R_z(0.3927)} & \targ{} & \gate{R_z(-0.3927)} & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \targX{} & \meter{} & \cw \\
        \end{quantikz}
    "#]].assert_eq(&tex);
}

#[test]
fn quantikz_rotation_circuit() {
    let tex = quantikz(
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
        "#
    ).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{X} & \gate{H} & \ctrl{1} & \gate{H} & \meter{} & \cw & \cw \\
        \lstick{$\ket{0}_{1}$} & \gate{X} & \qw & \targ{} & \gate{R_z(2.0944)} & \gate{H} & \meter{} & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_operation_rotation_circuit() {
    let source = r#"
        namespace Test {
            open Microsoft.Quantum.Math;
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
    "#;

    let tex = quantikz_operation("Test.Main", source).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{X} & \gate{H} & \ctrl{1} & \gate{H} & \meter{} & \cw & \cw \\
        \lstick{$\ket{0}_{1}$} & \gate{X} & \qw & \targ{} & \gate{R_z(2.0944)} & \gate{H} & \meter{} & \cw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_cat_state() {
    let tex = quantikz(
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
        "
    ).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{H} & \ctrl{1} & \ctrl{2} & \ctrl{3} & \ctrl{4} & \ctrl{5} & \ctrl{6} & \ctrl{7} & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{1}$} & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{2}$} & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{3}$} & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{4}$} & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{5}$} & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{6}$} & \qw & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{7}$} & \qw & \qw & \qw & \qw & \qw & \qw & \qw & \targ{} & \meter{} & \gate{\ket{0}} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}

#[test]
fn quantikz_operation_cat_state() {
    let source = r"
        namespace Test {
            import Std.Measurement.*;
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
    ";

    let tex = quantikz_operation("Test.Main", source).expect("quantikz generation should succeed");

    expect![[r#"
        \begin{quantikz}
        \lstick{$\ket{0}_{0}$} & \gate{H} & \ctrl{1} & \ctrl{2} & \ctrl{3} & \ctrl{4} & \ctrl{5} & \ctrl{6} & \ctrl{7} & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{1}$} & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{2}$} & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{3}$} & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{4}$} & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{5}$} & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{6}$} & \qw & \qw & \qw & \qw & \qw & \qw & \targ{} & \qw & \meter{} & \gate{\ket{0}} & \qw \\
        \lstick{$\ket{0}_{7}$} & \qw & \qw & \qw & \qw & \qw & \qw & \qw & \targ{} & \meter{} & \gate{\ket{0}} & \qw \\
        \end{quantikz}
    "#]]
    .assert_eq(&tex);
}