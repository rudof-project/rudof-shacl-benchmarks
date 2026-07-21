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
    line = __patch_xsd_dateTime(line)
    line = __patch_xsd_boolean(line)
    return line

def __patch_shapes(line: str) -> str:
    return line

# Transforms dateTimes with format "dd.MM.YYYY HH:MM:SS"
# into xsd:dateTime format: "YYYY-MM-ddTHH:MM:SS"
def __patch_xsd_dateTime(line: str) -> str:
    return re.sub(
        r"(?P<day>\d\d)\.(?P<month>\d\d)\.(?P<year>\d\d\d\d) (?P<hours>\d\d):(?P<minutes>\d\d):(?P<seconds>\d\d)",
        r"\g<year>-\g<month>-\g<day>T\g<hours>:\g<minutes>:\g<seconds>",
        line
    )

# Transforms "True"^^xsd:boolean into the correct version:
# "true"^^xsd:boolean
def __patch_xsd_boolean(line: str) -> str:
    return re.sub(
        r'"True"\^\^xsd:boolean',
        r'"true"^^xsd:boolean',
        line
    )
