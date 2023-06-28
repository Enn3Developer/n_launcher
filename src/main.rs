use std::{
    env,
    io::{self, Write},
};

use n_launcher::{command::NCommand, technic::Technic};
use n_launcher_proc::cross_path;

// TODO: check data
// TODO: if update, get update from the correct url
// TODO: when updated, check checksums of file and get what's changed
// TODO: remove file from `mods/` that aren't in the updated `mods/`

fn main() {
    let home = dirs::home_dir().expect("can't find home dir");
    let root = home.join(cross_path!(".technic/modpacks/rgbcraft-server-modpack"));
//    let mut name = String::new();
//    print!("Write your username: ");
//    io::stdout().flush().unwrap();
//    io::stdin().read_line(&mut name).unwrap();
//    let name = name.trim();
//    if let Err(e) = env::set_current_dir(root) {
//        eprintln!("{e}");
//    }

    let technic = Technic::new(String::from("rgbcraft-test"));
    let data = technic.get_data();

    //let command = NCommand::default().with_user(name.to_string()).with_ram(2);
    //command.build().wait().unwrap();
}
