use iced::{container, button, Color, Background};

pub struct StartButton;

impl button::StyleSheet for StartButton {
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

pub struct BackContainer;

impl container::StyleSheet for BackContainer {
    fn style(&self) -> container::Style {
        container::Style {
            background: Color::from_rgb(
                0x1e as f32 / 255.0,
                0x1e as f32 / 255.0,
                0x1e as f32 / 255.0,
            )
            .into(),
            text_color: Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}
