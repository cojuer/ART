use iced::theme::Theme;
use iced::{
    overlay::menu,
    widget::{button, container, pick_list, text_input},
    Background, Color,
};

pub struct StartButton;

impl button::StyleSheet for StartButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x00, 0x9d, 0x00))),
            border_radius: 4.0.into(),
            text_color: Color::WHITE,
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x00, 0xbd, 0x00))),
            text_color: Color::WHITE,
            ..self.active(style)
        }
    }
}

pub struct EdgyButton;

impl button::StyleSheet for EdgyButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE))),
            border_radius: 4.0.into(),
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE))),
            ..self.active(style)
        }
    }
}

pub struct MenuInput;

impl text_input::StyleSheet for MenuInput {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE)),
            border_radius: 4.0.into(),
            border_width: 1.0,
            border_color: Color::TRANSPARENT,
            icon_color: Color::TRANSPARENT,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE)),
            ..self.active(style)
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        self.hovered(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb8(0xAE, 0xAE, 0xAE)
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        Color::BLACK
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        Color::from_rgb8(0xAE, 0xAE, 0xAE)
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        text_input::Appearance {
            background: Background::Color(Color::BLACK),
            ..self.active(style)
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        self.placeholder_color(style)
    }
}

pub struct MenuPickList;

impl pick_list::StyleSheet for MenuPickList {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE)),
            placeholder_color: Color::BLACK,
            handle_color: Color::BLACK,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        pick_list::Appearance {
            background: Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE)),
            ..self.active(style)
        }
    }
}

impl menu::StyleSheet for MenuPickList {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> menu::Appearance {
        menu::Appearance {
            text_color: Color::BLACK,
            background: Background::Color(Color::from_rgb8(0xDE, 0xDE, 0xDE)),
            selected_text_color: Color::BLACK,
            selected_background: Background::Color(Color::from_rgb8(0xFE, 0xFE, 0xFE)),
            border_radius: 0.0.into(),
            border_width: 1.0,
            border_color: Color::WHITE,
        }
    }
}

pub struct IconButton;

impl button::StyleSheet for IconButton {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            ..button::Appearance::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x2e, 0x2e, 0x2e))),
            border_radius: 8.0,
            ..self.active(style)
        }
    }
}

pub struct BackContainer;

impl container::StyleSheet for BackContainer {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Color::from_rgb(
                0x1e as f32 / 255.0,
                0x1e as f32 / 255.0,
                0x1e as f32 / 255.0,
            )
            .into(),
            text_color: Color::WHITE.into(),
            ..container::Appearance::default()
        }
    }
}
