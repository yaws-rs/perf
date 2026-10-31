use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark_decode(c: &mut Criterion) {
    c.bench_function("OpenSSL-yTLS TeaPots go Brrr", |b| {
        b.iter(|| {
            yaws_perf::ab_tls_driver::teapot(yaws_perf::ab_ytls::init_server(), black_box(None));
        })
    });

    c.bench_function("OpenSSL-RusTLS TeaPots go Brrr", |b| {
        b.iter(|| {
            yaws_perf::ab_tls_driver::teapot(yaws_perf::ab_rustls::init_server(), black_box(None));
        })
    });

    c.bench_function("Speed of Rick Rolls", |b| {
        b.iter(|| {
            yaws_perf::ab_h11spec_driver::h11rick(
                yaws_perf::ab_h11spec::init_server(),
                black_box(None),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark_decode);

criterion_main!(benches);
