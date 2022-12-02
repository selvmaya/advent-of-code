use std::fs;

struct FoodItem {
    calories: u32
}
impl FoodItem {
    fn from_calories(calories: u32) -> Self {
        FoodItem {
            calories
        }
    }
}

struct Elf {
    food_items: Vec<FoodItem>
}
impl Elf {
    // calories
    fn from_foods(foods: Vec<FoodItem>) -> Self {
        Elf {
            food_items: foods
        }
    }

    fn total_calories(&self) -> u32 {
        let mut total: u32 = 0;
        for food_item in self.food_items.as_slice() {
            total += food_item.calories;
        }
        total
    }
}

const PATH: &str = "inputs/day_1.txt";

#[allow(unused)]
pub fn run() {
    let raw_content = fs::read_to_string(PATH).unwrap();

    let mut elves= Vec::new();
    for elf_str in raw_content.split("\n\n") {
        let mut foods = Vec::new();
        for food_str in elf_str.lines() {
            let food = food_str.parse::<u32>()
                .expect(format!("Could not parse {} to u32!", food_str)
                    .as_str());
            foods.push(FoodItem::from_calories(food));
        }
        elves.push(Elf::from_foods(foods));
    }

    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
    let sorted_elves = elves;

    println!("Elf with the most calories has {} calories.", sorted_elves[0].total_calories());
    println!("Elf with second most calories has {} calories.", sorted_elves[1].total_calories());
    println!("Elf with third most calories has {} calories.", sorted_elves[2].total_calories());

    let top_calories: Vec<u32> = sorted_elves[..=2].into_iter().map(|e| e.total_calories()).collect();
    let total: u32 = top_calories.iter().sum();
    println!("Calories of top three elves: {}", total)
}