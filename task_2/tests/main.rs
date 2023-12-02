use std::io::Cursor;

use task2::possible_game_sum;

#[test]
fn test_case1() {
    let mut data = Cursor::new(include_bytes!("input1.txt"));

    assert_eq!(possible_game_sum(&mut data), 8);
}
