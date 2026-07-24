#!/usr/bin/env python3

"""
Generate LaTeX tables from benchmark results.

Usage:
    python tools/generate_tables.py [criterion|engines-era|engines-icdd|engines-lubm|all]

Default: all
"""

import json
import statistics
import sys
from pathlib import Path

RESULTS = Path(__file__).parent.parent / "iswc-2026"
CRITERION = RESULTS / "criterion"
ENGINES = RESULTS / "engines"


# ---------------------------------------------------------------------------
# Data loading
# ---------------------------------------------------------------------------

def load_criterion_median_ms(case_dir: Path) -> float | None:
    estimates = case_dir / "base" / "estimates.json"
    if not estimates.exists():
        return None
    data = json.loads(estimates.read_text())
    return data["median"]["point_estimate"] / 1_000_000  # ns → ms


def load_engine_median_ms(json_path: Path) -> float | None:
    if not json_path.exists():
        return None
    doc = json.loads(json_path.read_text())
    ok_times = [
        r["validationTime"]
        for r in doc.get("iterationResults", [])
        if r.get("iterationResults") == "OK" and r.get("validationTime") is not None
    ]
    if not ok_times:
        return None
    return statistics.median(ok_times)


# ---------------------------------------------------------------------------
# Number formatting
# ---------------------------------------------------------------------------

def fmt_ms(v: float | None, dnf: str = r"\multicolumn{1}{c}{DNF}", siunitx: bool = False) -> str:
    if v is None:
        return dnf
    if v >= 1_000_000:
        # Extreme outlier: show in seconds
        return f"{v / 1000:,.0f}\\,s"
    if v >= 10_000:
        if siunitx:
            return f"{v:.3f}"
        formatted = f"{round(v):,}".replace(",", "\\,")
        return formatted
    if v >= 1_000:
        if siunitx:
            return f"{v:.3f}"
        formatted = f"{round(v):,}".replace(",", "\\,")
        return formatted
    return f"{v:.3f}"


def fmt_ratio(v1: float | None, v2: float | None) -> str:
    if v1 is None or v2 is None:
        return r"\multicolumn{1}{c}{---}"
    ratio = v1 / v2
    return f"{ratio:.3f}"


# ---------------------------------------------------------------------------
# Criterion table
# ---------------------------------------------------------------------------

# Ordered ERA cases: (dir_stem, display_dataset, display_case)
ERA_CRITERION_CASES = [
    ("es-data.ttl (core-shapes.ttl)",  "ERA",  r"\texttt{es}\,/\,\texttt{core}"),
    ("es-data.ttl (era-shapes.ttl)",   "ERA",  r"\texttt{es}\,/\,\texttt{era}"),
    ("es-data.ttl (tds-shapes.ttl)",   "ERA",  r"\texttt{es}\,/\,\texttt{tds}"),
    ("fr-data.ttl (core-shapes.ttl)",  "ERA",  r"\texttt{fr}\,/\,\texttt{core}"),
    ("fr-data.ttl (era-shapes.ttl)",   "ERA",  r"\texttt{fr}\,/\,\texttt{era}"),
    ("fr-data.ttl (tds-shapes.ttl)",   "ERA",  r"\texttt{fr}\,/\,\texttt{tds}"),
    ("era-data.ttl (core-shapes.ttl)", "ERA",  r"\texttt{era}\,/\,\texttt{core}"),
    ("era-data.ttl (era-shapes.ttl)",  "ERA",  r"\texttt{era}\,/\,\texttt{era}"),
    ("era-data.ttl (tds-shapes.ttl)",  "ERA",  r"\texttt{era}\,/\,\texttt{tds}"),
]

ICDD_CRITERION_CASES = [
    (f"data-{pat}-{scale}.ttl", "ICDD", rf"\texttt{{{pat}-{scale}}}")
    for pat in ("binary", "directed1ton", "directedbinary")
    for scale in range(1, 5)
]

LUBM_CRITERION_CASES = [
    (f"data-{n}.nt", "LUBM", f"{n:,} universities".replace(",", "\\,"))
    for n in (5, 10, 50, 100, 500)
]


def generate_criterion_table() -> str:
    group = CRITERION / "ERA Validation"
    v1_base = group / "rudof v1"
    v2_base = group / "rudof v2"

    rows_era = []
    for stem, dataset, label in ERA_CRITERION_CASES:
        v1 = load_criterion_median_ms(v1_base / stem)
        v2 = load_criterion_median_ms(v2_base / stem)
        rows_era.append((dataset, label, v1, v2))

    rows_icdd = []
    group_icdd = CRITERION / "ICDD Validation"
    for stem, dataset, label in ICDD_CRITERION_CASES:
        v1 = load_criterion_median_ms(group_icdd / "rudof v1" / stem)
        v2 = load_criterion_median_ms(group_icdd / "rudof v2" / stem)
        rows_icdd.append((dataset, label, v1, v2))

    rows_lubm = []
    group_lubm = CRITERION / "LUBM Validation"
    for stem, dataset, label in LUBM_CRITERION_CASES:
        v1 = load_criterion_median_ms(group_lubm / "rudof v1" / stem)
        v2 = load_criterion_median_ms(group_lubm / "rudof v2" / stem)
        rows_lubm.append((dataset, label, v1, v2))

    def render_rows(rows: list) -> str:
        lines = []
        for dataset, label, v1, v2 in rows:
            r1 = fmt_ms(v1, siunitx=True)
            r2 = fmt_ms(v2, siunitx=True)
            ratio = fmt_ratio(v1, v2)
            lines.append(f"    {dataset}  & {label:<50} & {r1:>10} & {r2:>10} & {ratio:>10} \\\\")
        return "\n".join(lines)

    body = (
        render_rows(rows_era)
        + "\n    \\midrule\n"
        + render_rows(rows_icdd)
        + "\n    \\midrule\n"
        + render_rows(rows_lubm)
    )

    return rf"""\begin{{table*}}[ht]
  \centering
  \small
  \caption{{Per-case median iteration time (ms) of rudof~v1 and rudof~v2}}
  \label{{tab:results-rudof-criterion}}
  \begin{{tabular}}{{l l S[table-format=6.3,group-separator={{\,}}] S[table-format=4.3,group-separator={{\,}}] S[table-format=4.3]}}
    \toprule
    Dataset & Case & {{$\widetilde{{t}}_{{v1}}$ (ms)}} & {{$\widetilde{{t}}_{{v2}}$ (ms)}} & {{$\widetilde{{t}}_{{v1}} / \widetilde{{t}}_{{v2}}$}} \\
    \midrule
{body}
    \bottomrule
  \end{{tabular}}
\end{{table*}}"""


# ---------------------------------------------------------------------------
# Engines tables
# ---------------------------------------------------------------------------

ENGINES_ORDER = [
    ("jena",       "Jena"),
    ("topbraid",   "TopBraid"),
    ("rdf4j",      "RDF4J"),
    ("corese",     "Corese"),
    ("rdfunit",    "RDFUnit"),
    ("maplib",     "maplib"),
    ("pyshacl_n", r"pySHACL (N)"),
    ("pyshacl",    "pySHACL"),
    ("rudof_v1",   "rudof~v1"),
    ("rudof_v2",   "rudof~v2"),
]

ERA_CASES = [
    ("es",  "core",  r"\texttt{es}/\texttt{core}"),
    ("es",  "era",   r"\texttt{es}/\texttt{era}"),
    ("es",  "tds",   r"\texttt{es}/\texttt{tds}"),
    ("fr",  "core",  r"\texttt{fr}/\texttt{core}"),
    ("fr",  "era",   r"\texttt{fr}/\texttt{era}"),
    ("fr",  "tds",   r"\texttt{fr}/\texttt{tds}"),
    ("era", "core",  r"\texttt{era}/\texttt{core}"),
    ("era", "era",   r"\texttt{era}/\texttt{era}"),
    ("era", "tds",   r"\texttt{era}/\texttt{tds}"),
]

ICDD_CASES = [
    (pat, str(scale), rf"\texttt{{{pat}-{scale}}}")
    for pat in ("binary", "directed1ton", "directedbinary")
    for scale in range(1, 5)
]

LUBM_CASES = [
    (str(n), rf"{n:,}".replace(",", "\\,") + r"\,uni")
    for n in (5, 10, 50, 100, 500)
]


def _engines_table(
    dataset_name: str,
    label: str,
    caption: str,
    cases: list,
    json_dir: Path,
    csv_name_fn,
) -> str:
    eng_labels = [lbl for _, lbl in ENGINES_ORDER]
    eng_keys   = [key for key, _ in ENGINES_ORDER]

    col_spec = "l " + " r" * len(ENGINES_ORDER)

    def header_row() -> str:
        heads = " & ".join(
            rf"\rotatebox{{90}}{{\small {lbl}}}" for lbl in eng_labels
        )
        return f"    Case & {heads} \\\\"

    def data_rows() -> str:
        lines = []
        for case_args in cases:
            *keys, label = case_args
            cells = []
            for key in eng_keys:
                csv = json_dir / csv_name_fn(key, *keys)
                val = load_engine_median_ms(csv)
                cells.append(fmt_ms(val, dnf=r"\multicolumn{1}{c}{---}"))
            row = f"    {label} & " + " & ".join(cells) + r" \\"
            lines.append(row)
        return "\n".join(lines)

    return rf"""\begin{{table*}}[ht]
  \centering
  \tiny
  \caption{{{caption}}}
  \label{{{label}}}
  \begin{{tabular}}{{{col_spec}}}
    \toprule
{header_row()}
    \midrule
{data_rows()}
    \bottomrule
  \end{{tabular}}
\end{{table*}}"""


def generate_engines_era_table() -> str:
    return _engines_table(
        dataset_name="ERA",
        label="tab:results-engines-era",
        caption=(
            "ERA benchmark: median iteration time (ms) per validator across the nine "
            r"combinations of data graph (\texttt{es}, \texttt{fr}, \texttt{era}) and "
            r"shape graph (\texttt{core}, \texttt{era}, \texttt{tds}). "
            "``---'' indicates DNF within the 10\\,h per-combination budget."
        ),
        cases=ERA_CASES,
        json_dir=ENGINES / "era",
        csv_name_fn=lambda eng, data, shapes: f"{eng}-{data}-{shapes}.json",
    )


def generate_engines_icdd_table() -> str:
    return _engines_table(
        dataset_name="ICDD",
        label="tab:results-engines-icdd",
        caption=(
            "ICDD benchmark: median iteration time (ms) per validator across the twelve "
            r"variants (\texttt{binary}, \texttt{directed1ton}, \texttt{directedbinary} "
            r"at four scale steps each)."
        ),
        cases=ICDD_CASES,
        json_dir=ENGINES / "icdd",
        csv_name_fn=lambda eng, pat, scale: f"{eng}-{pat}-{scale}.json",
    )


def generate_engines_lubm_table() -> str:
    return _engines_table(
        dataset_name="LUBM",
        label="tab:results-engines-lubm",
        caption=(
            "LUBM benchmark: median iteration time (ms) per validator "
            "across five university-count scale factors (5, 10, 50, 100, 500)."
        ),
        cases=[(n, label) for n, label in LUBM_CASES],
        json_dir=ENGINES / "lubm",
        csv_name_fn=lambda eng, n: f"{eng}-{n}.json",
    )


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------

GENERATORS = {
    "criterion":    generate_criterion_table,
    "engines-era":  generate_engines_era_table,
    "engines-icdd": generate_engines_icdd_table,
    "engines-lubm": generate_engines_lubm_table,
}

def main() -> None:
    targets = sys.argv[1:] or ["all"]
    if targets == ["all"]:
        targets = list(GENERATORS)

    for i, target in enumerate(targets):
        if target not in GENERATORS:
            print(f"Unknown target '{target}'. Available: {', '.join(GENERATORS)}", file=sys.stderr)
            sys.exit(1)
        if i > 0:
            print()
        print(f"% --- autogenerated - {target} ---")
        print(GENERATORS[target]())


if __name__ == "__main__":
    main()
