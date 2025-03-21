use std::env;
use std::collections::{HashMap, HashSet};

fn find_all_recipes(mut recipes: Vec<String>, mut ingredients: Vec<Vec<String>>, supplies: Vec<String>) -> Vec<String> {
    let n = recipes.len();
    let mut recipes_: HashMap<String, HashSet<String>> = HashMap::new();
    for _ in 0..n {
        recipes_.insert(
            recipes.pop().unwrap(),
            ingredients.pop().unwrap().into_iter().collect());
    }
    let mut recipes = recipes_;

    let mut supplies: HashSet<String> = supplies.into_iter().collect();
    let mut result = Vec::new();

    loop {
        let Some((r, _)) = recipes.iter()
            .find(|(_, i)| i.is_subset(&supplies)) else { break; };
        let r = r.clone();
        recipes.remove(&r);
        supplies.insert(r.clone());
        result.push(r);
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let recipes = args[1]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.trim_matches('"').to_string())
        .collect();
    let ingredients = args[2]
        .strip_prefix('[').unwrap()
        .strip_suffix(']').unwrap()
        .split("],[")
        .map(|row| row
             .trim_start_matches('[')
             .trim_end_matches(']')
             .split(',')
             .map(|item| item.trim_matches('"').to_string())
             .collect())
        .collect();
    let supplies = args[3]
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|item| item.trim_matches('"').to_string())
        .collect();
    println!("{:?} {:?} {:?}", recipes, ingredients, supplies);
    println!("{:?}", find_all_recipes(recipes, ingredients, supplies));
}
