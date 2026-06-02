mod common;

use crate::common::{criterion_config, load_config};
use ::common::RudofEngine;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rudof_qlever::RudofQleverEngine;
use rudof_v1::RudofV1Engine;
use rudof_v2::RudofV2Engine;
use std::hint::black_box;

fn era_bench_validation(c: &mut Criterion) {
    let cfg = load_config().era;
    let mut group = c.benchmark_group("ERA Validation");

    for d in &cfg.data {
        let data_file = format!("{d}-data.ttl");
        let data_path = format!("{}/{data_file}", cfg.path);

        for s in &cfg.shapes {
            let shapes_file = format!("{s}-shapes.ttl");
            let shapes_path = format!("{}/{shapes_file}", cfg.path);
            let variant = format!("{data_file} ({shapes_file})");

            group.bench_function(BenchmarkId::new(RudofV1Engine::DISPLAY_VERSION, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofV1Engine::new();

                    rudof.load_data(data_path.clone(), cfg.data_format);
                    rudof.load_shapes(shapes_path.clone(), cfg.shapes_format);

                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate())
                }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RudofV2Engine::DISPLAY_VERSION, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofV2Engine::new();

                    rudof.load_data(data_path.clone(), cfg.data_format);
                    rudof.load_shapes(shapes_path.clone(), cfg.shapes_format);

                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate())
                }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RudofQleverEngine::DISPLAY_VERSION, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofQleverEngine::new();

                    rudof.load_data(data_path.clone(), cfg.data_format);
                    rudof.load_shapes(shapes_path.clone(), cfg.shapes_format);

                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate())
                }, BatchSize::PerIteration);
            });
        }
    }
}

criterion_group!(name = era_benches; config = criterion_config(); targets = era_bench_validation);
criterion_main!(era_benches);
