use itertools::Itertools;
use std::cmp::max;

// Investigate schematic (laid out in ascii grid).
// Find every symbol coordinate, find every number with coordinate, then filter out every number whose coordinate is not close enough to a symbol.
pub(crate) fn part_1(input: &str) -> String {
    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.trim().chars().enumerate().filter_map(move |(x, c)| {
                if !c.is_ascii_digit() && c != '.' {
                    Some((x, y))
                } else {
                    None
                }
            })
        })
        .collect::<Vec<(usize, usize)>>();
    // dbg!(&symbols);

    let numbers = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(|c| if !c.is_ascii_digit() { ' ' } else { c }) // keep only the numbers (still in place)
                .collect::<String>()
                .split_whitespace()
                .flat_map(|number| {
                    line.match_indices(number)
                        .map(|(x, _)| ((x, y), number.parse::<u32>().unwrap()))
                })
                .collect::<Vec<_>>()
                .into_iter()
        })
        .collect::<Vec<((usize, usize), u32)>>();
    // dbg!(numbers);
    numbers
        .iter()
        .filter(|((x, y), number)| {
            // dbg!(x, y, number);
            let search_horizontal =
                (x.saturating_sub(1))..=(x + number.to_string().chars().count());
            let search_vertical = (y.saturating_sub(1))..=(y + 1);
            // dbg!(&search_horizontal, &search_vertical);
            symbols.iter().any(|(symbol_x, symbol_y)| {
                search_horizontal.contains(symbol_x) && search_vertical.contains(symbol_y)
            })
        })
        .map(|((_, _), number)| number)
        .sum::<u32>()
        .to_string()
}

// pub(crate) fn part_2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use crate::day_3::*;

    #[test]
    fn part_1_works() {
        assert_eq!(
            part_1(
                r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            "4361"
        );
    }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(part_2(""), "");
    // }
}