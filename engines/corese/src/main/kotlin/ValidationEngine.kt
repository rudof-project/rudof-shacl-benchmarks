package es.weso.rudof

interface ValidationEngine<R> {
    val name: String

    fun loadData(dataPath: String, dataFormat: String, shapesPath: String, shapesFormat: String)
    fun validate(): R
    fun generateReport(result: R): String
}
