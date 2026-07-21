import re, tempfile, os

def patch(mode: str, filename: str, other: list[str]) -> None:
    fn = None
    if mode.lower() == "data":
        fn = __patch_data
    elif mode.lower() == "shapes":
        fn = __patch_shapes
    else:
        raise Exception(f"Mode ({mode}) not valid")

    with tempfile.NamedTemporaryFile(
        mode="w", delete=False,
        dir=os.path.dirname(filename),
        encoding="utf-8"
    ) as tmp:
        with open(filename, mode="r", encoding="utf-8") as f:
            for line in f:
                tmp.write(fn(line))
    os.replace(tmp.name, filename)

def __patch_data(line: str) -> str:
    line = __patch_empty_numeric_triple(line)
    return line

def __patch_shapes(line: str) -> str:
    return line

__NUMERIC_XSD = (
    "integer|double|decimal|float"
    "|long|int|short|byte"
    "|nonNegativeInteger|nonPositiveInteger"
    "|positiveInteger|negativeInteger"
    "|unsignedLong|unsignedInt|unsignedShort|unsignedByte"
)

__EMPTY_NUMERIC_TRIPLE = re.compile(
    r'^\s*(?:<[^>]*>|_:\S+)\s+<[^>]*>\s+'
    r'""\^\^<[^>]*XMLSchema#(?:' + __NUMERIC_XSD + r')>\s*\.\s*$'
)

def __patch_empty_numeric_triple(line: str) -> str:
    return "" if __EMPTY_NUMERIC_TRIPLE.match(line) is not None else line
