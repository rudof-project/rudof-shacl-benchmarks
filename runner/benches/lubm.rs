mod common;

use crate::common::{criterion_config, load_config};
use ::common::RudofEngine;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rudof_v1::RudofV1Engine;
use rudof_v2::{RudofQleverEngine, RudofV2Engine};
use std::hint::black_box;

fn lubm_bench_validation(c: &mut Criterion) {
    let cfg = load_config().lubm;
    if cfg.disabled { return }

    let mut group = c.benchmark_group("LUBM Validation");

    let shapes_path: String = format!("{}/shapes.ttl", cfg.path);

    for s in &cfg.sizes {
        let data_file = format!("data-{s}.nt");
        let data_path = format!("{}/{data_file}", cfg.path);

        if cfg.engines.contains(&RudofV1Engine::ID.to_string()) {
            group.bench_function(BenchmarkId::new(RudofV1Engine::DISPLAY_VERSION, data_file.as_str()), |b| {
               b.iter_batched(|| {
                   let mut rudof = RudofV1Engine::new();

                   rudof.load_data(data_path.clone(), cfg.data_format);
                   rudof.load_shapes(shapes_path.clone(), cfg.shapes_format);

                   rudof
               }, |mut rudof| {
                   black_box(rudof.validate())
               }, BatchSize::PerIteration);
            });
        }

        if cfg.engines.contains(&RudofV2Engine::ID.to_string()) {
            group.bench_function(BenchmarkId::new(RudofV2Engine::DISPLAY_VERSION, data_file.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofV2Engine::new();

                    rudof.load_data(data_path.clone(), cfg.data_format);
                    rudof.load_shapes(shapes_path.clone(), cfg.shapes_format);

                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate())
                }, BatchSize::PerIteration);
            });
        }

        if cfg.engines.contains(&RudofQleverEngine::ID.to_string()) {
            let mut rudof = RudofQleverEngine::new();
            rudof.load_data(data_path.clone(), cfg.data_format);
            rudof.load_shapes(shapes_path.clone(), cfg.shapes_format);

            group.bench_function(BenchmarkId::new(RudofQleverEngine::DISPLAY_VERSION, data_file.as_str()), |b| {
                b.iter(|| { black_box(rudof.validate()) });
            });
        }
    }
}

criterion_group!(name = lubm_benches; config = criterion_config(); targets = lubm_bench_validation);
criterion_main!(lubm_benches);
