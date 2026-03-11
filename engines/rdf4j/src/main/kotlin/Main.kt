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


fun main(args: Array<String>) {
    val dataPath = args.getOrNull(0) ?: throw Exception("Missing data graph path")
    val dataFormat = when (args.getOrNull(1)?.lowercase() ?: throw Exception("Missing data format")) {
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
    val shapesFormat = when (args.getOrNull(3)?.lowercase() ?: throw Exception("Missing shapes format")) {
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
}