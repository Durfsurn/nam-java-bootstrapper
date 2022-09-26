fn main() {
    if std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .contains(".zip")
    {
        panic!("Running inside a .zip is unsupported!");
    }

    check_java();
}

#[cfg(target_os = "linux")]
fn check_java() {
    let cmd = std::process::Command::new("java")
        .arg("-version")
        .output()
        .unwrap();

    if cmd.status.success() {
        // you have java!
        std::process::Command::new("sudo")
            .args(["java", "-jar", "NetworkAddonMod_Setup_Version44.jar"])
            .spawn()
            .unwrap();
    } else {
        open::that("https://adoptium.net/temurin/releases/?version=8").unwrap();
    }
}

#[cfg(target_os = "macos")]
fn check_java() {
    let cmd = std::process::Command::new("java")
        .arg("-version")
        .output()
        .unwrap();

    if cmd.status.success() {
        // you have java!
        std::process::Command::new("sudo")
            .args(["java", "-jar", "NetworkAddonMod_Setup_Version44.jar"])
            .spawn()
            .unwrap();
    } else {
        open::that("https://adoptium.net/temurin/releases/?version=8").unwrap();
    }
}

#[cfg(target_os = "windows")]
fn check_java() {
    println!("Checking if Java is installed...");
    let cmd = std::process::Command::new("java")
        .arg("-version")
        .output()
        .unwrap();

    if cmd.status.success() {
        println!("\x1b[1;32mJava is installed!\x1b[0m");
        println!("Looking for SimCity 4.exe...");

        // patch sc4 exe
        let mut files = Vec::new();
        for folder in [
            r"C://Program Files/Steam/steamapps/common/SimCity 4 Deluxe",
            r"C://Program Files (x86)/Steam/steamapps/common/SimCity 4 Deluxe",
            r"C://Program Files/Maxis/SimCity 4",
            r"C://Program Files (x86)/Maxis/SimCity 4",
            r"C://Program Files/Maxis/SimCity 4 Deluxe",
            r"C://Program Files (x86)/Maxis/SimCity 4 Deluxe",
        ] {
            if std::fs::read_dir(folder).is_ok() {
                let mut pathbuf: std::path::PathBuf = folder.parse().unwrap();
                pathbuf.push("Apps");
                pathbuf.push("SimCity 4.exe");
                files.push(pathbuf);
            }
        }

        println!("\x1b[1;34mSimCity 4 Deluxe folder not found! Would you like a systemwide search (S) or provide a filepath to your SimCity 4.exe? (P)");
        println!("Any other character will exit.\x1b[0m");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_uppercase() == "P" {
            println!("{}[AInput your path:", 27u8 as char);
            let mut value = String::new();
            std::io::stdin().read_line(&mut value).unwrap();

            files.push(value.trim().parse().unwrap())
        } else if files.is_empty() && input.trim().to_uppercase() == "S" {
            println!("{}[A ", 27u8 as char);
            files = ('A'..='Z') // 'Z'
                .into_iter()
                .flat_map(|c| {
                    println!("Checking hard disk {c}:// for SimCity 4 Deluxe folder...");
                    walkdir::WalkDir::new(format!("{c}://"))
                        .max_depth(10)
                        .into_iter()
                        .filter_entry(|i| i.path().is_dir())
                        .filter_map(|i| i.ok())
                        .map(|i| i.path().to_path_buf())
                        .filter(|f| {
                            f.file_name().and_then(|f| f.to_str()) == Some("SimCity 4 Deluxe")
                        })
                        .map(|f| f.canonicalize().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
        } else {
            panic!("Invalid character entered.");
        }

        for mut file in files {
            file.push("Apps");
            file.push("SimCity 4.exe");
            let sc4_path = file.to_string_lossy().replace(r"\\?\", "");
            println!("Patching {sc4_path}...");

            std::process::Command::new(".\\4gb_patch.exe")
                .arg(sc4_path.clone())
                .output()
                .unwrap();

            if std::fs::read(sc4_path.replace(".exe", ".exe.Backup")).is_ok() {
                println!("\x1b[1;32mPatched {sc4_path}!\x1b[0m");

                // you have java and your exe is patched!
                std::process::Command::new("cmd")
                    .args([
                        "/c",
                        "start",
                        "/MIN",
                        "java",
                        "-jar",
                        ".\\NetworkAddonMod_Setup_Version44.jar",
                    ])
                    .spawn()
                    .unwrap();
            }
        }
    } else {
        open::that("https://adoptium.net/temurin/releases/?version=8").unwrap();
    }
}
