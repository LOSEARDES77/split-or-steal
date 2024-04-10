// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    let mut game = game::game::game::Game::new("Copy Opponent".to_owned(), "Yes No".to_owned(), 10, [3, 0, 5, 1]);

    for i in 0..game.rounds {
        game.next_round();

        println!("Round: {}", i);
        println!("Player 1: {}", game.player_1.name);
        println!("\t{:?}", game.player_1.history);
        println!("Player 2: {}", game.player_2.name);
        println!("\t{:?}", game.player_2.history);
        
    }
}
