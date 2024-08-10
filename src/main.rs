mod command_line_interface;
mod volume_control;
mod json_interface;

use clap::Parser;
use command_line_interface::Command;
use json_interface::{create_json, get_applications, get_browsers, get_current_application, 
    get_games, save_applications, save_browsers, save_current_application, save_games, CurrentApp};
use volume_control::{current_sessions, decrement_by_name, decrement_volume, increment_by_name, increment_volume};

fn session_exists(session_name: &str, sessions: &Vec<String>) -> bool {
    for session in sessions {
        if session.contains(session_name) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), clap::Error> {
    let cli = command_line_interface::Cli::parse();

    match cli.command {
        Command::GetCurrentSelection => {
            let current_application = get_current_application();
            match current_application.current_application {
                CurrentApp::Application(name) => println!("Current application: {}", name),
                CurrentApp::Browser => println!("Currently set to Browser"),
                CurrentApp::Game => println!("Currently set to Game"),
            }
            Ok(())
        }
        Command::ResetJSON => {
            create_json();
            Ok(())
        }
        Command::ListSessions => unsafe {
            volume_control::list_sessions();
            Ok(())
        }
        Command::AddGame(application) => {
            let mut games = get_games();
            games.games.push(application.name.unwrap());
            save_games(games);
            Ok(())
        }
        Command::AddBrowser(application) => {
            let mut browsers = get_browsers();
            browsers.browsers.push(application.name.unwrap());
            save_browsers(browsers);
            Ok(())
        }
        Command::AddApplication(application) => {
            let mut applications = get_applications();
            applications.applications.push(application.name.unwrap());
            save_applications(applications);
            Ok(())
        }
        Command::SetCurrentGame => {
            let mut current_application = get_current_application();
            current_application.current_application = CurrentApp::Game;
            save_current_application(current_application);
            Ok(())
        }
        Command::SetCurrentBrowser => {
            let mut current_application = get_current_application();
            current_application.current_application = CurrentApp::Browser;
            save_current_application(current_application);
            Ok(())
        }
        Command::SetCurrentApplication(application) => {
            let mut current_application = get_current_application();
            current_application.current_application = CurrentApp::Application(application.name.unwrap());
            save_current_application(current_application);
            Ok(())
        }
        Command::IncrementCurrentSelection => {
            let current_application = get_current_application();
            let current_sessions = unsafe { volume_control::current_sessions() };
            match current_application.current_application {
                CurrentApp::Application(name) => {
                    increment_by_name(&name)
                },
                CurrentApp::Browser => {
                    let browsers = get_browsers();
                    for browser in browsers.browsers {
                        if !session_exists(&browser, &current_sessions) {
                            continue;
                        }
                        increment_by_name(&browser)?;
                    }
                    Ok(())
                },
                CurrentApp::Game => {
                    let games = get_games();
                    for game in games.games {
                        if !session_exists(&game, &current_sessions) {
                            continue;
                        }
                        increment_by_name(&game)?;
                    }
                    Ok(())
                },
            }
        }
        Command::DecrementCurrentSelection => {
            let current_application = get_current_application();
            let current_sessions = unsafe { current_sessions() };
            match current_application.current_application {
                CurrentApp::Application(name) => {
                    decrement_by_name(&name)
                },
                CurrentApp::Browser => {
                    let browsers = get_browsers();
                    for browser in browsers.browsers {
                        if session_exists(&browser, &current_sessions) { decrement_by_name(&browser)?; }
                    }
                    Ok(())
                },
                CurrentApp::Game => {
                    let games = get_games();
                    for game in games.games {
                        if session_exists(&game, &current_sessions) { decrement_by_name(&game)?; }
                    }
                    Ok(())
                },
            }
        }
        Command::Increment(application) => unsafe {
            if let Some(session) = volume_control::get_session_by_name(&application.name.unwrap()) {
                increment_volume(&session)
            }
            else {
                eprintln!("Error getting session");
                Ok(())
            }
        },
        command_line_interface::Command::Decrement(application) => unsafe {
            if let Some(session) = volume_control::get_session_by_name(&application.name.unwrap()) {
                decrement_volume(&session)
            }
            else {
                eprintln!("Error getting session");
                Ok(())
            }
        }
        command_line_interface::Command::Toggle(application) => unsafe {
            if let Some(session) = volume_control::get_session_by_name(&application.name.unwrap()) {
                volume_control::toggle_session_mute(&session)
            }
            else {
                eprintln!("Error getting session");
                Ok(())
            }
        }
    }
}
