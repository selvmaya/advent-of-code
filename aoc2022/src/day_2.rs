use crate::Runnable;

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let str_rounds: Vec<&str> = input.lines().collect();

        part_1(&str_rounds);
        part_2(&str_rounds);
    }
}

#[cfg(test)]
mod tests {
    use crate::day_2::*;

    #[test]
    fn round_generation_works() {
        let full_str = "A Y";
        let str_rounds = vec!["A", "Y"];
        assert_eq!(str_rounds, Round::split_round_str(full_str));
        let round_example_1 = Round {
            opponent: Shape::Rock,
            player: Shape::Paper,
        };
        let opponent = Shape::from_opponent_str_abc(str_rounds[0]).unwrap();
        let player = Shape::from_player_str_xyz(str_rounds[1]).unwrap();
        assert_eq!(round_example_1, Round { opponent, player });
        assert_eq!(Ok(round_example_1), Round::from_str_part_1(full_str));
    }
    #[test]
    fn shape_generation_works_part_1() {
        let opponent_str = "A";
        assert_eq!(Ok(Shape::Rock), Shape::from_opponent_str_abc(opponent_str));
        let player_str = "Y";
        assert_eq!(Ok(Shape::Paper), Shape::from_player_str_xyz(player_str));
    }
    #[test]
    fn draw_outcomes_work() {
        for shape in &[Shape::Rock, Shape::Paper, Shape::Scissors] {
            assert_eq!(Outcome::Draw, Outcome::from_fight(shape, shape));
        }
    }
    #[test]
    fn win_lose_outcomes_work() {
        let winner_vec = vec![Shape::Paper, Shape::Scissors, Shape::Rock];
        let loser_vec = vec![Shape::Rock, Shape::Paper, Shape::Scissors];
        for i in (0..3).step_by(1) {
            let winner = &winner_vec[i];
            let loser = &loser_vec[i];
            assert_ne!(winner, loser);
            assert_eq!(Outcome::Win, Outcome::from_fight(winner, loser));
            assert_eq!(Outcome::Loss, Outcome::from_fight(loser, winner));
        }
    }
    #[test]
    fn points_work() {
        let player = Shape::Paper;
        let outcome = Outcome::Win;
        assert_eq!(2, player.points());
        assert_eq!(6, outcome.points());
        assert_ne!(player.points(), outcome.points());
    }
    #[test]
    fn round_outcome_works() {
        let round: Round = Round {
            player: Shape::Rock,
            opponent: Shape::Scissors,
        };
        assert_eq!(
            Outcome::Win,
            round.player_fight_outcome(),
            "{:?} beats {:?}.",
            round.player,
            round.opponent
        );
    }
    #[test]
    fn round_points_work() {
        let example_round_1: Round = Round {
            opponent: Shape::Rock,
            player: Shape::Paper,
        };
        assert_eq!(8, example_round_1.player_round_score());
        let example_round_2: Round = Round {
            opponent: Shape::Paper,
            player: Shape::Rock,
        };
        assert_eq!(1, example_round_2.player_round_score());
        let example_round_3: Round = Round {
            opponent: Shape::Scissors,
            player: Shape::Scissors,
        };
        assert_eq!(6, example_round_3.player_round_score());
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win,
}
#[derive(Copy, Clone, PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
#[derive(PartialEq, Debug)]
struct Round {
    player: Shape,
    opponent: Shape,
}

impl Outcome {
    fn from_fight(this: &Shape, other: &Shape) -> Self {
        // return outcome for self when fighting other
        match this {
            Shape::Rock => match other {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss,
            },
            Shape::Scissors => match other {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw,
            },
        }
    }
    fn from_outcome_str_xyz(s: &str) -> Result<Self, String> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(format!("{} is not a valid type to convert from XYZ!", s)),
        }
    }
    fn points(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl Shape {
    fn from_opponent_str_abc(s: &str) -> Result<Self, String> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err(format!("{} is not a valid type to convert from ABC!", s)),
        }
    }
    fn from_player_str_xyz(s: &str) -> Result<Self, String> {
        match s {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(format!("{} is not a valid type to convert from XYZ!", s)),
        }
    }
    fn from_outcome_with_other(outcome: &Outcome, other: &Self) -> Self {
        match outcome {
            Outcome::Loss => match &other {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => *other,
            Outcome::Win => match &other {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        }
    }
    fn points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Round {
    fn from_str_part_1(full_str: &str) -> Result<Round, String> {
        let str_rounds: Vec<&str> = Round::split_round_str(full_str);

        let opponent_str: &str = match str_rounds.first() {
            Some(s) => s,
            None => return Err("No opponent found.".to_string()),
        };
        let player_str: &str = match str_rounds.last() {
            Some(s) => s,
            None => return Err("No player found".to_string()),
        };

        let opponent: Shape = match Shape::from_opponent_str_abc(opponent_str) {
            Ok(shape) => shape,
            Err(e) => return Err(e),
        };
        let player: Shape = match Shape::from_player_str_xyz(player_str) {
            Ok(shape) => shape,
            Err(e) => return Err(e),
        };

        Ok(Round { player, opponent })
    }
    fn from_str_part_2(full_str: &str) -> Result<Round, String> {
        let str_rounds: Vec<&str> = Round::split_round_str(full_str);

        let opponent_str: &str = match str_rounds.first() {
            Some(s) => s,
            None => return Err("No opponent found.".to_string()),
        };
        let outcome_str: &str = match str_rounds.last() {
            Some(s) => s,
            None => return Err("No outcome found".to_string()),
        };

        let opponent: Shape = match Shape::from_opponent_str_abc(opponent_str) {
            Ok(shape) => shape,
            Err(e) => return Err(e),
        };
        let outcome: Outcome = match Outcome::from_outcome_str_xyz(outcome_str) {
            Ok(out) => out,
            Err(e) => return Err(e),
        };
        let player = Shape::from_outcome_with_other(&outcome, &opponent);

        Ok(Round { player, opponent })
    }
    fn split_round_str(full_str: &str) -> Vec<&str> {
        full_str.split(' ').collect()
    }

    fn player_fight_outcome(&self) -> Outcome {
        Outcome::from_fight(&self.player, &self.opponent)
    }
    fn player_round_score(&self) -> u32 {
        let shape_score = &self.player.points();
        let outcome_score = &self.player_fight_outcome().points();
        shape_score + outcome_score
    }
    fn player_total_points(rounds: &[Self]) -> u32 {
        rounds.iter().map(|r| r.player_round_score()).sum()
    }
}

#[allow(unused)]
fn part_2(str_rounds: &[&str]) {
    let rounds_part_2: Vec<Round> = str_rounds
        .iter()
        .map(|r_s| Round::from_str_part_2(r_s).unwrap())
        .collect();
    let points_part_2 = Round::player_total_points(&rounds_part_2);
    println!(
        "Total points after all rounds using rules from part 2: {}",
        points_part_2
    );
}

#[allow(unused)]
fn part_1(str_rounds: &[&str]) {
    let rounds_part_1: Vec<Round> = str_rounds
        .iter()
        .map(|r_s| Round::from_str_part_1(r_s).unwrap())
        .collect();
    let points_part_1 = Round::player_total_points(&rounds_part_1);
    println!(
        "Total points after all rounds using rules from part 1: {}",
        points_part_1
    );
}
