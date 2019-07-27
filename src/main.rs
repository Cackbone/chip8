use std::path::{ Path, PathBuf };
use std::convert::TryFrom;
use std::io;

use chip8::vm::VM;

fn main() -> io::Result<()> {
    let rom_path: PathBuf = Path::new("./roms/helloworld.rom").into();
    let vm = VM::try_from(rom_path)?;

    println!("Memory: {:?}", vm);
    Ok(())
}
