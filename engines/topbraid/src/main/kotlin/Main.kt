package es.weso.rudof

import org.apache.jena.util.FileUtils
import org.topbraid.jenax.util.JenaUtil
import org.topbraid.shacl.validation.ValidationUtil
import java.io.File
import kotlin.time.measureTimedValue

// Usage: java -jar topbraid.jar <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> [runs] [warm_up]
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
    val dataFormat = when (args.getOrNull(1)?.lowercase() ?: throw Exception("Missing data format")) {
        "turtle" -> FileUtils.langTurtle
        "rdfxml" -> FileUtils.langXML
        "n3" -> FileUtils.langN3
        "ntriples" -> FileUtils.langNTriple
        else -> throw Exception("Not supported format")
    }
    val shapesPath = args.getOrNull(2) ?: throw Exception("Missing shapes graph path")
    val shapesFormat = when (args.getOrNull(3)?.lowercase() ?: throw Exception("Missing shapes format")) {
        "turtle" -> FileUtils.langTurtle
        "rdfxml" -> FileUtils.langXML
        "n3" -> FileUtils.langN3
        "ntriples" -> FileUtils.langNTriple
        else -> throw Exception("Not supported format")
    }
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val runs = args.getOrNull(5)?.toInt() ?: 20
    val warmUp = args.getOrNull(6)?.toInt() ?: 10
    val results = mutableListOf<String>()

    val dataModel = JenaUtil.createMemoryModel().apply { read(dataPath, dataFormat) }
    val shapesModel = JenaUtil.createMemoryModel().apply { read(shapesPath, shapesFormat) }

    repeat(warmUp + runs) { idx ->
        val result = measureTimedValue {
            ValidationUtil.validateModel(dataModel, shapesModel, true)
        }

        if (idx >= warmUp) {
            results.add("${result.duration.inWholeMicroseconds / 1000.0}")
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
}
