pub mod additives;
pub mod effect;
pub mod ingredients;
pub mod recipe;

#[cfg(test)]
mod tests;

use additives::Additives;
use effect::Effect;
use enumset::EnumSet;
use iced::{
    Alignment, Element, Length, Padding, Task, Theme,
    widget::{
        button, checkbox, column, container, horizontal_space, pick_list, progress_bar, row,
        scrollable, text,
    },
};
use ingredients::{Base, Intermediate, PseudoQuality};
use recipe::Recipe;
use recipe::search_algorithms::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SortType {
    AlphabeticalAscending,
    AlphabeticalDescending,
    AddictivenessAscending,
    AddictivenessDescending,
    PriceModifierAscending,
    PriceModifierDescending,
}

impl std::fmt::Display for SortType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SortType::AlphabeticalAscending => String::from("Name (A-Z)"),
            SortType::AlphabeticalDescending => String::from("Name (Z-A)"),
            SortType::AddictivenessAscending => String::from("Addictiveness (low to high)"),
            SortType::AddictivenessDescending => String::from("Addictiveness (high to low)"),
            SortType::PriceModifierAscending => String::from("Price Modifier (low to high)"),
            SortType::PriceModifierDescending => String::from("Price Modifier (high to low)"),
        };

        write!(f, "{s}")
    }
}

impl SortType {
    pub const ALL: &'static [Self] = &[
        SortType::AlphabeticalAscending,
        SortType::AlphabeticalDescending,
        SortType::AddictivenessAscending,
        SortType::AddictivenessDescending,
        SortType::PriceModifierAscending,
        SortType::PriceModifierDescending,
    ];

    pub fn sort_function(&self) -> impl FnMut(&Effect, &Effect) -> std::cmp::Ordering {
        use SortType::*;
        match self {
            AlphabeticalAscending => |a: &Effect, b: &Effect| a.to_string().cmp(&b.to_string()),
            AlphabeticalDescending => |a: &Effect, b: &Effect| b.to_string().cmp(&a.to_string()),
            AddictivenessAscending => {
                |a: &Effect, b: &Effect| a.addictiveness().total_cmp(&b.addictiveness())
            }
            AddictivenessDescending => {
                |a: &Effect, b: &Effect| b.addictiveness().total_cmp(&a.addictiveness())
            }
            PriceModifierAscending => {
                |a: &Effect, b: &Effect| a.price_modifier().total_cmp(&b.price_modifier())
            }
            PriceModifierDescending => {
                |a: &Effect, b: &Effect| b.price_modifier().total_cmp(&a.price_modifier())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    ReverseCalculator,
    OptimalCalculator,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::ReverseCalculator => String::from("Reverse"),
            Mode::OptimalCalculator => String::from("Optimal"),
        };

        write!(f, "{s}")
    }
}

impl Mode {
    pub const ALL: &'static [Self] = &[Mode::ReverseCalculator, Mode::OptimalCalculator];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Metric {
    ProfitMargin,
    Profit,
    SellPrice,
    ProductionCost,
}

impl std::fmt::Display for Metric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Metric::ProfitMargin => String::from("Profit Margin"),
            Metric::Profit => String::from("Profit"),
            Metric::SellPrice => String::from("Sell Price"),
            Metric::ProductionCost => String::from("Production Cost"),
        };

        write!(f, "{s}")
    }
}

impl Metric {
    pub const ALL: &'static [Self] = &[
        Metric::ProductionCost,
        Metric::SellPrice,
        Metric::Profit,
        Metric::ProfitMargin,
    ];
}

#[derive(Clone, Debug)]
enum Message {
    ChangedMode(Mode),

    AddedEffect(Effect),
    RemovedEffect(Effect),
    UpdatedTargetEffects,

    ChangedSortType(SortType),

    ChangedBase(Base),
    ChangedMetric(Metric),
    ChangedDepth(u8),

    ToggledGrowTent(bool),
    ToggledPGR(bool),
    ToggledFertilizer(bool),
    ToggledSpeedGrow(bool),
    ChangedPseudo(PseudoQuality),

    CalculateRecipe,
    CalculateRecipeFinished(Option<Recipe>),

    ChangedTheme(Theme),
}

struct MixCalculator {
    mode: Mode,

    // Reverse calculator
    sort_type_selected: Option<SortType>,
    target_effects: EnumSet<Effect>,

    // Optimal calculator
    base_selected: Base,
    metric_selected: Metric,
    depth_selected: u8,

    additives: Additives,
    grow_tent: bool,
    pseudo: PseudoQuality,

    calculating_recipe: bool,
    active_recipe: Option<Recipe>,

    progress_state: f32,

    theme: Theme,
}

impl Default for MixCalculator {
    fn default() -> Self {
        let default_recipe = Recipe::with_base(Base::OGKush)
            .add_intermediate(Intermediate::Paracetamol)
            .add_intermediate(Intermediate::Cuke)
            .add_intermediate(Intermediate::Donut)
            .add_intermediate(Intermediate::Banana);
        Self {
            mode: Mode::ReverseCalculator,

            // Reverse calculator
            target_effects: Effect::Jennerising
                | Effect::Sneaky
                | Effect::Gingeritis
                | Effect::ThoughtProvoking
                | Effect::CalorieDense,
            sort_type_selected: Some(SortType::AlphabeticalAscending),

            // Optimal calculator
            base_selected: Base::OGKush,
            metric_selected: Metric::ProfitMargin,
            depth_selected: 4,

            additives: Additives::default(),
            grow_tent: false,
            pseudo: PseudoQuality::Low,

            calculating_recipe: false,
            active_recipe: Some(default_recipe),

            progress_state: Default::default(),

            theme: Theme::Nightfly,
        }
    }
}

impl MixCalculator {
    fn total_addictiveness(&self) -> f32 {
        effect::get_total_addictiveness(self.target_effects)
    }

    fn total_price_modifier(&self) -> f32 {
        effect::get_total_price_modifier(self.target_effects)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ChangedMode(mode) => {
                self.mode = mode;
                Task::none()
            }
            Message::AddedEffect(effect) => {
                self.target_effects.insert(effect);
                self.update(Message::UpdatedTargetEffects)
            }
            Message::RemovedEffect(effect) => {
                self.target_effects.remove(effect);
                self.update(Message::UpdatedTargetEffects)
            }
            Message::UpdatedTargetEffects => Task::none(),
            Message::ChangedSortType(sort_type) => {
                self.sort_type_selected = Some(sort_type);
                Task::none()
            }
            Message::ChangedBase(base) => {
                self.base_selected = base;
                Task::none()
            }
            Message::ChangedMetric(metric) => {
                self.metric_selected = metric;
                Task::none()
            }
            Message::ChangedDepth(depth) => {
                self.depth_selected = depth;
                Task::none()
            }
            Message::ToggledGrowTent(b) => {
                self.grow_tent = b;
                Task::none()
            }
            Message::ToggledPGR(b) => {
                self.additives.pgr = b;
                Task::none()
            }
            Message::ToggledFertilizer(b) => {
                self.additives.fertilizer = b;
                Task::none()
            }
            Message::ToggledSpeedGrow(b) => {
                self.additives.speed_grow = b;
                Task::none()
            }
            Message::ChangedPseudo(pseudo) => {
                self.pseudo = pseudo;
                Task::none()
            }
            Message::ChangedTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::CalculateRecipe if self.mode == Mode::OptimalCalculator => {
                self.calculating_recipe = true;

                let root = Recipe::with_base(self.base_selected);
                let f = match self.metric_selected {
                    Metric::ProfitMargin => |r: &Recipe| {
                        (100.0 * r.addictiveness().clamp(f32::MIN_POSITIVE, 1.0)) as i64
                            + (100.0 * r.profit_margin(None, None, None)) as i64
                    },
                    Metric::Profit => |r: &Recipe| {
                        (100.0 * r.addictiveness().clamp(f32::MIN_POSITIVE, 1.0)) as i64
                            + (100.0 * r.profit(None, None, None)) as i64
                    },
                    Metric::SellPrice => |r: &Recipe| {
                        (100.0 * r.addictiveness().clamp(f32::MIN_POSITIVE, 1.0)) as i64
                            + (100.0 * r.sell_price()) as i64
                    },
                    Metric::ProductionCost => |r: &Recipe| {
                        (100.0 * r.addictiveness().clamp(f32::MIN_POSITIVE, 1.0)) as i64
                            + (100.0 * -r.production_cost(None, None, None)) as i64
                    },
                };
                let depth = self.depth_selected as i8;
                Task::perform(
                    async move { search_for_recipe_max_dfs(root, f, depth) },
                    |r| Message::CalculateRecipeFinished(Some(r)),
                )
            }
            Message::CalculateRecipe => {
                self.calculating_recipe = true;
                let target_effects = self.target_effects;
                Task::perform(
                    async move {
                        search_for_recipe_find_iddfs(
                            move |r| target_effects.is_subset(r.calculate_effects()),
                            8,
                        )
                    },
                    Message::CalculateRecipeFinished,
                )
            }
            Message::CalculateRecipeFinished(recipe) => {
                self.calculating_recipe = false;
                match recipe {
                    Some(r) => {
                        self.active_recipe = Some(r);
                    }
                    None => {
                        self.active_recipe = None;
                    }
                }
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let toolbar = self.toolbar();
        let body = match self.mode {
            Mode::ReverseCalculator => self.body_reverse_calculator(),
            Mode::OptimalCalculator => self.body_optimal_calculator(),
        };
        let footer = self.footer();

        column![toolbar, body, footer]
            .width(Length::FillPortion(1))
            .height(Length::FillPortion(1))
            .into()
    }

    fn toolbar(&self) -> Element<'_, Message> {
        let mode_picker = pick_list(Mode::ALL, Some(self.mode), Message::ChangedMode)
            .text_size(12)
            .padding(2);

        let theme_picker = pick_list(Theme::ALL, Some(self.theme()), Message::ChangedTheme)
            .text_size(12)
            .padding(2);

        let grow_tent_checkbox =
            checkbox("Grow Tent", self.grow_tent).on_toggle(Message::ToggledGrowTent);
        let pgr_checkbox = checkbox("PGR", self.additives.pgr).on_toggle(Message::ToggledPGR);
        let fertilizer_checkbox =
            checkbox("Fertilizer", self.additives.fertilizer).on_toggle(Message::ToggledFertilizer);
        let speedgrow_checkbox =
            checkbox("Speed Grow", self.additives.speed_grow).on_toggle(Message::ToggledSpeedGrow);
        let pseudo_picker =
            pick_list(PseudoQuality::ALL, Some(self.pseudo), Message::ChangedPseudo).text_size(10);

        row![
            row![text("Mode").size(15), mode_picker,].spacing(5),
            row![text("Theme").size(15), theme_picker,].spacing(5),
            horizontal_space(),
            grow_tent_checkbox,
            pgr_checkbox,
            fertilizer_checkbox,
            speedgrow_checkbox,
            pseudo_picker
        ]
        .align_y(Alignment::Center)
        .spacing(15)
        .padding(5)
        .into()
    }

    fn recipe_button(&self) -> Element<'_, Message> {
        let on_press = (!self.calculating_recipe).then_some(Message::CalculateRecipe);
        let button_text = match self.calculating_recipe {
            false => "Search for recipe",
            true => "Working...",
        };

        button(button_text).on_press_maybe(on_press).into()
    }

    fn recipe_container(&self) -> Element<'_, Message> {
        container(match &self.active_recipe {
            Some(r) => scrollable(text(r.to_string())),
            None => scrollable("No recipe found!"),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(container::rounded_box)
        .padding(10)
        .into()
    }

    fn recipe_info(&self) -> Element<'_, Message> {
        let r = self.active_recipe.clone().unwrap_or_default();

        let production_cost = text(format!(
            "~${:.0}",
            r.production_cost(
                Some(self.additives),
                Some(self.grow_tent),
                Some(self.pseudo)
            )
        ))
        .size(15);
        let sell_price = text(format!("${:.0}", r.sell_price())).size(15);
        let profit_margin = text(format!(
            "{:.1}%",
            100.0
                * r.profit_margin(
                    Some(self.additives),
                    Some(self.grow_tent),
                    Some(self.pseudo)
                )
        ))
        .size(15);
        let addictiveness = text(format!(
            "{:.0}%",
            100.0 * r.addictiveness().clamp(f32::MIN_POSITIVE, 1.0)
        ));

        row![
            horizontal_space(),
            column![text("Production Cost"), production_cost].align_x(Alignment::Center),
            column![text("Sell Price"), sell_price].align_x(Alignment::Center),
            column![text("Profit Margin"), profit_margin].align_x(Alignment::Center),
            column![text("Addictiveness"), addictiveness].align_x(Alignment::Center),
            horizontal_space(),
        ]
        .spacing(20)
        .into()
    }

    fn recipe_column(&self) -> Element<'_, Message> {
        let calculate_recipe_button = self.recipe_button();

        let recipe_container = self.recipe_container();

        let recipe_info_box = self.recipe_info();

        column![calculate_recipe_button, recipe_container, recipe_info_box]
            .spacing(5)
            .into()
    }

    fn body_reverse_calculator(&self) -> Element<'_, Message> {
        // Sort picker
        let sort_picker = pick_list(
            SortType::ALL,
            self.sort_type_selected,
            Message::ChangedSortType,
        );

        // List of available effects
        let mut available_effects: Vec<Effect> = Effect::ALL
            .iter()
            .copied()
            .filter(|e| !self.target_effects.contains(*e))
            .collect();
        available_effects.sort_by(
            self.sort_type_selected
                .unwrap_or(SortType::AlphabeticalAscending)
                .sort_function(),
        );
        let available_effects_list = container(scrollable(
            column(
                available_effects
                    .iter()
                    .map(|e| {
                        let title = e.to_string();
                        let details = format!(
                            "Addictiveness: {:.2} | Price Modifier: {:.2}",
                            e.addictiveness(),
                            e.price_modifier()
                        );
                        let button_text =
                            column([text(title).into(), text(details).size(10).into()]);
                        let message =
                            (self.target_effects.len() < 8).then_some(Message::AddedEffect(*e));
                        button(button_text)
                            .on_press_maybe(message)
                            .width(Length::FillPortion(1))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .padding(5)
            .spacing(5),
        ))
        .style(container::rounded_box)
        .width(Length::FillPortion(1));

        // Target effects section
        // List of selected effects
        let mut target_effects: Vec<Effect> = Effect::ALL
            .iter()
            .copied()
            .filter(|e| self.target_effects.contains(*e))
            .collect();
        target_effects.sort_by(
            self.sort_type_selected
                .unwrap_or(SortType::AlphabeticalAscending)
                .sort_function(),
        );
        let target_effects_list = container(scrollable(
            column(
                target_effects
                    .iter()
                    .map(|e| {
                        let header = e.to_string();
                        let details = format!(
                            "Addictiveness: {:.2} | Price Modifier: {:.2}",
                            e.addictiveness(),
                            e.price_modifier()
                        );
                        let button_text =
                            column([text(header).into(), text(details).size(10).into()]);
                        button(button_text)
                            .on_press(Message::RemovedEffect(*e))
                            .width(Length::FillPortion(1))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            )
            .padding(5)
            .spacing(5),
        ))
        .style(container::rounded_box)
        .width(Length::FillPortion(1))
        .height(Length::FillPortion(1));

        // Info about selected effects
        let price_modifier = text(format!(
            "{:+.0}%",
            (100.0 * self.total_price_modifier().max(f32::MIN_POSITIVE)).floor()
        ))
        .size(15);
        let sell_price = text(format!(
            "~${:.0}",
            Base::OGKush.base_price() * (1.0 + self.total_price_modifier().max(f32::MIN_POSITIVE))
        ))
        .size(15);
        let addictiveness = text(format!(
            "{:.0}%",
            (100.0 * self.total_addictiveness().clamp(f32::MIN_POSITIVE, 1.0)).floor()
        ))
        .size(15);

        let effects_info_box = row![
            column![text("Price Modifier"), price_modifier].align_x(Alignment::Center),
            horizontal_space(),
            column![text("Sell Price"), sell_price].align_x(Alignment::Center),
            horizontal_space(),
            column![text("Addictiveness"), addictiveness].align_x(Alignment::Center)
        ]
        .padding(10);

        let target_effects_section = column![target_effects_list, effects_info_box];

        let recipe_section = self.recipe_column();

        column![
            sort_picker,
            row![
                available_effects_list,
                target_effects_section,
                recipe_section
            ]
            .spacing(20)
        ]
        .spacing(5)
        .padding(Padding::from([5, 10]))
        .into()
    }

    fn body_optimal_calculator(&self) -> Element<'_, Message> {
        let base_picker =
            pick_list(Base::ALL, Some(self.base_selected), Message::ChangedBase).text_size(12);
        let metric_picker = pick_list(
            Metric::ALL,
            Some(self.metric_selected),
            Message::ChangedMetric,
        )
        .text_size(12);

        let depth_range: Vec<_> = (0..=8).collect();
        let depth_slider = pick_list(
            depth_range,
            Some(self.depth_selected),
            Message::ChangedDepth,
        );

        let options_panel = container(scrollable(
            column![
                column![text("Base"), base_picker]
                    .spacing(5)
                    .align_x(Alignment::Center),
                column![text("Metric"), metric_picker]
                    .spacing(5)
                    .align_x(Alignment::Center),
                column![text("Depth"), depth_slider]
                    .spacing(5)
                    .align_x(Alignment::Center)
            ]
            .spacing(10)
            .align_x(Alignment::Center),
        ))
        .height(Length::Fill)
        .padding(20);

        row![
            column![options_panel, self.recipe_button()].align_x(Alignment::Center),
            column![self.recipe_container(), self.recipe_info()]
        ]
        .spacing(20)
        .padding(Padding::from([5, 10]))
        .into()
    }

    fn footer(&self) -> Element<'_, Message> {
        let exponentiation_warning = if self.mode == Mode::ReverseCalculator
            && self.target_effects.len() > 5
        {
            text(
                "!! Selecting more than 5 effects will create exponentially unreasonable calculation times !!",
            )
        } else if self.mode == Mode::OptimalCalculator && self.depth_selected > 5 {
            text(
                "!! Depths higher than 5 will create exponentially unreasonable calculation times !!",
            )
        } else {
            text("")
        };
        let progress_bar = progress_bar(0.0..=1.0, self.progress_state)
            .height(5)
            .width(100);

        row![exponentiation_warning, horizontal_space(), progress_bar]
            .align_y(Alignment::Center)
            .spacing(5)
            .padding(5)
            .into()
    }
}

fn main() -> iced::Result {
    iced::application(
        "Duck's Mixing Calculator",
        MixCalculator::update,
        MixCalculator::view,
    )
    .theme(MixCalculator::theme)
    .run()
}
