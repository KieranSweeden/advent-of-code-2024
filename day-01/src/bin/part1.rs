#[allow(unused)]

const BASE_TEN_RADIX: u32 = 10;

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    match output {
        Ok(result) => {
            dbg!(result);
        }
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

fn process(input: &str) -> Result<i32, &str> {
    input
        .lines()
        .try_fold(0, |total, line| -> Result<i32, &str> {
            let first_digit_string = get_first_base_10_digit(line)?;
            let last_digit_string = get_last_base_10_digit(line)?;
            let two_digit_string = first_digit_string + &last_digit_string;
            match two_digit_string.parse::<i32>() {
                Ok(parsed_digit) => Ok(total + parsed_digit),
                Err(_) => Err("Failed to parse two digit string into i32"),
            }
        })
}

fn get_first_base_10_digit(input: &str) -> Result<String, &str> {
    for character in input.chars() {
        if character.is_digit(BASE_TEN_RADIX) {
            return Ok(character.to_string());
        }
    }
    Err("No base-10 digit found in string")
}

fn get_last_base_10_digit(input: &str) -> Result<String, &str> {
    for character in input.chars().rev() {
        if character.is_digit(BASE_TEN_RADIX) {
            return Ok(character.to_string());
        }
    }
    Err("No base-10 digit found in string")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_passes() {
        let result = process(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, Ok(142 as i32));
    }

    #[test]
    fn process_returns_zero_for_empty_string() {
        let result = process("");
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn get_first_base_10_char_passes() {
        let input = "pqr3stu8vwx";
        let result = get_first_base_10_digit(input);
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn get_first_base_10_char_errors() {
        let input = "abcdefg";
        let result = get_first_base_10_digit(input);
        assert_eq!(result, Err("No base-10 digit found in string"));
    }

    #[test]
    fn get_last_base_10_char_passes() {
        let input = "pqr3stu8vwx";
        let result = get_last_base_10_digit(input);
        assert_eq!(result.unwrap(), "8");
    }

    #[test]
    fn get_last_base_10_char_errors() {
        let input = "abcdefg";
        let result = get_last_base_10_digit(input);
        assert_eq!(result, Err("No base-10 digit found in string"));
    }
}
