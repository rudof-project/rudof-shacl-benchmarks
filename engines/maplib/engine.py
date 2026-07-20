from maplib import Model, ValidationReport

from validation_engine import ValidationEngine

class Engine(ValidationEngine[ValidationReport]):
    name = "maplib"

    _DATA_GRAPH_IRI = "urn:bench:data"
    _SHAPES_GRAPH_IRI = "urn:bench:shapes"
    # _REPORT_GRAPH_IRI = "urn:bench:validation-report"
    _BASE_IRI = "http://ex.net/"

    def __init__(self) -> None:
        self._model: Model | None = None
        self._size_logged = False

    def load_data(self, data_path: str, data_format: str, shapes_path: str, shapes_format: str) -> None:
        self._model = Model()
        assert self._model is not None

        self._model.read(
            data_path,
            parallel=False, # Makes trouble while parsing prefixes
            graph=self._DATA_GRAPH_IRI,
            base_iri=self._BASE_IRI
        )
        self._model.read(shapes_path,
            parallel=False, # Makes trouble while parsing prefixes
            graph=self._SHAPES_GRAPH_IRI,
            base_iri=self._BASE_IRI
        )

        if not self._size_logged:
            print(f"[{self.name}] Data graph size:   {self._model.size(self._DATA_GRAPH_IRI)}")
            print(f"[{self.name}] Shapes graph size: {self._model.size(self._SHAPES_GRAPH_IRI)}")
            self._size_logged = True

    def validate(self) -> ValidationReport:
        assert self._model is not None
        return self._model.validate(
            data_graph=self._DATA_GRAPH_IRI,
            shape_graph=self._SHAPES_GRAPH_IRI,
            # report_graph=self._REPORT_GRAPH_IRI,
        )

    def generate_report(self, result: ValidationReport) -> str:
        assert self._model is not None
        return "TODO"
        # return self._model.writes(format="turtle", graph=self._REPORT_GRAPH_IRI)
