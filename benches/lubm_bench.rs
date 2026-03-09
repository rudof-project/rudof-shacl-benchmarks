mod common;

use std::hint::black_box;

use crate::common::{RUDOF_NEW_ID, RUDOF_OLD_ID, RdfFormat, Rudof, criterion_config, ShaclFormat};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};


fn bench_lubm_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("LUBM Validation");
    static SIZES: [&'static str; 5] = [ "5", "10", "50", "100", "500" ];
    static BASE_PATH: &'static str = "data/dist/lubm";
    static SHAPES_PATH: &'static str = "data/dist/lubm/shapes.ttl";

    for s in SIZES {
        let data_file = format!("data-{s}.nt");
        let data_path = format!("{BASE_PATH}/{data_file}");

        group.bench_function(BenchmarkId::new(RUDOF_OLD_ID, data_file.as_str()), |b| {
            b.iter_batched(|| {
                let mut rudof = Rudof::default_old();
                rudof.load_data(data_path.as_str(), &RdfFormat::Old(rudof_old::RDFFormat::NTriples));
                rudof.load_shapes(SHAPES_PATH, &ShaclFormat::Old(rudof_old::ShaclFormat::Turtle));

                let Rudof::Old(rudof) = rudof else { unreachable!() };
                rudof
            }, |mut rudof| {
                black_box(rudof.validate_shacl(black_box(&rudof_old::ShaclValidationMode::Native), black_box(&rudof_old::ShapesGraphSource::CurrentSchema)))
            }, BatchSize::PerIteration);
        });

        group.bench_function(BenchmarkId::new(RUDOF_NEW_ID, data_file.as_str()), |b| {
            b.iter_batched(|| {
                let mut rudof = Rudof::default_new();
                rudof.load_data(data_path.as_str(), &RdfFormat::New(rudof_new::RDFFormat::NTriples));
                rudof.load_shapes(SHAPES_PATH, &ShaclFormat::New(rudof_new::ShaclFormat::Turtle));

                let Rudof::New(rudof) = rudof else { unreachable!() };
                rudof
            }, |mut rudof| {
                black_box(rudof.validate_shacl(black_box(Some(&rudof_new::ShaclValidationMode::Native)), black_box(Some(&rudof_new::ShapesGraphSource::CurrentSchema))))
            }, BatchSize::PerIteration);
        });
    }
}

criterion_group!(name = lubm_benches; config = criterion_config(); targets = bench_lubm_validation);
criterion_main!(lubm_benches);
