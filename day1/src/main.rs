use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let path = "masses.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut sum: u32 = 0;
    for (_, line) in buffered.lines().enumerate() {
        let num: u32 = line.unwrap().trim().parse().unwrap();
        sum = sum + fuel(num);
    }
    println!("{}", sum);
}

fn fuel(mass: u32) -> u32 {
    return (mass/3)-2;
}

#[test]
fn test_fuel() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}
