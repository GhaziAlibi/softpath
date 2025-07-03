use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

fn stdlib_comparison(c: &mut Criterion) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let counter = AtomicUsize::new(0);

    // Standard library file creation
    c.bench_function("stdlib_file_create", |b| {
        b.iter_with_setup(
            || {
                let id = counter.fetch_add(1, Ordering::Relaxed);
                temp_dir.path().join(format!("std_{}.txt", id))
            },
            |path| {
                fs::File::create(&path).expect("Failed to create");
                fs::remove_file(&path).ok();
            },
        );
    });

    // Standard library write
    c.bench_function("stdlib_write", |b| {
        b.iter_with_setup(
            || {
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("write_{}.txt", id));
                fs::File::create(&path).expect("Failed to create");
                path
            },
            |path| {
                fs::write(&path, "test content").expect("Failed to write");
                fs::remove_file(&path).ok();
            },
        );
    });

    // Standard library read
    c.bench_function("stdlib_read", |b| {
        b.iter_with_setup(
            || {
                let id = counter.fetch_add(1, Ordering::Relaxed);
                let path = temp_dir.path().join(format!("read_{}.txt", id));
                fs::write(&path, "test content for reading").expect("Failed to setup");
                path
            },
            |path| {
                let _content = fs::read_to_string(&path).expect("Failed to read");
                fs::remove_file(&path).ok();
            },
        );
    });
}

criterion_group!(benches, stdlib_comparison);
criterion_main!(benches);
