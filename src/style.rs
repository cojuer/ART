use iced::{Background, Color, button, container, pick_list, text_input};

pub struct StartButton;

impl button::StyleSheet for StartButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0x00, 0x9d, 0x00))),
            text_color: Color::from_rgb8(0xDE, 0xDE, 0xDE),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0x00, 0xbd, 0x00))),
            text_color: Color::WHITE,
            ..self.active()
        }
    }
}

pub struct EdgyButton;

impl button::StyleSheet for EdgyButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE))),
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE))),
            ..self.active()
        }
    }
}

pub struct MenuInput;

impl text_input::StyleSheet for MenuInput {
    fn active(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE)),
            ..text_input::Style::default()
        }
    }

    fn focused(&self) -> text_input::Style {
        text_input::Style {
            background: Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE)),
            ..self.active()
        }
    }

    fn placeholder_color(&self) -> Color {
        Color::from_rgb8(0xA0, 0xA0, 0xA0)
    }

    fn value_color(&self) -> Color {
        Color::from_rgb8(0x20, 0x20, 0x20)
    }

    fn selection_color(&self) -> Color {
        Color::from_rgb8(0xDA, 0xDA, 0xDA)
    }
}

pub struct MenuPickList;

impl pick_list::StyleSheet for MenuPickList {
    fn menu(&self) -> pick_list::Menu {
        pick_list::Menu {
            background: Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE)),
            ..pick_list::Menu::default()
        }
    }

    fn active(&self) -> pick_list::Style {
        pick_list::Style {
            background: Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE)),
            ..pick_list::Style::default()
        }
    }

    fn hovered(&self) -> pick_list::Style {
        pick_list::Style {
            background: Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE)),
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
