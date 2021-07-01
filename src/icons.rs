use iced::{svg, Svg};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PLAY: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-player-play.svg").to_vec(),
    ));
    pub static ref NEXT: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-player-track-next.svg").to_vec(),
    ));
    pub static ref PREV: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-player-track-prev.svg").to_vec(),
    ));
    pub static ref PAUSE: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-player-pause.svg").to_vec(),
    ));
    pub static ref MENU: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-chart-candle.svg").to_vec(),
    ));
    pub static ref RESET: Svg = Svg::new(svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-rotate.svg").to_vec(),
    ));
}