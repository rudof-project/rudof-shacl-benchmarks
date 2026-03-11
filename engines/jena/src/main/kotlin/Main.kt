package es.weso.rudof

import org.apache.jena.geosparql.configuration.GeoSPARQLConfig
import org.apache.jena.riot.RDFDataMgr
import org.apache.jena.shacl.Shapes
import org.apache.jena.shacl.validation.ShaclPlainValidator
import java.io.File
import kotlin.time.measureTimedValue

fun main(args: Array<String>) {
    val dataPath = args.getOrNull(0) ?: throw Exception("Missing data graph path")
    val shapesPath = args.getOrNull(1) ?: throw Exception("Missing shapes graph path")
    val csvPath = args.getOrNull(2) ?: throw Exception("Missing csv report path")
    val runs = args.getOrNull(3)?.toInt() ?: 20
    val warmUp = args.getOrNull(4)?.toInt() ?: 10
    val results = mutableListOf<String>()

    val dataGraph = RDFDataMgr.loadGraph("file:$dataPath")
    val shapesGraph = RDFDataMgr.loadGraph("file:$shapesPath")

    GeoSPARQLConfig.setupMemoryIndex()
    val shapes = Shapes.parse(shapesGraph)

    repeat(warmUp + runs) { idx ->
        val validator = ShaclPlainValidator()
        val result = measureTimedValue { validator.validate(shapes, dataGraph) }

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