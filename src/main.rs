use std::{
    env, fs,
    io::{self, Write},
};

use n_launcher::{command::NCommand, data::Data, technic::Technic};

// TODO: check data
// TODO: if update, get update from the correct url
// TODO: when updated, check checksums of file and get what's changed
// TODO: remove file from `mods/` that aren't in the updated `mods/`

fn main() {
    let home = dirs::home_dir().expect("can't find home dir");
    let root = home.join(".rgbcraft");
    let saved_data_path = root.join("n_launcher.json");

    if !root.exists() {
        fs::create_dir(&root).expect(&format!("can't create .rgbcraft dir: {:?}", root));
    }

    let data = if saved_data_path.exists() {
        Some(serde_json::from_str::<Data>(&fs::read_to_string(&saved_data_path).unwrap()).unwrap())
    } else {
        None
    };

    let name = if data.is_none() {
        let mut name = String::new();
        print!("Write your username: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        if let Err(e) = env::set_current_dir(root) {
            eprintln!("{e}");
        }
        name.to_string()
    } else {
        data.as_ref().unwrap().name()
    };

    let technic = Technic::new(String::from("rgbcraft-test"));
    let technic_data = technic.get_data();

    if let Some(d) = data {
        if d.technic_data().needs_update(&technic_data) {
            // TODO: update
        }
    } else {
        // TODO: download
    }

    let command = NCommand::default().with_user(name).with_ram(2);
    command.build().wait().unwrap();
}
