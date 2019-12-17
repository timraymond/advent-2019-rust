const REQUIRED_LENGTH: usize = 6;

fn valid(password: &str) -> bool {
    // check length
    if password.len() != REQUIRED_LENGTH {
        return false;
    }

    // ensure increase
    let mut max = 0;
    let mut did_double = false;

    for digit in password.chars() {
        if let Some(dig) = digit.to_digit(10) {
            if dig < max {
                return false;
            } else if dig == max {
                did_double = true;
            }
            max = dig;
        } else {
            return false;
        }
    }

    return did_double;
}

#[test]
fn test_valid_with_valid() {
    assert_eq!(valid("111111"), true);
}

#[test]
fn test_valid_with_decrease() {
    assert_eq!(valid("223450"), false);
}

#[test]
fn test_valid_with_no_double() {
    assert_eq!(valid("123789"), false);
}

fn main() {
    let start = 284639;
    let end = 748759;

    let mut ctr = 0;

    for cur in start..end {
        if valid(&cur.to_string()) {
            ctr += 1;
        }
    }

    println!("Count: {}", ctr);
}
