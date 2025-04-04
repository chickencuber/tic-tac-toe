use std::collections::HashMap;

use iced::{widget::{button, column, container, row, stack, text, Column, Stack, vertical_space, pick_list}, *};

mod game;

use game::{board_to_string, calc_ai, is_won, new_turn, EndType, Turn};

use rand::seq::IndexedRandom;

#[derive(Debug, Clone, Copy)]
enum Message {
    Click(usize),
    None,
    Reset,
    Player(Turn),
}

#[derive(Clone)]
enum Command {
    None,
    Alert(String),
}

struct State {
    board: [Turn; 9],
    turn: Turn,
    command: Command,
    hash: HashMap<String, Vec<usize>>,
    player: Turn,
}

impl Default for State {
    fn default() -> Self {
        let mut s = Self {
            board: [Turn::None; 9],
            turn: Turn::X,
            command: Command::None,
            hash: calc_ai(), 
            player: Turn::Multi,
        };
        if s.player == Turn::O {
            let e = s.hash.get(&board_to_string(s.board)).unwrap();
            let n = e.choose(& mut rand::rng()).unwrap();
            s.play(*n);
        }
        return s;
    }
}

impl State {
    fn play(&mut self, v: usize) -> bool {
        if let Some(f) = self.board.get_mut(v) {
            if *f != Turn::None {
                return true;
            }
            *f = self.turn;
            let w = is_won(self.board);
            let mut t = false;
            if w != EndType::None {
                t = true;
                if w == EndType::Win {
                    self.command = Command::Alert(format!("{} Wins!", self.turn.to_string()))
                } else {
                    self.command = Command::Alert(format!("Draw!"))
                }
            }
            self.turn = new_turn(self.turn);
            return t;
        }
        return true;

    }
    fn update(&mut self, message: Message) {
        match message {
            Message::None => {}
            Message::Click(v) => {
                if self.turn != self.player && self.player != Turn::Multi {
                    return;
                }
                if self.play(v) {
                    return;
                }
                if self.player == Turn::Multi {
                    return;
                }
                let e = self.hash.get(&board_to_string(self.board)).unwrap();
                let n;
                if self.player == Turn::X {
                    n = e.last().unwrap();
                } else {
                    n = e.first().unwrap();
                }
                self.play(*n);
            }
            Message::Reset => {
                self.reset();
            }
            Message::Player(player) => {
                self.player = player;
                self.reset();
            }
        }
    }
    fn reset(&mut self) {
        self.command = Command::None;
        self.turn = Turn::X;
        self.board = [Turn::None; 9];
        if self.player == Turn::O {
            let e = self.hash.get(&board_to_string(self.board)).unwrap();
            let n = e.choose(&mut rand::rng()).unwrap();
            self.play(*n);
        }
    }
    fn create_button(&self, i: usize, active:bool) -> Column<Message> {
        let sides = 100.0;
        let size = 70;
        let b = button(
            text(self.board[i].to_string())
            .width(Length::Fixed(sides))
            .center()
            .height(Length::Fixed(sides))
            .font(Font {
                weight: font::Weight::Bold,
                ..Default::default()
            })
            .size(size))
            .style(button::secondary)
            .on_press(if active {Message::None} else {Message::Click(i)});
        column![b].padding(5)
    }
    fn view(&self) -> Stack<Message> {
        let active;
        let message;
        let turns = [
            Turn::Multi,
            Turn::X,
            Turn::O,
        ];
        if let Command::Alert(v) = &self.command {
            active = true;
            message = v.clone();
        } else {
            active = false;
            message = "".to_string();
        }
        stack![
            column![
                text("Tic Tac Toe").size(50).font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }).width(Length::Fixed(330.0)).center(),
                row![
                    self.create_button(0, active),
                    self.create_button(1, active),
                    self.create_button(2, active),
                ],
                row![
                    self.create_button(3, active),
                    self.create_button(4, active),
                    self.create_button(5, active),
                ],
                row![
                    self.create_button(6, active),
                    self.create_button(7, active),
                    self.create_button(8, active),
                ],
                row![
                button(text("reset").size(20).font(Font {
                    weight: font::Weight::Bold,
                    ..Default::default()
                }).width(Length::Fixed(290.0)).center(),).style(button::secondary).on_press(Message::Reset),
                pick_list(turns, Some(self.player), Message::Player) 
                ],
                ],
                if active {
                    column! [
                        vertical_space(),
                        container(
                            column![
                            text(message).size(20).font(Font {
                                weight: font::Weight::Bold,
                                ..Default::default()
                            }).width(330.0).center(),
                            button(text("reset").size(20).font(Font {
                                weight: font::Weight::Bold,
                                ..Default::default()
                            }).width(Length::Fixed(330.0)).center(),).style(button::secondary).on_press(Message::Reset),
                            ].padding(10)
                        ).style(container::bordered_box)
                    ].height(230)
                } else {
                    column![
                        container(column![])
                    ]
                }
                ]
    }
}

fn main() -> iced::Result {
    application("Tic Tac Toe", State::update, State::view).window_size((430.0, 500.0)).resizable(false).run()
}
