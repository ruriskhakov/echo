use clap::{command, Arg, ArgAction};

fn main() {
    let matches = command!()
        .name("echo")
        .version("0.1.0")
        .author("Rustem Iskhakov <ruriskhakov@gmail.com>")
        .about("write arguments to the standard output")
        .arg(Arg::new("text"))
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    if *matches.get_one::<bool>("omit_newline").unwrap() {
        match matches.get_one::<String>("text") {
            Some(text) => print!("{:?}", text),
            _ => print!(""),
        }
    } else {
        match matches.get_one::<String>("text") {
            Some(text) => println!("{}", text),
            _ => println!(""),
        }
    }
}
