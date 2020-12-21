use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};

type Allergen = String;
type Ingredient = String;
#[derive(Debug)]
pub struct Food {
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

#[aoc_generator(day21)]
pub fn input_parser(input: &str) -> Vec<Food> {
    // abc def (contains ghi, jkl)
    input
        .lines()
        .map(|s| {
            let mut split = s.split(" (contains ");
            let ingredients = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            let allergens = split
                .next()
                .unwrap()
                .strip_suffix(")")
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

fn map_allergens(foods: &[Food]) -> HashMap<&Allergen, HashSet<Ingredient>> {
    let allergens = foods.iter().fold(HashSet::new(), |mut hs, f| {
        hs.extend(&f.allergens);
        hs
    });

    allergens
        .into_iter()
        .map(|allergen| {
            let ingredients = foods
                .iter()
                .filter(|f| f.allergens.contains(allergen))
                .fold(HashSet::new(), |hs, f| {
                    if hs.is_empty() {
                        f.ingredients.clone()
                    } else {
                        hs.intersection(&f.ingredients).cloned().collect()
                    }
                });
            (allergen, ingredients)
        })
        .collect()
}

#[aoc(day21, part1)]
pub fn part1(foods: &[Food]) -> usize {
    let allergens = map_allergens(foods);

    let ingredients_with_allergens: HashSet<&Ingredient> =
        allergens.values().flat_map(|f| f.iter()).collect();

    foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|ing| !ingredients_with_allergens.contains(ing))
        .count()
}

#[aoc(day21, part2)]
pub fn part2(foods: &[Food]) -> String {
    let mut allergens = map_allergens(foods);

    let mut allergen_list: Vec<(&Allergen, Ingredient)> = Vec::new();
    loop {
        let ingredient: Ingredient;
        if let Some((allergen, ingredients)) = allergens.iter_mut().find(|(_, v)| v.len() == 1) {
            ingredient = ingredients.drain().next().unwrap();
            allergen_list.push((allergen.clone(), ingredient.clone()));
        } else {
            break;
        }

        for ingrs in allergens.values_mut() {
            ingrs.remove(&ingredient);
        }
    }

    allergen_list.sort();
    let res: Vec<Ingredient> = allergen_list.into_iter().map(|(_, ingr)| ingr).collect();
    res.join(",")
}

#[cfg(test)]
mod test_day21 {
    use super::*;

    const TESTCASE: &str = "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), "mxmxvkd,sqjhc,fvjkl")
    }
}
