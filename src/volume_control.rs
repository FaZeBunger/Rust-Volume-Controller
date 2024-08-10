use winmix::WinMix;
use clap::Error;

const INCREMENT_VALUE: f32 = 0.05;

pub unsafe fn current_sessions() -> Vec<String> {
    let winmix = WinMix::default();
    let sessions = winmix.enumerate().unwrap();
    let mut session_names = Vec::new();
    for session in sessions {
        session_names.push(session.path.clone());
    }
    session_names
}

pub unsafe fn list_sessions() {
    let winmix = WinMix::default();
    let sessions = winmix.enumerate().unwrap();
    for session in sessions {
        println!("Path: {}", session.path);
        println!("PID: {}", session.pid);
        println!("Volume: {}", session.vol.get_master_volume().unwrap());
        println!("Muted: {}", session.vol.get_mute().unwrap());
        println!();
    }
}

pub fn increment_by_name(session_name: &str) -> Result<(), Error> {
    unsafe {
        if let Some(session) = get_session_by_name(session_name) {
            increment_volume(&session)?;
        }
    }
    Ok(())
}

pub fn decrement_by_name(session_name: &str) -> Result<(), Error> {
    unsafe {
        if let Some(session) = get_session_by_name(session_name) {
            decrement_volume(&session)?;
        }
    }
    Ok(())
}

pub unsafe fn increment_volume(session: &winmix::Session) -> Result<(), Error> {
    if let Ok(cur_volume) = session.vol.get_master_volume() {
        let new_volume = cur_volume + INCREMENT_VALUE;
        if new_volume <= 1.0 {
            match session.vol.set_master_volume(new_volume) {
                Ok(_) => (),
                Err(e) => eprintln!("Error setting volume: {}", e),
            }
        }
        else {
            match session.vol.set_master_volume(1.0) {
                Ok(_) => (),
                Err(e) => eprintln!("Error setting volume: {}", e),
            }
        }
    }
    else {
        eprintln!("Error getting volume");
    }
    Ok(())
}

pub unsafe fn decrement_volume(session: &winmix::Session) -> Result<(), Error> {
    if let Ok(cur_volume) = session.vol.get_master_volume() {
        let new_volume = cur_volume - INCREMENT_VALUE;
        if new_volume >= 0.0 {
            match session.vol.set_master_volume(new_volume) {
                Ok(_) => (),
                Err(e) => eprintln!("Error setting volume: {}", e),
            }
        }
        else {
            match session.vol.set_master_volume(0.0) {
                Ok(_) => (),
                Err(e) => eprintln!("Error setting volume: {}", e),
            }
        }
    }
    else {
        eprintln!("Error getting volume");
    }
    Ok(())
}

pub unsafe fn set_session_volume(session: &winmix::Session, volume: f32) {
    match session.vol.set_master_volume(volume) {
        Ok(_) => (),
        Err(e) => eprintln!("Error setting volume: {}", e),
    }
}

pub unsafe fn get_session_by_name(name: &str) -> Option<winmix::Session> {
    unsafe {
        let winmix = WinMix::default();
        let sessions = winmix.enumerate().unwrap();
        for session in sessions {
            if session.path.to_lowercase().contains(&name.to_lowercase()) {
                return Some(session);
            }
        }
    }
    None
}

pub unsafe fn get_session_by_pid(pid: u32) -> Option<winmix::Session> {
    unsafe {
        let winmix = WinMix::default();
        let sessions = winmix.enumerate().unwrap();
        for session in sessions {
            if session.pid == pid {
                return Some(session);
            }
        }
    }
    None
}

pub unsafe fn toggle_session_mute(session: &winmix::Session) -> Result<(), Error>{
    match session.vol.get_mute() {
        Ok(muted) => {
            match session.vol.set_mute(!muted) {
                Ok(_) => (),
                Err(e) => eprintln!("Error setting mute: {}", e),
            }
        },
        Err(e) => eprintln!("Error getting mute: {}", e),
    }
    Ok(())
}
