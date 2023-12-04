use std::cmp::max;

// Of all games (input), which ones are possible with specific constraints?
struct CubeAmount {
    red: u32,
    green: u32,
    blue: u32,
}
pub(crate) fn part_1(input: &str) -> String {
    const UPPER_LIMIT: CubeAmount = CubeAmount {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .lines()
        .filter_map(|line| {
            let (_, line) = line.split_once(' ').unwrap();
            let (id, subsets) = line.split_once(": ").unwrap();
            // find out how many of each color there has to be in each game
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for subset in subsets.split("; ") {
                for data in subset.split(", ") {
                    let (amount, color) = data.split_once(' ').unwrap();
                    let amount = amount.parse::<u32>().unwrap();
                    match color {
                        "red" => min_red = max(min_red, amount),
                        "green" => min_green = max(min_green, amount),
                        "blue" => min_blue = max(min_blue, amount),
                        _ => panic!("unknown color?"),
                    }
                }
            }
            if min_red <= UPPER_LIMIT.red
                && min_green <= UPPER_LIMIT.green
                && min_blue <= UPPER_LIMIT.blue
            {
                Some(id.parse::<u32>().unwrap())
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    const UPPER_LIMIT: CubeAmount = CubeAmount {
        red: 12,
        green: 13,
        blue: 14,
    };
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(' ').unwrap();
            let (id, subsets) = line.split_once(": ").unwrap();
            // find out how many of each color there has to be in each game
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            for subset in subsets.split("; ") {
                for data in subset.split(", ") {
                    let (amount, color) = data.split_once(' ').unwrap();
                    let amount = amount.parse::<u32>().unwrap();
                    match color {
                        "red" => min_red = max(min_red, amount),
                        "green" => min_green = max(min_green, amount),
                        "blue" => min_blue = max(min_blue, amount),
                        _ => panic!("unknown color?"),
                    }
                }
            }
            min_red * min_green * min_blue
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;

    #[test]
    fn part_1_works() {
        assert_eq!(
            part_1(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "8"
        );
    }

    #[test]
    fn part_2_works() {
        assert_eq!(
            part_2(
                r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            "2286"
        );
    }
}
