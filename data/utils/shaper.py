from shexer.shaper import Shaper
from shexer.consts import NT, SHACL_TURTLE

target_classes = [
    "data/univ-bench.owl#Department",
    "data/univ-bench.owl#FullProfessor",
    "data/univ-bench.owl#University"
]

with open("out.test.nt", "r", encoding="utf-8") as f:
    shaper = Shaper(target_classes=target_classes, raw_graph=f.read(), input_format=NT)

    # shaper.shex_graph(output_file="shapes.shacl", acceptance_threshold=0.1, output_format=SHACL_TURTLE)
    shaper.shex_graph(output_file="shapes.shex", acceptance_threshold=0.1)
