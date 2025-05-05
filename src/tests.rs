use crate::ingredients::{Base, Intermediate};
use crate::recipe::Recipe;
use crate::search_for_recipe_max_dfs;

#[allow(unused_macros)]
macro_rules! time {
    ($($stmt:stmt)*) => {
        let then = std::time::Instant::now();
        $($stmt)*
        println!("{:.2}ms", then.elapsed().as_micros() as f64 / 1000.);
    };
}

#[test]
fn generic_test() {
    search_for_recipe_max_dfs(
        Recipe::with_base(Base::Meth),
        |r| (100.0 * r.sell_price()) as i64,
        8,
    );
}

#[test]
fn calculate_interactions_test() {
    for ingredient in Intermediate::ALL {
        assert_eq!(
            ingredient.interactions_hardcoded(),
            ingredient.interactions().clone()
        );
    }
}
