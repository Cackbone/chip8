use std::path::{ Path, PathBuf };
use std::convert::TryFrom;
use std::io;

use log::{ info, error };

use chip8::vm::VM;

fn main() -> io::Result<()> {
    env_logger::init();
    let rom_path: PathBuf = Path::new("./roms/helloworld.rom").into();
    let mut vm = VM::try_from(rom_path)?;

    // println!("Memory: {:?}", vm);

    while vm.run() {
        match vm.execute_next() {
            Ok(i) => info!("Instruction \"{}\" executed", i),
            Err(e) => error!("Error: {}", e)
        }
    }
    Ok(())
}
