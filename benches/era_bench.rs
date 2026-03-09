mod common;

use std::hint::black_box;

use crate::common::{criterion_config, RUDOF_NEW_ID, RUDOF_OLD_ID, Rudof, RdfFormat, ShaclFormat};
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

fn bench_era_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ERA Validation");
    // static DATA: [&'static str; 3] = ["era", "es", "fr"];
    static DATA: [&'static str; 1] = ["es"];
    static SHAPES: [&'static str; 3] = ["core", "era", "tds"];
    static BASE_PATH: &'static str = "data/dist/era";

    for d in DATA {
        let data_file = format!("{d}-data.ttl");
        let data_path = format!("{BASE_PATH}/{data_file}");
        for s in SHAPES {
            let shapes_file = format!("{s}-shapes.ttl");
            let shapes_path = format!("{BASE_PATH}/{shapes_file}");
            let variant = format!("{data_file} ({shapes_file})");

            group.bench_function(BenchmarkId::new(RUDOF_OLD_ID, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = Rudof::default_old();
                    rudof.load_data(data_path.as_str(), &RdfFormat::Old(rudof_old::RDFFormat::Turtle));
                    rudof.load_shapes(shapes_path.as_str(), &ShaclFormat::Old(rudof_old::ShaclFormat::Turtle));

                    let Rudof::Old(rudof) = rudof else { unreachable!() };
                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate_shacl(black_box(&rudof_old::ShaclValidationMode::Native), black_box(&rudof_old::ShapesGraphSource::CurrentSchema)))
                }, BatchSize::PerIteration);
            });

            group.bench_function(BenchmarkId::new(RUDOF_NEW_ID, variant.as_str()), |b| {
                b.iter_batched(|| {
                    let mut rudof = Rudof::default_new();
                    rudof.load_data(data_path.as_str(), &RdfFormat::New(rudof_new::RDFFormat::Turtle));
                    rudof.load_shapes(data_path.as_str(), &ShaclFormat::New(rudof_new::ShaclFormat::Turtle));

                    let Rudof::New(rudof) = rudof else { unreachable!() };
                    rudof
                }, |mut rudof| {
                    black_box(rudof.validate_shacl(black_box(Some(&rudof_new::ShaclValidationMode::Native)), black_box(Some(&rudof_new::ShapesGraphSource::CurrentSchema))))
                }, BatchSize::PerIteration);
            });
        }
    }
}

criterion_group!(name = era_benches; config = criterion_config(); targets = bench_era_validation);
criterion_main!(era_benches);
