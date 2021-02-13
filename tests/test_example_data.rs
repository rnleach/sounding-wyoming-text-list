use sounding_wyoming_text_list::parse_text;
use std::{path::Path, fs::File, io::Read};

const EXAMPLE_DIR: &str = "example_data";

#[test]
fn test_counts() {
    let example_dir = Path::new(EXAMPLE_DIR);

    example_dir
        .read_dir()
        .unwrap()
        .map(|res| res.unwrap().path())
        .filter(|path| path.is_file() && path.to_str().unwrap().ends_with(".html"))
        .map(|path|{
            let mut contents = String::new();

            let mut f = File::open(&path).expect("Error opening file");

            f.read_to_string(&mut contents).expect("Error reading file");

            (path.to_string_lossy().to_string(), contents)
        })
        .for_each(|(source, text_data)| {
            let iter = parse_text(&source, &text_data);
            println!("{} has {} soundings.", source, iter.count());
        });

}
