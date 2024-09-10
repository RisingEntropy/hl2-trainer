use std::cmp::min;
use std::fmt::Display;
use proc_mem::{Process};
pub struct Hl2TrainerBackend {
    process: Process,
}

impl Hl2TrainerBackend {
    fn write_chain<T:Default + PartialOrd + Display>(process: &Process, chain: &Vec<usize>, final_offset:i32, value:T, max_value:T)
                              -> Result<(), &'static str>{
        let mut address = chain[0];
        let mut i = 1;
        while i<chain.len()-1{
            address += chain[i];
            if process.iswow64 {
                address = process.read_mem::<u32>(address).ok().ok_or("Unable to read memory")? as usize;
            } else {
                address = process.read_mem::<u64>(address).ok().ok_or("Unable to read memory")? as usize;
            }
            i += 1;
        }
        address += chain[i];
        address = (address as i32 + final_offset)as usize;
        let target_mem_value = process.read_mem::<T>(address).ok().ok_or("Unable to read memory")?;
        if target_mem_value <= max_value{

            process.write_mem::<T>(address,value);
            Ok(())
        }else{
            Err("Target memory shows an invalid value, ignore!")
        }

    }
    pub fn new()->Result<Hl2TrainerBackend,&'static str>{
        let proc = Process::with_name("hl2.exe").ok().ok_or("Unable to get handler of hl2.exe")?;
        Ok(Hl2TrainerBackend{
            process: proc
        })
    }

    pub fn change_health(&self, tar:i32)->Result<(), &'static str>{
        let server_dll = self.process.module("server.dll").ok().ok_or("Unable to get handler of server.dll")?;
        let client_dll = self.process.module("client.dll").ok().ok_or("Unable to get handler of client.dll")?;
        let chain = vec![server_dll.base_address(), 0x635334, 0xE0];
        let ui_chain = vec![client_dll.base_address(), 0x4A46CC, 0x88];
        Self::write_chain(&self.process, &chain, 0, min(tar,120), 120).ok().ok_or("Fail to write memory")?;
        Self::write_chain(&self.process, &ui_chain,0,min(tar,120),120).ok().ok_or("Fail to write to UI memory")?;
        Ok(())
    }

    pub fn change_energy(&self, tar:i32)->Result<(), &'static str>{
        let server_dll = self.process.module("server.dll").ok().ok_or("Unable to get handler of server.dll")?;
        let chain = vec![server_dll.base_address(), 0x635334, 0x10D4];
        Self::write_chain(&self.process, &chain, 0, min(tar,0x42C80000), 0x42C80000).ok().ok_or("Fail to write memory")?;
        Ok(())
    }

    pub fn change_shield(&self, tar:i32)->Result<(), &'static str>{

        let server_dll = self.process.module("server.dll").ok().ok_or("Unable to get handler of server.dll")?;
        let chain = vec![server_dll.base_address(), 0x635334, 0xD30];
        Self::write_chain(&self.process, &chain, 0, min(tar,200), 200).ok().ok_or("Fail to write memory")?;
        Ok(())
    }
    pub fn change_ammo(&self, bullet:i32, grand:i32, sketch_ball:i32)->Result<(), &'static str>{

        let server_dll = self.process.module("server.dll").ok().ok_or("Unable to get handler of server.dll")?;
        let client_dll = self.process.module("client.dll").ok().ok_or("Unable to get handler of client.dll")?;

        for i in -4..6{
            let chain = vec![server_dll.base_address(),0x635334,0x6E0];
            let ui_chain = vec![client_dll.base_address(), 0x4A46CC, 0xCB0];
            Self::write_chain(&self.process, &chain, 4*i, min(bullet,300), 300).ok().ok_or("Fail to write memory")?;
            Self::write_chain(&self.process, &ui_chain, 4*i, min(bullet,300), 300).ok().ok_or("Fail to write to UI memory")?;
        }
        let chain = vec![server_dll.base_address(), 0x635334,0x700];
        let ui_chain = vec![client_dll.base_address(), 0x4A46CC, 0xCD0];
        Self::write_chain(&self.process, &chain,0,min(grand,300), 300).ok().ok_or("Fail to write memory")?;
        Self::write_chain(&self.process, &ui_chain,0,min(grand, 300), 300).ok().ok_or("Fail to write to UI memory")?;

        let chain = vec![server_dll.base_address(), 0x635334, 0x728];
        Self::write_chain(&self.process, &chain, 0, min(sketch_ball, 1000), 1000).ok().ok_or("Fail to write memory")?;
        Ok(())
    }
}