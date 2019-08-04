use std::path::{ Path, PathBuf };
use std::convert::TryFrom;
use std::io;

use log::{ info };

use chip8::config::Config;
use chip8::vm::VM;

fn main() -> io::Result<()> {
    env_logger::init();
    let config = Config::from_args()?;
    let rom_path: PathBuf = Path::new(&(config.file)).into();
    let mut vm = VM::try_from(rom_path)?;

    if config.disassemble {
        for (i, x) in vm.get_program().iter().enumerate() {
            println!("{} {}", i, x);
        }
    } else {
        while vm.run() {
            let instruction = vm.execute_next()?;
            info!("({} -> {}) Instruction executed", instruction.to_asm(), instruction);
        }
    }
    Ok(())
}
