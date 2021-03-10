
fn main() {
    use clap::{load_yaml, App};

    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();

    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _ => unreachable!(),
        }
    }

    if m.is_present("subcmd") {
        println!("Ran subcmd")
    }
}
