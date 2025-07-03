use criterion::{black_box, criterion_group, criterion_main, Criterion};
use softpath::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

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
        b.iter_custom(|iters| {
            let mut total_time = Duration::new(0, 0);

            for _ in 0..iters {
                // Setup: Create a unique file path for each iteration
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("test_file_{}.txt", id));

                // Time only the file creation operation
                let start = Instant::now();
                path.create_file().expect("Failed to create file");
                total_time += start.elapsed();

                // Cleanup (not timed)
                std::fs::remove_file(&path).ok();
            }

            total_time
        });
    });

    c.bench_function("write_string", |b| {
        b.iter_custom(|iters| {
            let mut total_time = Duration::new(0, 0);

            for _ in 0..iters {
                // Setup: Create a unique file path and pre-create the file
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("write_test_{}.txt", id));
                path.create_file()
                    .expect("Failed to create file for write test");

                // Time only the write operation
                let start = Instant::now();
                path.write_string(black_box("test content"))
                    .expect("Failed to write to file");
                total_time += start.elapsed();

                // Cleanup (not timed)
                std::fs::remove_file(&path).ok();
            }

            total_time
        });
    });

    c.bench_function("read_string", |b| {
        b.iter_custom(|iters| {
            let mut total_time = Duration::new(0, 0);

            for _ in 0..iters {
                // Setup: Create a file with content for each iteration
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("read_test_{}.txt", id));
                path.write_string("test content for reading")
                    .expect("Failed to setup read test file");

                // Time only the read operation
                let start = Instant::now();
                let result = path.read_to_string().expect("Failed to read file");
                total_time += start.elapsed();

                // Prevent optimization of result
                black_box(result);

                // Cleanup (not timed)
                std::fs::remove_file(&path).ok();
            }

            total_time
        });
    });
}

criterion_group!(benches, path_operations);
criterion_main!(benches);
