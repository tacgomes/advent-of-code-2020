use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use regex::Regex;

fn find_allergens(file_name: impl AsRef<Path>) -> (usize, String) {
    let content = fs::read_to_string(file_name).unwrap();

    let re = Regex::new(r"(?P<ingredients>.+) \(contains (?P<allergens>.+)\)").unwrap();

    let mut all_ingredients = vec![];
    let mut allergen_ingredients_lists = HashMap::new();

    for line in content.trim().split('\n') {
        let caps = re.captures(&line).unwrap();
        let ingredients = caps["ingredients"].split_whitespace().collect::<Vec<_>>();
        let allergens = caps["allergens"].split(", ").collect::<Vec<_>>();

        for ingredient in &ingredients {
            all_ingredients.push(ingredient.to_string());
        }

        for allergen in &allergens {
            allergen_ingredients_lists
                .entry(allergen.to_string())
                .or_insert(Vec::new())
                .push(
                    ingredients
                        .iter()
                        .map(|ingredient| ingredient.to_string())
                        .collect::<HashSet<String>>(),
                );
        }
    }

    // Ingredient candidates for an allergen
    let mut allergen_candidates = HashMap::new();

    for (allergen, ingredients_lists) in &allergen_ingredients_lists {
        // Get the list of ingredients that can contain this allergen.
        // For an ingredient to be a candidate, it must appear in all
        // ingredient lists that have that allergen.
        let intersect = ingredients_lists[0]
            .iter()
            .filter(|x| ingredients_lists[1..].iter().all(|s| s.contains(*x)))
            .cloned()
            .collect::<HashSet<String>>();
        allergen_candidates.insert(allergen.clone(), intersect.clone());
    }

    let safe_ingredients_count = all_ingredients
        .iter()
        .filter(|&ingredient| {
            !allergen_candidates
                .values()
                .any(|candidates| candidates.contains(ingredient))
        })
        .count();

    // Ingredient found to correspond to an allergen
    let mut allergens_discovered = vec![];

    while allergens_discovered.len() != allergen_ingredients_lists.len() {
        let (allergen, candidates) = allergen_candidates
            .iter_mut()
            .find(|(_, candidates)| candidates.len() == 1)
            .unwrap();
        let allergen = allergen.clone();
        let ingredient = candidates.iter().next().unwrap().clone();

        allergens_discovered.push((allergen.clone(), ingredient.clone()));

        allergen_candidates.remove(&allergen);
        for candidates in &mut allergen_candidates.values_mut() {
            candidates.remove(&ingredient);
        }
    }

    allergens_discovered.sort_by_key(|(allergen, _)| allergen.clone());
    let unsafe_ingredients = allergens_discovered
        .iter()
        .map(|(_, ingredient)| ingredient.to_string())
        .collect::<Vec<_>>()
        .join(",");

    (safe_ingredients_count, unsafe_ingredients)
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let res = find_allergens(env::args().nth(1).unwrap());
    println!("Result: {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let s = "mxmxvkd,sqjhc,fvjkl".to_string();
        assert_eq!(find_allergens("example.txt"), (5, s));
    }

    #[test]
    fn test_puzzle_input() {
        let s = "kqv,jxx,zzt,dklgl,pmvfzk,tsnkknk,qdlpbt,tlgrhdh".to_string();
        assert_eq!(find_allergens("input.txt"), (2493, s));
    }
}
