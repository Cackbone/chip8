use std::path::{ Path, PathBuf };
use std::convert::TryFrom;
use std::io;


use chip8::vm::VM;

fn main() -> io::Result<()> {
    env_logger::init();
    let rom_path: PathBuf = Path::new("./roms/helloworld.rom").into();
    let mut vm = VM::try_from(rom_path)?;

    while vm.run() {
        vm.execute_next()?;
        //println!("{:?}", vm);
    }
    Ok(())
}
