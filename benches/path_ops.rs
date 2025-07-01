use criterion::{black_box, criterion_group, criterion_main, Criterion};
use softpath::prelude::*;

fn path_operations(c: &mut Criterion) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let test_path = temp_dir.path().join("test_file.txt");

    c.bench_function("path_creation", |b| {
        b.iter(|| {
            let path: &str = black_box("~/test/file.txt");
            path.into_path().expect("Failed to convert path")
        })
    });

    c.bench_function("file_creation", |b| {
        b.iter(|| {
            let path = black_box(&test_path);
            path.create_file().expect("Failed to create file")
        })
    });

    c.bench_function("write_string", |b| {
        b.iter(|| {
            let path = black_box(&test_path);
            path.write_string(black_box("test content")).expect("Failed to write to file")
        })
    });

    c.bench_function("read_string", |b| {
        b.iter(|| {
            let path = black_box(&test_path);
            path.read_to_string().expect("Failed to read file")
        })
    });
}

criterion_group!(benches, path_operations);
criterion_main!(benches);
