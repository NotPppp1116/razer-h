use std::env;

mod interactive_settings;
mod protocol;
mod setulp;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let handle = setulp::set_up();
        interactive_settings::set_interactive_settings(&handle);
    } else {
        print!("args aren't supported for now");
    }
}
