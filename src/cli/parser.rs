#[derive(Debug)]
pub enum CliOption {
    TrueBool(String),
    OptionValue(String, String),
    Positional(String),
}

pub fn parse_options() -> Vec<CliOption> {
    let mut options = Vec::new();
    let mut args = std::env::args();
    args.next();

    loop {
        if let Some(current) = args.next() {
            if current.starts_with("--") {
                let delimeter = current
                    .chars()
                    .position(|x| x == '=')
                    .expect("CLI: Expected a '=' after option name");
                let name = current
                    .chars()
                    .skip(2)
                    .take(delimeter - 2)
                    .collect::<String>();
                let value = current.chars().skip(delimeter + 1).collect::<String>();
                options.push(CliOption::OptionValue(name, value));
            } else if current.starts_with("-") {
                let name = current.chars().skip(1).collect::<String>();
                options.push(CliOption::TrueBool(name));
            } else {
                options.push(CliOption::Positional(current));
            }
        } else {
            break;
        }
    }

    options
}
