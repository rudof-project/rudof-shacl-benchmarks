package es.weso.rudof

import org.eclipse.rdf4j.common.transaction.IsolationLevels
import org.eclipse.rdf4j.model.vocabulary.RDF4J
import org.eclipse.rdf4j.repository.sail.SailRepository
import org.eclipse.rdf4j.rio.RDFFormat
import org.eclipse.rdf4j.sail.memory.MemoryStore
import org.eclipse.rdf4j.sail.shacl.ShaclSail
import org.eclipse.rdf4j.sail.shacl.ShaclSailConnection
import java.io.File
import kotlin.time.measureTimedValue

// Usage: java -jar rdf4j.jar <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> [runs] [warm_up]
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
    val dataFormatStr = args.getOrNull(1)?.lowercase() ?: throw Exception("Missing data format")
    val dataFormat = when (dataFormatStr) {
        "turtle" -> RDFFormat.TURTLE
        "n3" -> RDFFormat.N3
        "rdfxml" -> RDFFormat.RDFXML
        "ntriples" -> RDFFormat.NTRIPLES
        "trig" -> RDFFormat.TRIG
        "nquads" -> RDFFormat.NQUADS
        "jsonld" -> RDFFormat.JSONLD
        else -> throw Exception("Format not supported")
    }
    val shapesPath = args.getOrNull(2) ?: throw Exception("Missing shapes graph path")
    val shapesFormatStr = args.getOrNull(3)?.lowercase() ?: throw Exception("Missing shapes format")
    val shapesFormat = when (shapesFormatStr) {
        "turtle" -> RDFFormat.TURTLE
        "n3" -> RDFFormat.N3
        "rdfxml" -> RDFFormat.RDFXML
        "ntriples" -> RDFFormat.NTRIPLES
        "trig" -> RDFFormat.TRIG
        "nquads" -> RDFFormat.NQUADS
        "jsonld" -> RDFFormat.JSONLD
        else -> throw Exception("Format not supported")
    }
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val runs = args.getOrNull(5)?.toInt() ?: 20
    val warmUp = args.getOrNull(6)?.toInt() ?: 10
    val results = mutableListOf<String>()

    println("[rdf4j] Data:    $dataPath ($dataFormatStr)")
    println("[rdf4j] Shapes:  $shapesPath ($shapesFormatStr)")
    println("[rdf4j] CSV:     $csvPath")
    println("[rdf4j] Runs:    $runs, warm-up: $warmUp")

    val shaclSail = ShaclSail(MemoryStore()).apply {
        validationResultsLimitTotal = -1
        validationResultsLimitPerConstraint = -1
    }

    SailRepository(shaclSail)
        .apply { init() }
        .connection
        .use { conn ->
            conn.apply {
                begin(IsolationLevels.NONE)
                add(File(shapesPath), shapesFormat, RDF4J.SHACL_SHAPE_GRAPH)
                commit()
            }

            repeat(warmUp + runs) { idx ->
                conn.apply {
                    begin(IsolationLevels.NONE, ShaclSail.TransactionSettings.ValidationApproach.Disabled)
                    clear()
                    add(File(dataPath), dataFormat)
                    commit()

                    val result = measureTimedValue {
                        begin(IsolationLevels.NONE)
                        (sailConnection as ShaclSailConnection).revalidate()
                        commit()
                    }

                    if (idx >= warmUp) {
                        results.add("${result.duration.inWholeMilliseconds}")
                        results.add("${result.duration.inWholeMicroseconds / 1000.0}")
                    }
                    if (idx == warmUp - 1) {
                        println("[rdf4j] Warm-up complete")
                    }
                }
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

    println("[rdf4j] Done -> $csvPath")
}
