use crate::asm::{Assemble, Context};
use std::process::Command;
use std::{
    fs::{self, OpenOptions},
    io::{Error, Write},
};

pub fn compile_ctx(context: Context) -> Result<(), Error> {
    fs::create_dir_all("build")?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("./build/source.nasm")?;

    file.write_all(context.to_asm().as_bytes())?;

    Command::new("nasm")
        .arg("-felf64")
        .arg("./build/source.nasm")
        .arg("-o ./build/source.o")
        .spawn()?
        .wait()?;
    Command::new("gcc")
        .arg("-no-pie")
        .arg("-m64")
        .arg("-osource.out")
        .arg("./source.o")
        .current_dir("./build")
        .spawn()?
        .wait()?;
    Command::new("./build/source.out").spawn()?.wait()?;

    Ok(())
}
