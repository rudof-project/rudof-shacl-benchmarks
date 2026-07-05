package es.weso.rudof

import fr.inria.corese.core.Graph
import fr.inria.corese.core.load.Load
import fr.inria.corese.core.shacl.Shacl
import fr.inria.corese.core.transform.Transformer
import kotlin.time.measureTimedValue
import java.io.File

// Usage: java -jar corese.jar <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> <report_path> [runs] [warm_up]
//
// - data_path: Path to an RDF file containing the data graph
// - data_format: RDF format of the <data_path>
// - shapes_path: Path to a SHACL shapes file
// - shapes_format: RDF format of the <shapes_path>
// - csv_path: Path to save the CSV report file
// - report_path: Path to save the SHACL validation report (Turtle)
// - runs: Number of benchmark runs (Result runs = runs - warm_up)
// - warm_up: Number of runs for warm up
fun main(args: Array<String>) {
    val dataPath = args.getOrNull(0) ?: throw Exception("Missing data graph path")
    val dataFormat = args.getOrNull(1) ?: throw Exception("Missing data format")
    val shapesPath = args.getOrNull(2) ?: throw Exception("Missing shapes graph path")
    val shapesFormat = args.getOrNull(3) ?: throw Exception("Missing shapes format")
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val reportPath = args.getOrNull(5) ?: throw Exception("Missing validation report path")
    val runs = args.getOrNull(6)?.toInt() ?: 20
    val warmUp = args.getOrNull(7)?.toInt() ?: 10
    val results = mutableListOf<String>()
    var lastReport: Graph? = null

    println("[corese] Data:    $dataPath ($dataFormat)")
    println("[corese] Shapes:  $shapesPath ($shapesFormat)")
    println("[corese] CSV:     $csvPath")
    println("[corese] Report:  $reportPath")
    println("[corese] Runs:    $runs, warm-up: $warmUp")

    repeat(warmUp + runs) { idx ->
        val shacl = generateShacl(dataPath, shapesPath, idx)

        System.gc()
        val result = measureTimedValue { shacl.eval() }
        lastReport = result.value

        if (idx >= warmUp) {
            results.add("${result.duration.inWholeMicroseconds / 1000.0}")
        }
        if (idx == warmUp - 1) {
            println("[corese] Warm-up complete")
        }
    }

    File(csvPath).bufferedWriter().use { writer ->
        results.forEach {
            writer.apply {
                write(it)
                newLine()
            }
        }
    }

    val turtle = Transformer.create(lastReport!!, Transformer.TURTLE).transform()
    File(reportPath).writeText(turtle)
    println("[corese] Done -> $csvPath, $reportPath")
}

fun generateShacl(dataPath: String, shapesPath: String, idx: Int): Shacl {
    val dataGraph = Graph.create()
    val shapeGraph = Graph.create()

    Load.create(dataGraph).parse(dataPath)
    Load.create(shapeGraph).parse(shapesPath)

    if (idx == 0) {
        println("[corese] Data graph size: ${dataGraph.size()}")
        println("[corese] Shapes graph size: ${shapeGraph.size()}")
    }

    return Shacl(dataGraph, shapeGraph)
}
