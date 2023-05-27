use iced::widget::svg;
use once_cell::sync::Lazy;

pub static PLAY: Lazy<svg::Handle> = Lazy::new(|| {
    svg::Handle::from_memory(include_bytes!("../assets/icons/tabler-icon-player-play.svg").to_vec())
});
pub static NEXT: Lazy<svg::Handle> = Lazy::new(|| {
    svg::Handle::from_memory(include_bytes!("../assets/icons/chevrons-right.svg").to_vec())
});
pub static PREV: Lazy<svg::Handle> = Lazy::new(|| {
    svg::Handle::from_memory(include_bytes!("../assets/icons/chevrons-left.svg").to_vec())
});
pub static PAUSE: Lazy<svg::Handle> = Lazy::new(|| {
    svg::Handle::from_memory(
        include_bytes!("../assets/icons/tabler-icon-player-pause.svg").to_vec(),
    )
});
pub static MENU: Lazy<svg::Handle> =
    Lazy::new(|| svg::Handle::from_memory(include_bytes!("../assets/icons/settings.svg").to_vec()));
pub static RESET: Lazy<svg::Handle> = Lazy::new(|| {
    svg::Handle::from_memory(include_bytes!("../assets/icons/tabler-icon-rotate.svg").to_vec())
});
