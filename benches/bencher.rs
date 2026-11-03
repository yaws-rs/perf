use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark_decode(c: &mut Criterion) {
    // Measures TLS: yTLS
    c.bench_function("OpenSSL-yTLS TeaPots go Brrr", |b| {
        b.iter(|| {
            yaws_perf::ab_tls_driver::teapot(yaws_perf::ab_ytls::init_server(), black_box(None));
        })
    });

    // Measures TLS: RusTLS
    c.bench_function("OpenSSL-RusTLS TeaPots go Brrr", |b| {
        b.iter(|| {
            yaws_perf::ab_tls_driver::teapot(yaws_perf::ab_rustls::init_server(), black_box(None));
        })
    });

    // Measures H11Spec + App
    c.bench_function("Speed of Plain Rick Rolls", |b| {
        b.iter(|| {
            yaws_perf::ab_h11spec_driver::h11rick(
                yaws_perf::ab_h11spec::init_server(),
                black_box(None),
            );
        })
    });

    // Measures yTLS + H11Spec + App
    c.bench_function("Speed of yTLS Rick Rolls", |b| {
        b.iter(|| {
            yaws_perf::ab_c_tls_h11spec_driver::c_rickroll(
                yaws_perf::ab_c_ytls_h11spec::init_composition(),
                black_box(None),
            );
        })
    });

    // Measures RusTLS + H11Spec + App
    c.bench_function("Speed of RusTLS Rick Rolls", |b| {
        b.iter(|| {
            yaws_perf::ab_c_tls_h11spec_driver::c_rickroll(
                yaws_perf::ab_c_rustls_h11spec::init_composition(),
                black_box(None),
            );
        })
    });
}

criterion_group!(benches, criterion_benchmark_decode);

criterion_main!(benches);
