use itertools::Itertools;

type Number = u64;

const STARTING_SPEED: Number = 0; // speed is in mm
const ACCEL_PER_MS: Number = 1; // accel is defined as mm per ms

pub(crate) fn part_1(input: &str) -> String {
    // parse
    let (time_limits, record_distances) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|num| num.parse().unwrap())
        })
        .collect_tuple()
        .unwrap();

    // solve
    time_limits
        .zip(record_distances)
        .map(|(time_limit, record_distance)| {
            distances_possible(time_limit)
                .filter(|&distance| distance > record_distance)
                .count() as Number
        })
        .product::<Number>()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    // parse
    let (time_limit, record_distance) = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .collect::<String>()
                .parse::<u64>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    // solve
    distances_possible(time_limit)
        .filter(|&distance| distance > record_distance)
        .count()
        .to_string()
}

fn distances_possible(time_limit: Number) -> impl Iterator<Item = Number> {
    (0..=time_limit).map(move |(time_accelerating)| {
        let speed = STARTING_SPEED + time_accelerating * ACCEL_PER_MS;
        let time_moving = time_limit - time_accelerating;
        speed * time_moving
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_INPUT), "288");
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_INPUT), "71503");
    }
}