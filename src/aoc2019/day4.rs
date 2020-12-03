///

fn parse_digits(guess: &i64) -> Vec<u32> {
    guess
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

fn is_valid_password(guess: &i64) -> bool {
    let digits = parse_digits(guess);

    if digits.len() != 6 {
        return false;
    }

    if digits
        .iter()
        .take(digits.len() - 1)
        .enumerate()
        .any(|(i, d)| d > &digits[i + 1])
    {
        //        println!("{} has digits not always increasing", guess);
        return false;
    }

    if digits
        .iter()
        .take(digits.len() - 1)
        .enumerate()
        .any(|(i, d)| d == &digits[i + 1])
    {
        //        println!("{} has duplicate adjacent", guess);
        return true;
    }

    false
}

fn is_valid_password_extra(guess: &i64) -> bool {
    if is_valid_password(guess) {
        let digits = parse_digits(guess);

        let mut idx = 0;
        while idx < digits.len() - 1 {
            let current_digit = digits[idx];

            let mut adjacent_digit_count = 1;
            while digits[idx + 1] == current_digit {
                adjacent_digit_count += 1;
                idx += 1;
                if idx == digits.len() - 1 {
                    break;
                }
            }

            if adjacent_digit_count == 2 {
                return true;
            }

            idx += 1;
        }
    }

    false
}

pub fn solve_part_one(input: &[String]) -> String {
    let bounds: Vec<&str> = input[0].split('-').collect();
    let lower: i64 = bounds[0].parse().unwrap();
    let upper: i64 = bounds[1].parse().unwrap();

    let potential_passwords = (lower..=upper).filter(is_valid_password).count();

    potential_passwords.to_string()
}

pub fn solve_part_two(input: &[String]) -> String {
    let bounds: Vec<&str> = input[0].split('-').collect();
    let lower: i64 = bounds[0].parse().unwrap();
    let upper: i64 = bounds[1].parse().unwrap();

    let potential_passwords = (lower..=upper).filter(is_valid_password_extra).count();

    potential_passwords.to_string()
}

#[test]
fn examples_part_one() {
    assert_eq!(true, is_valid_password(&111111));
    assert_eq!(false, is_valid_password(&223450));
    assert_eq!(false, is_valid_password(&123789));
}

#[test]
fn examples_part_two() {
    assert_eq!(true, is_valid_password_extra(&112233));
    assert_eq!(false, is_valid_password_extra(&123444));
    assert_eq!(true, is_valid_password_extra(&111122));
}
