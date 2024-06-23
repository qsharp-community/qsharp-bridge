use qsharp_bridge::sim::{estimate, estimate_expression, qir, run_qs, run_qs_shots};
use serde_json::{Value, from_str};

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

#[test]
fn test_qir() {
    let result = qir("{ operation Foo() : Result { use q = Qubit(); let r = M(q); Reset(q); r }; Foo() }").unwrap();

    println!("{}", result.clone());

    let expected = r####"%Result = type opaque
%Qubit = type opaque

define void @ENTRYPOINT__main() #0 {
block_0:
  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 1 to %Qubit*))
  call void @__quantum__qis__cz__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Qubit* inttoptr (i64 0 to %Qubit*))
  call void @__quantum__qis__h__body(%Qubit* inttoptr (i64 1 to %Qubit*))
  call void @__quantum__qis__m__body(%Qubit* inttoptr (i64 1 to %Qubit*), %Result* inttoptr (i64 0 to %Result*))
  call void @__quantum__rt__result_record_output(%Result* inttoptr (i64 0 to %Result*), i8* null)
  ret void
}

declare void @__quantum__qis__h__body(%Qubit*)

declare void @__quantum__qis__cz__body(%Qubit*, %Qubit*)

declare void @__quantum__rt__result_record_output(%Result*, i8*)

declare void @__quantum__qis__m__body(%Qubit*, %Result*) #1

attributes #0 = { "entry_point" "output_labeling_schema" "qir_profiles"="base_profile" "required_num_qubits"="2" "required_num_results"="1" }
attributes #1 = { "irreversible" }

; module flags

!llvm.module.flags = !{!0, !1, !2, !3}

!0 = !{i32 1, !"qir_major_version", i32 1}
!1 = !{i32 7, !"qir_minor_version", i32 0}
!2 = !{i32 1, !"dynamic_qubit_management", i1 false}
!3 = !{i32 1, !"dynamic_result_management", i1 false}
"####;

    compare_strings(&result.clone(), expected);
    assert_eq!(result, expected);
}

#[test]
fn test_estimation() {
    let source = std::fs::read_to_string("tests/assets/entanglement.qs").unwrap();
    let result = estimate(&source, None).unwrap();
    
    //println!("{}", result.clone());

    let v: Value = from_str(result.as_str()).unwrap();
    assert_eq!(v.as_array().map_or(0, |x| x.len()), 1);
    assert_eq!(v[0]["status"].as_str(), Some("success"));
    assert_eq!(v[0]["jobParams"].is_object(), true);
    assert_eq!(v[0]["physicalCounts"]["physicalQubits"].as_i64().map_or(false, |n| n > 0), true);
    assert_eq!(v[0]["logicalCounts"]["numQubits"].as_i64().map_or(false, |n| n > 0), true);

}

#[test]
fn test_estimation_expression() {
    let result = estimate_expression("{ operation Foo() : Result { use q = Qubit(); let r = M(q); Reset(q); r }; Foo() }", None).unwrap();

    //println!("{}", result.clone());

    let v: Value = from_str(result.as_str()).unwrap();
    assert_eq!(v.as_array().map_or(0, |x| x.len()), 1);
    assert_eq!(v[0]["status"].as_str(), Some("success"));
    assert_eq!(v[0]["jobParams"].is_object(), true);
    assert_eq!(v[0]["physicalCounts"]["physicalQubits"].as_i64().map_or(false, |n| n > 0), true);
    assert_eq!(v[0]["logicalCounts"]["numQubits"].as_i64().map_or(false, |n| n > 0), true);
}

fn compare_strings(left: &str, right: &str) {
    let left_bytes = left.as_bytes();
    let right_bytes = right.as_bytes();

    for (i, (lb, rb)) in left_bytes.iter().zip(right_bytes.iter()).enumerate() {
        if lb != rb {
            println!("Difference at byte {}: left '{}', right '{}'", i, lb, rb);
            break;
        }
    }

    if left_bytes.len() != right_bytes.len() {
        println!(
            "Strings differ in length: left is {}, right is {}",
            left_bytes.len(),
            right_bytes.len()
        );
    }
}