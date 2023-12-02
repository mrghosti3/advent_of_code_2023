use std::fs::File;
use std::io::BufReader;

fn main() {
    let fname = std::env::args()
        .skip(1)
        .next()
        .expect("Missing input file name!");

    let mut fopen = BufReader::new(File::open(fname).expect("Error opening file."));

    println!("{}", task_1::sum_numbers(&mut fopen));
}
