use crate::{effect::Effect, expenses::Expenses};
use ahash::AHashMap;
use enumset::EnumSet;
use std::sync::LazyLock;

/// A base ingredient, probably for a `Recipe`.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Base {
    OGKush,
    SourDiesel,
    GreenCrack,
    GranddaddyPurple,
    Meth,
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Base::OGKush => String::from("OG Kush"),
            Base::SourDiesel => String::from("Sour Diesel"),
            Base::GreenCrack => String::from("Green Crack"),
            Base::GranddaddyPurple => String::from("Granddaddy Purple"),
            Base::Meth => String::from("Meth"),
        };

        write!(f, "{s}")
    }
}

impl Base {
    pub const ALL: &'static [Self] = &[
        Base::OGKush,
        Base::SourDiesel,
        Base::GreenCrack,
        Base::GranddaddyPurple,
        Base::Meth,
    ];

    /// Returns the addictiveness of this `Base`.
    pub fn addictiveness(&self) -> f32 {
        use Base::*;
        match self {
            OGKush => 0.0,
            SourDiesel => 0.0,
            GreenCrack => 0.0,
            GranddaddyPurple => 0.0,
            Meth => 0.60,
        }
    }

    /// Returns the base effect of this `Base`.
    pub fn effect(&self) -> Option<Effect> {
        use Base::*;
        match self {
            OGKush => Some(Effect::Calming),
            SourDiesel => Some(Effect::Refreshing),
            GreenCrack => Some(Effect::Energizing),
            GranddaddyPurple => Some(Effect::Sedating),
            Meth => None,
        }
    }

    /// Calculates the cost to produce this base as a negative `f32`.
    pub fn production_cost(&self, expenses: Expenses) -> f32 {
        use Base::*;
        let weed_batch_size = 12.0 * expenses.grow_tent_multiplier() + expenses.pgr_weed_bonus();

        const ACID_PRICE: f32 = -40.0;
        const PHOSPHOROUS_PRICE: f32 = -40.0;
        const METH_BATCH_SIZE: f32 = 10.0;

        match self {
            OGKush => (-30.0 + expenses.additives_cost() + expenses.soil_cost()) / weed_batch_size,
            SourDiesel => {
                (-35.0 + expenses.additives_cost() + expenses.soil_cost()) / weed_batch_size
            }
            GreenCrack => {
                (-40.0 + expenses.additives_cost() + expenses.soil_cost()) / weed_batch_size
            }
            GranddaddyPurple => {
                (-45.0 + expenses.additives_cost() + expenses.soil_cost()) / weed_batch_size
            }
            Meth => (expenses.pseudo_cost() + ACID_PRICE + PHOSPHOROUS_PRICE) / METH_BATCH_SIZE,
        }
    }

    /// Returns the sell price modifier of this `Base` as a postitive `f32`.
    pub fn sell_price(&self) -> f32 {
        use Base::*;
        match self {
            OGKush => 35.0,
            SourDiesel => 35.0,
            GreenCrack => 35.0,
            GranddaddyPurple => 35.0,
            Meth => 70.0,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Intermediate {
    Addy,
    Banana,
    Battery,
    Chilli,
    Cuke,
    Donut,
    EnergyDrink,
    FluMedicine,
    Gasoline,
    HorseSemen,
    Iodine,
    MegaBean,
    MotorOil,
    MouthWash,
    Paracetamol,
    Viagra,
}

impl Intermediate {
    pub const ALL: &'static [Self] = &[
        Intermediate::Addy,
        Intermediate::Banana,
        Intermediate::Battery,
        Intermediate::Chilli,
        Intermediate::Cuke,
        Intermediate::Donut,
        Intermediate::EnergyDrink,
        Intermediate::FluMedicine,
        Intermediate::Gasoline,
        Intermediate::HorseSemen,
        Intermediate::Iodine,
        Intermediate::MegaBean,
        Intermediate::MotorOil,
        Intermediate::MouthWash,
        Intermediate::Paracetamol,
        Intermediate::Viagra,
    ];

    pub fn interactions(&self) -> &'static AHashMap<Effect, Effect> {
        macro_rules! generate_interactions_static {
            ($intermediate:expr) => {{
                static STATIC_INTERMEDIATE: LazyLock<AHashMap<Effect, Effect>> =
                    std::sync::LazyLock::new(|| {
                        const RADIUS: f32 = 0.4;
                        let mut map: AHashMap<Effect, Effect> =
                            AHashMap::with_capacity(Effect::ALL.len());

                        let ingredient_mix_modifier = $intermediate.effect().mix_direction()
                            * $intermediate.effect().mix_magnitude();
                        for from_effect in Effect::ALL {
                            let mix_pos = from_effect.mix_map_postion();
                            let new_mix_pos = mix_pos + ingredient_mix_modifier;
                            let to_effect = Effect::ALL.iter().find(|e| {
                                new_mix_pos.metric_distance(&e.mix_map_postion()) <= RADIUS
                            });

                            if let Some(to_effect) = to_effect {
                                map.insert(*from_effect, *to_effect);
                            }
                        }

                        map
                    });

                &STATIC_INTERMEDIATE
            }};
        }

        match self {
            Intermediate::Addy => generate_interactions_static!(Intermediate::Addy),
            Intermediate::Banana => generate_interactions_static!(Intermediate::Banana),
            Intermediate::Battery => generate_interactions_static!(Intermediate::Battery),
            Intermediate::Chilli => generate_interactions_static!(Intermediate::Chilli),
            Intermediate::Cuke => generate_interactions_static!(Intermediate::Cuke),
            Intermediate::Donut => generate_interactions_static!(Intermediate::Donut),
            Intermediate::EnergyDrink => generate_interactions_static!(Intermediate::EnergyDrink),
            Intermediate::FluMedicine => generate_interactions_static!(Intermediate::FluMedicine),
            Intermediate::Gasoline => generate_interactions_static!(Intermediate::Gasoline),
            Intermediate::HorseSemen => generate_interactions_static!(Intermediate::HorseSemen),
            Intermediate::Iodine => generate_interactions_static!(Intermediate::Iodine),
            Intermediate::MegaBean => generate_interactions_static!(Intermediate::MegaBean),
            Intermediate::MotorOil => generate_interactions_static!(Intermediate::MotorOil),
            Intermediate::MouthWash => generate_interactions_static!(Intermediate::MouthWash),
            Intermediate::Paracetamol => generate_interactions_static!(Intermediate::Paracetamol),
            Intermediate::Viagra => generate_interactions_static!(Intermediate::Viagra),
        }
    }

    pub fn name(&self) -> String {
        use Intermediate::*;
        match self {
            Addy => String::from("Addy"),
            Banana => String::from("Banana"),
            Battery => String::from("Battery"),
            Chilli => String::from("Chilli"),
            Cuke => String::from("Cuke"),
            Donut => String::from("Donut"),
            EnergyDrink => String::from("Energy Drink"),
            FluMedicine => String::from("Flu Medicine"),
            Gasoline => String::from("Gasoline"),
            HorseSemen => String::from("Horse Semen"),
            Iodine => String::from("Iodine"),
            MegaBean => String::from("Mega Bean"),
            MotorOil => String::from("Motor Oil"),
            MouthWash => String::from("Mouth Wash"),
            Paracetamol => String::from("Paracetamol"),
            Viagra => String::from("Viagra"),
        }
    }

    /// Prefer `interactions` which returns statically calculated maps instead of being a potentially incorrect hardcoded interaction map.
    pub fn interactions_hardcoded(&self) -> AHashMap<Effect, Effect> {
        use Effect::*;
        use Intermediate::*;
        match self {
            Addy => AHashMap::from([
                (Explosive, Euphoric),
                (Foggy, Energizing),
                (Glowing, Refreshing),
                (LongFaced, Electrifying),
                (Sedating, Gingeritis),
            ]),
            Banana => AHashMap::from([
                (Calming, Sneaky),
                (Cyclopean, Energizing),
                (Disorienting, Focused),
                (Energizing, ThoughtProvoking),
                (Focused, SeizureInducing),
                (LongFaced, Refreshing),
                (Paranoia, Jennerising),
                (Smelly, AntiGravity),
                (Toxic, Smelly),
            ]),
            Battery => AHashMap::from([
                (Cyclopean, Glowing),
                (Electrifying, Euphoric),
                (Euphoric, Zombifying),
                (Laxative, CalorieDense),
                (Munchies, TropicThunder),
                (Shrinking, Munchies),
            ]),
            Chilli => AHashMap::from([
                (AntiGravity, TropicThunder),
                (Athletic, Euphoric),
                (Laxative, LongFaced),
                (Munchies, Toxic),
                (Shrinking, Refreshing),
                (Sneaky, BrightEyed),
            ]),
            Cuke => AHashMap::from([
                (Euphoric, Laxative),
                (Foggy, Cyclopean),
                (Gingeritis, ThoughtProvoking),
                (Munchies, Athletic),
                (Slippery, Munchies),
                (Sneaky, Paranoia),
                (Toxic, Euphoric),
            ]),
            Donut => AHashMap::from([
                (AntiGravity, Slippery),
                (Balding, Sneaky),
                (CalorieDense, Explosive),
                (Focused, Euphoric),
                (Jennerising, Gingeritis),
                (Munchies, Calming),
                (Shrinking, Energizing),
            ]),
            EnergyDrink => AHashMap::from([
                (Disorienting, Electrifying),
                (Euphoric, Energizing),
                (Focused, Shrinking),
                (Foggy, Laxative),
                (Glowing, Disorienting),
                (Schizophrenia, Balding),
                (Sedating, Munchies),
                (Spicy, Euphoric),
                (TropicThunder, Sneaky),
            ]),
            FluMedicine => AHashMap::from([
                (Athletic, Munchies),
                (Calming, BrightEyed),
                (Cyclopean, Foggy),
                (Electrifying, Refreshing),
                (Euphoric, Toxic),
                (Focused, Calming),
                (Laxative, Euphoric),
                (Munchies, Slippery),
                (Shrinking, Paranoia),
                (ThoughtProvoking, Gingeritis),
            ]),
            Gasoline => AHashMap::from([
                (Disorienting, Glowing),
                (Electrifying, Disorienting),
                (Energizing, Euphoric),
                (Euphoric, Spicy),
                (Gingeritis, Smelly),
                (Jennerising, Sneaky),
                (Laxative, Foggy),
                (Munchies, Sedating),
                (Paranoia, Calming),
                (Shrinking, Focused),
                (Sneaky, TropicThunder),
            ]),
            HorseSemen => AHashMap::from([
                (AntiGravity, Calming),
                (Gingeritis, Refreshing),
                (SeizureInducing, Energizing),
                (ThoughtProvoking, Electrifying),
            ]),
            Iodine => AHashMap::from([
                (Calming, Balding),
                (CalorieDense, Gingeritis),
                (Euphoric, SeizureInducing),
                (Foggy, Paranoia),
                (Refreshing, ThoughtProvoking),
                (Toxic, Sneaky),
            ]),
            MegaBean => AHashMap::from([
                (Athletic, Laxative),
                (Calming, Glowing),
                (Energizing, Cyclopean),
                (Focused, Disorienting),
                (Jennerising, Paranoia),
                (SeizureInducing, Focused),
                (Shrinking, Electrifying),
                (Slippery, Toxic),
                (Sneaky, Calming),
                (ThoughtProvoking, Energizing),
            ]),
            MotorOil => AHashMap::from([
                (Energizing, Munchies),
                (Euphoric, Sedating),
                (Foggy, Toxic),
                (Munchies, Schizophrenia),
                (Paranoia, AntiGravity),
            ]),
            MouthWash => AHashMap::from([
                (Calming, AntiGravity),
                (CalorieDense, Sneaky),
                (Explosive, Sedating),
                (Focused, Jennerising),
            ]),
            Paracetamol => AHashMap::from([
                (Calming, Slippery),
                (Electrifying, Athletic),
                (Energizing, Paranoia),
                (Focused, Gingeritis),
                (Foggy, Calming),
                (Glowing, Toxic),
                (Munchies, AntiGravity),
                (Paranoia, Balding),
                (Spicy, BrightEyed),
                (Toxic, TropicThunder),
            ]),
            Viagra => AHashMap::from([
                (Athletic, Sneaky),
                (Disorienting, Toxic),
                (Euphoric, BrightEyed),
                (Laxative, Calming),
                (Shrinking, Gingeritis),
            ]),
        }
    }

    pub fn interaction(&self, effect: &Effect) -> Option<Effect> {
        let interactions = self.interactions();
        interactions.get(effect).copied()
    }

    pub fn apply_to_effect_set(&self, effect_set: &mut EnumSet<Effect>) {
        let frozen_effect_set = *effect_set;

        if effect_set.len() < 8 {
            effect_set.insert(self.effect());
        }

        for (from_effect, to_effect) in self.interactions() {
            if frozen_effect_set.contains(*from_effect) && !frozen_effect_set.contains(*to_effect) {
                effect_set.remove(*from_effect);
                effect_set.insert(*to_effect);
            }
        }
    }

    /// Returns the price of this ingredient as a negative `f32`.
    pub fn purchase_price(&self) -> f32 {
        use Intermediate::*;
        match self {
            Addy => -9.0,
            Banana => -2.0,
            Battery => -8.0,
            Chilli => -7.0,
            Cuke => -2.0,
            Donut => -3.0,
            EnergyDrink => -6.0,
            FluMedicine => -5.0,
            Gasoline => -5.0,
            HorseSemen => -9.0,
            Iodine => -8.0,
            MegaBean => -7.0,
            MotorOil => -6.0,
            MouthWash => -4.0,
            Paracetamol => -3.0,
            Viagra => -4.0,
        }
    }

    pub fn effect(&self) -> Effect {
        use Intermediate::*;
        match self {
            Addy => Effect::ThoughtProvoking,
            Banana => Effect::Gingeritis,
            Battery => Effect::BrightEyed,
            Chilli => Effect::Spicy,
            Cuke => Effect::Energizing,
            Donut => Effect::CalorieDense,
            EnergyDrink => Effect::Athletic,
            FluMedicine => Effect::Sedating,
            Gasoline => Effect::Toxic,
            HorseSemen => Effect::LongFaced,
            Iodine => Effect::Jennerising,
            MegaBean => Effect::Foggy,
            MotorOil => Effect::Slippery,
            MouthWash => Effect::Balding,
            Paracetamol => Effect::Sneaky,
            Viagra => Effect::TropicThunder,
        }
    }
}

impl std::fmt::Display for Intermediate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Intermediate::Addy => String::from("Addy"),
            Intermediate::Banana => String::from("Banana"),
            Intermediate::Battery => String::from("Battery"),
            Intermediate::Chilli => String::from("Chilli"),
            Intermediate::Cuke => String::from("Cuke"),
            Intermediate::Donut => String::from("Donut"),
            Intermediate::EnergyDrink => String::from("Energy Drink"),
            Intermediate::FluMedicine => String::from("Flu Medicine"),
            Intermediate::Gasoline => String::from("Gasoline"),
            Intermediate::HorseSemen => String::from("Horse Semen"),
            Intermediate::Iodine => String::from("Iodine"),
            Intermediate::MegaBean => String::from("Mega Bean"),
            Intermediate::MotorOil => String::from("Motor Oil"),
            Intermediate::MouthWash => String::from("Mouth Wash"),
            Intermediate::Paracetamol => String::from("Paracetamol"),
            Intermediate::Viagra => String::from("Viagra"),
        };

        write!(f, "{s}")
    }
}
