use std::{panic};

use color_eyre::{
    Result,
    config::HookBuilder,
    eyre,
};

use crate::storage::Storage;

mod tui;
mod app;
mod storage;
mod util;
mod login;
mod user;
mod objects;
mod components;

fn main() -> Result<()> {
    let storage_path = std::env::args().nth(1);
    install_hooks()?;

    let storage = match storage_path {
        Some(path) => Storage::load(path).expect("storage path to be valid"),
        None => match Storage::load(String::from("default.json")) {
            Ok(storage) => storage,
            Err(err) => {
                println!("{:?}", err);
                Storage::new(String::from("fallback.json"))
            }
        },
    };
    let mut terminal = tui::init()?;

    // let user = login::LoginScreen::new(storage.users.clone()).run(&mut terminal)?;
    let user = storage.users.first().cloned().unwrap_or(user::User {
        username: String::from("captain"),
        password_start: String::new(),
        password_attempts: 0,
        password_attempts_max: 0,
        password: String::new(),
        pos_x: 0.0,
        pos_y: 0.0,
        fuel: 2,
        crystals: 0,
        reputation: 0,
    });
    let _app_result = app::App::new(storage, user).run(&mut terminal);

    tui::restore()?;
    Ok(())
}

pub fn install_hooks() -> color_eyre::Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        tui::restore().unwrap();
        panic_hook(panic_info);
    }));

    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            tui::restore().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}

