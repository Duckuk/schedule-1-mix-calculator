/// TODO: convert this to an `EnumSet`

#[derive(Clone, Copy)]
#[derive(Default)]
pub struct Additives {
    pub pgr: bool,
    pub speed_grow: bool,
    pub fertilizer: bool,
}


#[allow(dead_code)]
impl Additives {
    pub fn new() -> Self {
        Additives::default()
    }

    pub fn pgr(mut self) -> Self {
        self.pgr = true;
        self
    }

    pub fn speed_grow(mut self) -> Self {
        self.speed_grow = true;
        self
    }

    pub fn fertilizer(mut self) -> Self {
        self.fertilizer = true;
        self
    }

    pub fn price_offset(&self) -> f32 {
        let mut offset = 0.0;
        if self.fertilizer {
            offset += 30.0;
        }

        if self.pgr {
            offset += 30.0
        }

        if self.speed_grow {
            offset += 30.0
        }

        offset
    }
}
