use criterion::{Criterion, criterion_group, criterion_main};
use mdimg::{Options, map_images_with};
use std::hint::black_box;
use std::{
    fs,
    path::{Path, PathBuf},
};

fn walk_md_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).unwrap().flatten() {
            let path = entry.path();
            if entry.file_name().to_string_lossy().starts_with('.') {
                continue;
            }
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("md")) {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

fn corpus() -> String {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/test_data/fixtures");
    let mut text = String::new();
    for path in walk_md_files(&dir) {
        text.push_str(&fs::read_to_string(path).unwrap());
        text.push_str("\n\n");
    }
    text
}

fn bench_map_images(c: &mut Criterion) {
    let text = corpus();
    c.bench_function("map_images", |b| {
        b.iter(|| {
            map_images_with(black_box(&text), Options::default(), |image| {
                image.url().map_or(image.raw, |_| "X")
            })
        })
    });
}

criterion_group!(benches, bench_map_images);
criterion_main!(benches);
