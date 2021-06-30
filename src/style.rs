use iced::{container, radio, rule, scrollable};


use iced::{button, Color, Background};

pub struct ButtonStyle;

impl button::StyleSheet for ButtonStyle {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0x0b, 0x9d, 0x00))),
            text_color: Color::from_rgb8(0xEE, 0xEE, 0xEE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            text_color: Color::WHITE,
            ..self.active()
        }
    }
}

pub struct IconButton;

impl button::StyleSheet for IconButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0x90, 0x90, 0x90))),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0xde, 0xde, 0xde))),
            ..self.active()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Light, Theme::Dark];
}

impl Default for Theme {
    fn default() -> Theme {
        Theme::Light
    }
}

impl From<Theme> for Box<dyn container::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Container.into(),
        }
    }
}

impl From<Theme> for Box<dyn radio::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Radio.into(),
        }
    }
}

impl From<Theme> for Box<dyn scrollable::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Scrollable.into(),
        }
    }
}

impl From<Theme> for Box<dyn rule::StyleSheet> {
    fn from(theme: Theme) -> Self {
        match theme {
            Theme::Light => Default::default(),
            Theme::Dark => dark::Rule.into(),
        }
    }
}

mod dark {
    use iced::{container, radio, rule, scrollable, Color};

    const BACKGROUND: Color = Color::from_rgb(
        0x1e as f32 / 255.0,
        0x1e as f32 / 255.0,
        0x1e as f32 / 255.0,
    );

    const SURFACE: Color = Color::from_rgb(
        0x40 as f32 / 255.0,
        0x44 as f32 / 255.0,
        0x4B as f32 / 255.0,
    );

    const ACCENT: Color = Color::from_rgb(
        0x6F as f32 / 255.0,
        0xFF as f32 / 255.0,
        0xE9 as f32 / 255.0,
    );

    const ACTIVE: Color = Color::from_rgb(
        0x72 as f32 / 255.0,
        0x89 as f32 / 255.0,
        0xDA as f32 / 255.0,
    );

    const SCROLLBAR: Color = Color::from_rgb(
        0x2E as f32 / 255.0,
        0x33 as f32 / 255.0,
        0x38 as f32 / 255.0,
    );

    const SCROLLER: Color = Color::from_rgb(
        0x20 as f32 / 255.0,
        0x22 as f32 / 255.0,
        0x25 as f32 / 255.0,
    );

    pub struct Container;

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Color {
                    ..BACKGROUND
                }
                .into(),
                text_color: Color::WHITE.into(),
                ..container::Style::default()
            }
        }
    }

    pub struct Radio;

    impl radio::StyleSheet for Radio {
        fn active(&self) -> radio::Style {
            radio::Style {
                background: SURFACE.into(),
                dot_color: ACTIVE,
                border_width: 1.0,
                border_color: ACTIVE,
            }
        }

        fn hovered(&self) -> radio::Style {
            radio::Style {
                background: Color { a: 0.5, ..SURFACE }.into(),
                ..self.active()
            }
        }
    }

    pub struct Scrollable;

    impl scrollable::StyleSheet for Scrollable {
        fn active(&self) -> scrollable::Scrollbar {
            scrollable::Scrollbar {
                background: Color {
                    a: 0.8,
                    ..SCROLLBAR
                }
                .into(),
                border_radius: 2.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                scroller: scrollable::Scroller {
                    color: Color { a: 0.7, ..SCROLLER },
                    border_radius: 2.0,
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
            }
        }

        fn hovered(&self) -> scrollable::Scrollbar {
            let active = self.active();

            scrollable::Scrollbar {
                background: SCROLLBAR.into(),
                scroller: scrollable::Scroller {
                    color: SCROLLER,
                    ..active.scroller
                },
                ..active
            }
        }

        fn dragging(&self) -> scrollable::Scrollbar {
            let hovered = self.hovered();

            scrollable::Scrollbar {
                scroller: scrollable::Scroller {
                    color: ACCENT,
                    ..hovered.scroller
                },
                ..hovered
            }
        }
    }

    pub struct Rule;

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: SURFACE,
                width: 2,
                radius: 1.0,
                fill_mode: rule::FillMode::Percent(30.0),
            }
        }
    }
}
