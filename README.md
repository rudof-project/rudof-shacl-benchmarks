<div align="center">
<h1>Rudof SHACL validator Benchmarks</h1>
</div>

---

# Usage instructions

## Clone the repository

In order to run the benchmarks, you need to clone the repository with recursive submodules:

```bash
git clone --recursive
```

>[!NOTE]
>If you have already cloned the repository without the `--recursive` flag, you can initialize the submodules with the following command:
>```bash
>task init
>```


## Datasets generation

The datasets used in the benchmarks can be generated with the following command:

```bash
task generate_data
```

>[!NOTE]
>Currently, the data generation process relies on bash scripts and requires a Unix-like environment.
>If you are using Windows, you can use the Windows Subsystem for Linux (WSL) to run them.
> 
>Also, the data generation process relies on in external dependencies, such as:
>- java 
>- ant 
>- gzip 
>- rapper 
>- sed 
>- curl 
>- xz 
>- rev 
>- cut

## Running the benchmarks

The benchmarks can be run with the following command:

```bash
task bench
```

The results can be accessed in the `target/criterion/report/index.html` file.

### Profiling data

In order to generate the flamegraphs for the benchmarks, you can run the following command:

```bash
cargo bench -- --profile-time <duration>
```

Where `<duration>` is the duration of the benchmark in seconds.

The flamegraphs can be accessed in the `target/criterion/<benchmark_name>/<benchmark_variant>/profile/flamegraph.svg` file.

---

>[!NOTE]
>In order to run the `task` commands, you need to have the [Taskfile](https://taskfile.dev) runner installed.