from dataclasses import dataclass

@dataclass
class Args:
    data_path: str
    data_format: str

    shapes_path: str
    shapes_format: str

    stats_path: str
    report_path: str

    runs: int
    warm_up: int
    timeout: int  # Seconds
    min_valid_iter: int  # Minimum of valid runs (inclusive)

    def print(self, name: str) -> None:
        print(f"[{name}] Data:               {self.data_path} ({self.data_format})")
        print(f"[{name}] Shapes:             {self.shapes_path} ({self.shapes_format})")
        print(f"[{name}] Stats:              {self.stats_path}")
        print(f"[{name}] Report:             {self.report_path}")
        print(f"[{name}] Runs:               {self.runs}")
        print(f"[{name}] Warm-up:            {self.warm_up}")
        print(f"[{name}] Timeout:            {self.timeout} s")
        print(f"[{name}] Minimum valid runs: {self.min_valid_iter} (inclusive)")


def parse_args(argv: list[str]) -> Args:
    def get(idx: int, msg: str, default: str | None = None) -> str:
        try:
            return argv[idx]
        except IndexError:
            if default is None:
                raise Exception(msg)
            return default

    return Args(
        data_path=get(1, "Missing data graph path"),
        data_format=get(2, "Missing data format"),
        shapes_path=get(3, "Missing shapes graph path"),
        shapes_format=get(4, "Missing shapes format"),
        stats_path=get(5, "Missing stats report path"),
        report_path=get(6, "Missing validation report path"),
        runs=int(get(7, "", "20")),
        warm_up=int(get(8, "", "10")),
        timeout=int(get(9, "", "300")),
        min_valid_iter=int(get(10, "", "8")),
    )
