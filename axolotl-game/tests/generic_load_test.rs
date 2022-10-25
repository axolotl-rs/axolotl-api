use axolotl_game::GameConfig;
use log::warn;
use std::path::PathBuf;
use std::thread::sleep;

#[test]
pub fn load_game() {
    simple_log::quick!();
    let config = GameConfig {
        data_dump: PathBuf::from(env!("DATA_DUMP")),
        data_packs: vec![],
        axolotl_data: PathBuf::from(env!("AXOLOTL_DATA")),
    };
    let game = axolotl_game::AxolotlGame::load(config).unwrap();
}
