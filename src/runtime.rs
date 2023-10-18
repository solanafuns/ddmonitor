use {
    chrono::{DateTime, Local},
    log::info,
    serde::{Deserialize, Serialize},
    solana_program::pubkey::Pubkey,
    std::{env, fs, path, str::FromStr},
};

pub const DEFAULT_PROGRAM: &str = "CkJ4NC4KCQfoXvyYj9Xxs4LkGDi34zNzE1e2EEeq1h9x";

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
    info!("check path : [ {} ] is dir...", path.display());
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

pub fn app_path(sub: &str) -> String {
    let app_dir = get_base_dir();
    let path = path::Path::new(&app_dir);
    path.join(sub).to_str().unwrap().to_string()
}

pub fn program_account(program_id: String) -> Pubkey {
    Pubkey::from_str(&program_id).unwrap()
}

pub fn current_timestamp() -> i64 {
    let local: DateTime<Local> = Local::now();
    local.timestamp_millis()
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerPrivate {
    pub secret: String,
}
