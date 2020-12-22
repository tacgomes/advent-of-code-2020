use std::env;
use std::fs;
use std::path::Path;
use std::process;
use std::collections::HashSet;
use std::collections::HashMap;

use regex::Regex;

fn calculate_part1(file_name: impl AsRef<Path>) -> usize {
    let content = fs::read_to_string(file_name).unwrap();

    let re = Regex::new(r"(?P<ingredients>.+) \(contains (?P<allergens>.+)\)").unwrap();

    let mut ingredients2allergens = HashMap::<String, Option<String>>::new();
    //let mut ingredients = vec![];
    //let mut allergens = vec![];
    let mut all_ingredients = vec![];
    let mut ads = HashMap::new();

    for line in content.trim().split('\n') {
        let caps = re.captures(&line).unwrap();
        let ingr = caps["ingredients"].split_whitespace().collect::<Vec<_>>();
        let allerg = caps["allergens"].split(", ").collect::<Vec<_>>();
        println!("ingredients: {:?}", ingr);
        println!("allergens: {:?}", allerg);


        for i in &ingr {
        //    ingredients2allergens.insert(i.to_string(), None);
            all_ingredients.push(i.to_string());
        }

        //let ingr = ingr.iter().map(|x| x.to_string()).collect::<HashSet<String>>();
        //ingredients.push(ingr);

        //let allerg = allerg.iter().map(|x| x.to_string()).collect::<HashSet<String>>();
        //allergens.push(allerg.clone());

        for a in &allerg {
            let mut e = ads.entry(a.to_string()).or_insert(Vec::new());
            e.push(ingr.iter().map(|x| x.to_string()).collect::<HashSet<String>>());
        }
    }

    let mut all_intersections = HashSet::<String>::new();

    let mut part2 = HashMap::<String, HashSet<String>>::new();

    for (k, v) in &ads {
        //for set in v {
        //}
        //

        let r = v[0].iter().filter(|x| v[1..].iter().all(|s| s.contains(*x))).cloned().collect::<HashSet<String>>();
        for rr in &r {
            all_intersections.insert(rr.to_string());
        }
        part2.insert(k.clone(), r.clone());

        println!("K={} intersect={:?}", k, r);
    }

    println!("ADS: {:#?}", ads);


    let mut results = vec![];
    while results.len() != ads.len() {
        //for (k, v) in &part2 {
        //    if v.len() == 1
        //}
        let (k, v) = part2.iter_mut().filter(|(k,v)| v.len() == 1).next().unwrap();
        let alergen = k.clone();
        let ingredient = v.iter().next().unwrap().clone();
        results.push((alergen.clone(), ingredient.clone()));
       // part2.iter_mut().for_each(|(kkk,vvv)| vvv.remove(ingredient));
       //
       part2.remove(&alergen);
       for (k2, v2) in &mut part2 {
            v2.remove(&ingredient);
       }
    }

    println!("RESULTS: {:?}", results);

    results.sort_by_key(|(k, v)| k.clone());

    let s = results.iter().map(|(k, v)| v.to_string()).collect::<Vec<_>>().join(",");
    println!("PART 2: {}", s);


    //println!("Ingredients: {:?}", ingredients2allergens);
    //let copy_for_counts = ingredients.clone();

   // for (i, set1) in ingredients.iter().enumerate() {
   //     for (j, set2) in ingredients.iter().enumerate().skip(i) {
   //         let intersect1: HashSet<_> = set1.intersection(&set2).collect();
   //         if intersect1.len() == 1 {
   //             let allergen_intersect: HashSet<_> = allergens[i].intersection(&allergens[j]).collect();
   //             if allergen_intersect.len() == 1 {
   //                 //remove = Some((intersect1.iter().next().unwrap().to_string(), allergen_intersect.iter().next().unwrap().to_string()));
   //                 //println!("Ingredient {} has allergen {}", intersect1.iter().next().unwrap().to_string(), allergen_intersect.iter().next().unwrap().to_string());
   //             }
   //         }
   //     }
   // }

/*

    let mut remove: Option<(String, String)> = None;
    loop {
        if let Some((ref i, ref a)) = remove {
            ingredients2allergens.insert(i.to_string(), Some(a.to_string()));
            for set in &mut ingredients {
                set.remove(i);
            }
            for set in &mut allergens {
                set.remove(a);
            }
        }
        remove = None;

        'a: for (i, set1) in ingredients.iter().enumerate() {
            for (j, set2) in ingredients.iter().enumerate().skip(i) {
                let intersect1: HashSet<_> = set1.intersection(&set2).collect();
                if intersect1.len() == 1 {
                    let allergen_intersect: HashSet<_> = allergens[i].intersection(&allergens[j]).collect();
                    if allergen_intersect.len() == 1 {
                        remove = Some((intersect1.iter().next().unwrap().to_string(), allergen_intersect.iter().next().unwrap().to_string()));
                        println!("Ingredient {} has allergen {}", intersect1.iter().next().unwrap().to_string(), allergen_intersect.iter().next().unwrap().to_string());
                        break 'a;
                    }
                }
            }
        }
        if remove.is_none() {
            break;
        }
    }
*/
    println!("\ningredients2allergens: {:?}", ingredients2allergens);
    // ingredients2allergens.iter().filter(|(_, v)| v.is_none()).count()
    // copy_for_counts.iter().flat_map(|v| v.iter()).filter(|&x| !ingredients2allergens.contains_key(x)).count()
    all_ingredients.iter().filter(|&i| !all_intersections.contains(i)).count()
    // 0
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = calculate_part1(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(calculate_part1("example.txt"), 5);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(calculate_part1("input.txt"), 0);
    }
}
