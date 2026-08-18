use mdimg::{Image, Options, map_images_with};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn map_fn(image: Image<'_>) -> String {
    format!("{image:?}")
}

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

fn iter_options() -> impl Iterator<Item = (String, Options)> {
    let markdown =
        [("none".to_owned(), pulldown_cmark::Options::empty())]
            .into_iter()
            .chain(pulldown_cmark::Options::all().iter_names().map(|(name, markdown)| {
                (name.trim_start_matches("ENABLE_").to_lowercase(), markdown)
            }))
            .chain([("all".to_owned(), pulldown_cmark::Options::all())]);

    markdown.flat_map(|(name, markdown)| {
        [true, false].map(move |html| {
            let name = if html { name.clone() } else { format!("{name}_no_html") };
            (name, Options { markdown, html })
        })
    })
}

fn check(expected_name: &str, options: Options) {
    // unsafe {
    //     env::set_var("UPDATE_EXPECT", "1");
    // }

    let fixtures_dir = Path::new(CRATE_ROOT).join("tests/test_data/fixtures");
    let expected_dir = Path::new(CRATE_ROOT).join("tests/test_data/expected").join(expected_name);

    let update = env::var_os("UPDATE_EXPECT").is_some();
    let fixture_paths = walk_md_files(&fixtures_dir);
    assert!(!fixture_paths.is_empty(), "No fixtures found in {:?}", fixtures_dir);

    for path in &fixture_paths {
        let relative = path.strip_prefix(&fixtures_dir).unwrap();
        let fixture = fs::read_to_string(path).unwrap();
        let expected_path = expected_dir.join(relative);
        let result = map_images_with(&fixture, options, map_fn);

        if update && fs::read_to_string(&expected_path).ok().as_deref() != Some(result.as_str()) {
            fs::create_dir_all(expected_path.parent().unwrap()).unwrap();
            fs::write(&expected_path, &result).unwrap();
        }
        let expected = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("Missing expected output: {:?}", expected_path));
        assert_eq!(result, expected, "Failed for expected output: {:?}", expected_path);
    }

    for path in walk_md_files(&expected_dir) {
        let relative = path.strip_prefix(&expected_dir).unwrap();
        let fixture_path = fixtures_dir.join(relative);
        assert!(fixture_path.exists(), "Missing fixture for expected output: {:?}", path);
    }
}

#[test]
fn raw_is_identity() {
    let fixtures_dir = Path::new(CRATE_ROOT).join("tests/test_data/fixtures");
    for path in walk_md_files(&fixtures_dir) {
        let fixture = fs::read_to_string(&path).unwrap();
        for (name, options) in iter_options() {
            let result = map_images_with(&fixture, options, |image| {
                assert_eq!(image.raw, &fixture[image.span.clone()]);
                image.raw
            });
            assert_eq!(result, fixture, "Failed for fixture {:?} with options {}", path, name);
        }
    }
}

#[test]
fn check_fixtures() {
    let mut names = Vec::new();
    for (name, options) in iter_options() {
        check(&name, options);
        names.push(name);
    }

    let expected_root = Path::new(CRATE_ROOT).join("tests/test_data/expected");
    for entry in fs::read_dir(&expected_root).unwrap().map(|entry| entry.unwrap()) {
        let name = entry.file_name().into_string().unwrap();
        assert!(names.contains(&name), "Unexpected snapshot directory: {:?}", entry.path());
    }
}
