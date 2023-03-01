use crate::Runnable;

struct FoodItem {
    calories: u32,
}
impl FoodItem {
    fn from_calories(calories: u32) -> Self {
        FoodItem { calories }
    }
}

struct Elf {
    food_items: Vec<FoodItem>,
}
impl Elf {
    // calories
    fn from_foods(foods: Vec<FoodItem>) -> Self {
        Elf { food_items: foods }
    }

    fn total_calories(&self) -> u32 {
        let mut total: u32 = 0;
        for food_item in self.food_items.as_slice() {
            total += food_item.calories;
        }
        total
    }
}

pub struct Solution;
impl Runnable for Solution {
    fn run_with_input(&self, input: String) {
        let mut elves = Vec::new();
        let mut new_foods = Vec::new();
        for line in input.lines() {
            if line.is_empty() {
                elves.push(Elf::from_foods(new_foods));
                new_foods = Vec::new();
            } else {
                let calories = line.parse::<u32>().expect("Could not convert to int!");
                new_foods.push(FoodItem::from_calories(calories))
            }
        }

        elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));
        let sorted_elves = elves;

        println!(
            "Elf with the most calories has {} calories.",
            sorted_elves[0].total_calories()
        );
        println!(
            "Elf with second most calories has {} calories.",
            sorted_elves[1].total_calories()
        );
        println!(
            "Elf with third most calories has {} calories.",
            sorted_elves[2].total_calories()
        );

        let top_calories: Vec<u32> = sorted_elves[..=2]
            .iter()
            .map(|e| e.total_calories())
            .collect();
        let total: u32 = top_calories.iter().sum();
        println!("Calories of top three elves: {}", total)
    }
}
