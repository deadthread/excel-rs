use std::fs::File;

use excel_rs::WorkBook;

#[test]
fn main_test() {
    let file = File::open("tests/res/empty.xlsx").unwrap();
    let _wb = WorkBook::from_seek_reader(file);
}