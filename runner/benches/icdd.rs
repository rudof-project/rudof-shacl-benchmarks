mod common;

use crate::common::{criterion_config, load_config};
use ::common::RudofEngine;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rudof_v1::RudofV1Engine;
use rudof_v2::{RudofQleverEngine, RudofV2Engine};
use std::hint::black_box;

fn icdd_bench_validation(c: &mut Criterion) {
    let cfg = load_config().icdd;
    if cfg.disabled { return }

    let mut group = c.benchmark_group("ICDD Validation");

    for t in &cfg.types {
        let shape_file_path = format!("{}/shapes-{t}.ttl", cfg.path);

        for i in &cfg.sizes {
            let data_file_name = format!("data-{t}-{i}.ttl");
            let data_file_path = format!("{}/{data_file_name}", cfg.path);

            if cfg.engines.contains(&RudofV1Engine::ID.to_string()) {
                group.bench_function(BenchmarkId::new(RudofV1Engine::DISPLAY_VERSION, data_file_name.as_str()), |b| {
                   b.iter_batched(|| {
                       let mut rudof = RudofV1Engine::new();

                       rudof.load_data(data_file_path.clone(), cfg.data_format);
                       rudof.load_shapes(shape_file_path.clone(), cfg.shapes_format);

                       rudof
                   }, |mut rudof| {
                       black_box(rudof.validate())
                   }, BatchSize::PerIteration);
                });
            }

            if cfg.engines.contains(&RudofV2Engine::ID.to_string()) {
                group.bench_function(BenchmarkId::new(RudofV2Engine::DISPLAY_VERSION, data_file_name.as_str()), |b| {
                   b.iter_batched(|| {
                       let mut rudof = RudofV2Engine::new();

                       rudof.load_data(data_file_path.clone(), cfg.data_format);
                       rudof.load_shapes(shape_file_path.clone(), cfg.shapes_format);

                       rudof
                   }, |mut rudof| {
                       black_box(rudof.validate())
                   }, BatchSize::PerIteration);
                });
            }

            if cfg.engines.contains(&RudofQleverEngine::ID.to_string()) {
                let mut rudof = RudofQleverEngine::new();
                rudof.load_data(data_file_path.clone(), cfg.data_format);
                rudof.load_shapes(shape_file_path.clone(), cfg.shapes_format);

                group.bench_function(BenchmarkId::new(RudofQleverEngine::DISPLAY_VERSION, data_file_name.as_str()), |b| {
                    b.iter(|| { black_box(rudof.validate()) });
                });
            }
        }
    }
}

criterion_group!(name = icdd_benches; config = criterion_config(); targets = icdd_bench_validation);
criterion_main!(icdd_benches);
