package es.weso.rudof

import kotlinx.serialization.Serializable
import kotlin.math.sqrt

enum class IterationResultStatus {
    OK, // Iteration completed successfully
    ERROR, // Some exception was thrown
    OOM, // Out of memory error
    DNF // Run exceeded timeout
}

enum class ResultStatus {
    SUCCESS, // All runs are OK
    PARTIAL, // More than `minValidIter` runs are OK
    FAILED // There are not enough OK cases
}

@Serializable
data class IterationResults(
    val iteration: Int,
    val loadTime: Double?,
    val validationTime: Double?,
    val iterationResults: IterationResultStatus
)

class ResultState (
    private val minIter: Int,
) {
    private val results = mutableListOf<IterationResults>()

    fun addResult(result: IterationResults) {
        results.add(result)
    }

    private fun meanOf(times: List<Double>): Double? =
        times.takeIf { it.isNotEmpty() }?.average()

    private fun stdOf(times: List<Double>): Double? {
        if (times.isEmpty()) return null
        val meanValue = times.average()
        return sqrt(times.map { (it - meanValue) * (it - meanValue) }.average())
    }

    fun generateResults(): Results {
        val okResults = results.filter { it.iterationResults == IterationResultStatus.OK }

        val loadTimes = okResults.mapNotNull { it.loadTime }
        val validationTimes = okResults.mapNotNull { it.validationTime }

        val okCount = okResults.size
        val status = when {
            okCount == results.size -> ResultStatus.SUCCESS
            okCount >= minIter -> ResultStatus.PARTIAL
            else -> ResultStatus.FAILED
        }

        return Results(
            iterationResults = results,
            status = status,
            loadMean = meanOf(loadTimes),
            loadStd = stdOf(loadTimes),
            validationMean = meanOf(validationTimes),
            validationStd = stdOf(validationTimes)
        )
    }
}

@Serializable
data class Results (
    val iterationResults: List<IterationResults>,
    val status: ResultStatus,
    val loadMean: Double?,
    val loadStd: Double?,
    val validationMean: Double?,
    val validationStd: Double?,
)
