fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    if std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .contains(".zip")
    {
        panic!("Running inside a .zip is unsupported!");
    }

    match check_java() {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {e} on {:#?}\nPress enter to exit.", e.backtrace());
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }
    }
}

fn get_jar_name() -> anyhow::Result<String> {
    let files = std::fs::read_dir(std::env::current_dir()?)?;
    files
        .into_iter()
        .filter_map(|f| f.ok())
        .find(|f| {
            f.file_name()
                .to_string_lossy()
                .contains("NetworkAddonMod_Setup_Version")
        })
        .map(|f| f.file_name().to_string_lossy().to_string())
        .ok_or_else(|| anyhow::anyhow!("No installer file found!"))
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn check_java() -> anyhow::Result<()> {
    let cmd = std::process::Command::new("java").arg("-version").output();

    if cmd.map(|s| s.status.success()).ok() == Some(true) {
        // you have java!
        std::process::Command::new("java")
            .args(["-jar", &get_jar_name()?])
            .spawn()?;
    } else {
        open::that("https://adoptium.net/temurin/releases/?version=8")?;
    }

    Ok(())
}

// #[cfg(target_os = "macos")]
// fn check_java() -> anyhow::Result<()> {
//     let cmd = std::process::Command::new("java").arg("-version").output();

//     if cmd.map(|s| s.status.success()).ok() == Some(true) {
//         // you have java!
//         std::process::Command::new("sudo")
//             .args(["java", "-jar", get_jar_name()?])
//             .spawn()
//             ?;
//     } else {
//         open::that("https://adoptium.net/temurin/releases/?version=8")?;
//     }
// }

#[cfg(target_os = "windows")]
fn check_java() -> anyhow::Result<()> {
    println!("Checking if Java is installed...");
    let cmd = std::process::Command::new("java").arg("-version").output();

    if cmd.map(|s| s.status.success()).ok() == Some(true) {
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
                let mut pathbuf: std::path::PathBuf = folder.parse()?;
                pathbuf.push("Apps");
                pathbuf.push("SimCity 4.exe");
                files.push(pathbuf);
            }
        }

        println!("\x1b[1;34mSimCity 4 Deluxe folder not found! Would you like a systemwide search (S) or provide a filepath to your SimCity 4.exe? (P)");
        println!("Any other character will exit.\x1b[0m");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_uppercase() == "P" {
            println!("{}[AInput your path:", 27u8 as char);
            let mut value = String::new();
            std::io::stdin().read_line(&mut value)?;

            files.push(value.trim().parse()?)
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
                .output()?;

            if std::fs::read(sc4_path.replace(".exe", ".exe.Backup")).is_ok() {
                println!("\x1b[1;32mPatched {sc4_path}!\x1b[0m");

                // you have java and your exe is patched!
                std::process::Command::new("cmd")
                    .args(["/c", "start", "/MIN", "java", "-jar", &get_jar_name()?])
                    .spawn()?;
            }
        }
    } else {
        open::that("https://adoptium.net/temurin/releases/?version=8")?;
    }

    Ok(())
}
