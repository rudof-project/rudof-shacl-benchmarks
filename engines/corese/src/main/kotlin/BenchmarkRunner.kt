package es.weso.rudof

import java.io.File
import java.util.concurrent.Callable
import java.util.concurrent.ExecutionException
import java.util.concurrent.Executors
import java.util.concurrent.TimeUnit
import java.util.concurrent.TimeoutException
import kotlin.time.measureTimedValue

class BenchmarkRunner<R>(
    private val engine: ValidationEngine<R>,
    private val args: Args,
) {
    private data class RunOutcome(val loadTimeMs: Double, val validationTimeMs: Double, val report: String)

    fun run(): ResultState {
        val results = ResultState(args.minValidIter)

        repeat(args.warmUp + args.runs) { idx ->
            val (iterationResult, report) = runIteration(idx)

            if (idx >= args.warmUp) {
                results.addResult(iterationResult.copy(iteration = iterationResult.iteration - args.warmUp))
                report?.let { File(args.reportPath).writeText(it) }
            }
            if (idx == args.warmUp - 1) {
                println("[${engine.name}] Warm-up complete")
            }
        }

        return results
    }

    private fun runIteration(idx: Int): Pair<IterationResults, String?> {
        val executor = Executors.newSingleThreadExecutor { r ->
            Thread(r, "${engine.name}-run-$idx").apply { isDaemon = true }
        }

        val future = executor.submit(Callable {
            System.gc()
            val load = measureTimedValue { engine.loadData(args.dataPath, args.dataFormat, args.shapesPath, args.shapesFormat) }
            System.gc()
            val validation = measureTimedValue { engine.validate() }
            val report = engine.generateReport(validation.value)
            RunOutcome(
                loadTimeMs = load.duration.inWholeMicroseconds / 1000.0,
                validationTimeMs = validation.duration.inWholeMicroseconds / 1000.0,
                report = report,
            )
        })

        return try {
            val outcome = future.get(args.timeout.toLong(), TimeUnit.SECONDS)
            val result = IterationResults(idx, outcome.loadTimeMs, outcome.validationTimeMs, IterationResultStatus.OK)
            result to outcome.report
        } catch (_: TimeoutException) {
            println("[${engine.name}] Run $idx exceeded timeout of ${args.timeout}s")
            future.cancel(true)
            IterationResults(idx, null, null, IterationResultStatus.DNF) to null
        } catch (e: ExecutionException) {
            val cause = e.cause
            val status = if (cause is OutOfMemoryError) IterationResultStatus.OOM else IterationResultStatus.ERROR
            println("[${engine.name}] Run $idx failed: ${cause?.message}")
            IterationResults(idx, null, null, status) to null
        } finally {
            executor.shutdownNow()
        }
    }
}
