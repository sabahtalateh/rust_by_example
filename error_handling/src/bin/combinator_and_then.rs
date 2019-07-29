#![allow(dead_code)]

#[derive(Debug)]
enum Food {
    CordonBlue,
    Steak,
    Sushi,
}

#[derive(Debug)]
enum Day {
    Monday,
    Tuesday,
    Wednesday,
}

// We don't have the ingredients to make Sushi
fn have_ingredients(food: Food) -> Option<Food> {
    match food {
        Food::Sushi => None,
        _ => Some(food),
    }
}

// We have the recipe for everything except Cordon Bleu
fn have_recipe(food: Food) -> Option<Food> {
    match food {
        Food::CordonBlue => None,
        _ => Some(food),
    }
}

// To make a dish, we need both the recipe and the ingredients
// We can represent the logic with a chain of `match`es:
fn cookable_v1(food: Food) -> Option<Food> {
    match have_ingredients(food) {
        None => None,
        Some(food) => match have_recipe(food) {
            None => None,
            Some(food) => Some(food),
        },
    }
}

// Equivalent to `cookable_v1`
fn cookable_v2(food: Food) -> Option<Food> {
    have_recipe(food).and_then(have_ingredients)
}

fn eat(food: Food, day: Day) {
    match cookable_v2(food) {
        Some(food) => println!("Yay! On {:?} we get to eat {:?}", day, food),
        None => println!("Yay!! We will eat nothing on {:?}", day),
    }
}

fn main() {
    let food = cookable_v1(Food::Steak);
    println!("{:?}", food);

    let (cordon_blue, steak, sushi) = (Food::CordonBlue, Food::Steak, Food::Sushi);
    eat(cordon_blue, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);
}
