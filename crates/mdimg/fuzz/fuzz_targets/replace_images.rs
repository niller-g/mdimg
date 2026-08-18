#![no_main]
use libfuzzer_sys::fuzz_target;
use mdimg::Options;
use mdimg::pulldown_cmark;

fuzz_target!(|input: (bool, u32, &str)| {
    let (html, markdown, text) = input;
    mdimg::map_images_with(
        text,
        Options { html, markdown: pulldown_cmark::Options::from_bits_truncate(markdown) },
        |image| format!("{:?}", image.url()),
    );
});
