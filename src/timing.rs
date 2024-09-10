use std::thread;
use std::sync::mpsc::{Receiver};
use std::time::Duration;
use crate::backend::Hl2TrainerBackend;
#[derive(Clone)]
pub struct State{
    pub inf_energy:bool,
    pub inf_health:bool,
    pub inf_shield:bool,
    pub inf_ammo:bool
}
impl State{
    pub fn new()->State{
        State{
            inf_energy:false,
            inf_health:false,
            inf_shield:false,
            inf_ammo:false
        }
    }
}
pub enum Action{
    EnableInfEnergy,
    DisableInfEnergy,
    EnableInfHealth,
    DisableInfHealth,
    EnableInfShield,
    DisableInfShield,
    EnableInfAmmo,
    DisableInfAmmo
}
pub fn loop_thread(rx:Receiver<Action>){

    let mut state = State::new();
    loop{
        match rx.try_recv() {
            Ok(s) => {
                match s {
                    Action::EnableInfAmmo => state.inf_ammo = true,
                    Action::DisableInfAmmo => state.inf_ammo = false,
                    Action::EnableInfEnergy => state.inf_energy = true,
                    Action::DisableInfEnergy => state.inf_energy = false,
                    Action::EnableInfHealth => state.inf_health = true,
                    Action::DisableInfHealth => state.inf_health = false,
                    Action::EnableInfShield => state.inf_shield = true,
                    Action::DisableInfShield => state.inf_shield = false
                }
            },
            Err(_) => {}
        }
        let state_ref = &state;
        let backend = Hl2TrainerBackend::new();
        match backend {
            Ok(b) => {
                if state_ref.inf_health{
                    let res = b.change_health(120);
                    if res.is_err(){
                        println!("{}",res.unwrap_err());
                    }
                }
                if state_ref.inf_energy{
                    let res = b.change_energy(0x42C80000);
                    if res.is_err(){
                        println!("{}",res.unwrap_err());
                    }
                }
                if state_ref.inf_shield{
                    let res = b.change_shield(120);
                    if res.is_err(){
                        println!("{}",res.unwrap_err());
                    }
                }
                if state_ref.inf_ammo{
                    let res = b.change_ammo(255,255,255);
                    if res.is_err(){
                        println!("{}",res.unwrap_err());
                    }
                }
            },
            Err(e) => {
                println!("{}",e);
            }
        }

        thread::sleep(Duration::from_millis(100));
    }
}