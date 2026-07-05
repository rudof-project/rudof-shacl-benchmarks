package es.weso.rudof

import fr.inria.corese.core.Graph
import fr.inria.corese.core.load.Load
import fr.inria.corese.core.shacl.Shacl
import fr.inria.corese.core.transform.Transformer

class Engine : ValidationEngine<Graph> {
    override val name = "corese"

    private lateinit var shacl: Shacl
    private var sizeLogged = false

    override fun loadData(dataPath: String, dataFormat: String, shapesPath: String, shapesFormat: String) {
        val dataGraph = Graph.create()
        val shapesGraph = Graph.create()

        Load.create(dataGraph).parse(dataPath)
        Load.create(shapesGraph).parse(shapesPath)

        if (!sizeLogged) {
            println("[$name] Data graph size:   ${dataGraph.size()}")
            println("[$name] Shapes graph size: ${shapesGraph.size()}")
            sizeLogged = true
        }

        shacl = Shacl(dataGraph, shapesGraph)
    }

    override fun validate(): Graph = shacl.eval()

    override fun generateReport(result: Graph): String =
        Transformer.create(result, Transformer.TURTLE).transform()
}
