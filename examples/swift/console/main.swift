import qsharp_bridgeFFI

let qsharpSource = """
namespace MyQuantumApp {
    @EntryPoint()
    operation Main() : Unit {
        use q = Qubit();
        H(q);
        let result = MResetZ(q);
        Message($"{result}");
    }
}
"""

print("Shots: 10")

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