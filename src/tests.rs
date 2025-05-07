use crate::effect::Effect;
use crate::expenses::Expenses;
use crate::ingredients::{Base, Intermediate};
use crate::recipe::Recipe;

#[expect(unused_macros)]
macro_rules! time {
    ($($stmt:stmt)*) => {
        let then = std::time::Instant::now();
        $($stmt)*
        println!("{:.2}ms", then.elapsed().as_micros() as f64 / 1000.);
    };
}

#[test]
fn calculate_effects_test() {
    let donut = Recipe::with_base(Base::GreenCrack)
        .add_intermediate(Intermediate::Paracetamol)
        .add_intermediate(Intermediate::Donut)
        .add_intermediate(Intermediate::Cuke)
        .add_intermediate(Intermediate::Banana);
    assert_eq!(
        donut.calculate_effects(),
        Effect::CalorieDense
            | Effect::Gingeritis
            | Effect::Jennerising
            | Effect::Sneaky
            | Effect::ThoughtProvoking
    )
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

#[test]
fn test() {
    let mut e = Expenses::default();
    e.additives.insert(crate::expenses::Additive::PGR);
    let recipe = Recipe::with_base(Base::Meth).add_intermediate(Intermediate::Addy);
    println!("{}", recipe.production_cost(e));
}
