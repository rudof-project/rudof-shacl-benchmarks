package es.weso.rudof

import org.apache.jena.rdf.model.Model
import org.apache.jena.rdf.model.Resource
import org.apache.jena.util.FileUtils
import org.topbraid.jenax.util.JenaUtil
import org.topbraid.shacl.validation.ValidationUtil
import java.io.StringWriter

class Engine : ValidationEngine<Resource> {
    override val name = "topbraid"

    private lateinit var dataModel: Model
    private lateinit var shapesModel: Model
    private var sizeLogged = false

    override fun loadData(dataPath: String, dataFormat: String, shapesPath: String, shapesFormat: String) {
        dataModel = JenaUtil.createMemoryModel().apply { read(dataPath, dataFormat.asFormat()) }
        shapesModel = JenaUtil.createMemoryModel().apply { read(shapesPath, shapesFormat.asFormat()) }

        if (!sizeLogged) {
            println("[$name] Data graph size:   ${dataModel.size()}")
            println("[$name] Shapes graph size: ${shapesModel.size()}")
            sizeLogged = true
        }
    }

    override fun validate(): Resource = ValidationUtil.validateModel(dataModel, shapesModel, false)

    override fun generateReport(result: Resource): String =
        StringWriter().use { os ->
            result.model.write(os, FileUtils.langTurtle)
            os.toString()
        }
}
