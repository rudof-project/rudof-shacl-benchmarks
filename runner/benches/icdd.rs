mod common;

use crate::common::criterion_config;
use ::common::{RdfFormat, RudofEngine};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rudof_qlever::RudofQleverEngine;
use rudof_v1::RudofV1Engine;
use rudof_v2::RudofV2Engine;
use std::hint::black_box;

fn icdd_bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ICDD Validation");

    static IDX: [&'static str; 4] = [ "1", "2", "3", "4" ];
    static TYPES: [&'static str; 3] = [ "binary", "directed1ton", "directedbinary" ];
    static BASE_PATH: &'static str = "data/dist/icdd";
    static DATA_FORMAT: &'static RdfFormat = &RdfFormat::Turtle;
    static SHAPES_FORMAT: &'static RdfFormat = &RdfFormat::Turtle;

    for t in TYPES {
        let shape_file_path = format!("{BASE_PATH}/shapes-{t}.ttl");

        for i in IDX {
            let data_file_name = format!("data-{t}-{i}.ttl");
            let data_file_path = format!("{BASE_PATH}/{data_file_name}");

            group.bench_function(BenchmarkId::new(RudofV1Engine::DISPLAY_VERSION, data_file_name.as_str()), |b| {
               b.iter_batched(|| {
                   let mut rudof = RudofV1Engine::new();

                   rudof.load_data(data_file_path.clone(), *DATA_FORMAT);
                   rudof.load_shapes(shape_file_path.clone(), *SHAPES_FORMAT);

                   rudof
               }, |mut rudof| {
                   black_box(rudof.validate())
               }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RudofV2Engine::DISPLAY_VERSION, data_file_name.as_str()), |b| {
               b.iter_batched(|| {
                   let mut rudof = RudofV2Engine::new();

                   rudof.load_data(data_file_path.clone(), *DATA_FORMAT);
                   rudof.load_shapes(shape_file_path.clone(), *SHAPES_FORMAT);

                   rudof
               }, |mut rudof| {
                   black_box(rudof.validate())
               }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RudofQleverEngine::DISPLAY_VERSION, data_file_name.as_str()), |b| {
               b.iter_batched(|| {
                   let mut rudof = RudofQleverEngine::new();

                   rudof.load_data(data_file_path.clone(), *DATA_FORMAT);
                   rudof.load_shapes(shape_file_path.clone(), *SHAPES_FORMAT);

                   rudof
               }, |mut rudof| {
                   black_box(rudof.validate())
               }, BatchSize::PerIteration);
            });
        }
    }
}

criterion_group!(name = icdd_benches; config = criterion_config(); targets = icdd_bench_validation);
criterion_main!(icdd_benches);
