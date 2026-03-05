mod common;

use crate::common::criterion_config;
use crate::common::{RdfFormat, Rudof, ShaclFormat};
use crate::common::{RUDOF_NEW_ID, RUDOF_OLD_ID};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::hint::black_box;

fn bench_icdd_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ICDD Validation");
    static IDX: [&'static str; 4] = [ "1", "2", "3", "4" ];
    static TYPES: [&'static str; 3] = ["binary", "directed1ton", "directedbinary"];
    static BASE_PATH: &'static str = "data/dist/icdd";

    for t in TYPES {
        let shape_file_path = format!("{BASE_PATH}/shapes-{t}.ttl");
        for i in IDX {
            let data_file_name = format!("data-{t}-{i}.ttl");
            let data_file_path = format!("{BASE_PATH}/{data_file_name}");


            group.bench_function(BenchmarkId::new(RUDOF_OLD_ID, data_file_name.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = Rudof::default_old();
                    rudof.load_data(data_file_path.as_str(), &RdfFormat::Old(rudof_old::RDFFormat::Turtle));
                    rudof.load_shapes(shape_file_path.as_str(), &ShaclFormat::Old(rudof_old::ShaclFormat::Turtle));

                    let Rudof::Old(out) = rudof else { unreachable!() };
                    out
                }, |mut rudof| {
                    black_box(rudof.validate_shacl(black_box(&rudof_old::ShaclValidationMode::Native), black_box(&rudof_old::ShapesGraphSource::CurrentSchema)))
                }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RUDOF_NEW_ID, data_file_name.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = Rudof::default_new();
                    rudof.load_data(data_file_path.as_str(), &RdfFormat::New(rudof_new::RDFFormat::Turtle));
                    rudof.load_shapes(shape_file_path.as_str(), &ShaclFormat::New(rudof_new::ShaclFormat::Turtle));

                    let Rudof::New(out) = rudof else { unreachable!() };
                    out
                }, |mut rudof| {
                    black_box(rudof.validate_shacl(black_box(Some(&rudof_new::ShaclValidationMode::Native)), black_box(Some(&rudof_new::ShapesGraphSource::CurrentSchema))))
                }, BatchSize::PerIteration);
            });
        }
    }
}

criterion_group!(name = icdd_benches; config = criterion_config(); targets = bench_icdd_validation);
criterion_main!(icdd_benches);