use std::{env, io};

mod interactive_settings;
mod protocol;
mod setulp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.is_empty() {
        let handle = setulp::set_up();
        interactive_settings::set_interactive_settings(&handle);
    } else {
        print!("args arent supported for now");
    }
}
