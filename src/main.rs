use reqwest::blocking::Client;
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
        println!("creating directory");
        fs::create_dir(&root).unwrap_or_else(|_| panic!("can't create .rgbcraft dir: {:?}", &root));
    }

    let mut data = if saved_data_path.exists() {
        println!("saved data found");
        if let Ok(data) =
            serde_json::from_str::<Data>(&fs::read_to_string(&saved_data_path).unwrap())
        {
            Some(data)
        } else {
            None
        }
    } else {
        println!("no saved data found");
        None
    };

    let name = if data.is_none() {
        let mut name = String::new();
        print!("Write your username: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim();
        env::set_current_dir(&root).expect("can't change dir");
        name.to_string()
    } else {
        println!("got name from saved data");
        data.as_ref().unwrap().name()
    };

    println!("getting data from technic");
    let technic = Technic::new(String::from("rgbcraft-test"));
    let technic_data = technic.get_data();

    if let Some(d) = data.as_mut() {
        println!("checking updates");
        if d.technic_data().needs_update(&technic_data) {
            println!("found new update");
            if root.join("mods").exists() {
                fs::remove_dir_all(root.join("mods")).expect("can't remove mods folder");
            }

            download(&root, &technic_data);

            d.set_technic_data(technic_data);
        }
    } else {
        println!("downloading for the first time");
        download(&root, &technic_data);

        let client = Client::builder()
            .timeout(None)
            .build()
            .expect("can't create client");

        println!("downloading minecraft.jar");
        let resp = client
            .get("https://launcher.mojang.com/v1/objects/53ed4b9d5c358ecfff2d8b846b4427b888287028/client.jar")
            .send()
            .expect("can't download minecraft.jar")
            .bytes()
            .expect("can't get minecraft bytes");

        fs::write(root.join("bin").join("minecraft.jar"), resp).expect("can't write minecraft.jar");

        println!("downloading icon");
        let resp = reqwest::blocking::get(technic_data.icon_url())
            .expect("can't download icon")
            .bytes()
            .expect("can't read image bytes");

        fs::write(root.join("icon.png"), resp).expect("can't write icon.png");

        data = Some(Data::new(
            name.clone(),
            technic_data,
            2,
            String::from("java"),
        ));
    }

    println!("saving data");
    fs::write(
        saved_data_path,
        serde_json::to_string(&data.unwrap()).expect("invalid json"),
    )
    .expect("can't save data");

    println!("starting project fps");
    let command = NCommand::default().with_user(name).with_ram(2);
    command.build(root).wait().unwrap();
}
