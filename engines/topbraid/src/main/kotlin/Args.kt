package es.weso.rudof

import org.apache.jena.util.FileUtils

class Args(
    val dataPath: String,
    val dataFormat: String,

    val shapesPath: String,
    val shapesFormat: String,

    val statsPath: String,
    val reportPath: String,

    runs: String,
    warmUp: String,
    timeout: String, // Seconds
    minValidIter: String, // Minimun of valid runs (inclusive)
) {

    val runs = runs.toInt()
    val warmUp = warmUp.toInt()
    val timeout = timeout.toInt()
    val minValidIter = minValidIter.toInt()

    fun print(name: String) {
        println("[$name] Data:               $dataPath ($dataFormat)")
        println("[$name] Shapes:             $shapesPath ($shapesFormat)")
        println("[$name] Stats:              $statsPath")
        println("[$name] Report:             $reportPath")
        println("[$name] Runs:               $runs")
        println("[$name] Warm-up:            $warmUp")
        println("[$name] Timeout:            $timeout s")
        println("[$name] Minimum valid runs: $minValidIter (inclusive)")
    }
}

fun parseArgs(args: Array<String>): Args {
    return Args(
        args.getOrNull(0) ?: throw Exception("Missing data graph path"),
        args.getOrNull(1) ?: throw Exception("Missing data format"),
        args.getOrNull(2) ?: throw Exception("Missing shapes graph path"),
        args.getOrNull(3) ?: throw Exception("Missing shapes format"),
        args.getOrNull(4) ?: throw Exception("Missing stats report path"),
        args.getOrNull(5) ?: throw Exception("Missing validation report path"),
        args.getOrNull(6) ?: "20",
        args.getOrNull(7) ?: "10",
        args.getOrNull(8) ?: "300",
        args.getOrNull(9) ?: "8",
    )
}

fun String.asFormat(): String =
    when (this) {
        "turtle" -> FileUtils.langTurtle
        "rdfxml" -> FileUtils.langXML
        "n3" -> FileUtils.langN3
        "ntriples" -> FileUtils.langNTriple
        else -> throw Exception("Not supported format")
    }
