use std::path::PathBuf;
use std::process::{Child, Command};

use n_launcher_proc::cross_path;

#[cfg(not(target_os = "windows"))]
const COMMAND: &str = "/bin/bash";

#[cfg(target_os = "windows")]
const COMMAND: &str = "cmd";

const COMMON: &[&str] = &[
    "-XX:+UnlockExperimentalVMOptions",
    "-XX:+UseG1GC",
    "-XX:G1NewSizePercent=20",
    "-XX:G1ReservePercent=20",
    "-XX:MaxGCPauseMillis=50",
    "-XX:G1HeapRegionSize=32M",
    "-cp",
    cross_path!("'bin/jopt-simple-4.5.jar:bin/jinput.jar:bin/jutils-1.0.0.jar:bin/lwjgl.jar:bin/lwjgl_util.jar:bin/legacywrapper-1.2.1.jar:bin/modpack.jar:bin/minecraft.jar'"),
    cross_path!("-Djava.library.path=bin/natives"),
    "-Dfml.ignoreInvalidMinecraftCertificates=true",
    "-Dfml.ignorePatchDiscrepancies=true",
    "net.technicpack.legacywrapper.Launch",
];

const FINAL: &[&str] = &["--assetsDir", "resources", "--icon", "icon.png", "--title"];
pub struct NCommand {
    user: String,
    ram: u32,
    title: String,
    java: String,
}

impl NCommand {
    pub fn new(user: String, ram: u32, title: String, java: String) -> Self {
        Self {
            user,
            ram,
            title,
            java,
        }
    }

    pub fn with_user(mut self, user: String) -> Self {
        self.user = user;
        self
    }

    pub fn with_ram(mut self, ram: u32) -> Self {
        self.ram = ram;
        self
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn with_java(mut self, java: String) -> Self {
        self.java = java;
        self
    }

    pub fn build(&self, dir: PathBuf) -> Child {
        let xms = format!("-Xms{}G", self.ram);
        let xmx = format!("-Xmx{}G", self.ram);
        let java = format!(
            "{} {} {} {} {} --gameDir {} {} {}",
            self.java,
            xms,
            xmx,
            COMMON.join(" "),
            self.user,
            dir.to_string_lossy(),
            FINAL.join(" "),
            self.title
        );

        #[cfg(not(target_os = "windows"))]
        let c = "-c";

        #[cfg(target_os = "windows")]
        let c = "/c";

        Command::new(COMMAND)
            .current_dir(&dir)
            .arg(c)
            .arg(java)
            .spawn()
            .expect("can't execute command")
    }
}

impl Default for NCommand {
    fn default() -> Self {
        Self::new(
            String::new(),
            1,
            String::from("'Project FPS'"),
            String::from("java"),
        )
    }
}
