pub fn panic(message: String) -> ! {
    log::error!("{}", message);
    std::process::exit(1)
}
