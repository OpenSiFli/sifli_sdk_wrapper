use ctrlc;
use std::env;
use std::io;
use std::path::Path;
use std::process::{exit, Command, Stdio};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn run() -> Result<i32, Box<dyn std::error::Error>> {
    ctrlc::set_handler(|| {
        // we do nothing here
    })
    .unwrap_or_else(|err| {
        eprintln!("Error setting Ctrl-C handler: {}", err);
    });

    let args: Vec<String> = env::args().collect();
    let exe_path = env::current_exe()?;
    let exe_filename = exe_path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    // print version
    if args.len() == 2
        && exe_filename.to_lowercase().ends_with(".exe")
        && (args[1] == "--version" || args[1] == "-v")
    {
        println!("{}", VERSION);
        return Ok(0);
    }

    // Get the SIFLI_SDK_PATH environment variable
    let idf_path = env::var("SIFLI_SDK_PATH").map_err(|_| "SIFLI_SDK_PATH environment variable needs to be set to use this tool")?;

    let script_path = Path::new(&idf_path).join("tools").join("sdk.py");
    let script_str = script_path
        .to_str()
        .unwrap();

    let mut command = Command::new("python.exe");
    command.arg(script_str);
    for arg in args.iter().skip(1) {
        command.arg(arg);
    }

    command.env("SIFLI_SDK_PY_PROGRAM_NAME", "sdk.py");
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                return Err("Can not find Python\n".into())
            } else {
                return Err(format!("Unkown error: {}\n", e).into());
            }
        }
    };

    let exit_status = child.wait()?;
    let code = exit_status.code().unwrap_or(1);

    Ok(code)
}

fn main() {
    match run() {
        Ok(code) => exit(code),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
