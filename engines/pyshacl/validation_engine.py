from abc import ABC, abstractmethod
from typing import Generic, TypeVar

R = TypeVar("R")

class ValidationEngine(ABC, Generic[R]):
    name: str

    @abstractmethod
    def load_data(
        self,
        data_path: str,
        data_format: str,
        shapes_path: str,
        shapes_format: str,
    ) -> None: ...

    @abstractmethod
    def validate(self) -> R: ...

    @abstractmethod
    def generate_report(self, result: R) -> str: ...
