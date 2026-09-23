use std::env;
use std::path::PathBuf;
use std::process::Command;

fn find_resource_compiler() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    if let Some(sdk_dir) = env::var_os("WindowsSdkDir") {
        candidates.push(PathBuf::from(sdk_dir).join("bin"));
    }
    if let Some(program_files) = env::var_os("ProgramFiles(x86)") {
        candidates.push(PathBuf::from(program_files).join(r"Windows Kits\10\bin"));
    }
    for bin_dir in candidates {
        let versions = std::fs::read_dir(bin_dir).ok()?;
        for version in versions.flatten() {
            let compiler = version.path().join("x64").join("rc.exe");
            if compiler.is_file() {
                return Some(compiler);
            }
        }
    }
    None
}

fn main() {
    println!("cargo:rerun-if-changed=health-reminder.ico");
    println!("cargo:rerun-if-changed=health-reminder-paused.ico");
    println!("cargo:rerun-if-changed=health-reminder-water.ico");
    println!("cargo:rerun-if-changed=health-reminder-chair.ico");
    println!("cargo:rerun-if-changed=health-reminder.rc");

    if env::var_os("CARGO_CFG_WINDOWS").is_none() {
        return;
    }

    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let resource = out_dir.join("health-reminder.res");
    let compiler = find_resource_compiler().unwrap_or_else(|| PathBuf::from("rc.exe"));
    let status = Command::new(compiler)
        .current_dir(&manifest_dir)
        .args(["/nologo", "/fo"])
        .arg(&resource)
        .arg("health-reminder.rc")
        .status();

    if status.map(|result| result.success()).unwrap_or(false) {
        println!("cargo:rustc-link-arg={}", resource.display());
    } else {
        println!("cargo:warning=rc.exe was not found; the executable will use the tray icon fallback");
    }
}
