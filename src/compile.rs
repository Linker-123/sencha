use crate::asm::{Assemble, Context};
use std::path::{Path, PathBuf};
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
        .arg(fs::canonicalize(&Path::new("./build/source.nasm").as_os_str()).unwrap())
        .arg("-o ./build/source.o")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new("gcc")
        .arg("-no-pie")
        .arg("-m64")
        .arg("-osource.out")
        .arg("./source.o")
        .current_dir("./build")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    Command::new("./build/source.out").spawn().unwrap().wait().unwrap();

    Ok(())
}
