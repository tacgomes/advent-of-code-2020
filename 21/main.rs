use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use regex::Regex;

type AlergenIngredientsLists = HashMap<String, Vec<HashSet<String>>>;

fn find_allergens(ingredients: &[String], allergens: &AlergenIngredientsLists) -> (usize, String) {
    // Ingredient candidates for an allergen
    let mut allergen_candidates = HashMap::new();

    for (allergen, ingredients_lists) in allergens {
        // Get the list of ingredients that can contain this allergen.
        // For an ingredient to be a candidate, it must appear in all
        // ingredient lists that have that allergen.
        let intersect = ingredients_lists[0]
            .iter()
            .filter(|x| ingredients_lists[1..].iter().all(|s| s.contains(*x)))
            .collect::<HashSet<_>>();
        allergen_candidates.insert(allergen, intersect);
    }

    let safe_ingredients_count = ingredients
        .iter()
        .filter(|&ingredient| {
            !allergen_candidates
                .values()
                .any(|candidates| candidates.contains(ingredient))
        })
        .count();

    // (allergen, ingredient) tuples with the ingredient that was found
    // to correspond to a given allergen.
    let mut allergens_discovered = vec![];

    while allergens_discovered.len() != allergens.len() {
        let (allergen, candidates) = allergen_candidates
            .iter_mut()
            .find(|(_, candidates)| candidates.len() == 1)
            .unwrap();
        let allergen = allergen.clone();
        let ingredient = candidates.iter().next().unwrap().clone();

        allergen_candidates.remove(&allergen);
        for candidates in &mut allergen_candidates.values_mut() {
            candidates.remove(&ingredient);
        }

        allergens_discovered.push((allergen, ingredient));
    }

    allergens_discovered.sort_by_key(|(allergen, _)| allergen.clone());
    let unsafe_ingredients = allergens_discovered
        .into_iter()
        .map(|(_, ingredient)| ingredient.to_owned())
        .collect::<Vec<_>>()
        .join(",");

    (safe_ingredients_count, unsafe_ingredients)
}

fn parse_input(file_name: impl AsRef<Path>) -> (Vec<String>, AlergenIngredientsLists) {
    let re = Regex::new(r"(?P<ingredients>.+) \(contains (?P<allergens>.+)\)").unwrap();
    let content = fs::read_to_string(file_name).unwrap();

    let mut ingredients = vec![];
    let mut allergens = HashMap::new();

    for line in content.trim().split('\n') {
        let caps = re.captures(&line).unwrap();
        let parsed_ingredients = caps["ingredients"].split_whitespace();
        let parsed_allergens = caps["allergens"].split(", ");

        for ingredient in parsed_ingredients.clone() {
            ingredients.push(ingredient.to_owned());
        }

        for allergen in parsed_allergens {
            allergens
                .entry(allergen.to_string())
                .or_insert_with(Vec::new)
                .push(
                    parsed_ingredients
                        .clone()
                        .map(|ingredient| ingredient.to_owned())
                        .collect::<HashSet<String>>(),
                );
        }
    }

    (ingredients, allergens)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let file_name = env::args().nth(1).unwrap();
    let (ingredients, allergens) = parse_input(&file_name);
    let res = find_allergens(&ingredients, &allergens);
    println!("Result: {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let (ingredients, allergens) = parse_input("example.txt");
        let s = "mxmxvkd,sqjhc,fvjkl".to_string();
        assert_eq!(find_allergens(&ingredients, &allergens), (5, s));
    }

    #[test]
    fn test_puzzle_input() {
        let (ingredients, allergens) = parse_input("input.txt");
        let s = "kqv,jxx,zzt,dklgl,pmvfzk,tsnkknk,qdlpbt,tlgrhdh".to_string();
        assert_eq!(find_allergens(&ingredients, &allergens), (2493, s));
    }
}
