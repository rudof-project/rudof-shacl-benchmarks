package es.weso.rudof

import kotlinx.serialization.json.Json
import java.io.File

// Usage: java -jar corese.jar <data_path> <data_format> <shapes_path> \
//          <shapes_format> <stats_path> <report_path> [runs] [warm_up] [timeout] [min_validation_iterations]
//
// - data_path: Path to an RDF file containing the data graph
// - data_format: RDF format of the <data_path>
// - shapes_path: Path to a SHACL shapes file
// - shapes_format: RDF format of the <shapes_path>
// - stats_path: Path to save the stats report file
// - report_path: Path to save the SHACL validation report (Turtle)
// - runs: Number of benchmark runs (Result runs = runs - warm_up)
// - warm_up: Number of runs for warm up
// - timeout: Timeout in seconds for each run
// - min_valid_iterations: Minimum number of valid runs (inclusive) to consider the benchmark successful
fun main(rawArgs: Array<String>) {
    val args = parseArgs(rawArgs)
    val engine = Engine()
    args.print(engine.name)

    val results = BenchmarkRunner(engine, args).run()
    File(args.statsPath).writeText(Json.encodeToString(results.generateResults()))

    println("[${engine.name}] Done -> ${args.statsPath}, ${args.reportPath}")
}
