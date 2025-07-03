use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use softpath::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn path_operations(c: &mut Criterion) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let counter = AtomicUsize::new(0);

    c.bench_function("path_creation", |b| {
        b.iter(|| {
            let path: &str = black_box("~/test/file.txt");
            path.into_path().expect("Failed to convert path")
        })
    });

    c.bench_function("file_creation", |b| {
        b.iter_batched(
            || {
                // Setup: Create a unique file path for each iteration
                let id = counter.fetch_add(1, Ordering::Relaxed);
                temp_dir.path().join(format!("test_file_{}.txt", id))
            },
            |path| {
                // Benchmark: Only time the file creation
                path.create_file().expect("Failed to create file");
                // Immediate cleanup to prevent disk space issues
                std::fs::remove_file(&path).ok();
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("write_string", |b| {
        b.iter_batched(
            || {
                // Setup: Create a unique file path and pre-create the file
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("write_test_{}.txt", id));
                path.create_file()
                    .expect("Failed to create file for write test");
                path
            },
            |path| {
                // Benchmark: Only time the write operation
                path.write_string(black_box("test content"))
                    .expect("Failed to write to file");
                // Cleanup immediately to prevent disk space issues
                std::fs::remove_file(&path).ok();
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("read_string", |b| {
        b.iter_batched(
            || {
                // Setup: Create a file with content for each iteration
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("read_test_{}.txt", id));
                path.write_string("test content for reading")
                    .expect("Failed to setup read test file");
                path
            },
            |path| {
                // Benchmark: Only time the read operation
                let result = path.read_to_string().expect("Failed to read file");
                // Cleanup immediately after read
                std::fs::remove_file(&path).ok();
                result
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, path_operations);
criterion_main!(benches);
