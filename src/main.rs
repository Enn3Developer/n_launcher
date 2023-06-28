use std::{
    env, fs,
    io::{self, Write},
};

use n_launcher::{command::NCommand, data::Data, download, technic::Technic};

fn main() {
    let home = dirs::home_dir().expect("can't find home dir");
    let root = home.join(".rgbcraft");
    let saved_data_path = root.join("n_launcher.json");

    if !root.exists() {
        fs::create_dir(&root).unwrap_or_else(|_| panic!("can't create .rgbcraft dir: {:?}", &root));
    }

    let mut data = if saved_data_path.exists() {
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
        if let Err(e) = env::set_current_dir(&root) {
            eprintln!("{e}");
        }
        name.to_string()
    } else {
        data.as_ref().unwrap().name()
    };

    let technic = Technic::new(String::from("rgbcraft-test"));
    let technic_data = technic.get_data();

    if let Some(d) = data.as_mut() {
        if d.technic_data().needs_update(&technic_data) {
            if root.join("mods").exists() {
                fs::remove_dir_all(root.join("mods")).expect("can't remove mods folder");
            }

            download(&root, &technic_data);

            d.set_technic_data(technic_data);
        }
    } else {
        download(&root, &technic_data);

        data = Some(Data::new(
            name.clone(),
            technic_data,
            2,
            String::from("java"),
        ));
    }

    fs::write(
        saved_data_path,
        serde_json::to_string(&data.unwrap()).expect("invalid json"),
    )
    .expect("can't save data");

    let command = NCommand::default().with_user(name).with_ram(2);
    command.build().wait().unwrap();
}
