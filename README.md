<div align="center">
<h1>Rudof SHACL Validator Benchmarks</h1>
</div>

A benchmarking suite that compares multiple SHACL validation engines across different datasets. Also rudof versions can be measured directly via Criterion.

> [!NOTE]
> All task commands require [Taskfile](https://taskfile.dev) to be installed.

---

## Dependencies

### System

The following dependencies must be installed to run the benchmarks:

- `docker` and `docker-buildx` for building and running the Docker-based engines
- `cargo` for compiling and running the Criterion Rust benchmarks
- `go-task` a task runner to simplify all the operations

### Toolchain Docker images

Builder images must be present before building engine images. Build them once from the root folder:

```bash
task toolchain:docker:build
```

This produces: `rudof/rust_builder`, `rudof/kotlin_builder`, `rudof/python_builder`, `rudof/dotnet_builder` and `rudof/dataset_builder`.

### Python wheels

The `maplib` engine depends on a pre-built wheel that **is not bundled** in the repository. You must place the wheel manually inside the `maplib/wheels` folder.

The `pyproject.toml` in `engines/maplib/` pins the exact version. Update both the wheel and the `[tool.uv.sources]` entry if the version changes.

---

## Configuration

Criterion benchmark behaviour is controlled by a TOML config file. The path is set via the `RUDOF_BENCH_CFG` environment variable (default: `default_config.toml`).

### Config file reference

```toml
[era]
data   = ["es"]          # ERA data variants: "es", "fr", "era"
shapes = ["core"]        # ERA shape sets: "core", "tds", "era"
disabled = false         # set true to skip this dataset entirely
engines = ["rudof_v1"]   # engines to include in rudof Criterion runs

[lubm]
sizes    = ["5", "10", "50", "100", "500"]  # university counts
disabled = true
engines  = ["rudof_v1", "rudof_v2"]

[icdd]
sizes  = ["1", "2", "3", "4"]
types  = ["binary", "directed1ton", "directedbinary"]
disabled = false
engines  = ["rudof_v1", "rudof_v2"]
```

Also qlever can be configured with the `qlever_config.toml`.

To use a non-default config, set the environment variable before running:

```bash
RUDOF_BENCH_CFG=./low_resources.toml cargo bench --bench=era
```

### Taskfile variables (`taskfiles/vars.yml`)

The Docker-based engine runs are controlled by variables at the of `taskfiles/vars.yml`:

---

## Setup

### 1. Clone with submodules

```bash
git clone --recursive <repo-url>
```

If already cloned without `--recursive`:

```bash
git submodule update --init --recursive
```

### 2. Build toolchain images

```bash
task toolchain:docker:build
```

### 3. Generate datasets

```bash
task data:docker:generate
```

Generated files land in `data/dist/<dataset>/`.

### 4. Build engine images

Build all engines:

```bash
task engine:docker:build
```

> [!NOTE]
> For `maplib`, place the wheel in `engines/maplib/wheels/` before building its image.

---

## Running benchmarks

### Docker-based engines

Run all configured engines across all configured datasets:

```bash
task docker:engine:bench
```

### Criterion benchmarks (rudof versions)

Run inside Docker:

```bash
task docker:rudof:bench
```

### Profiling (flamegraphs)

```bash
task docker:rudof:prof
```

---

## Results

### Engine benchmark results

Raw CSV and Turtle report files are written to:

```
results/dist/<dataset>/
```

One CSV per engine/variant combination (e.g. `maplib-es-core.csv`), containing one timing in milliseconds per row (warm-up runs excluded). The Turtle file contains the last SHACL validation report.

### Criterion results (rudof versions)

HTML reports are generated at:

```
target/criterion/report/index.html
```

Flamegraph SVGs (from profiling runs) are at:

```
target/criterion/<benchmark>/<variant>/profile/flamegraph.svg
```

### Graphics

The `results/generator/` Python project reads CSVs from `results/dist/` and produces comparison charts:

```bash
tasks results:generate
```

Output PNGs and a `report.json` are written to `results/dist/`.

---

## Available engines

| Engine | Language | Docker image |
|--------|----------|-------------|
| `rudof_v1` | Rust | `rudof/rudof_v1` |
| `rudof_v2` | Rust | `rudof/rudof_v2` |
| `maplib` | Python | `rudof/maplib` |
| `pyshacl` | Python | `rudof/pyshacl` |
| `corese` | Java | `rudof/corese` |
| `jena` | Java | `rudof/jena` |
| `rdf4j` | Java | `rudof/rdf4j` |
| `rdfunit` | Java| `rudof/rdfunit` |
| `topbraid` | Java | `rudof/topbraid` |
