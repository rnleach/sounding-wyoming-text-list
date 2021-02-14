use sounding_wyoming_text_list::parse_text;
use std::{fs::File, io::Read, path::Path};

const EXAMPLE_DIR: &str = "example_data";

#[test]
fn test_counts() {
    let example_dir = Path::new(EXAMPLE_DIR);

    example_dir
        .read_dir()
        .unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| path.is_file() && path.to_str().unwrap().ends_with(".html"))
        .map(|path| {
            let mut contents = String::new();

            let mut f = File::open(&path).expect("Error opening file");

            f.read_to_string(&mut contents).expect("Error reading file");

            (path.to_string_lossy().to_string(), contents)
        })
        .for_each(|(source, text_data)| {
            println!("Checking: {}", &source);

            let count = parse_text(&source, &text_data).count();
            if source.find("otx").is_some() {
                assert_eq!(count, 1);
            } else if source.find("tfx").is_some() {
                assert_eq!(count, 20);
            } else if source.find("OUN").is_some() {
                assert_eq!(count, 12);
            } else {
                panic!("Unknown test file.");
            }
        });
}
