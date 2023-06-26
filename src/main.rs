use std::{
    env,
    io::{self, Write},
    process::{exit, Command},
};

use n_launcher_proc::cross_path;

fn main() {
    let home = dirs::home_dir().expect("can't find home dir");
    let root = home.join(cross_path!(".technic/modpacks/rgbcraft-server-modpack"));
    let mut name = String::new();
    print!("Write your username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    if let Err(e) = env::set_current_dir(&root) {
        eprintln!("{e}");
    }

    println!("{}", cross_path!("-Djava.library.path=bin/natives"));

    //    let mut command = Command::new("java")
    //        .args([
    //            "-XX:+UnlockExperimentalVMOptions",
    //            "-XX:+UseG1GC",
    //            "-XX:G1NewSizePercent=20",
    //            "-XX:G1ReservePercent=20",
    //            "-XX:MaxGCPauseMillis=50",
    //            "-XX:G1HeapRegionSize=32M",
    //            "-Xms2048m",
    //            "-Xmx2048m",
    //            #[cfg(not(target_os="windows"))]
    //            "-Djava.library.path=bin/natives",
    //            #[cfg(target_os="windows")]
    //            "-Djava.library.path=bin\\natives",
    //            "-Dfml.core.libraries.mirror=http://mirror.technicpack.net/Technic/lib/fml/%s",
    //            "-Dfml.ignoreInvalidMinecraftCertificates=true",
    //            "-Dfml.ignorePatchDiscrepancies=true",
    //            "-Dminecraft.applet.TargetDirectory=.",
    //            "-Duser.language=en",
    //            "-cp",
    //            #[cfg(not(target_os="windows"))]
    //            "../../cache/net/sf/jopt-simple/jopt-simple/4.5/jopt-simple-4.5.jar:../../cache/org/ow2/asm/asm-all/4.1/asm-all-4.1.jar:../../cache/net/java/jinput/jinput/2.0.5/jinput-2.0.5.jar:../../cache/net/java/jutils/jutils/1.0.0/jutils-1.0.0.jar:../../cache/org/lwjgl/lwjgl/lwjgl/2.9.0/lwjgl-2.9.0.jar:../../cache/org/lwjgl/lwjgl/lwjgl_util/2.9.0/lwjgl_util-2.9.0.jar:../../cache/net/technicpack/legacywrapper/1.2.1/legacywrapper-1.2.1.jar:bin/modpack.jar:bin/minecraft.jar",
    //            #[cfg(target_os="windows")]
    //            "..\\..\\cache\\net\\sf\\jopt-simple\\jopt-simple\\4.5\\jopt-simple-4.5.jar:..\\..\\cache\\org\\ow2\\asm\\asm-all\\4.1\\asm-all-4.1.jar:..\\..\\cache\\net\\java\\jinput\\jinput\\2.0.5\\jinput-2.0.5.jar:..\\..\\cache\\net\\java\\jutils\\jutils\\1.0.0\\jutils-1.0.0.jar:..\\..\\cache\\org\\lwjgl\\lwjgl\\lwjgl\\2.9.0\\lwjgl-2.9.0.jar:..\\..\\cache\\org\\lwjgl\\lwjgl\\lwjgl_util\\2.9.0\\lwjgl_util-2.9.0.jar:..\\..\\cache\\net\\technicpack\\legacywrapper\\1.2.1\\legacywrapper-1.2.1.jar:bin\\modpack.jar:bin\\minecraft.jar",
    //            "net.technicpack.legacywrapper.Launch",
    //            &name,
    //            "--gameDir",
    //            ".",
    //            "--assetsDir",
    //            "resources",
    //            "--title",
    //            "Not Project FPS, Launched by NLauncher",
    //            "--icon",
    //            "icon.png",
    //        ])
    //        .spawn()
    //        .expect("can't execute java");
    //    command.wait().unwrap();
}
