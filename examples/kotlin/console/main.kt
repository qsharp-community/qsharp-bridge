import qsharp.bridge.runQsWithOptions
import qsharp.bridge.ExecutionState
import qsharp.bridge.ExecutionOptions

fun main(args: Array<String>) {
    System.setProperty("jna.library.path", "/Users/filipw/dev/qsharp-bridge/examples/kotlin/console/deps")

    val qsharpSource = """
    namespace MyQuantumApp {
            @EntryPoint()
            operation Main() : Unit {
                use q = Qubit();
                H(q);
                let result = MResetZ(q);
                Message(${'$'}"{result}");
            }
    }
    """
    println("Shots: 10")

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