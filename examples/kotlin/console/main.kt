import qsharp.bridge.runQsShots
import qsharp.bridge.ExecutionState

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

    val resultShots = runQsShots(qsharpSource, 10.toUInt())

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