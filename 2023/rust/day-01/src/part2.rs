use crate::custom_error::AocError;

#[tracing::instrument]
fn replace_prefix(line: &str) -> (char, bool) {
    let prefixes = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for (prefix, replacement) in prefixes {
        if line.starts_with(prefix) {
            return (replacement, true);
        }
    }

    (' ', false)
}

fn process_lines(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        let mut first = ' ';
        let mut last = ' ';

        let chars = line.chars().collect::<Vec<char>>();

        for i in 0..chars.len() {
            if chars[i].is_ascii_digit() {
                first = chars[i];
                break;
            }

            let (replacement, found) =
                replace_prefix(&line[i..]);
            if found {
                first = replacement;
                break;
            }
        }

        for i in (0..chars.len()).rev() {
            if chars[i].is_ascii_digit() {
                last = chars[i];
                break;
            }

            let (replacement, found) =
                replace_prefix(&line[i..]);
            if found {
                last = replacement;
                break;
            }
        }

        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        let number = first * 10 + last;
        total += number;
    }

    total
}

pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let output: u32 = process_lines(input);
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
