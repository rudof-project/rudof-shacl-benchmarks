package es.weso.rudof

import fr.inria.corese.core.Graph
import fr.inria.corese.core.load.Load
import fr.inria.corese.core.shacl.Shacl
import kotlin.time.measureTimedValue
import java.io.File

fun main(args: Array<String>) {
    val dataPath = args.getOrNull(0) ?: throw Exception("Missing data graph path")
    args.getOrNull(1) ?: throw Exception("Missing data format")
    val shapesPath = args.getOrNull(2) ?: throw Exception("Missing shapes graph path")
    args.getOrNull(3) ?: throw Exception("Missing shapes format")
    val csvPath = args.getOrNull(4) ?: throw Exception("Missing csv report path")
    val runs = args.getOrNull(5)?.toInt() ?: 20
    val warmUp = args.getOrNull(6)?.toInt() ?: 10
    val results = mutableListOf<String>()

    repeat(warmUp + runs) { idx ->
        val shacl = generateShacl(dataPath, shapesPath)
        val result = measureTimedValue { shacl.eval() }

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

fun generateShacl(dataPath: String, shapesPath: String): Shacl {
    val dataGraph = Graph.create()
    val shapeGraph = Graph.create()

    Load.create(dataGraph).parse(dataPath)
    Load.create(shapeGraph).parse(shapesPath)

    return Shacl(dataGraph, shapeGraph)
}
