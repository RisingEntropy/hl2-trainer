// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod backend;
mod timing;

use std::error::Error;
use std::sync::mpsc::{channel};
use timing::Action;
use std::thread;
use webbrowser;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let (tx, rx) = channel::<Action>();



    ui.on_open_url(|| {
        println!("Open URL: https://risingentropy.top");
        match webbrowser::open("https://risingentropy.top") {
            Ok(_) => println!("Opened URL"),
            Err(e) => println!("Error opening URL: {}", e),
        }
    });
    let tx_thread = tx.clone();
    ui.on_enable_inf_ammo( move ||  {
        println!("Enable inf ammo");
        tx_thread.send(Action::EnableInfAmmo).unwrap();
    });
    let tx_thread = tx.clone();
    ui.on_disable_inf_ammo( move ||  {
        println!("Disable inf ammo");
        tx_thread.send(Action::DisableInfAmmo).unwrap();
    });

    let tx_thread = tx.clone();
    ui.on_enable_inf_energy( move ||  {

        println!("Enable inf energy");
        tx_thread.send(Action::EnableInfEnergy).unwrap();
    });
    let tx_thread = tx.clone();
    ui.on_disable_inf_energy( move ||  {

        println!("Disable inf energy");
        tx_thread.send(Action::DisableInfEnergy).unwrap();
    });

    let tx_thread = tx.clone();
    ui.on_enable_inf_health( move ||  {
        println!("Enable inf health");
        tx_thread.send(Action::EnableInfHealth).unwrap();
    });
    let tx_thread = tx.clone();
    ui.on_disable_inf_health( move ||  {
        println!("Disable inf health");
        tx_thread.send(Action::DisableInfHealth).unwrap();
    });

    let tx_thread = tx.clone();
    ui.on_enable_inf_shield( move ||  {
        println!("Enable inf shield");
        tx_thread.send(Action::EnableInfShield).unwrap();
    });
    let tx_thread = tx.clone();
    ui.on_disable_inf_shield( move ||  {
        println!("Disable inf shield");
        tx_thread.send(Action::DisableInfShield).unwrap();
    });

    thread::spawn(move || {
        timing::loop_thread(rx);
    });

    ui.run()?;

    Ok(())
}
