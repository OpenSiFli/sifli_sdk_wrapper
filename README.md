# SDK.py Wrapper

This project is a lightweight command-line wrapper designed for Windows. It facilitates the execution of a Python script (typically `sdk.py`) located within the directory specified by the `SIFLI_SDK_PATH` environment variable. The wrapper forwards command-line parameters, sets up the necessary environment, and manages process signals.

## Project Overview

The sdk.py Wrapper primarily performs the following operations:

- **Environment Variable Reading:**  
  Reads the `SIFLI_SDK_PATH` environment variable, which should point to the root directory where the `tools/sdk.py` script is located.

- **Command Construction:**  
  Constructs the full path to the Python script by joining `SIFLI_SDK_PATH` with `tools/sdk.py` and then prepares the command line to invoke `python.exe` with this script along with any additional command-line arguments provided by the user.

- **Environment Setup for Subprocess:**  
  Sets an environment variable `SIFLI_SDK_PY_PROGRAM_NAME` to `"sdk.py"` before launching the Python subprocess. This can be used by the script for internal configuration or logging purposes.

- **Signal Handling (Ctrl+C):**  
  Installs a handler using the `ctrlc` crate to capture the Ctrl+C signal and ignore it in order to prevent the parent process from immediately terminating.

- **Version Reporting:**  
  If the executable is run with `--version` or `-v` (and its name ends with `.exe`), the program prints the version number (synchronized with the version set in `Cargo.toml` via `CARGO_PKG_VERSION`) and exits.

## Supported Command-Line Parameters

- `--version` or `-v`:  
  When provided (and if the executable's name ends with `.exe`), the wrapper prints its version (from `Cargo.toml`) and exits immediately.

- **Additional Arguments:**  
  Any other command-line arguments are passed through to the underlying `sdk.py` script.

## Building the Project

Ensure you have Rust installed (preferably via [rustup](https://rustup.rs/)). Then, execute the following commands:

1. **Build the Project in Release Mode:**
    ```bash
    cargo build --release
    ```

2. **Rename the Executable (Optional):**  
   By default, the binary will be named `sdk_py.exe` (on Windows) because crate names must follow certain naming rules. If you require the executable to be named `sdk.py.exe`, you can rename it manually:
    ```batch
    cd target\release
    ren sdk_py.exe sdk.py.exe
    ```
   Alternatively, you can automate renaming with a custom build or deployment script.

## Environment Setup

Before running the executable, make sure the `SIFLI_SDK_PATH` environment variable is set. For example:

- **For Windows CMD:**
    ```batch
    set SIFLI_SDK_PATH=C:\path\to\SiFli-SDK
    ```
- **For PowerShell:**
    ```powershell
    $env:SIFLI_SDK_PATH="C:\path\to\SiFli-SDK"
    ```

## Usage Example

After setting up the environment, you can run the wrapper from the command line:

- To execute the Python script with additional arguments:
    ```batch
    sdk.py.exe --some-argument
    ```

- To display the version information:
    ```batch
    sdk.py.exe --version
    ```

## License

This project is licensed under the Apache-2.0. Replace this text with the actual license if applicable.

## Contributing

Contributions, issues, and feature requests are welcome! Please feel free to submit a pull request or open an issue to discuss any changes.