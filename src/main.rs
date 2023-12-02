fn main() {}

fn d1p1() {}

fn parse_string(input: &str) -> u64 {
    let first_num = input.clone().chars().find(|c| c.is_numeric());
    let first_num = match first_num {
        Some(num) => num,
        None => panic!("invalid string"),
    };
    let last_num = input.chars().rev().find(|c| c.is_numeric());
    let last_num = match last_num {
        Some(num) => num,
        None => panic!("invalid string"),
    };
    let mut full_num: String = "".to_string();
    full_num.push(first_num);
    full_num.push(last_num);
    match str::parse::<u64>(&full_num) {
        Ok(num) => num,
        Err(_) => panic!("didn't work!"),
    }
}

#[test]
fn test_d1p1() {
    let test_string = "1abc2";
    let output = parse_string(test_string);
    assert_eq!(output, 12);
}
