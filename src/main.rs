use std::path::{ Path, PathBuf };
use std::convert::TryFrom;
use std::io;


use chip8::vm::VM;

fn main() -> io::Result<()> {
    env_logger::init();
    let rom_path: PathBuf = Path::new("./roms/helloworld.rom").into();
    let vm = VM::try_from(rom_path)?;

    for (i, x) in vm.get_program().iter().enumerate() {
        println!("{} {}", i, x);
    }
    // while vm.run() {
    //     let instruction = vm.execute_next()?;
    //     println!("{}", instruction.to_asm());
    // }
    Ok(())
}
