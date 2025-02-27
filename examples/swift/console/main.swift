//import qsharp_bridgeFFI

let qsharpSource = """
@EntryPoint()
operation Run() : (Result, Result) {
    use (control, target) = (Qubit(), Qubit());
    PrepareBellState(control, target);
    
    let resultControl = MResetZ(control);
    let resultTarget = MResetZ(target);
    return (resultControl, resultTarget);
}

operation PrepareBellState(q1 : Qubit, q2: Qubit) : Unit {
    H(q1);
    CNOT(q1, q2);
}
"""

let qasmGenerationOptions = QasmGenerationOptions(includeQelib: false, resetBehavior: .supported);
let qasm = try! qasm2(source: qsharpSource, generationOptions: qasmGenerationOptions);
print("Generated QASM:");
print(qasm);


let options = ExecutionOptions.fromShots(shots: 10)
let resultShots = try! runQsWithOptions(source: qsharpSource, options: options)

for i in 0..<10 {
    print()
    print("Shot \(i+1) of 10")
    printOutcome(resultShots[i])
    print()
}

func printOutcome(_ result: ExecutionState) {
    print("Messages:")
    for msg in result.messages {
        print("  \(msg)")
    }

    print("Output:")
    if let resultValue = result.result {
        print("  \(resultValue)")
    } else {
        print("  No result available")
    }
}