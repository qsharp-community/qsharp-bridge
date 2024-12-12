using uniffi.qsharp_bridge;

var qsharpSource = """@EntryPoint()
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
""";

var qasmGenerationOptions = new QasmGenerationOptions(includeQelib: false, resetBehavior: QasmResetBehavior.Supported);
var qasm = QsharpBridge.Qasm2(qsharpSource, qasmGenerationOptions);
Console.WriteLine("Generated QASM:");
Console.WriteLine(qasm);

var executionOptions = ExecutionOptions.FromShots(10);
var resultShots = QsharpBridge.RunQsWithOptions(qsharpSource, executionOptions);
for (var i=0; i<10; i++) {
    Console.WriteLine();
    Console.WriteLine($"Shot {i+1} of 10");
    PrintOutcome(resultShots[i]);
    Console.WriteLine();
}

void PrintOutcome(ExecutionState result)
{
    Console.WriteLine("Messages:");
    foreach (var msg in result.messages) {
        Console.WriteLine($"  {msg}");
    }

    Console.WriteLine("Output:");
    Console.WriteLine($"  {result.result}");
}