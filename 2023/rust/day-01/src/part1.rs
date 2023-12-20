use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let output: u32 = input
        .lines()
        .map(|line| {
            let mut iter =
                line.chars().filter_map(|c| c.to_digit(10));

            let first =
                iter.next().expect("Should be a number.");

            let last = iter.last();

            match last {
                Some(num) => first * 10 + num,
                None => first * 10 + first,
            }
        })
        .sum();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
