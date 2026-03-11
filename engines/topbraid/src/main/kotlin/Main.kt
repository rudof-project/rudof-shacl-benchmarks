package es.weso.rudof

import org.apache.jena.util.FileUtils
import org.topbraid.jenax.util.JenaUtil
import org.topbraid.shacl.validation.ValidationUtil
import java.io.File
import kotlin.time.measureTimedValue

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