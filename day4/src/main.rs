const REQUIRED_LENGTH: usize = 6;
const REQUIRED_SEQ: i32 = 2;

fn valid(password: &str) -> bool {
    // check length
    if password.len() != REQUIRED_LENGTH {
        return false;
    }

    let mut cur = 0;
    let mut seq_len = 0;
    let mut min_seq_len = REQUIRED_LENGTH as i32;

    for digit in password.chars() {
        if let Some(dig) = digit.to_digit(10) {
            if dig > cur {
                if seq_len < min_seq_len && seq_len > 1 {
                    min_seq_len = seq_len;
                }
                cur = dig;
                seq_len = 1;
            } else if dig == cur {
                seq_len += 1;
            } else {
                // decreasing sequence
                return false;
            }
        } else {
            return false;
        }
    }

    // such a hack to repeat this:
    if seq_len < min_seq_len && seq_len > 1 {
        min_seq_len = seq_len;
    }

    return min_seq_len == REQUIRED_SEQ;
}

#[test]
fn test_valid_with_same() {
    assert_eq!(valid("111111"), false);
}

#[test]
fn test_valid_with_valid() {
    assert_eq!(valid("112233"), true);
}

#[test]
fn test_valid_with_long_double() {
    assert_eq!(valid("123444"), false);
}

#[test]
fn test_valid_with_decrease() {
    assert_eq!(valid("223450"), false);
}

#[test]
fn test_valid_with_no_double() {
    assert_eq!(valid("123789"), false);
}

#[test]
fn test_valid_with_odd_and_pair() {
    assert_eq!(valid("111122"), true);
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
