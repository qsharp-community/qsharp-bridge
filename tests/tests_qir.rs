use qsharp_bridge::sim::qir;

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