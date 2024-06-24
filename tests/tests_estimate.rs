use qsharp_bridge::sim::{estimate, estimate_expression};
use serde_json::{Value, from_str};

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