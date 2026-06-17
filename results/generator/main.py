from pathlib import Path
import re
import json
import math
import statistics

import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.gridspec import GridSpecFromSubplotSpec
from matplotlib.lines import Line2D

DIST_DIR = Path(__file__).parent.parent / "dist"
OUT_DIR = DIST_DIR

_FILENAME_RE = re.compile(r"^(?P<engine>\w+)-(?P<variant>.+)\.csv$")
_MISSING_COLOR = "lightgray"
_PALETTE = plt.colormaps["tab10"].colors
_OUTLIER_THRESHOLD = 5
_BREAK_D = 0.015


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


def _draw_broken_subplot(
    fig, subplot_spec, engines, means, stds, colors, subplot_stats, variant, ylims
):
    """Draw a broken y-axis subplot. Returns (ax_top, ax_bot) for right-border stitching."""
    (_, lower_top), (upper_bot, upper_top) = ylims
    height_ratios = [1, 2]

    sub_gs = GridSpecFromSubplotSpec(
        2, 1, subplot_spec=subplot_spec,
        height_ratios=height_ratios,
        hspace=0.05,
    )
    ax_top = fig.add_subplot(sub_gs[0])
    ax_bot = fig.add_subplot(sub_gs[1])

    bar_kwargs = dict(yerr=stds, capsize=4, color=colors, error_kw={"elinewidth": 1})
    ax_top.bar(engines, means, **bar_kwargs)
    ax_bot.bar(engines, means, **bar_kwargs)

    ax_top.set_ylim(upper_bot, upper_top)
    ax_bot.set_ylim(0, lower_top)

    ax_top.spines["bottom"].set_visible(False)
    ax_bot.spines["top"].set_visible(False)
    # Right spines are hidden here; a single continuous line is drawn after tight_layout
    ax_top.spines["right"].set_visible(False)
    ax_bot.spines["right"].set_visible(False)
    ax_top.tick_params(bottom=False, right=False)
    ax_bot.tick_params(top=False, right=False)

    # x-tick labels only on bottom section
    ax_top.set_xticklabels([])
    ax_top.tick_params(axis="x", length=0)
    ax_bot.tick_params(axis="x", rotation=45)

    # Break marks on the LEFT (y-axis) side only.
    # d_y is scaled inversely by height ratio so both marks have equal
    # physical size and therefore appear at the same angle (parallel).
    d_x = _BREAK_D
    d_y_bot = _BREAK_D
    d_y_top = _BREAK_D * (height_ratios[1] / height_ratios[0])  # = 2 × d_y_bot

    mk = dict(color="k", clip_on=False, linewidth=1)
    ax_top.plot((-d_x, +d_x), (-d_y_top, +d_y_top), transform=ax_top.transAxes, **mk)
    ax_bot.plot((-d_x, +d_x), (1 - d_y_bot, 1 + d_y_bot), transform=ax_bot.transAxes, **mk)

    ax_top.set_title(variant)
    ax_bot.set_ylabel("Time (ms)")

    for i, engine in enumerate(engines):
        if subplot_stats[engine] is None:
            ax_bot.text(i, 0.5, "N/A", ha="center", va="bottom", fontsize=8, color="gray")

    return ax_top, ax_bot


def _add_right_border(fig, ax_top, ax_bot):
    """Draw a single continuous right border spanning both broken-axis sections."""
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
            if not f.is_file() or f.suffix != ".csv":
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


def compute_stats(path: Path | None) -> dict | None:
    if path is None:
        return None
    series = pd.read_csv(path, header=None).iloc[:, 0]
    return {
        "mean": round(float(series.mean()), 3),
        "std": round(float(series.std()), 3),
        "min": round(float(series.min()), 3),
        "max": round(float(series.max()), 3),
        "median": round(float(series.median()), 3),
        "count": int(series.count()),
    }


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
        variant_names = sorted(variants.keys(), key=_natural_key)
        n = len(variant_names)
        cols = min(n, 4)
        rows = math.ceil(n / cols)

        fig = plt.figure(figsize=(5 * cols, 4 * rows))
        gs = fig.add_gridspec(rows, cols)
        broken_axes = []

        for idx, variant in enumerate(variant_names):
            r, c = divmod(idx, cols)
            engines = sorted(variants[variant].keys(), key=_natural_key)
            subplot_stats = stats[bench][variant]

            means, stds, colors = [], [], []
            for engine in engines:
                s = subplot_stats[engine]
                if s is None:
                    means.append(0.0)
                    stds.append(0.0)
                    colors.append(_MISSING_COLOR)
                else:
                    means.append(s["mean"])
                    stds.append(s["std"])
                    colors.append(engine_color[engine])

            ylims = _broken_ylims(means, stds)

            if ylims is not None:
                ax_top, ax_bot = _draw_broken_subplot(
                    fig, gs[r, c], engines, means, stds,
                    colors, subplot_stats, variant, ylims,
                )
                broken_axes.append((ax_top, ax_bot))
            else:
                ax = fig.add_subplot(gs[r, c])
                ax.bar(
                    engines, means, yerr=stds, capsize=4,
                    color=colors, error_kw={"elinewidth": 1},
                )
                ax.set_title(variant)
                ax.set_ylabel("Time (ms)")
                ax.tick_params(axis="x", rotation=45)
                for i, engine in enumerate(engines):
                    if subplot_stats[engine] is None:
                        ax.text(
                            i, 0.5, "N/A", ha="center", va="bottom", fontsize=8, color="gray"
                        )

        for j in range(n, rows * cols):
            r, c = divmod(j, cols)
            fig.add_subplot(gs[r, c]).set_visible(False)

        out_path = out_dir / f"{bench}.png"
        fig.suptitle(f"Benchmark: {bench.upper()}")
        fig.tight_layout(rect=[0, 0, 1, 0.96])

        # Right borders must be drawn after tight_layout so axis positions are final
        for ax_top, ax_bot in broken_axes:
            _add_right_border(fig, ax_top, ax_bot)

        print(f"  Saving {out_path}")
        plt.savefig(out_path, dpi=150)
        plt.close()


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
    stats = {
        bench: {
            variant: {engine: compute_stats(path) for engine, path in engines.items()}
            for variant, engines in variants.items()
        }
        for bench, variants in data.items()
    }
    generate_plots(data, stats, OUT_DIR)
    generate_report(stats, OUT_DIR)
    print("Done.")


if __name__ == "__main__":
    main()
