import gc, sys, time
from maplib import Model

# Usage: python maplib <data_path> <data_format> <shapes_path> <shapes_format> <csv_path> <report_path> [runs] [warm_up]
#
# - data_path: Path to an RDF file containing the data graph
# - data_format: RDF format of the <data_path>
# - shapes_path: Path to a SHACL shapes file
# - shapes_format: RDF format of the <shapes_path>
# - csv_path: Path to save the CSV report file
# - report_path: Path to save the SHACL validation report (Turtle)
# - runs: Number of benchmark runs (Result runs = runs - warm_up)
# - warm_up: Number of runs for warm up
def main() -> None:
    data_path = get_arg(1, "Missing data graph path")
    data_format = get_arg(2, "Missing data format")
    shapes_path = get_arg(3, "Missing shapes graph path")
    shapes_format = get_arg(4, "Missing shapes format")
    csv_path = get_arg(5, "Missing csv report path")
    report_path = get_arg(6, "Missing validation report path")
    runs = int(get_arg(7, "", 20))
    warm_up = int(get_arg(8, "", 10))
    results: list[str] = []
    last_report = None
    last_model = None

    print(f"[maplib] Data:    {data_path} ({data_format})")
    print(f"[maplib] Shapes:  {shapes_path} ({shapes_format})")
    print(f"[maplib] CSV:     {csv_path}")
    print(f"[maplib] Report:  {report_path}")
    print(f"[maplib] Runs:    {runs}, warm-up: {warm_up}")

    data_graph_iri = "urn:bench:data"
    shapes_graph_iri = "urn:bench:shapes"
    report_graph_iri = "urn:bench:validation-report"

    for i in range(runs + warm_up):
        model = Model()

        model.read(data_path, parallel=True, graph=data_graph_iri)
        model.read(shapes_path, parallel=True, graph=shapes_graph_iri)

        gc.collect()
        gc.disable()
        start = time.time()
        last_report = model.validate(
            data_graph=data_graph_iri,
            shape_graph=shapes_graph_iri,
            report_graph=report_graph_iri,
        )
        delta = time.time() - start
        gc.enable()
        last_model = model

        if i >= warm_up:
            results.append(f"{delta * 1000:.3f}\n")
        if i == warm_up - 1:
            print("[maplib] Warm-up complete")

    with open(csv_path, mode="w", encoding="utf-8") as f:
        f.writelines(results)

    last_model.write(report_path, format="turtle", graph=report_graph_iri)
    print(f"[maplib] Done -> {csv_path}, {report_path}")

def get_arg(idx: int, msg: str, default=None) -> str:
    arg = None
    try:
        arg = sys.argv[idx]
    except:
        pass

    if arg == None:
        if default == None:
            raise Exception(msg)
        arg = default

    return arg

if __name__ == "__main__":
    main()
