import rdflib
from pyshacl import validate
from rdflib import Graph

from validation_engine import ValidationEngine

class Engine(ValidationEngine[Graph]):
    name = "pyshacl"

    def __init__(self) -> None:
        self._data_graph: Graph | None = None
        self._shapes_graph: Graph | None = None
        self._size_logged = False

    def load_data(self, data_path: str, data_format: str, shapes_path: str, shapes_format: str) -> None:
        self._data_graph = rdflib.Graph()
        assert self._data_graph is not None
        with open(data_path, "r", encoding="utf-8") as f:
            self._data_graph.parse(data=f.read())

        self._shapes_graph = rdflib.Graph()
        assert self._shapes_graph is not None
        with open(shapes_path, "r", encoding="utf-8") as f:
            self._shapes_graph.parse(data=f.read())

        if not self._size_logged:
            print(f"[{self.name}] Data graph size:   {len(self._data_graph)}")
            print(f"[{self.name}] Shapes graph size: {len(self._shapes_graph)}")
            self._size_logged = True

    def validate(self) -> Graph:
        assert self._data_graph is not None
        assert self._shapes_graph is not None
        _, results_graph, _ = validate(data_graph=self._data_graph, shacl_graph=self._shapes_graph, inference="")
        return results_graph

    def generate_report(self, result: Graph) -> str:
        return result.serialize(format="turtle")
