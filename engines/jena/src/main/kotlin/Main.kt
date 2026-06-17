package es.weso.rudof

import org.apache.jena.geosparql.configuration.GeoSPARQLConfig
import org.apache.jena.riot.RDFDataMgr
import org.apache.jena.shacl.Shapes
import org.apache.jena.shacl.validation.ShaclPlainValidator
import java.io.File
import kotlin.time.measureTimedValue

// Usage: java -jar jena.jar <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> [runs] [warm_up]
//
// - data_path: Path to an RDF file containing the data graph
// - data_format: RDF format of the <data_path>
// - shapes_path: Path to a SHACL shapes file
// - shapes_format: RDF format of the <shapes_path>
// - csv_path: Path to save the CSV report file
// - runs: Number of benchmark runs (Result runs = runs - warm_up)
// - warm_up: Number of runs for warm up
fun main(args: Array<String>) {
    val dataPath = args.getOrNull(0) ?: throw Exception("Missing data graph path")
    val dataFormat = args.getOrNull(1) ?: throw Exception("Missing data format")
    val shapesPath = args.getOrNull(2) ?: throw Exception("Missing shapes graph path")
    val shapesFormat = args.getOrNull(3) ?: throw Exception("Missing shapes format")
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val runs = args.getOrNull(5)?.toInt() ?: 20
    val warmUp = args.getOrNull(6)?.toInt() ?: 10
    val results = mutableListOf<String>()

    println("[jena] Data:    $dataPath ($dataFormat)")
    println("[jena] Shapes:  $shapesPath ($shapesFormat)")
    println("[jena] CSV:     $csvPath")
    println("[jena] Runs:    $runs, warm-up: $warmUp")

    val dataGraph = RDFDataMgr.loadGraph("file:$dataPath")
    val shapesGraph = RDFDataMgr.loadGraph("file:$shapesPath")

    GeoSPARQLConfig.setupMemoryIndex()
    val shapes = Shapes.parse(shapesGraph)

    repeat(warmUp + runs) { idx ->
        val validator = ShaclPlainValidator()

        System.gc()
        val result = measureTimedValue { validator.validate(shapes, dataGraph) }

        if (idx >= warmUp) {
            results.add("${result.duration.inWholeMicroseconds / 1000.0}")
        }
        if (idx == warmUp - 1) {
            println("[jena] Warm-up complete")
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

    println("[jena] Done -> $csvPath")
}
