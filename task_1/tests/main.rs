use std::io::Cursor;

use task_1::sum_numbers;

#[test]
fn test_case1() {
    let mut data = Cursor::new(include_bytes!("input1.txt"));

    assert_eq!(sum_numbers(&mut data), 142);
}

#[test]
fn test_case2() {
    let mut data = Cursor::new(include_bytes!("input2.txt"));

    assert_eq!(sum_numbers(&mut data), 281);
}

#[test]
fn test_case3() {
    let mut data = Cursor::new("eightwothree");

    assert_eq!(sum_numbers(&mut data), 83);
}
