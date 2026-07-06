from pathlib import Path
import re
import json
import math
import statistics

import matplotlib.pyplot as plt
from matplotlib.gridspec import GridSpecFromSubplotSpec
from matplotlib.lines import Line2D
from matplotlib.patches import Patch

DIST_DIR = Path(__file__).parent.parent / "dist"
OUT_DIR = DIST_DIR

_FILENAME_RE = re.compile(r"^(?P<engine>\w+)-(?P<variant>.+)\.json$")
_MISSING_COLOR = "lightgray"
_FAILED_COLOR = "#c0392b"
_PALETTE = plt.colormaps["tab10"].colors
_OUTLIER_THRESHOLD = 5
_BREAK_D = 0.015
_PARTIAL_HATCH = "///"
_METRICS = (("load", "loadTime", "loadMean", "loadStd"),
            ("validation", "validationTime", "validationMean", "validationStd"))


def _natural_key(s: str) -> list:
    return [int(p) if p.isdigit() else p for p in re.split(r"(\d+)", s)]


def _broken_ylims(means: list[float], stds: list[float]):
    """Return ((0, lower_top), (upper_bot, upper_top)) if an outlier is detected, else None."""
    valid = [(m, s) for m, s in zip(means, stds) if m > 0]
    if len(valid) < 2:
        return None

    med = statistics.median(m for m, _ in valid)
    if med == 0:
        return None

    normal = [(m, s) for m, s in valid if m <= _OUTLIER_THRESHOLD * med]
    outliers = [(m, s) for m, s in valid if m > _OUTLIER_THRESHOLD * med]

    if not outliers or not normal:
        return None

    lower_top = max(m + s for m, s in normal) * 1.2
    upper_bot = min(max(m - s, 0) for m, s in outliers) * 0.85
    upper_top = max(m + s for m, s in outliers) * 1.15

    if upper_bot <= lower_top:
        return None

    return (0, lower_top), (upper_bot, upper_top)


def _badge_text(s: dict | None) -> str | None:
    """Compact 'N DNF · M ERROR' string of non-OK iteration counts, or None."""
    if s is None:
        return None
    counts = s.get("counts") or {}
    non_ok = [(k, v) for k, v in counts.items() if k != "OK" and v > 0]
    if not non_ok:
        return None
    order = {"DNF": 0, "ERROR": 1, "OOM": 2}
    non_ok.sort(key=lambda kv: order.get(kv[0], 99))
    return " · ".join(f"{v} {k}" for k, v in non_ok)


def _failed_text(s: dict) -> str:
    counts = s.get("counts") or {}
    parts = [f"{v} {k}" for k, v in counts.items() if k != "OK" and v > 0]
    detail = ", ".join(parts) if parts else "no OK runs"
    return f"FAILED ({detail})"


def _annotate_flat(ax, engines, subplot_stats, y_top: float):
    """Annotate a single non-broken bar axis."""
    y_offset = y_top * 0.02
    for i, engine in enumerate(engines):
        s = subplot_stats[engine]
        if s is None:
            ax.text(i, y_offset, "N/A", ha="center", va="bottom",
                    fontsize=8, color="gray")
            continue
        status = s.get("status")
        if status == "FAILED" or s.get("mean") is None:
            ax.text(i, y_offset, _failed_text(s), ha="center", va="bottom",
                    fontsize=7, color=_FAILED_COLOR, fontweight="bold",
                    rotation=90)
            continue
        badge = _badge_text(s)
        if badge is not None:
            mean = s["mean"] or 0.0
            std = s.get("std") or 0.0
            ax.text(i, mean + std + y_offset, badge, ha="center", va="bottom",
                    fontsize=7, color=_FAILED_COLOR)


def _annotate_broken(ax_top, ax_bot, engines, subplot_stats,
                     lower_top: float, upper_bot: float, upper_top: float):
    """Annotate a broken-axis subplot once per engine, on the axis that owns y=0
    or the axis that contains the bar top."""
    y_offset_bot = lower_top * 0.02
    y_offset_top = (upper_top - upper_bot) * 0.02
    for i, engine in enumerate(engines):
        s = subplot_stats[engine]
        if s is None:
            ax_bot.text(i, y_offset_bot, "N/A", ha="center", va="bottom",
                        fontsize=8, color="gray")
            continue
        status = s.get("status")
        if status == "FAILED" or s.get("mean") is None:
            ax_bot.text(i, y_offset_bot, _failed_text(s), ha="center", va="bottom",
                        fontsize=7, color=_FAILED_COLOR, fontweight="bold",
                        rotation=90)
            continue
        badge = _badge_text(s)
        if badge is not None:
            mean = s["mean"] or 0.0
            std = s.get("std") or 0.0
            anchor = mean + std
            if anchor >= upper_bot:
                ax_top.text(i, anchor + y_offset_top, badge, ha="center",
                            va="bottom", fontsize=7, color=_FAILED_COLOR)
            else:
                ax_bot.text(i, anchor + y_offset_bot, badge, ha="center",
                            va="bottom", fontsize=7, color=_FAILED_COLOR)


def _draw_broken_subplot(fig, subplot_spec, engines, means, stds, colors,
                         hatches, subplot_stats, variant, ylims):
    (_, lower_top), (upper_bot, upper_top) = ylims
    height_ratios = [1, 2]

    sub_gs = GridSpecFromSubplotSpec(
        2, 1, subplot_spec=subplot_spec,
        height_ratios=height_ratios,
        hspace=0.05,
    )
    ax_top = fig.add_subplot(sub_gs[0])
    ax_bot = fig.add_subplot(sub_gs[1])

    for ax in (ax_top, ax_bot):
        bars = ax.bar(engines, means, yerr=stds, capsize=4, color=colors,
                      error_kw={"elinewidth": 1})
        for bar, hatch in zip(bars, hatches):
            if hatch:
                bar.set_hatch(hatch)
                bar.set_edgecolor("black")
                bar.set_linewidth(0.5)

    ax_top.set_ylim(upper_bot, upper_top)
    ax_bot.set_ylim(0, lower_top)

    ax_top.spines["bottom"].set_visible(False)
    ax_bot.spines["top"].set_visible(False)
    ax_top.spines["right"].set_visible(False)
    ax_bot.spines["right"].set_visible(False)
    ax_top.tick_params(bottom=False, right=False)
    ax_bot.tick_params(top=False, right=False)

    ax_top.set_xticklabels([])
    ax_top.tick_params(axis="x", length=0)
    ax_bot.tick_params(axis="x", rotation=45)

    d_x = _BREAK_D
    d_y_bot = _BREAK_D
    d_y_top = _BREAK_D * (height_ratios[1] / height_ratios[0])

    mk = dict(color="k", clip_on=False, linewidth=1)
    ax_top.plot((-d_x, +d_x), (-d_y_top, +d_y_top), transform=ax_top.transAxes, **mk)
    ax_bot.plot((-d_x, +d_x), (1 - d_y_bot, 1 + d_y_bot), transform=ax_bot.transAxes, **mk)

    ax_top.set_title(variant)
    ax_bot.set_ylabel("Time (ms)")

    _annotate_broken(ax_top, ax_bot, engines, subplot_stats,
                     lower_top, upper_bot, upper_top)

    return ax_top, ax_bot


def _add_right_border(fig, ax_top, ax_bot):
    pos_top = ax_top.get_position()
    pos_bot = ax_bot.get_position()
    lw = plt.rcParams.get("axes.linewidth", 0.8)
    fig.add_artist(Line2D(
        [pos_top.x1, pos_top.x1],
        [pos_bot.y0, pos_top.y1],
        transform=fig.transFigure,
        color="black",
        linewidth=lw,
        clip_on=False,
        zorder=10,
    ))


def discover_data(dist_dir: Path) -> dict:
    data = {}
    for bench_dir in sorted(dist_dir.iterdir()):
        if not bench_dir.is_dir():
            continue
        bench = bench_dir.name
        engines: set[str] = set()
        variants: set[str] = set()
        files: dict[tuple[str, str], Path] = {}

        for f in bench_dir.iterdir():
            if not f.is_file() or f.suffix != ".json":
                continue
            m = _FILENAME_RE.fullmatch(f.name)
            if not m:
                continue
            engine, variant = m.group("engine"), m.group("variant")
            engines.add(engine)
            variants.add(variant)
            files[(variant, engine)] = f

        engines_sorted = sorted(engines, key=_natural_key)
        variants_sorted = sorted(variants, key=_natural_key)
        data[bench] = {
            v: {e: files.get((v, e)) for e in engines_sorted}
            for v in variants_sorted
        }
    return data


def _load_json(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def compute_stats(path: Path | None, metric: str) -> dict | None:
    """Stats for one metric ('load' or 'validation') from a single JSON file."""
    if path is None:
        return None
    doc = _load_json(path)
    time_key = f"{metric}Time"
    mean_key = f"{metric}Mean"
    std_key = f"{metric}Std"

    iterations = doc.get("iterationResults", [])
    ok_times = [r[time_key] for r in iterations
                if r.get("iterationResults") == "OK" and r.get(time_key) is not None]

    counts: dict[str, int] = {}
    for r in iterations:
        status = r.get("iterationResults", "UNKNOWN")
        counts[status] = counts.get(status, 0) + 1

    def _r(x):
        return round(float(x), 3) if x is not None else None

    return {
        "mean": _r(doc.get(mean_key)),
        "std": _r(doc.get(std_key)),
        "min": _r(min(ok_times)) if ok_times else None,
        "max": _r(max(ok_times)) if ok_times else None,
        "median": _r(statistics.median(ok_times)) if ok_times else None,
        "count": len(ok_times),
        "totalIters": len(iterations),
        "status": doc.get("status"),
        "counts": counts,
    }


def _resolve_style(s: dict | None, engine_color: tuple) -> tuple:
    """(bar_color, hatch) for a stats dict."""
    if s is None:
        return _MISSING_COLOR, None
    if s.get("status") == "PARTIAL":
        return engine_color, _PARTIAL_HATCH
    return engine_color, None


def _draw_legend(fig):
    handles = [
        Patch(facecolor="lightsteelblue", edgecolor="black", label="SUCCESS"),
        Patch(facecolor="lightsteelblue", edgecolor="black",
              hatch=_PARTIAL_HATCH, label="PARTIAL (some DNF/ERROR)"),
        Patch(facecolor="white", edgecolor=_FAILED_COLOR,
              label="FAILED (no OK runs)"),
        Patch(facecolor=_MISSING_COLOR, edgecolor="black", label="N/A (not run)"),
    ]
    fig.legend(handles=handles, loc="lower center", ncol=4,
               frameon=False, fontsize=8, bbox_to_anchor=(0.5, 0.0))


def generate_metric_plot(bench: str, variants_data: dict, stats: dict,
                         engine_color: dict, metric_name: str,
                         out_path: Path) -> None:
    variant_names = sorted(variants_data.keys(), key=_natural_key)
    n = len(variant_names)
    if n == 0:
        print(f"  Skipping {bench} ({metric_name}): no data")
        return
    cols = min(n, 4)
    rows = math.ceil(n / cols)

    fig = plt.figure(figsize=(5 * cols, 4 * rows))
    gs = fig.add_gridspec(rows, cols)
    broken_axes = []

    for idx, variant in enumerate(variant_names):
        r, c = divmod(idx, cols)
        engines = sorted(variants_data[variant].keys(), key=_natural_key)
        subplot_stats = stats[bench][variant]

        means, stds, colors, hatches = [], [], [], []
        for engine in engines:
            s = subplot_stats[engine]
            color, hatch = _resolve_style(s, engine_color[engine])
            colors.append(color)
            hatches.append(hatch)
            is_failed = (s is not None and s.get("status") == "FAILED")
            if s is None or s.get("mean") is None or is_failed:
                means.append(0.0)
                stds.append(0.0)
            else:
                means.append(s["mean"])
                stds.append(s.get("std") or 0.0)

        ylims = _broken_ylims(means, stds)

        if ylims is not None:
            ax_top, ax_bot = _draw_broken_subplot(
                fig, gs[r, c], engines, means, stds,
                colors, hatches, subplot_stats, variant, ylims,
            )
            broken_axes.append((ax_top, ax_bot))
        else:
            ax = fig.add_subplot(gs[r, c])
            bars = ax.bar(engines, means, yerr=stds, capsize=4,
                          color=colors, error_kw={"elinewidth": 1})
            for bar, hatch in zip(bars, hatches):
                if hatch:
                    bar.set_hatch(hatch)
                    bar.set_edgecolor("black")
                    bar.set_linewidth(0.5)
            ax.set_title(variant)
            ax.set_ylabel("Time (ms)")
            ax.tick_params(axis="x", rotation=45)
            y_top = max(m + s for m, s in zip(means, stds)) if any(m > 0 for m in means) else 1.0
            ax.set_ylim(bottom=0)
            _annotate_flat(ax, engines, subplot_stats, y_top)

    for j in range(n, rows * cols):
        r, c = divmod(j, cols)
        fig.add_subplot(gs[r, c]).set_visible(False)

    metric_label = "Data loading" if metric_name == "load" else "Validation"
    fig.suptitle(f"Benchmark: {bench.upper()} — {metric_label} time")
    fig.tight_layout(rect=[0, 0.04, 1, 0.96])

    for ax_top, ax_bot in broken_axes:
        _add_right_border(fig, ax_top, ax_bot)

    _draw_legend(fig)

    print(f"  Saving {out_path}")
    plt.savefig(out_path, dpi=150)
    plt.close()


def generate_plots(data: dict, stats: dict, out_dir: Path) -> None:
    print(f"Generating plots for {len(data)} benchmark(s)...")

    all_engines: set[str] = set()
    for variants in data.values():
        for engines in variants.values():
            all_engines.update(engines.keys())

    engine_color = {
        e: _PALETTE[i % len(_PALETTE)]
        for i, e in enumerate(sorted(all_engines, key=_natural_key))
    }

    for bench, variants in data.items():
        for metric_name, _, _, _ in _METRICS:
            out_path = out_dir / f"{bench}-{metric_name}.png"
            generate_metric_plot(bench, variants, stats[metric_name],
                                 engine_color, metric_name, out_path)


def generate_report(stats: dict, out_dir: Path) -> None:
    out_path = out_dir / "report.json"
    print(f"Generating report at {out_path}")
    with open(out_path, "w") as f:
        json.dump(stats, f, indent=2)


def main() -> None:
    print(f"Discovering data in {DIST_DIR}")
    data = discover_data(DIST_DIR)
    print(f"Found {len(data)} benchmark(s): {', '.join(sorted(data))}")
    print("Computing statistics...")

    stats: dict = {metric: {} for metric, _, _, _ in _METRICS}
    for metric_name, _, _, _ in _METRICS:
        for bench, variants in data.items():
            stats[metric_name][bench] = {
                variant: {engine: compute_stats(path, metric_name)
                          for engine, path in engines.items()}
                for variant, engines in variants.items()
            }

    generate_plots(data, stats, OUT_DIR)
    generate_report(stats, OUT_DIR)
    print("Done.")


if __name__ == "__main__":
    main()
