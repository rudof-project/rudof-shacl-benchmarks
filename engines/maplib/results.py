import math
from dataclasses import dataclass, asdict
from enum import Enum

class IterationResultStatus(str, Enum):
    OK = "OK"  # Iteration completed successfully
    ERROR = "ERROR"  # Some exception was thrown
    OOM = "OOM"  # Out of memory error
    DNF = "DNF"  # Run exceeded timeout

class ResultStatus(str, Enum):
    SUCCESS = "SUCCESS" # All runs are OK
    PARTIAL = "PARTIAL" # More than `min_valid_iter` runs are OK
    FAILED = "FAILED" # There are not enough OK cases

@dataclass
class IterationResults:
    iteration: int
    loadTime: float | None
    validationTime: float | None
    iterationResults: IterationResultStatus

@dataclass
class Results:
    iterationResults: list[IterationResults]
    status: ResultStatus
    loadMean: float | None
    loadStd: float | None
    validationMean: float | None
    validationStd: float | None

    def to_dict(self) -> dict:
        data = asdict(self)
        data["status"] = self.status.value
        for entry in data["iterationResults"]:
            entry["iterationResults"] = entry["iterationResults"].value \
                if isinstance(entry["iterationResults"], IterationResultStatus) \
                else entry["iterationResults"]
        return data

class ResultState:
    def __init__(self, min_iter: int):
        self._min_iter = min_iter
        self._results: list[IterationResults] = []

    def add_result(self, result: IterationResults) -> None:
        self._results.append(result)

    @staticmethod
    def _mean_of(times: list[float]) -> float | None:
        if not times:
            return None
        return sum(times) / len(times)

    @staticmethod
    def _std_of(times: list[float]) -> float | None:
        if not times:
            return None
        mean_value = sum(times) / len(times)
        return math.sqrt(sum((t - mean_value) ** 2 for t in times) / len(times))

    def generate_results(self) -> Results:
        ok_results = [r for r in self._results if r.iterationResults == IterationResultStatus.OK]

        load_times = [r.loadTime for r in ok_results if r.loadTime is not None]
        validation_times = [r.validationTime for r in ok_results if r.validationTime is not None]

        ok_count = len(ok_results)
        if ok_count == len(self._results):
            status = ResultStatus.SUCCESS
        elif ok_count >= self._min_iter:
            status = ResultStatus.PARTIAL
        else:
            status = ResultStatus.FAILED

        return Results(
            iterationResults=list(self._results),
            status=status,
            loadMean=self._mean_of(load_times),
            loadStd=self._std_of(load_times),
            validationMean=self._mean_of(validation_times),
            validationStd=self._std_of(validation_times),
        )
