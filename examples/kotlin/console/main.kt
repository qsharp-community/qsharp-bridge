import qsharp.bridge.runQsWithOptions
import qsharp.bridge.qasm2
import qsharp.bridge.ExecutionState
import qsharp.bridge.ExecutionOptions
import qsharp.bridge.QasmGenerationOptions
import qsharp.bridge.QasmResetBehavior

fun main(args: Array<String>) {
    // set to correct absolute path
    System.setProperty("jna.library.path", "/Users/filipw/dev/qsharp-bridge/examples/kotlin/console/deps")

    val qsharpSource = """
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

    val qasmGenerationOptions = QasmGenerationOptions(false, QasmResetBehavior.SUPPORTED);
    val qasm = qasm2(qsharpSource, qasmGenerationOptions);
    println("Generated QASM:");
    println(qasm);

    val options = ExecutionOptions.fromShots(10.toUInt())
    val resultShots = runQsWithOptions(qsharpSource, options)

    for (i in 0 until 10) {
        println()
        println("Shot ${i + 1} of 10")
        printOutcome(resultShots[i])
        println()
    }
}

fun printOutcome(result: ExecutionState) {
    println("Messages:")
    for (msg in result.messages) {
        println("  $msg")
    }

    println("Output:")
    if (result.result != null) {
        println("  ${result.result}")
    } else {
        println("  No result available")
    }
}
