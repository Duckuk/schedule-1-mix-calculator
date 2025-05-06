use super::Recipe;
use crate::ingredients::{Base, Intermediate};
use rayon::prelude::*;

/// A brute-force DFS search algorithm that goes through all possible recipes starting from `root` and returns the recipe with the maximum value computed by the given function.  
/// 
/// The size of the search space is about `16^depth`. Depths greater than `5` tend to produce exponentially unreasonable search times.
pub fn search_for_recipe_max_dfs<K, F>(root: Recipe, f: F, depth: i8) -> Recipe
where
    K: Ord + Send,
    F: Fn(&Recipe) -> K + Sync + std::marker::Send + Clone,
{
    if depth <= 0 {
        return root;
    }

    let best_recipe: Recipe = Intermediate::ALL
        .par_iter()
        .map(|i| search_for_recipe_max_dfs(root.clone().add_intermediate(*i), f.clone(), depth - 1))
        .chain([root.clone()])
        .max_by_key(f.clone())
        .expect("PANIC AAAAHHHH");

    best_recipe
}

/// A brute-force iterative deepening depth-first search algorithm that goes through all possible recipes starting from `root` and returns the first recipe that satifies the given predicate.  
/// 
/// Differs from a traditional depth-first search in that it gradually increases the depth until it finds a match. This means that it mimics the search pattern of a breadth-first search while keeping the small memory footprint of a depth-first search.
///
/// The size of the search space is about `16^depth`. Depths greater than `5` tend to produce exponentially unreasonable search times.
pub fn search_for_recipe_find_iddfs<F>(f: F, depth: i8) -> Option<Recipe>
where
    F: Fn(&Recipe) -> bool + Sync + std::marker::Send,
{
    for depth in 0..=depth {
        let matching_recipe = Base::ALL
            .iter()
            .find_map(|b| search_for_recipe_find_dfs(Recipe::with_base(*b), &f, depth));
        if let Some(r) = matching_recipe {
            return Some(r);
        }
    }

    None
}

/// A brute-force depth-first search algorithm that goes through all possible recipes starting from `root` and returns the first recipe that satifies the given predicate.  
/// 
/// The size of the search space is about `16^depth`. Depths greater than `5` tend to produce exponentially unreasonable search times.
pub fn search_for_recipe_find_dfs<P>(root: Recipe, f: P, depth: i8) -> Option<Recipe>
where
    P: Fn(&Recipe) -> bool + Sync + std::marker::Send + Clone,
{
    if depth <= 0 {
        return f(&root).then_some(root);
    }

    let best_recipe = Intermediate::ALL
        .par_iter()
        .map(|i| {
            search_for_recipe_find_dfs(root.clone().add_intermediate(*i), f.clone(), depth - 1)
        })
        .filter_map(|r| r)
        .chain([root.clone()])
        .find_any(f.clone());

    best_recipe
}