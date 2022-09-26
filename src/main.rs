fn main() {
    if std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .contains(".zip")
    {
        eprintln!("Running inside a .zip is unsupported!");
    }

    check_java();
}

#[cfg(target_os = "linux")]
fn check_java() {
    let cmd = std::process::Command::new("java")
        .arg("-version")
        .output()
        .unwrap();

    if String::from_utf8_lossy(&cmd.stderr).contains("version")
        && String::from_utf8_lossy(&cmd.stderr).contains("Runtime Environment")
    {
        // you have java!
        std::process::Command::new("sudo")
            .args(["java", "-jar", "NetworkAddonMod_Setup_Version44.jar"])
            .spawn()
            .unwrap();
    }
    else {
        open::that("https://www.java.com/en/download/").unwrap();
    }
}

#[cfg(target_os = "macos")]
fn check_java() {
    let cmd = std::process::Command::new("java")
        .arg("-version")
        .output()
        .unwrap();

    if String::from_utf8_lossy(&cmd.stderr).contains("version")
        && String::from_utf8_lossy(&cmd.stderr).contains("Runtime Environment")
    {
        // you have java!
        std::process::Command::new("sudo")
            .args(["java", "-jar", "NetworkAddonMod_Setup_Version44.jar"])
            .spawn()
            .unwrap();
    }
    else {
        open::that("https://www.java.com/en/download/").unwrap();
    }
}

#[cfg(target_os = "windows")]
fn check_java() {
    
    println!("Checking if Java is installed...");
    let cmd = std::process::Command::new("java")
        .arg("-version")
        .output()
        .unwrap();

    if String::from_utf8_lossy(&cmd.stderr).contains("version")
        && String::from_utf8_lossy(&cmd.stderr).contains("Runtime Environment")
    {
        println!("Looking for SimCity 4.exe...");
        // patch sc4 exe
        let then = std::time::Instant::now();
        let files = ('A'..='Z')
            .into_iter()
            .flat_map(|c| {
                println!("Checking hard disk {c}:// for SimCity 4 Deluxe folder...");
                walkdir::WalkDir::new(format!("{c}://"))
                    .max_depth(10)
                    .into_iter()
                    .filter_entry(|i| i.path().is_dir()) // || i.path().extension().and_then(|e| e.to_str()) == Some("exe"))
                    .filter_map(|i| i.ok())
                    .map(|i| i.path().to_path_buf())
                    .filter(|f| f.file_name().and_then(|f| f.to_str()) == Some("SimCity 4 Deluxe"))
                    .map(|f| f.canonicalize().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let new_inst = std::time::Instant::now();
        dbg!(new_inst.duration_since(then));

        
        for file in files {
            println!("Patching {}...", file.to_str().unwrap());
            let mut sc4_path = file;
            sc4_path.push("Apps");
            sc4_path.push("SimCity 4.exe");
            
            std::process::Command::new(".\\4gb_patch.exe")
                .arg(sc4_path)
                .output()
                .unwrap();
        }

        // you have java!
        // std::process::Command::new("java")
        //     .args(["-jar", ".\\NetworkAddonMod_Setup_Version44.jar"])
        //     .spawn()
        //     .unwrap();
    }
    else {
        open::that("https://www.java.com/en/download/").unwrap();
    }
}
