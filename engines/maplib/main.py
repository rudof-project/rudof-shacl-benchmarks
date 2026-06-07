import sys, time
from maplib import Model

def main() -> None:
    data_path = get_arg(1, "Missing data graph path")
    get_arg(2, "Missing data format")
    shapes_path = get_arg(3, "Missing shapes graph path")
    get_arg(4, "Missing shapes format")
    csv_path = get_arg(5, "Missing csv report path")
    runs = int(get_arg(6, "", 20))
    warm_up = int(get_arg(7, "", 10))
    results: list[str] = []

    for i in range(runs + warm_up):
        model = Model()

        model.read(data_path, parallel=True)
        model.read(shapes_path)

        start = time.time()
        model.validate(include_shape_graph=True)
        delta = time.time() - start

        if i >= warm_up:
            results.append(f"{delta}")

    with open(csv_path, mode="w", encoding="utf-8") as f:
        f.writelines(results)

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
