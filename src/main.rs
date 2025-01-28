pub mod game;

use std::{
    env::args,
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use console_utils::input::{reveal, select, spinner, SpinnerType};
use game::*;
use serde::{Deserialize, Serialize};

pub const TIME_BETWEEN: f64 = 0.025;

/// The config struct holds general Config for Player and Enemy with saving/loading from a file
#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    player: PlayerType,
    enemy: Monster,
}

impl Config {
    pub fn _new() -> Self {
        Self::default()
    }

    /// Loads the config from json file if it exists
    pub fn load_from_file(path: &PathBuf) -> Config {
        if path.exists() {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            let config: Self = serde_json::from_reader(reader).unwrap();
            reveal(
                &format!("Konfigurationsdatei geladen von: {:?}\n", path),
                TIME_BETWEEN,
            );
            config
        } else {
            reveal(
                &format!("Konfigurationsdatei erstellt bei: {:?}\n", path),
                TIME_BETWEEN,
            );
            let config = Config::default();
            Self::save_to_file(config, path).unwrap()
        }
    }

    /// Saves the current config to a json file
    pub fn save_to_file(config: Config, path: &PathBuf) -> std::io::Result<Config> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &config)?;
        Ok(config)
    }
}

/// The player type loaded from the file
#[derive(Debug, Serialize, Deserialize)]
enum PlayerType {
    Fighter(Fighter),
    Mage(Mage),
}

impl Default for PlayerType {
    fn default() -> Self {
        Self::Fighter(Fighter::default())
    }
}

fn main() {
    // Coole intro Scene
    reveal(
        "Simple Fantasy Game Emulator von Nils Wrenger\n",
        TIME_BETWEEN,
    );
    spinner(1.5, SpinnerType::Dots);

    // Get the first argument: ./simple-fantasy-game [HERE]
    let path = PathBuf::from(
        args()
            .nth(1)
            .expect("Expected a path parameter: ./simple-fantasy-game [HERE]"),
    );
    let mut config = Config::load_from_file(&path);

    // Determine Difficulty by user input
    let options = ["Easy", "Normal", "Hard"];
    let i = select("Schwierigkeit auswählen (Pfeiltasten, Enter)", &options);
    let mut game_rules = GameRules::new(Difficulty::from_i(i));

    // Start fight
    let monster = &mut config.enemy;
    match &mut config.player {
        PlayerType::Fighter(fighter) => {
            fighter.fight(monster, &mut game_rules);
        }
        PlayerType::Mage(mage) => {
            mage.fight(monster, &mut game_rules);
        }
    }
}
