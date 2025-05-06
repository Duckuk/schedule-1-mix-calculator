pub mod search_algorithms;

use enumset::EnumSet;

use crate::{
    additives::Additives,
    effect::{self, Effect},
    ingredients::{Base, Intermediate, PseudoQuality},
};

#[derive(Clone, Debug)]
pub struct Recipe {
    base: Base,
    intermediates: Vec<Intermediate>,
}

impl Default for Recipe {
    fn default() -> Self {
        Recipe {
            base: Base::OGKush,
            intermediates: Vec::default(),
        }
    }
}

#[expect(clippy::to_string_trait_impl)]
impl ToString for Recipe {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("Base:\n");
        s.push_str(format!("  {}\n\n", self.base).as_str());

        let mut i = 1;
        s.push_str("Steps:\n");
        for ingredient in &self.intermediates {
            s.push_str(
                format!(
                    "  {i}. Add {} to the mixture and stir.\n",
                    ingredient.name()
                )
                .as_str(),
            );
            i += 1;
        }
        s.push_str(format!("  {}. ???\n  {}. Profit!\n\n", i, i + 1).as_str());

        let mut effects: Vec<_> = self.calculate_effects().into_iter().collect();
        effects.sort_by_key(|e| e.to_string());
        s.push_str("Effects:\n");
        effects
            .iter()
            .for_each(|e| s.push_str(format!("  - {}\n", e).as_str()));

        s
    }
}

#[allow(dead_code)]
impl Recipe {
    pub fn with_base(base: Base) -> Self {
        Self {
            base,
            intermediates: Vec::new(),
        }
    }

    pub fn add_intermediate(mut self, intermediate: Intermediate) -> Self {
        self.intermediates.push(intermediate);

        self
    }

    pub fn base(&self) -> Base {
        self.base
    }
    pub fn intermediates(&self) -> &Vec<Intermediate> {
        &self.intermediates
    }

    pub fn calculate_effects(&self) -> EnumSet<Effect> {
        let mut set = EnumSet::new();
        if let Some(e) = self.base.base_effect() {
            set.insert(e);
        }

        for i in &self.intermediates {
            i.apply_to_effect_set(&mut set);
        }

        set
    }

    pub fn production_cost(
        &self,
        additives: Option<Additives>,
        grow_tent: Option<bool>,
        pseudo: Option<PseudoQuality>,
    ) -> f32 {
        self.base.production_cost(additives, grow_tent, pseudo)
            + self
                .intermediates
                .iter()
                .map(|x| x.base_purchase_price())
                .sum::<f32>()
    }

    pub fn sell_price(&self) -> f32 {
        self.base.base_price() * (1.0 + effect::get_total_price_modifier(self.calculate_effects()))
    }

    pub fn profit(
        &self,
        additives: Option<Additives>,
        grow_tent: Option<bool>,
        pseudo: Option<PseudoQuality>,
    ) -> f32 {
        self.sell_price() - self.production_cost(additives, grow_tent, pseudo)
    }

    pub fn profit_margin(
        &self,
        additives: Option<Additives>,
        grow_tent: Option<bool>,
        pseudo: Option<PseudoQuality>,
    ) -> f32 {
        (self.sell_price() - self.production_cost(additives, grow_tent, pseudo)) / self.sell_price()
    }

    pub fn addictiveness(&self) -> f32 {
        self.base.base_addictiveness()
            + self
                .calculate_effects()
                .iter()
                .map(|e| e.addictiveness())
                .sum::<f32>()
    }
}
