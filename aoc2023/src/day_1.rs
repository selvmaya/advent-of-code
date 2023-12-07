// take each line, add first and last digit in it, then sum them together.
pub(crate) fn part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            let (first, second) = (numbers[0], numbers[numbers.len() - 1]);
            first * 10 + second // first is 2nd decimal place or whatever
        })
        .sum::<u32>()
        .to_string()
}

// take each line, add first and last digit in it, then sum them together.
pub(crate) fn part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let numbers: Vec<_> = line
                .chars()
                .enumerate()
                .filter_map(|(index, c)| c.to_digit(10).map(|digit| (index, digit)))
                .collect();
            let (mut first, mut last) = (numbers.first().cloned(), numbers.last().cloned()); // using just char conversions
            let first_spelled = find_first_spelling(line);
            // eprintln!("First spelled: {:?}", first_spelled);
            if let Some((spelled_index, _spelled_digit)) = first_spelled {
                if let Some((first_index, _)) = first {
                    if spelled_index < first_index {
                        first = first_spelled;
                    }
                } else {
                    first = first_spelled;
                }
            }
            let last_spelled = find_last_spelling(line);
            // eprintln!("Last spelled: {:?}", last_spelled);
            if let Some((spelled_index, _spelled_digit)) = last_spelled {
                if let Some((last_index, _)) = last {
                    if spelled_index > last_index {
                        last = last_spelled;
                    }
                } else {
                    last = last_spelled;
                }
            }
            // first is 2nd decimal place or whatever
            // eprintln!("Line sum: {}", line_sum);
            (first.unwrap().1 * 10) + last.unwrap().1
        })
        .sum::<u32>()
        .to_string()
}

fn find_first_spelling(line: &str) -> Option<(usize, u32)> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .filter_map(|(digit, spelling)| line.find(spelling).map(|index| (index, digit as u32 + 1)))
    .min_by(|(a_pos, _a_digit), (b_pos, _b_digit)| a_pos.cmp(b_pos))
}

fn find_last_spelling(line: &str) -> Option<(usize, u32)> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .filter_map(|(digit, spelling)| {
        line.rfind(spelling)
            .map(|index| (index + spelling.chars().count(), digit as u32 + 1))
    })
    .max_by(|(a_pos, _a_digit), (b_pos, _b_digit)| a_pos.cmp(b_pos))
}

#[cfg(test)]
mod tests {
    use crate::day_1::*;

    #[test]
    fn part_1_works() {
        assert_eq!(
            part_1(
                r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            "142"
        );
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            part_2(
                r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            "281"
        );
    }
}