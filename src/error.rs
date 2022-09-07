pub fn panic(message: String) -> ! {
    log::error!("{}", message);
    std::process::exit(1)
}

pub fn panic_str(message: &str) -> ! {
    log::error!("{}", message);
    std::process::exit(1)
}
