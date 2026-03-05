use crate::common::FlamegraphProfiler;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::fs::File;
use std::hint::black_box;

mod common;

pub enum Rudof {
    Old(rudof_old::Rudof),
    New(rudof_new::Rudof),
}

fn load_data(data_path: &str, rudof: &mut Rudof) {
    let mut reader = File::open(data_path).expect("Failed to open data file");

    match rudof {
        Rudof::Old(rudof) => rudof.read_data(&mut reader, "Bench", &rudof_old::RDFFormat::Turtle, None, &rudof_old::ReaderMode::Strict, false).unwrap(),
        Rudof::New(rudof) => rudof.read_data(&mut reader, "Bench", Some(&rudof_new::RDFFormat::Turtle), None, Some(&rudof_new::ReaderMode::Strict), Some(false)).unwrap(),
    };
}

fn load_shapes(shapes_path: &str, rudof: &mut Rudof) {
    let mut reader = File::open(shapes_path).expect("Filed to open shapes file");

    match rudof {
        Rudof::Old(rudof) => rudof.read_shacl(&mut reader, "Bench", &rudof_old::ShaclFormat::Turtle, None, &rudof_old::ReaderMode::Strict).unwrap(),
        Rudof::New(rudof) => rudof.read_shacl(&mut reader, "Bench", Some(&rudof_new::ShaclFormat::Turtle), None, Some(&rudof_new::ReaderMode::Strict)).unwrap(),
    }
}

fn bench_icdd_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("ICDD Validation");

    group.bench_function(BenchmarkId::new("RUDOF 0.1", 1), |b| {
        b.iter_batched(|| {
            let rold_config = rudof_old::RudofConfig::default_config().unwrap();
            let mut rold = Rudof::Old(rudof_old::Rudof::new(&rold_config).unwrap());

            load_data("data/dist/icdd/data-binary-1.ttl", &mut rold);
            load_shapes("data/dist/icdd/shapes-binary.ttl", &mut rold);

            match rold {
                Rudof::Old(rudof) => rudof,
                Rudof::New(_) => unreachable!(),
            }
        }, |mut rudof| {
            black_box(rudof.validate_shacl(black_box(&rudof_old::ShaclValidationMode::Native), black_box(&rudof_old::ShapesGraphSource::CurrentSchema)))
        }, BatchSize::PerIteration);
    });

    group.bench_function(BenchmarkId::new("RUDOF 0.2", 2), |b| {
        b.iter_batched(|| {
            let rnew_config = rudof_new::RudofConfig::default_config().unwrap();
            let mut rnew = Rudof::New(rudof_new::Rudof::new(&rnew_config).unwrap());

            load_data("data/dist/icdd/data-binary-1.ttl", &mut rnew);
            load_shapes("data/dist/icdd/shapes-binary.ttl", &mut rnew);

            match rnew {
                Rudof::Old(_) => unreachable!(),
                Rudof::New(rudof) => rudof,
            }
        }, |mut rudof| {
            black_box(rudof.validate_shacl(black_box(Some(&rudof_new::ShaclValidationMode::Native)), black_box(Some(&rudof_new::ShapesGraphSource::CurrentSchema))))
        }, BatchSize::PerIteration);
    });


    group.finish();
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .with_profiler(FlamegraphProfiler::new(100))
}

criterion_group!(name = benches; config = criterion_config(); targets = bench_icdd_validation);
criterion_main!(benches);
