#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use native_dialog::{MessageDialog, MessageType};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Box::new(App::default())),
    );
}

struct App {
    game_data: [[u8; 3]; 3],
    player_one: bool,
    game_running: bool
}

impl Default for App {
    fn default() -> Self {
        Self {
            game_data: [
                [0, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
            ],
            game_running: true,
            player_one: true
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tic Tac Toe");

            let mut remaining:i8 = 0;
            let mut was_empty = false;
            for (i, row) in self.game_data.into_iter().enumerate() {
                ui.horizontal(|ui| {
                    for(y, col) in row.into_iter().enumerate() {
                        let mut text = "   ";
                        if col == 0 {
                            remaining = remaining + 1;
                            was_empty = true;
                        }
                        else {
                            was_empty = false;
                        }
                        if col == 1 {
                            text = "O";
                        }
                        else if col == 2 {
                            text = "X"
                        }
                        

                        if ui.button(text).clicked() && self.game_running {
                            let current_slot = &mut self.game_data[i][y];
                            if current_slot != &0 {
                                return
                            };
                            if self.player_one {
                                *current_slot = 1;
                            }
                            else {
                                *current_slot = 2;
                            }
                            self.player_one = !self.player_one;
                            if was_empty {
                                remaining = remaining - 1;
                            }
                        }

                    }
                });   
            }

            if ui.button("Reset").clicked() {
                self.game_data = [
                    [0, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ];
                self.player_one = true;
                self.game_running = true;
            }
            
            let winner = check_winner(self.game_data);
            if winner != 0 && self.game_running {
                MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title(&format!("Player {} won!", winner))
                    .set_text("What a guy!")
                    .show_alert()
                    .unwrap();
                self.game_running = false;
            }

            if remaining == 0 && self.game_running {
                MessageDialog::new()
                    .set_type(MessageType::Info)
                    .set_title("Board is full")
                    .set_text("crazy stuff, telling you, it was crazy stuff back there")
                    .show_alert()
                    .unwrap();
                self.game_running = false;
            }

            let mut current_player_text = "X";
            if self.player_one {
                current_player_text = "O";
            }
            ui.label(&format!("Current player : {}", current_player_text));
        });
    }

    
}

// kinda stupid but whatever ¯\_(ツ)_/¯
fn check_winner (table:[[u8; 3]; 3]) -> u8 {
    let first_row = table[0];
    let second_row = table[1];
    let third_row = table[2];
    let mut winner:u8 = 0;

    if first_row[0] != 0 && first_row[0] == first_row[1] && first_row[1] == first_row[2] {
        winner = first_row[0];
    }

    else if second_row[0] != 0 && second_row[0] == second_row[1] && second_row[1] == second_row[2] {
        winner = second_row[0];
    }
    
    else if third_row[0] != 0 && third_row[0] == third_row[1] && third_row[1] == third_row[2] {
        winner = third_row[0];
    }

    else if first_row[0] != 0 && first_row[0] == second_row[0] && second_row[0] == third_row[0] {
        winner = first_row[0];
    }

    else if first_row[1] != 0 && first_row[1] == second_row[1] && second_row[1] == third_row[1] {
        winner = first_row[1];
    }

    else if first_row[2] != 0 && first_row[2] == second_row[2] && second_row[2] == third_row[2] {
        winner = first_row[2];
    }

    else if first_row[0] != 0 && first_row[0] == second_row[1] && second_row[1] == third_row[2] {
        winner = first_row[0];
    }

    else if first_row[2] != 0 && first_row[2] == second_row[1] && second_row[1] == third_row[0] {
        winner = first_row[2];
    }
    return winner;
    
}