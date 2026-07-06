import sys
import json

from args import parse_args
from benchmark_runner import BenchmarkRunner
from engine import Engine

# Usage: python main.py <data_path> <data_format> <shapes_path> \
#          <shapes_format> <stats_path> <report_path> [runs] [warm_up] [timeout] [min_validation_iterations]
#
# - data_path: Path to an RDF file containing the data graph
# - data_format: RDF format of the <data_path>
# - shapes_path: Path to a SHACL shapes file
# - shapes_format: RDF format of the <shapes_path>
# - stats_path: Path to save the stats report file
# - report_path: Path to save the SHACL validation report (Turtle)
# - runs: Number of benchmark runs (Result runs = runs - warm_up)
# - warm_up: Number of runs for warm up
# - timeout: Timeout in seconds for each run
# - min_valid_iterations: Minimum number of valid runs (inclusive) to consider the benchmark successful
def main() -> None:
    args = parse_args(sys.argv)
    engine = Engine()
    args.print(engine.name)

    results = BenchmarkRunner(engine, args).run()
    with open(args.stats_path, "w", encoding="utf-8") as f:
        json.dump(results.generate_results().to_dict(), f)

    print(f"[{engine.name}] Done -> {args.stats_path}, {args.report_path}")

if __name__ == "__main__":
    main()
