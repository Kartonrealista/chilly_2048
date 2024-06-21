use std::{result, time};

use iced::Event::Keyboard;
use iced::{
    alignment::{self, Horizontal, Vertical},
    executor, keyboard, subscription,
    theme::{self},
    widget::{self, button, container, text, text_input, Column, Row, Text},
    Alignment, Application, Command, Element, Event, Length, Renderer, Settings, Subscription,
    Theme,
};
use rand::seq::IteratorRandom;
use rand::{seq::SliceRandom, thread_rng};

fn main() -> iced::Result {
    Game::run(Settings::default())
}

struct Game {
    menu: Menu,
    board: Board,
    has_ended: bool,
}
fn pair_to_index(i: usize, j: usize, width: usize) -> usize {
    j + i * width
}

struct Menu {
    width_inptut: String,
    height_inptut: String,
    width: usize,
    height: usize,
    start_pressed: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct Board(Vec<Tile>);
impl Board {
    const TWO_OR_FOUR: [usize; 10] = [2, 2, 2, 2, 2, 2, 2, 2, 2, 4];
    fn new(height: usize, width: usize) -> Board {
        let mut out: Vec<Tile> = (0..(width * height))
            .map(|id| Tile {
                tilecontent: None,
                id,
            })
            .collect();
        let mut ids: Vec<usize> = (0..(width * height)).collect();
        rand::seq::SliceRandom::shuffle(ids.as_mut_slice(), &mut thread_rng());
        ids.iter().take(2).for_each(|&id| {
            out[id].tilecontent = Some(*Self::TWO_OR_FOUR.choose(&mut thread_rng()).unwrap())
        });
        Board(out)
    }
    fn move_tile_content(&mut self, direction: keyboard::KeyCode, height: usize, width: usize) {
        let old_board = self.clone();
        let mut previous = Tile {
            tilecontent: None,
            id: 0,
        };
        match direction {
            keyboard::KeyCode::Left => {
                self.collapse_left(height, width);
                (0..height).for_each(|h| {
                    (0..width).for_each(|w| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, direction);
                    });
                });
                self.collapse_left(height, width);
            }
            keyboard::KeyCode::Right => {
                self.collapse_right(height, width);
                (0..height).for_each(|h| {
                    (0..width).rev().for_each(|w| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, direction)
                    });
                });
                self.collapse_right(height, width);
            }
            keyboard::KeyCode::Up => {
                self.collapse_up(height, width);
                (0..width).for_each(|w| {
                    (0..height).for_each(|h| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, direction)
                    });
                });
                self.collapse_up(height, width);
            }
            keyboard::KeyCode::Down => {
                self.collapse_down(height, width);
                (0..width).for_each(|w| {
                    (0..height).rev().for_each(|h| {
                        self.merge_neighbouring(h, w, &mut previous, width, height, direction)
                    });
                });
                self.collapse_down(height, width);
            }
            _ => {}
        }
        std::thread::sleep(time::Duration::from_secs_f64(0.05));
        if old_board != *self {
            let empty_ids = self
                .0
                .iter()
                .filter(|&&tile| tile.tilecontent.is_none())
                .map(|&tile| tile.id);
            let chosen_id = empty_ids.choose(&mut thread_rng()).unwrap();
            self.0[chosen_id].tilecontent =
                Some(*Self::TWO_OR_FOUR.choose(&mut thread_rng()).unwrap())
        }
    }
    fn collapse_left(&mut self, height: usize, width: usize) {
        (0..height).for_each(|h| {
            let mut row = vec![];
            (0..width).for_each(|w| row.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = row.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == row {
                return;
            }
            collapsed
                .clone()
                .copied()
                .enumerate()
                .for_each(|(w, tile)| {
                    self.0[pair_to_index(h, w, width)].tilecontent = tile.tilecontent;
                });
            (collapsed.count()..width).for_each(|w| {
                self.0[pair_to_index(h, w, width)].tilecontent = None;
            })
        });
    }
    fn collapse_right(&mut self, height: usize, width: usize) {
        (0..height).for_each(|h| {
            let mut row = vec![];
            (0..width)
                .rev()
                .for_each(|w| row.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = row.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == row {
                return;
            }
            collapsed
                .clone()
                .copied()
                .enumerate()
                .for_each(|(w, tile)| {
                    self.0[pair_to_index(h, width - w - 1, width)].tilecontent = tile.tilecontent;
                });
            (collapsed.count()..width).for_each(|w| {
                self.0[pair_to_index(h, width - w - 1, width)].tilecontent = None;
            })
        });
    }
    fn collapse_up(&mut self, height: usize, width: usize) {
        (0..width).for_each(|w| {
            let mut column = vec![];
            (0..height).for_each(|h| column.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = column.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == column {
                return;
            }
            collapsed.clone().enumerate().for_each(|(h, tile)| {
                self.0[pair_to_index(h, w, width)].tilecontent = tile.tilecontent;
            });
            (collapsed.count()..height).for_each(|h| {
                self.0[pair_to_index(h, w, width)].tilecontent = None;
            })
        });
    }
    fn collapse_down(&mut self, height: usize, width: usize) {
        (0..width).for_each(|w| {
            let mut column = vec![];
            (0..height)
                .rev()
                .for_each(|h| column.push(self.0[pair_to_index(h, w, width)]));
            let collapsed = column.iter().filter(|&tile| tile.tilecontent.is_some());
            if collapsed.clone().copied().collect::<Vec<Tile>>() == column {
                return;
            }
            collapsed.clone().enumerate().for_each(|(h, tile)| {
                self.0[pair_to_index(height - h - 1, w, width)].tilecontent = tile.tilecontent;
            });
            (collapsed.count()..height).for_each(|h| {
                self.0[pair_to_index(height - h - 1, w, width)].tilecontent = None;
            })
        });
    }
    fn merge_neighbouring(
        &mut self,
        h: usize,
        w: usize,
        previous: &mut Tile,
        width: usize,
        height: usize,
        direction: keyboard::KeyCode,
    ) {
        match (h, w) {
            (_, 0) if direction == keyboard::KeyCode::Left => {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            (_, w) if direction == keyboard::KeyCode::Right && w == width - 1 => {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            (0, _) if direction == keyboard::KeyCode::Up => {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            (h, _) if direction == keyboard::KeyCode::Down && h == height - 1 => {
                *previous = self.0[pair_to_index(h, w, width)];
            }
            _ => {
                let mut current = self.0[pair_to_index(h, w, width)];
                if previous.tilecontent == current.tilecontent {
                    if let Some(content) = previous.tilecontent {
                        self.0[previous.id].tilecontent = Some(content * 2);
                        self.0[current.id].tilecontent = None;
                        current.tilecontent = None
                    }
                }
                *previous = current;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Tile {
    tilecontent: Option<usize>,
    id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Reset,
    FontLoaded(result::Result<(), iced::font::Error>),
    InputWidth(String),
    InputHeight(String),
    InputMineCount(String),
    StartPressed,
    GotoMenu,
    Event(Event),
}

impl Application for Game {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Game, iced::Command<Message>) {
        let game = Game {
            board: Board::new(4, 4),
            has_ended: false,
            menu: Menu {
                width_inptut: String::from("4"),
                height_inptut: String::from("4"),
                width: 4,
                height: 4,
                start_pressed: false,
            },
        };
        (game, Command::none())
    }

    fn title(&self) -> String {
        String::from("2048")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::GotoMenu => {
                (*self, _) = Game::new(());
            }
            Message::StartPressed => {
                self.menu.width = self.menu.width_inptut.parse().unwrap();
                self.menu.height = self.menu.height_inptut.parse().unwrap();

                self.board = Board::new(self.menu.width, self.menu.height);
                self.menu.start_pressed = true;
            }
            Message::InputWidth(input) => self.menu.width_inptut = input,
            Message::InputHeight(input) => self.menu.height_inptut = input,
            Message::Reset => {
                self.board = Board::new(self.menu.width, self.menu.height);
                self.has_ended = false;
            }
            Message::Event(Keyboard(keyboard::Event::KeyPressed { key_code, .. })) => self
                .board
                .move_tile_content(key_code, self.menu.height, self.menu.width),
            _ => {}
        };

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self.menu.start_pressed {
            false => menu(self),
            true => playfield(self),
        }
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::Event)
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

fn playfield(game: &Game) -> iced::widget::Container<Message> {
    let tilebutton = |id: usize| match game.board.0[id] {
        Tile {
            tilecontent: Some(2),
            ..
        } => button(centralize_tile_content(text(2)))
            .style(theme::Button::Positive)
            .height(50)
            .width(50),
        Tile {
            tilecontent: Some(content),
            ..
        } => button(centralize_tile_content(text(content)))
            .style(theme::Button::Positive)
            .height(50)
            .width(50),
        Tile {
            tilecontent: None, ..
        } => button("")
            .style(theme::Button::Secondary)
            .height(50)
            .width(50),
    };
    let playboard = (0..game.menu.width).fold(Row::new(), |acc, column| {
        let new_column = (0..game.menu.height).fold(Column::new(), |acc2, row| {
            acc2.push(tilebutton(pair_to_index(row, column, game.menu.width)))
        });
        acc.push(new_column.spacing(2).align_items(Alignment::Center))
    });
    let menu_button = button("MENU")
        .on_press(Message::GotoMenu)
        .style(theme::Button::Positive);
    let reset_button = button("RESET")
        .on_press(Message::Reset)
        .style(theme::Button::Destructive);
    container(
        widget::column![
            widget::row![menu_button, reset_button]
                .padding(20)
                .spacing(20)
                .align_items(Alignment::Center),
            playboard.spacing(2).align_items(Alignment::Center),
        ]
        .padding(20)
        .align_items(Alignment::Center),
    )
}

fn centralize_tile_content(tile_content: Text<Renderer>) -> Text<Renderer> {
    tile_content
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
}

fn menu<'a>(game: &Game) -> iced::widget::Container<'a, Message> {
    let width_box = text_input("", &game.menu.width_inptut).on_input(Message::InputWidth);
    let height_box = text_input("", &game.menu.height_inptut).on_input(Message::InputHeight);
    let start_game_button = button(centralize_tile_content(text("START")))
        .on_press(Message::StartPressed)
        .style(theme::Button::Positive)
        .width(96)
        .height(55);
    container(
        iced::widget::column![
            iced::widget::row![text("Width: "), width_box.width(40)].align_items(Alignment::Center),
            iced::widget::row![text("Height: "), height_box.width(40)]
                .align_items(Alignment::Center),
            start_game_button
        ]
        .spacing(20)
        .align_items(Alignment::Center),
    )
}
