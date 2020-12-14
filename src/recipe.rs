mod csv;

enum Unit {
    TABLESPOON,
    OUNCE,
    GRAM
}

type Calories = i32;
struct CalorieCost {
    unit: Unit,
    unit_cost: Calories
}

struct Ingredient {
    id: i32,
    name: String,
    calorie_cost: CalorieCost
}
struct Ingredients {
    Vec<Ingredient> ingredients
}

struct RecipeIngredient {
    ingredient: Ingredient,
    quantity: i32
}

pub struct Recipe {
    name: String,
    ingredients: Vec<RecipeIngredient> 
}

#[config(test)]
mod recipe_tests {
    #[test]
    fn test_recipes() {
        let recipes = Recipes::load("etc/recipes.csv");
        assert_eq!(2, recipes.recipes().len());
        let olive_oil = Ingredient {
            id: 1,
            name: "olive oil".to_string(),
            calorie_cost: CalorieCost {
                unit: Unit::TABLESPOON,
                unit_cost: 24
            }
        }
        let egg = Ingredient {
            id: 2,
            name: "egg".to_string(),
            calorie_cost: CalorieCost {
                unit: Unit::GRAM,
                unit_cost: 100
            }
        }
        let first = recipes.recipes()[0];
        assert_eq!(first, Recipe { name: "scrambled eggs".to_string(), ingredients: vec![olive_oil, egg] });
    }
}
