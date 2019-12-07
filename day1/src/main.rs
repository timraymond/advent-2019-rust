use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let path = "masses.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);

    let mut sum: i64 = 0;
    for (_, line) in buffered.lines().enumerate() {
        let num: i64 = line.unwrap().trim().parse().unwrap();
        sum = sum + all_fuel(num);
    }
    println!("{}", sum);
}

fn all_fuel(mass: i64) -> i64 {
    let mut sum: i64 = 0;
    let mut cur = mass;
    loop {
        cur = fuel(cur);
        if cur <= 0 {
            break;
        }
        sum += cur;
    }
    return sum;
}

fn fuel(mass: i64) -> i64 {
    return (mass/3)-2;
}

#[test]
fn test_all_fuel() {
    assert_eq!(all_fuel(1969), 966);
    assert_eq!(all_fuel(100756), 50346);
}

#[test]
fn test_fuel() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}
