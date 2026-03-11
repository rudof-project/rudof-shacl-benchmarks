package es.weso.rudof

import org.aksw.rdfunit.enums.TestCaseExecutionType
import org.aksw.rdfunit.validate.wrappers.RDFUnitStaticValidator
import org.aksw.rdfunit.validate.wrappers.RDFUnitTestSuiteGenerator
import org.apache.jena.riot.Lang
import org.apache.jena.riot.RDFDataMgr
import java.io.File
import kotlin.time.measureTimedValue

fun main(args: Array<String>) {
    val dataPath = args.getOrNull(0) ?: throw Exception("Missing data graph path")
    val dataFormat = when (args.getOrNull(1)?.lowercase() ?: throw Exception("Missing data format")) {
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
    // Missing shapes format
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val runs = args.getOrNull(5)?.toInt() ?: 20
    val warmUp = args.getOrNull(6)?.toInt() ?: 10
    val results = mutableListOf<String>()

    val dataModel = RDFDataMgr.loadModel(dataPath, dataFormat)

    RDFUnitStaticValidator.initWrapper(
        RDFUnitTestSuiteGenerator.Builder()
            .addSchemaURI("local-shacl", shapesPath)
            .build()
    )

    repeat(warmUp + runs) { idx ->
        val result = measureTimedValue {
            RDFUnitStaticValidator.validate(dataModel, TestCaseExecutionType.shaclTestCaseResult)
        }

        if (idx >= warmUp) {
            results.add("${result.duration.inWholeMilliseconds}")
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