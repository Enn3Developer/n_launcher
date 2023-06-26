use std::process::{Child, Command};

use n_launcher_proc::cross_path;

const COMMON: &[&str] = &[
    "-XX:+UnlockExperimentalVMOptions",
    "-XX:+UseG1GC",
    "-XX:G1NewSizePercent=20",
    "-XX:G1ReservePercent=20",
    "-XX:MaxGCPauseMillis=50",
    "-XX:G1HeapRegionSize=32M",
    cross_path!("-Djava.library.path=bin/natives"),
    "-Dfml.core.libraries.mirror=http://mirror.technicpack.net/Technic/lib/fml/%s",
    "-Dfml.ignoreInvalidMinecraftCertificates=true",
    "-Dfml.ignorePatchDiscrepancies=true",
    "-Dminecraft.applet.TargetDirectory=.",
    "-Duser.language=en",
    "-cp",
    cross_path!("../../cache/net/sf/jopt-simple/jopt-simple/4.5/jopt-simple-4.5.jar:../../cache/org/ow2/asm/asm-all/4.1/asm-all-4.1.jar:../../cache/net/java/jinput/jinput/2.0.5/jinput-2.0.5.jar:../../cache/net/java/jutils/jutils/1.0.0/jutils-1.0.0.jar:../../cache/org/lwjgl/lwjgl/lwjgl/2.9.0/lwjgl-2.9.0.jar:../../cache/org/lwjgl/lwjgl/lwjgl_util/2.9.0/lwjgl_util-2.9.0.jar:../../cache/net/technicpack/legacywrapper/1.2.1/legacywrapper-1.2.1.jar:bin/modpack.jar:bin/minecraft.jar")
];

const LAUNCHER: &str = "net.technicpack.legacywrapper.Launch";

const FINAL: &[&str] = &[
    "--gameDir",
    ".",
    "--assetsDir",
    "resources",
    "--icon",
    "icon.png",
    "--title",
];
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

    pub fn build(&self) -> Child {
        let xms = format!("-Xms{}G", self.ram);
        let xmx = format!("-Xmx{}G", self.ram);
        Command::new(&self.java)
            .arg(xms)
            .arg(xmx)
            .args(COMMON)
            .arg(LAUNCHER)
            .arg(&self.user)
            .args(FINAL)
            .arg(&self.title)
            .spawn()
            .unwrap_or_else(|_| panic!("can't execute {}", self.java))
    }
}

impl Default for NCommand {
    fn default() -> Self {
        Self::new(
            String::new(),
            0,
            String::from("Project FPS"),
            String::from("java"),
        )
    }
}
