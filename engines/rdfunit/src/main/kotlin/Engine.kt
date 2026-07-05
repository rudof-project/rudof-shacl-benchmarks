package es.weso.rudof

import org.aksw.rdfunit.enums.TestCaseExecutionType
import org.aksw.rdfunit.model.interfaces.results.TestExecution
import org.aksw.rdfunit.model.writers.results.TestExecutionWriter
import org.aksw.rdfunit.validate.wrappers.RDFUnitStaticValidator
import org.aksw.rdfunit.validate.wrappers.RDFUnitTestSuiteGenerator
import org.apache.jena.rdf.model.Model
import org.apache.jena.rdf.model.ModelFactory
import org.apache.jena.riot.Lang
import org.apache.jena.riot.RDFDataMgr
import java.io.StringWriter

class Engine : ValidationEngine<TestExecution> {
    override val name = "rdfunit"

    private lateinit var dataModel: Model
    private var sizeLogged = false

    override fun loadData(dataPath: String, dataFormat: String, shapesPath: String, shapesFormat: String) {
        dataModel = RDFDataMgr.loadModel(dataPath, dataFormat.asFormat())
        RDFUnitStaticValidator.initWrapper(
            RDFUnitTestSuiteGenerator.Builder()
                .addSchemaURI("local-shacl", shapesPath)
                .build()
        )

        if (!sizeLogged) {
            println("[$name] Data graph size:   ${dataModel.size()}")
            println("[$name] Shapes graph size: TODO")
            sizeLogged = true
        }
    }

    override fun validate(): TestExecution =
        RDFUnitStaticValidator.validate(dataModel, TestCaseExecutionType.shaclTestCaseResult)

    override fun generateReport(result: TestExecution): String =
        StringWriter().use { os ->
            val model = ModelFactory.createDefaultModel()
            TestExecutionWriter.create(result).write(model)
            RDFDataMgr.write(os, model, Lang.TURTLE)
            os.toString()
        }
}
