mod common;

use crate::common::criterion_config;
use ::common::{RdfFormat, RudofEngine};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rudof_qlever::RudofQleverEngine;
use rudof_v1::RudofV1Engine;
use rudof_v2::RudofV2Engine;
use std::hint::black_box;

fn era_bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ERA Validation");

    // static DATA: [&'static str; 3] = [ "era", "es", "fr" ];
    static DATA: [&'static str; 2] = [ "es", "fr" ];
    static SHAPES: [&'static str; 3] = [ "core", "era", "tds" ];
    static BASE_PATH: &'static str = "data/dist/era";
    static DATA_FORMAT: &'static RdfFormat = &RdfFormat::Turtle;
    static SHAPES_FORMAT: &'static RdfFormat = &RdfFormat::Turtle;

    for d in DATA {
        let data_file = format!("{d}-data.ttl");
        let data_path = format!("{BASE_PATH}/{data_file}");

        for s in SHAPES {
            let shapes_file = format!("{s}-shapes.ttl");
            let shapes_path = format!("{BASE_PATH}/{shapes_file}");
            let variant = format!("{data_file} ({shapes_file})");

            group.bench_function(BenchmarkId::new(RudofV1Engine::DISPLAY_VERSION, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofV1Engine::new();

                    rudof.load_data(data_path.clone(), *DATA_FORMAT);
                    rudof.load_shapes(shapes_path.clone(), *SHAPES_FORMAT);

                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate())
                }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RudofV2Engine::DISPLAY_VERSION, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofV2Engine::new();

                    rudof.load_data(data_path.clone(), *DATA_FORMAT);
                    rudof.load_shapes(shapes_path.clone(), *SHAPES_FORMAT);

                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate())
                }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RudofQleverEngine::DISPLAY_VERSION, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = RudofQleverEngine::new();

                    rudof.load_data(data_path.clone(), *DATA_FORMAT);
                    rudof.load_shapes(shapes_path.clone(), *SHAPES_FORMAT);

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
