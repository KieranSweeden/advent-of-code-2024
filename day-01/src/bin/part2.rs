#[allow(unused)]
use phf::phf_map;

const BASE_TEN_RADIX: u32 = 10;

const NUMBER_NAMES: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

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

fn get_digit_string_for_matching_number_name(input: &str) -> Option<String> {
    for (index, number_name) in NUMBER_NAMES.iter().enumerate() {
        if input.contains(number_name) {
            return Some((index + 1).to_string());
        }
    }

    None
}

fn get_first_base_10_digit(input: &str) -> Result<String, &str> {
    let mut char_history = String::new();

    for character in input.chars() {
        if character.is_digit(BASE_TEN_RADIX) {
            return Ok(character.to_string());
        }

        char_history.push(character);

        if let Some(digit_string) = get_digit_string_for_matching_number_name(&char_history) {
            return Ok(digit_string);
        }
    }

    Err("No base-10 digit found in string")
}

fn get_last_base_10_digit(input: &str) -> Result<String, &str> {
    let mut char_history = String::new();

    for character in input.chars().rev() {
        let character_string = character.to_string();

        if character.is_digit(BASE_TEN_RADIX) {
            return Ok(character_string);
        }

        char_history.insert_str(0, &character_string);

        if let Some(digit_string) = get_digit_string_for_matching_number_name(&char_history) {
            return Ok(digit_string);
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
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, Ok(281 as i32));
    }

    #[test]
    fn process_returns_zero_for_empty_string() {
        let result = process("");
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn get_first_base_10_char_passes() {
        let input = "eightwothree";
        let result = get_first_base_10_digit(input);
        assert_eq!(result.unwrap(), "8");
    }

    #[test]
    fn get_first_base_10_char_errors() {
        let input = "abcdefg";
        let result = get_first_base_10_digit(input);
        assert_eq!(result, Err("No base-10 digit found in string"));
    }

    #[test]
    fn get_last_base_10_char_passes() {
        let input = "eightwothree";
        let result = get_last_base_10_digit(input);
        assert_eq!(result.unwrap(), "3");
    }

    #[test]
    fn get_last_base_10_char_errors() {
        let input = "abcdefg";
        let result = get_last_base_10_digit(input);
        assert_eq!(result, Err("No base-10 digit found in string"));
    }
}
