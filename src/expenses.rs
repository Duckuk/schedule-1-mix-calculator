use enumset::{EnumSet, EnumSetType};

/// TODO: convert this to an `EnumSet`

#[derive(Clone, Copy)]
pub struct Expenses {
    pub additives: EnumSet<Additive>,
    pub pseudo: PseudoQuality,
    pub grow_tent: bool,
    pub soil: Soil,
}

impl Default for Expenses {
    fn default() -> Self {
        Expenses {
            additives: EnumSet::new(),
            pseudo: PseudoQuality::Low,
            grow_tent: false,
            soil: Soil::Normal,
        }
    }
}

impl Expenses {
    /// Returns the cost of the additives as a negative `f32`.
    pub fn additives_cost(&self) -> f32 {
        -30.0 * self.additives.len() as f32
    }

    /// Returns the weed plant quantity bonus from PGR.
    pub fn pgr_weed_bonus(&self) -> f32 {
        match self.additives.contains(Additive::PGR) {
            true => 4.0,
            false => 0.0,
        }
    }

    /// Returns the coca plant quantity bonus from PGR.
    pub fn pgr_cocaine_bonus(&self) -> f32 {
        match self.additives.contains(Additive::PGR) {
            true => match self.grow_tent {
                true => 5.0,
                false => 7.0,
            },
            false => 0.0,
        }
    }

    /// Returns the cost of the pseudo as a negative `f32`.
    pub fn pseudo_cost(&self) -> f32 {
        match self.pseudo {
            PseudoQuality::Low => -60.0,
            PseudoQuality::Medium => -80.0,
            PseudoQuality::High => -110.0,
        }
    }

    /// Returns the grow tent quantity penalty as a multiplier.
    pub fn grow_tent_multiplier(&self) -> f32 {
        match self.grow_tent {
            true => 2.0 / 3.0,
            false => 1.0,
        }
    }

    /// Returns the cost of the soil as a negative `f32`.
    pub fn soil_cost(&self) -> f32 {
        match self.soil {
            Soil::Normal => -10.0,
            Soil::LongLife => -30.0 / 2.0,
            Soil::ExtraLongLife => -60.0 / 3.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PseudoQuality {
    Low,
    Medium,
    High,
}

impl PseudoQuality {
    pub const ALL: &'static [Self] = &[
        PseudoQuality::Low,
        PseudoQuality::Medium,
        PseudoQuality::High,
    ];
}

impl std::fmt::Display for PseudoQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PseudoQuality::Low => String::from("Low Quality Pseudo"),
            PseudoQuality::Medium => String::from("Medium Quality Pseudo"),
            PseudoQuality::High => String::from("High Quality Pseudo"),
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Soil {
    Normal,
    LongLife,
    ExtraLongLife,
}

impl std::fmt::Display for Soil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Soil::Normal => "Soil",
            Soil::LongLife => "Long-Life Soil",
            Soil::ExtraLongLife => "Extra Long-Life Soil",
        };

        write!(f, "{s}")
    }
}

impl Soil {
    pub const ALL: &'static [Self] = &[Soil::Normal, Soil::LongLife, Soil::ExtraLongLife];
}

#[derive(EnumSetType)]
pub enum Additive {
    PGR,
    SpeedGrow,
    Fertilizer,
}
