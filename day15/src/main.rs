#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn find_best_recipe() -> ([usize; 4], i64) {
    // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
    // PeanutButter: capacity -1, durability 3, flavor 0, texture 0, calories 1
    // Frosting: capacity 0, durability -1, flavor 4, texture 0, calories 6
    // Sugar: capacity -1, durability 0, flavor 0, texture 2, calories 8

    let mut max_score = 0;
    let mut best_recipe = [0; 4];

    for n_sprinkles in 0..=100 {
        for n_peanut_butter in 0..=(100 - n_sprinkles) {
            for n_frosting in 0..=(100 - n_sprinkles - n_peanut_butter) {
                for n_sugar in 0..=(100 - n_sprinkles - n_peanut_butter - n_frosting) {
                    let capacity = (5 * n_sprinkles - n_peanut_butter - n_sugar).max(0);
                    let durability = (-n_sprinkles + 3 * n_peanut_butter - n_frosting).max(0);
                    let flavor = (4 * n_frosting).max(0);
                    let texture = (2 * n_sugar).max(0);

                    let score = capacity * durability * flavor * texture;

                    if score > max_score {
                        max_score = score;
                        best_recipe = [
                            n_sprinkles as usize,
                            n_peanut_butter as usize,
                            n_frosting as usize,
                            n_sugar as usize,
                        ];
                    }
                }
            }
        }
    }

    debug_assert_eq!(best_recipe.iter().sum::<usize>(), 100);

    (best_recipe, max_score)
}

fn find_best_recipe_calories(calories: usize) -> ([usize; 4], i64) {
    // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
    // PeanutButter: capacity -1, durability 3, flavor 0, texture 0, calories 1
    // Frosting: capacity 0, durability -1, flavor 4, texture 0, calories 6
    // Sugar: capacity -1, durability 0, flavor 0, texture 2, calories 8

    let mut max_score: i64 = 0;
    let mut best_recipe = [0; 4];

    for n_sprinkles in 0..=100 {
        for n_peanut_butter in 0..=(100 - n_sprinkles) {
            for n_frosting in 0..=(100 - n_sprinkles - n_peanut_butter) {
                for n_sugar in 0..=(100 - n_sprinkles - n_peanut_butter - n_frosting) {
                    let capacity = (5 * n_sprinkles - n_peanut_butter - n_sugar).max(0);
                    let durability = (-n_sprinkles + 3 * n_peanut_butter - n_frosting).max(0);
                    let flavor = (4 * n_frosting).max(0);
                    let texture = (2 * n_sugar).max(0);

                    let score = capacity * durability * flavor * texture;

                    let current_calories =
                        (5 * n_sprinkles + n_peanut_butter + 6 * n_frosting + 8 * n_sugar) as usize;

                    if score > max_score {
                        if current_calories != calories {
                            continue;
                        }

                        max_score = score;
                        best_recipe = [
                            n_sprinkles as usize,
                            n_peanut_butter as usize,
                            n_frosting as usize,
                            n_sugar as usize,
                        ];
                    }
                }
            }
        }
    }

    (best_recipe, max_score)
}

fn main() {
    let (recipe, score) = find_best_recipe();
    println!("Recipe: {:?}, Score: {}", recipe, score);

    let (recipe, score) = find_best_recipe_calories(500);
    println!("Recipe (500 calories): {:?}, Score: {}", recipe, score);
}
