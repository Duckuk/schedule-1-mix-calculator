use super::Recipe;
use crate::ingredients::{Base, Intermediate};
use rayon::prelude::*;

// Searches through all possible recipes for one that maximizes the value given by the key function F
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

// Searches through all possible recipes for one that matches the predicate F
pub fn search_for_recipe_find_dfs<F>(root: Recipe, f: F, depth: i8) -> Option<Recipe>
where
    F: Fn(&Recipe) -> bool + Sync + std::marker::Send + Clone,
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