mod common;

use crate::common::criterion_config;
use ::common::{RdfFormat, RudofEngine};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rudof_qlever::RudofQleverEngine;
use rudof_v1::RudofV1Engine;
use rudof_v2::RudofV2Engine;
use std::hint::black_box;

fn lubm_bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("LUBM Validation");

    static SIZES: [&'static str; 5] = [ "5", "10", "50", "100", "500" ];
    static BASE_PATH: &'static str = "data/dist/lubm";
    static SHAPES_PATH: &'static str = "data/dist/lubm/shapes.ttl";
    static DATA_FORMAT: &'static RdfFormat = &RdfFormat::NTriples;
    static SHAPES_FORMAT: &'static RdfFormat = &RdfFormat::Turtle;

    for s in SIZES {
        let data_file = format!("data-{s}.nt");
        let data_path = format!("{BASE_PATH}/{data_file}");

        group.bench_function(BenchmarkId::new(RudofV1Engine::DISPLAY_VERSION, data_file.as_str()), |b| {
           b.iter_batched(|| {
               let mut rudof = RudofV1Engine::new();

               rudof.load_data(data_path.clone(), *DATA_FORMAT);
               rudof.load_shapes(SHAPES_PATH, *SHAPES_FORMAT);

               rudof
           }, |mut rudof| {
               black_box(rudof.validate())
           }, BatchSize::PerIteration);
        });

        group.bench_function(BenchmarkId::new(RudofV2Engine::DISPLAY_VERSION, data_file.as_str()), |b| {
            b.iter_batched(|| {
                let mut rudof = RudofV2Engine::new();

                rudof.load_data(data_path.clone(), *DATA_FORMAT);
                rudof.load_shapes(SHAPES_PATH, *SHAPES_FORMAT);

                rudof
            }, |mut rudof| {
                black_box(rudof.validate())
            }, BatchSize::PerIteration);
        });

        group.bench_function(BenchmarkId::new(RudofQleverEngine::DISPLAY_VERSION, data_file.as_str()), |b| {
            b.iter_batched(|| {
                let mut rudof = RudofQleverEngine::new();

                rudof.load_data(data_path.clone(), *DATA_FORMAT);
                rudof.load_shapes(SHAPES_PATH, *SHAPES_FORMAT);

                rudof
            }, |mut rudof| {
                black_box(rudof.validate())
            }, BatchSize::PerIteration);
        });
    }
}

criterion_group!(name = lubm_benches; config = criterion_config(); targets = lubm_bench_validation);
criterion_main!(lubm_benches);
