use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;

// Note names for applications must be the name of the exe file
const APPLICATIONS: [&str; 2]= ["spotify", "discord"];
const BROWSERS: [&str; 2] = ["chrome", "firefox"];
const GAMES: [&str; 16] = ["osu!", "rainbowsix", "minecraft", "valorant", 
    "rocketleague", "apex", "overwatch", "eldenring", "terraria", 
    "p5r", "ghostoftsushima", "discovery", "stellaris", "hollow_knight", 
    "ersc_launcher", "bg3"];

const HOME: &str = "C:/Users/Ethan"; // Change this to the path of the home directory

#[derive(Serialize, Deserialize, Debug)]
pub enum CurrentApp {
    Application(String),
    Browser,
    Game,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentApplication {
    pub current_application: CurrentApp,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationContainer {
    pub applications: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrowserContainer {
    pub browsers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameContainer {
    pub games: Vec<String>,
}

pub fn get_current_application() -> CurrentApplication {
    let current_application_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/current_application.json");
    let current_application_json = fs::read_to_string(current_application_path).expect("Unable to read file");
    let current_application: CurrentApplication = serde_json::from_str(&current_application_json).unwrap();
    current_application
}

pub fn get_applications() -> ApplicationContainer {
    let applications_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/applications.json");
    let applications_json = fs::read_to_string(applications_path).expect("Unable to read file");
    let applications: ApplicationContainer = serde_json::from_str(&applications_json).unwrap();
    applications
}

pub fn get_browsers() -> BrowserContainer {
    let browsers_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/browsers.json");
    let browsers_json = fs::read_to_string(browsers_path).expect("Unable to read file");
    let browsers: BrowserContainer = serde_json::from_str(&browsers_json).unwrap();
    browsers
}

pub fn get_games() -> GameContainer {
    let games_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/games.json");
    let games_json = fs::read_to_string(games_path).expect("Unable to read file");
    let games: GameContainer = serde_json::from_str(&games_json).unwrap();
    games
}

pub fn save_current_application(current_application: CurrentApplication) {
    let current_application_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/current_application.json");
    let current_application_json = serde_json::to_string_pretty(&current_application).unwrap();
    fs::write(current_application_path, current_application_json).expect("Unable to write file");
}

pub fn save_applications(applications: ApplicationContainer) {
    let applications_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/applications.json");
    let applications_json = serde_json::to_string_pretty(&applications).unwrap();
    fs::write(applications_path, applications_json).expect("Unable to write file");
}

pub fn save_browsers(browsers: BrowserContainer) {
    let browsers_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/browsers.json");
    let browsers_json = serde_json::to_string_pretty(&browsers).unwrap();
    fs::write(browsers_path, browsers_json).expect("Unable to write file");
}

pub fn save_games(games: GameContainer) {
    let games_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/games.json");
    let games_json = serde_json::to_string_pretty(&games).unwrap();
    fs::write(games_path, games_json).expect("Unable to write file");
}

pub fn create_json() {
    let current_application = CurrentApplication {
        current_application: CurrentApp::Application("spotify".to_string()),
    };

    let applications = ApplicationContainer {
        applications: APPLICATIONS.iter().map(|s| s.to_string()).collect(),
    };
    let browsers = BrowserContainer {
        browsers: BROWSERS.iter().map(|s| s.to_string()).collect(),
    };
    let games = GameContainer {
        games: GAMES.iter().map(|s| s.to_string()).collect(),
    };

    let applications_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/applications.json");
    let browsers_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/browsers.json");
    let games_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/games.json");
    let current_application_path = PathBuf::from(HOME).join("AppData/Keyboard Mixer/current_application.json");

    let applications_json = serde_json::to_string_pretty(&applications).unwrap();
    let browsers_json = serde_json::to_string_pretty(&browsers).unwrap();
    let games_json = serde_json::to_string_pretty(&games).unwrap();
    let current_application_json = serde_json::to_string_pretty(&current_application).unwrap();

    fs::write(applications_path, applications_json).expect("Unable to write file");
    fs::write(browsers_path, browsers_json).expect("Unable to write file");
    fs::write(games_path, games_json).expect("Unable to write file");
    fs::write(current_application_path, current_application_json).expect("Unable to write file");
}
