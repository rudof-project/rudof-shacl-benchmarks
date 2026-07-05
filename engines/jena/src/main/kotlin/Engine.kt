package es.weso.rudof

import org.apache.jena.geosparql.configuration.GeoSPARQLConfig
import org.apache.jena.graph.Graph
import org.apache.jena.riot.Lang
import org.apache.jena.riot.RDFDataMgr
import org.apache.jena.shacl.Shapes
import org.apache.jena.shacl.ValidationReport
import org.apache.jena.shacl.validation.ShaclPlainValidator
import java.io.StringWriter

class Engine : ValidationEngine<ValidationReport> {
    override val name = "jena"

    private lateinit var shapes: Shapes
    private lateinit var dataGraph: Graph
    private var sizeLogged = false

    init {
        GeoSPARQLConfig.setupMemoryIndex()
    }

    override fun loadData(dataPath: String, dataFormat: String, shapesPath: String, shapesFormat: String) {
        dataGraph = RDFDataMgr.loadGraph("file:$dataPath")
        val shapesGraph = RDFDataMgr.loadGraph("file:$shapesPath")

        if (!sizeLogged) {
            println("[$name] Data graph size:   ${dataGraph.size()}")
            println("[$name] Shapes graph size: ${shapesGraph.size()}")
            sizeLogged = true
        }

        shapes = Shapes.parse(shapesGraph)
    }

    override fun validate(): ValidationReport =
        ShaclPlainValidator().validate(shapes, dataGraph)

    override fun generateReport(result: ValidationReport): String =
        StringWriter().use { os ->
            RDFDataMgr.write(os, result.model, Lang.TURTLE)
            os.toString()
        }
}
