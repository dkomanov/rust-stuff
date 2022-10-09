use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main, Throughput};

use base64_bench::*;

pub fn bench_encode(c: &mut Criterion) {
    bench_encode_all_inputs(c, "base64_encode_config", |s| { base64_encode_config(s); });
    bench_encode_all_inputs(c, "crypto2_encode_config", |s| { crypto2_encode_config(s); });
}

pub fn bench_decode(c: &mut Criterion) {
    bench_decode_all_inputs(c, "base64_decode_config_slice", |s| { base64_decode_config_slice(s); });
    bench_decode_all_inputs(c, "base64_decode_config", |s| { base64_decode_config(s); });
    bench_decode_all_inputs(c, "crypto2_decode_config", |s| { crypto2_decode_config(s); });
}

pub fn bench_encode_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("encode");
    for td in get_all_test_data() {
        let size = td.size;
        let payload = td.get_payload();
        group.throughput(Throughput::Bytes(payload.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("base64", size),
            &payload,
            |b, s| b.iter(|| base64_encode_config(&s)),
        );
        group.bench_with_input(
            BenchmarkId::new("crypto2", size),
            &payload,
            |b, s| b.iter(|| crypto2_encode_config(&s)),
        );
    }
    group.finish();
}

pub fn bench_decode_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode");
    for td in get_all_test_data() {
        group.throughput(Throughput::Bytes(td.encoded.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("base64", td.size),
            &td.encoded,
            |b, s| b.iter(|| base64_decode_config(&s)),
        );
        group.bench_with_input(
            BenchmarkId::new("base64_slice", td.size),
            &td.encoded,
            |b, s| b.iter(|| base64_decode_config_slice(&s)),
        );
        group.bench_with_input(
            BenchmarkId::new("crypto2", td.size),
            &td.encoded,
            |b, s| b.iter(|| crypto2_decode_config(&s)),
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_encode,
    bench_encode_diff,
    bench_decode,
    bench_decode_diff,
);
criterion_main!(benches);

fn bench_decode_all_inputs<R>(c: &mut Criterion, name: &str, mut f: R)
    where
        R: FnMut(&String) -> (),
{
    let mut group = c.benchmark_group(name);
    for td in get_all_test_data() {
        group.throughput(Throughput::Bytes(td.encoded.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(td.size),
            &td.encoded,
            |b, s| {
                b.iter(|| f(&s))
            },
        );
    }
    group.finish();
}

fn bench_encode_all_inputs<R>(c: &mut Criterion, name: &str, mut f: R)
    where
        R: FnMut(&Vec<u8>) -> (),
{
    let mut group = c.benchmark_group(name);
    for td in get_all_test_data() {
        let size = td.size;
        let payload = td.get_payload();
        group.throughput(Throughput::Bytes(payload.len() as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &payload,
            |b, s| {
                b.iter(|| f(&s))
            },
        );
    }
    group.finish();
}
