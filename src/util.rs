use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_file_input(path: &str) -> Vec<String> {
    let f = File::open(path).unwrap();
    let file = BufReader::new(&f);
    file.lines()
        .map(|l| l.unwrap().trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

#[test]
fn test_reading_file_input() {
    assert_eq!(2, read_file_input("resources/test.txt").len());
}
