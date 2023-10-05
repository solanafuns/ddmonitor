use {
    solana_program::pubkey::Pubkey,
    std::{env, fs, path, str::FromStr},
};

pub const PROGRAME_ID: &str = "HZRahcg3oLXw4GScUN7bzCfHWx33G6SBrg6G1vVL1qEm";
pub const RPC_URL: &str = "http://127.0.0.1:8899";
pub const WS_URL: &str = "ws://127.0.0.1:8900";
pub const AIRDROP: bool = true;
pub const LAMPORTS_PER_SOL: u64 = u64::pow(10, 9);

pub fn get_base_dir() -> String {
    let mut dir_path = String::from("./");
    const BASE_DIR_NAME: &str = "app_base";
    if env::var_os(BASE_DIR_NAME).is_some() {
        if let Ok(value) = env::var(BASE_DIR_NAME) {
            dir_path = value.to_owned();
        }
    }
    dir_path
}

pub fn init_app() {
    confirm_dir();
}

pub fn confirm_dir() {
    confirm_app_dir("");
    confirm_app_dir("private");
    confirm_app_dir("public");
}

pub fn confirm_app_dir(sub: &str) {
    let check_path = app_path(sub);
    let path = path::Path::new(&check_path);
    println!("check path : [ {} ] is dir...", path.display());
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

pub fn app_path(sub: &str) -> String {
    let app_dir = get_base_dir();
    let path = path::Path::new(&app_dir);
    path.join(sub).to_str().unwrap().to_string()
}

pub fn program_account() -> Pubkey {
    Pubkey::from_str(PROGRAME_ID).unwrap()
}
