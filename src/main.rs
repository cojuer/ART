use iced::theme::Theme;
use iced::Command;
use iced::{
    alignment, executor,
    widget::{button, image, pick_list, svg, text_input, Column, Container, Row, Text},
    window, Alignment, Application, Element, Length, Settings, Subscription,
};
use nfd::Response;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;

mod icons;
mod style;

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            icon: window::icon::from_file_data(
                include_bytes!("../assets/icons/clock-hour-3.png"),
                None,
            )
            .ok(),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct App {
    state: AppState,

    folder_path: String,
    image_paths: Vec<std::path::PathBuf>,
    image_num: usize,

    seconds: usize,
    selected_order: ImageOrder,
    running: bool,
    duration: Duration,
    last_tick: Instant,
    default_duration: Duration,
    error: String,
    input_data: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            last_tick: Instant::now(),
            state: AppState::default(),
            folder_path: String::default(),
            image_paths: Vec::<std::path::PathBuf>::default(),
            image_num: usize::default(),
            selected_order: ImageOrder::default(),
            running: bool::default(),
            duration: Duration::default(),
            default_duration: Duration::from_secs(30),
            error: String::default(),
            seconds: 0,
            input_data: String::default(),
        }
    }
}

enum AppState {
    Test,
    ShowImage { canvas: Canvas },
}

impl Default for AppState {
    fn default() -> Self {
        AppState::Test
    }
}

#[derive(Debug, Clone)]
enum Message {
    Load,
    PrevImage,
    NextImage,
    Tick(Instant),
    OrderSelected(ImageOrder),
    ChangeTimerState,
    ResetTimer,
    Start,
    BackToMenu,
    RoundSizeEdited(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageOrder {
    Name,
    Random,
}

impl ImageOrder {
    const ALL: [ImageOrder; 2] = [ImageOrder::Name, ImageOrder::Random];
}

impl Default for ImageOrder {
    fn default() -> ImageOrder {
        ImageOrder::Random
    }
}

impl std::fmt::Display for ImageOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageOrder::Name => "Name",
                ImageOrder::Random => "Random",
            }
        )
    }
}

impl Application for App {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Art Reference Tool")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Load => {
                let result = nfd::open_pick_folder(None).unwrap_or_else(|e| {
                    panic!("{}", e);
                });
                if let Response::Okay(file_path) = result {
                    self.folder_path = file_path;
                }
                let extensions: HashSet<_> = ["jpg", "jpeg", "png"].iter().cloned().collect();
                self.image_paths = fs::read_dir(&self.folder_path)
                    .unwrap() // TODO: handle error
                    .into_iter()
                    .map(|x| x.map(|entry| entry.path()))
                    .filter_map(|x| x.ok())
                    .filter(|x| x.extension().is_some())
                    .filter(|x| extensions.contains(x.extension().and_then(OsStr::to_str).unwrap()))
                    .collect();
                if self.image_paths.len() != 0 {
                    self.error = String::new();
                }
            }
            Message::Start => {
                if self.seconds != 0 {
                    self.default_duration = Duration::from_secs(self.seconds as u64);
                } else {
                    self.default_duration = Duration::from_secs(30);
                }
                match &self.selected_order {
                    &ImageOrder::Name => self.image_paths.sort_by(|p1, p2| {
                        p1.file_name()
                            .and_then(OsStr::to_str)
                            .unwrap_or("default")
                            .cmp(p2.file_name().and_then(OsStr::to_str).unwrap_or("default"))
                    }),
                    &ImageOrder::Random => {
                        self.image_paths.shuffle(&mut thread_rng());
                    }
                }
                if self.image_paths.len() != 0 {
                    self.state = AppState::ShowImage {
                        canvas: Canvas {
                            image: Canvas::fetch_image(&self.image_paths[self.image_num]),
                        },
                    };
                    self.running = true;
                    self.duration = self.default_duration;
                    self.last_tick = Instant::now();
                    self.error = String::new();
                } else {
                    self.error = String::from("no images found in folder");
                }
            }
            Message::NextImage => {
                self.image_num = self.image_num + 1;
                if self.image_num == self.image_paths.len() {
                    self.image_num = 0;
                }
                self.state = AppState::ShowImage {
                    canvas: Canvas {
                        image: Canvas::fetch_image(&self.image_paths[self.image_num]),
                    },
                };
                self.duration = self.default_duration;
            }
            Message::PrevImage => {
                if self.image_num == 0 && self.image_paths.len() > 0 {
                    self.image_num = self.image_paths.len() - 1
                } else if self.image_num == 0 {
                    self.image_num = 0
                } else {
                    self.image_num -= 1
                }
                self.state = AppState::ShowImage {
                    canvas: Canvas {
                        image: Canvas::fetch_image(&self.image_paths[self.image_num]),
                    },
                };
                self.duration = self.default_duration;
            }
            Message::Tick(now) => {
                let time_elapsed = now - self.last_tick;
                self.last_tick = now;
                if time_elapsed < self.duration {
                    self.duration -= time_elapsed;
                } else {
                    self.duration = self.default_duration;
                    self.image_num = self.image_num + 1;
                    if self.image_num == self.image_paths.len() {
                        self.image_num = 0;
                    }
                    self.state = AppState::ShowImage {
                        canvas: Canvas {
                            image: Canvas::fetch_image(&self.image_paths[self.image_num]),
                        },
                    }
                }
            }
            Message::ResetTimer => self.duration = self.default_duration,
            Message::ChangeTimerState => {
                self.running = !self.running;
                self.last_tick = Instant::now();
            }
            Message::BackToMenu => {
                self.state = AppState::Test;
                self.image_num = 0;
            }
            Message::OrderSelected(order) => {
                self.selected_order = order;
            }
            Message::RoundSizeEdited(size_str) => {
                if size_str.len() == 0 {
                    self.seconds = 0;
                    self.input_data = size_str;
                } else {
                    if let Ok(parsed) = size_str.parse::<usize>() {
                        self.seconds = parsed;
                        self.input_data = size_str;
                        self.error = String::new();
                    } else {
                        self.error = String::from("invalid number of seconds");
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        match &self.state {
            AppState::ShowImage { .. } => {
                if self.running {
                    iced::time::every(Duration::from_millis(200)).map(Message::Tick)
                } else {
                    Subscription::none()
                }
            }
            _ => Subscription::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self.state {
            AppState::Test => {
                let folder_name = if self.folder_path.is_empty() {
                    self.folder_path.clone()
                } else {
                    String::from(
                        PathBuf::from_str(&self.folder_path)
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    )
                };
                // let folder_char_vec = self.folder_path.chars().collect::<Vec<_>>();
                // let short_folder = if folder_char_vec.len() < 15 {self.folder_path.clone()} else {folder_char_vec[0..14].iter().cloned().collect::<String>() + ".."};
                let pick_list = pick_list(
                    &ImageOrder::ALL[..],
                    Some(self.selected_order),
                    Message::OrderSelected,
                )
                .style(iced::theme::PickList::Custom(
                    Rc::new(style::MenuPickList),
                    Rc::new(style::MenuPickList),
                ));
                Column::new()
                    .push(
                        Text::new(&self.error)
                            .width(220)
                            .height(20)
                            .horizontal_alignment(alignment::Horizontal::Center),
                    )
                    .push(
                        Column::new()
                            .push(
                                button(
                                    Text::new("Choose folder")
                                        .horizontal_alignment(alignment::Horizontal::Center),
                                )
                                .on_press(Message::Load)
                                .style(iced::theme::Button::Custom(Box::new(style::EdgyButton)))
                                .width(Length::Fixed(220.0)),
                            )
                            .push(Column::new().height(Length::Fixed(5.0)))
                            .push(
                                Text::new(String::from("Chosen: ") + &folder_name)
                                    .size(15)
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .width(Length::Fixed(220.0)),
                            )
                            .push(
                                Text::new(self.image_paths.len().to_string() + " images found")
                                    .size(15)
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .width(Length::Fixed(220.0)),
                            )
                            .spacing(2),
                    )
                    .push(
                        text_input(" secs per round (def. 30)", &self.input_data)
                            .on_input(Message::RoundSizeEdited)
                            .style(iced::theme::TextInput::Custom(Box::new(style::MenuInput)))
                            .width(Length::Fixed(220.0))
                            .padding(5),
                    )
                    .push(
                        Row::new()
                            .push(Row::new().width(Length::Fixed(10.0)))
                            .push(Text::new("Sort by"))
                            .push(Row::new().width(Length::Fixed(10.0)))
                            .push(pick_list)
                            .align_items(Alignment::Center),
                    )
                    .push(
                        button(
                            Text::new("Start")
                                .size(25)
                                .horizontal_alignment(alignment::Horizontal::Center),
                        )
                        .on_press(Message::Start)
                        .width(Length::Fixed(220.0))
                        .style(iced::theme::Button::Custom(Box::new(style::StartButton))),
                    )
                    .spacing(10)
                    .padding(10)
                    .align_items(Alignment::Start)
            }
            AppState::ShowImage { ref canvas } => {
                const MINUTE: u64 = 60;
                const HOUR: u64 = 60 * MINUTE;

                let mut seconds = self.duration.as_secs();
                if self.duration != self.default_duration {
                    // do not show full round time as it changes after one tick and it is distracting
                    seconds = self.duration.as_secs() + 1; // show x secs when x - 1 secs y millis left, so last second it will show 1 sec and not 0
                }

                let duration = Text::new(format!(
                    "{:0>2}.{:0>2}",
                    (seconds % HOUR) / MINUTE,
                    seconds % MINUTE,
                ))
                // static width prevents menu jitter: default font is not monospaced
                .width(Length::Fixed(80.0))
                .size(35);

                Column::new().push(canvas.view()).push(
                    Row::new()
                        .push(
                            button(svg(icons::MENU.clone()))
                                .style(iced::theme::Button::Custom(Box::new(style::IconButton)))
                                .on_press(Message::BackToMenu),
                        )
                        .push(Row::new().width(Length::Fixed(80.0))) // to balance duration width
                        .push(Row::new().width(Length::Fill))
                        .push(
                            button(svg(icons::PREV.clone()))
                                .style(iced::theme::Button::Custom(Box::new(style::IconButton)))
                                .on_press(Message::PrevImage),
                        )
                        .push(
                            button(if self.running {
                                svg(icons::PAUSE.clone())
                            } else {
                                svg(icons::PLAY.clone())
                            })
                            .style(iced::theme::Button::Custom(Box::new(style::IconButton)))
                            .on_press(Message::ChangeTimerState),
                        )
                        .push(
                            button(svg(icons::NEXT.clone()))
                                .style(iced::theme::Button::Custom(Box::new(style::IconButton)))
                                .on_press(Message::NextImage),
                        )
                        .push(Row::new().width(Length::Fill))
                        .push(
                            button(svg(icons::RESET.clone()))
                                .style(iced::theme::Button::Custom(Box::new(style::IconButton)))
                                .on_press(Message::ResetTimer),
                        )
                        .push(duration)
                        .spacing(5)
                        .padding(10)
                        .width(Length::Fill)
                        .align_items(Alignment::Center),
                )
            }
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(
                style::BackContainer,
            )))
            .into()
    }
}

#[derive(Debug, Clone)]
struct Canvas {
    image: image::Handle,
}

impl Canvas {
    fn view(&self) -> Element<Message> {
        Row::new()
            .push(
                image::viewer(self.image.clone())
                    .height(Length::Fill)
                    .width(Length::Fill),
            )
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }

    fn fetch_image(path: &PathBuf) -> image::Handle {
        image::Handle::from_path(path)
    }
}
