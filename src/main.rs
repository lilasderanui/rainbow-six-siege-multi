use std::process::Command;
use tokio::task;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let game_process = start_game("RainbowSixSiege.exe").await;
    if game_process.is_ok() {
        let features = vec!["aimbot", "esp", "wallhack"];
        activate_features(features).await;
    }
}

async fn start_game(game_name: &str) -> std::io::Result<()> {
    Command::new(game_name).spawn()?;
    Ok(())
}

async fn activate_features(features: Vec<&str>) {
    for feature in features {
        match feature {
            "aimbot" => enable_aimbot().await,
            "esp" => enable_esp().await,
            "wallhack" => enable_wallhack().await,
            _ => {}
        }
    }
}

async fn enable_aimbot() {
    println!("Aimbot activated.");
}

async fn enable_esp() {
    println!("ESP activated.");
}

async fn enable_wallhack() {
    println!("Wallhack activated.");
}