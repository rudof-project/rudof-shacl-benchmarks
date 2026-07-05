package es.weso.rudof

import org.eclipse.rdf4j.common.transaction.IsolationLevels
import org.eclipse.rdf4j.model.impl.LinkedHashModel
import org.eclipse.rdf4j.model.vocabulary.RDF4J
import org.eclipse.rdf4j.repository.sail.SailRepository
import org.eclipse.rdf4j.repository.sail.SailRepositoryConnection
import org.eclipse.rdf4j.rio.RDFFormat
import org.eclipse.rdf4j.rio.Rio
import org.eclipse.rdf4j.sail.memory.MemoryStore
import org.eclipse.rdf4j.sail.shacl.ShaclSail
import org.eclipse.rdf4j.sail.shacl.ShaclSailConnection
import org.eclipse.rdf4j.sail.shacl.results.ValidationReport
import java.io.File
import java.io.StringWriter

class Engine : ValidationEngine<ValidationReport> {
    override val name = "rdf4j"

    private lateinit var conn: SailRepositoryConnection
    private var sizeLogged = false

    override fun loadData(dataPath: String, dataFormat: String, shapesPath: String, shapesFormat: String) {
        val shaclSail = ShaclSail(MemoryStore()).apply {
            validationResultsLimitTotal = -1
            validationResultsLimitPerConstraint = -1
        }

        conn = SailRepository(shaclSail)
            .apply { init() }
            .connection

        conn.apply {
            begin(IsolationLevels.NONE)
            add(File(shapesPath), shapesFormat.asFormat(), RDF4J.SHACL_SHAPE_GRAPH)
            commit()

            begin(IsolationLevels.NONE, ShaclSail.TransactionSettings.ValidationApproach.Disabled)
            add(File(dataPath), dataFormat.asFormat())
            commit()
        }

        if (!sizeLogged) {
            println("[$name] Data graph size:   TODO")
            println("[$name] Shapes graph size: TODO")
            sizeLogged = true
        }
    }

    override fun validate(): ValidationReport {
        return conn.let {
            it.begin(IsolationLevels.NONE)
            val report = (it.sailConnection as ShaclSailConnection).revalidate()
            it.commit()
            report
        }
    }

    override fun generateReport(result: ValidationReport): String =
        StringWriter().use { os ->
            val model = LinkedHashModel()
            result.asModel(model)
            Rio.write(model, os, RDFFormat.TURTLE)
            os.toString()
        }

    fun closeConnection() {
        conn.close()
    }
}
