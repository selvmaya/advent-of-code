use std::fs;

#[cfg(test)]
mod tests {
    use crate::day_2::*;

    #[test]
    fn shape_generation_works() {
        let opponent_str = "A";
        assert_eq!(Shape::Rock, Shape::from_abc(opponent_str).unwrap());
        let player_str = "Y";
        assert_eq!(Shape::Paper, Shape::from_xyz(player_str).unwrap());
    }

    #[test]
    fn outcome_works() {
        let player = Shape::Paper;
        let opponent = Shape::Rock;
        assert_eq!(Outcome::Win, Outcome::from_fight_between(&player, &opponent));
    }

    #[test]
    fn points_work() {
        let player = Shape::Paper;
        let outcome = Outcome::Win;
        assert_eq!(8, player.shape_points() + outcome.outcome_points())
    }
}

#[derive(PartialEq, Debug)]
enum Outcome {
    Loss,
    Draw,
    Win
}
#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}
struct Round {
    player: Shape,
    opponent: Shape
}

impl Outcome {
    fn outcome_points(self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6
        }
    }
    fn from_fight_between(this: &Shape, other: &Shape) -> Self {
        // return outcome for self when fighting other
        match this {
            Shape::Rock => match other {
                Shape::Rock => Outcome::Draw,
                Shape::Paper => Outcome::Loss,
                Shape::Scissors => Outcome::Win
            },
            Shape::Paper => match other {
                Shape::Rock => Outcome::Win,
                Shape::Paper => Outcome::Draw,
                Shape::Scissors => Outcome::Loss
            },
            Shape::Scissors => match other {
                Shape::Rock => Outcome::Loss,
                Shape::Paper => Outcome::Win,
                Shape::Scissors => Outcome::Draw
            }
        }
    }
}

impl Shape {
    fn from_abc(s: &str) -> Result<Shape, String> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err(format!("{s} is not a valid type to convert from ABC!"))
        }
    }
    fn from_xyz(s: &str) -> Result<Shape, String> {
        match s {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(format!("{s} is not a valid type to convert from XYZ!"))
        }
    }
    fn shape_points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }
}

impl Round {
    fn from_str(round_str: &str) -> Result<Round, String> {
        let plays: Vec<&str> = round_str.split(" ").collect();

        let opponent: &str;
        match plays.first() {
            Some(s) => opponent = s,
            None => return Err("No opponent found.".to_string())
        };
        let player: &str;
        match plays.last() {
            Some(s) => player = s,
            None => return Err("No player found".to_string())
        };

        let opponent_shape: Shape;
        match Shape::from_abc(opponent) {
            Ok(shape) => opponent_shape = shape,
            Err(e) => return Err(e)
        }
        let player_shape: Shape;
        match Shape::from_xyz(player) {
            Ok(shape) => player_shape = shape,
            Err(e) => return Err(e),
        };

        Ok(Round {
            player: opponent_shape,
            opponent: player_shape
        })
    }
    fn player_round_score(&self) -> u32 {
        let shape_score = &self.opponent.shape_points();
        let outcome_score = Outcome::from_fight_between
            (&self.player, &self.opponent)
            .outcome_points();
        shape_score + outcome_score
    }
}

const PATH: &str = "inputs/day_2.txt";

#[allow(unused)]
pub fn run() {
    let raw_string: String = fs::read_to_string(PATH)
        .expect(format!("Could not read file {PATH}").as_str());

    let str_rounds: Vec<&str> = raw_string.lines().collect();

    let mut rounds: Vec<Round> = Vec::new();
    for str_round in str_rounds {
        let round = Round::from_str(str_round).unwrap();
        rounds.push(round);
    }

    let round_points: Vec<u32> = rounds.iter().map(|r| r.player_round_score()).collect();
    let total_points: u32 = round_points.iter().sum();

    println!("Total points after all rounds: {total_points}")
}