package es.weso.rudof

import org.aksw.rdfunit.enums.TestCaseExecutionType
import org.aksw.rdfunit.validate.wrappers.RDFUnitStaticValidator
import org.aksw.rdfunit.validate.wrappers.RDFUnitTestSuiteGenerator
import org.aksw.rdfunit.model.interfaces.results.TestExecution
import org.aksw.rdfunit.model.writers.results.TestExecutionWriter
import org.apache.jena.rdf.model.ModelFactory
import org.apache.jena.riot.Lang
import org.apache.jena.riot.RDFDataMgr
import java.io.File
import java.io.FileOutputStream
import kotlin.time.measureTimedValue

// Usage: java -jar rdfunit.jar <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> <report_path> [runs] [warm_up]
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
    val dataFormatStr = args.getOrNull(1)?.lowercase() ?: throw Exception("Missing data format")
    val dataFormat = when (dataFormatStr) {
        "turtle" -> Lang.TURTLE
        "rdfxml" -> Lang.RDFXML
        "n3" -> Lang.N3
        "ntriples" -> Lang.NTRIPLES
        "jsonld" -> Lang.JSONLD
        "trig" -> Lang.TRIG
        "nquads" -> Lang.NQUADS
        else -> throw Exception("Format not supported")
    }
    val shapesPath = args.getOrNull(2) ?: throw Exception("Missing shapes graph path")
    val shapesFormat = args.getOrNull(3) ?: throw Exception("Missing shapes format")
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val reportPath = args.getOrNull(5) ?: throw Exception("Missing validation report path")
    val runs = args.getOrNull(6)?.toInt() ?: 20
    val warmUp = args.getOrNull(7)?.toInt() ?: 10
    val results = mutableListOf<String>()

    println("[rdfunit] Data:    $dataPath ($dataFormatStr)")
    println("[rdfunit] Shapes:  $shapesPath ($shapesFormat)")
    println("[rdfunit] CSV:     $csvPath")
    println("[rdfunit] Report:  $reportPath")
    println("[rdfunit] Runs:    $runs, warm-up: $warmUp")

    repeat(warmUp + runs) { idx ->
        val dataModel = RDFDataMgr.loadModel(dataPath, dataFormat)
        RDFUnitStaticValidator.initWrapper(
            RDFUnitTestSuiteGenerator.Builder()
                .addSchemaURI("local-shacl", shapesPath)
                .build()
        )

        if (idx == 0) {
            println("[rdfunit] Data graph size: ${dataModel.size()}")
            println("[rdfunit] Shapes graph size: TODO")
        }

        System.gc()
        val result = measureTimedValue {
            RDFUnitStaticValidator.validate(dataModel, TestCaseExecutionType.shaclTestCaseResult)
        }

        if (idx >= warmUp) {
            results.add("${result.duration.inWholeMicroseconds / 1000.0}")

            FileOutputStream(reportPath).use { os ->
                val model = ModelFactory.createDefaultModel()
                TestExecutionWriter.create(result.value!!).write(model)
                RDFDataMgr.write(os, model, Lang.TURTLE)
            }
        }
        if (idx == warmUp - 1) {
            println("[rdfunit] Warm-up complete")
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

    println("[rdfunit] Done -> $csvPath, $reportPath")
}
