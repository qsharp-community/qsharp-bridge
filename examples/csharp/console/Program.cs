using uniffi.qsharp_bridge;

var qsharpSource = """
namespace MyQuantumApp {
        @EntryPoint()
        operation Main() : Unit {
            Message("Hello");
        }
}
""";

Console.WriteLine("Shots: 10");
var resultShots = QsharpBridgeMethods.RunQsShots(qsharpSource, 10);
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