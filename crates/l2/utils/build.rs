use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .status()?;

    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let binding = PathBuf::from(manifest_dir);
    let root_dir = binding
        .parent() // go up to l2
        .and_then(|p| p.parent()) // go up to crates
        .and_then(|p| p.parent()) // go up to root
        .ok_or("Could not find root directory")?;

    match check_dependencies() {
        Ok(_) => println!("All required dependencies are installed."),
        Err(e) => {
            println!("Missing Dependencies Error:");
            println!("{}", e);
            println!("\nPlease install the missing dependencies and try again.");
            std::process::exit(1);
        }
    }

    Command::new("make")
        .current_dir(root_dir)
        .arg("l2-artifacts")
        .status()?;
    Ok(())
}

#[derive(Debug)]
pub struct DependencyError {
    pub missing_deps: Vec<String>,
    pub install_instructions: String,
}

impl std::fmt::Display for DependencyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Missing dependencies: {}\n\nInstallation instructions:\n{}",
            self.missing_deps.join(", "),
            self.install_instructions
        )
    }
}

impl std::error::Error for DependencyError {}

pub fn check_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let mut missing_deps = Vec::new();
    let mut install_instructions = String::new();

    // Check ganache
    if !check_command("ganache", &["--version"]) {
        missing_deps.push("ganache".to_string());
        install_instructions.push_str(
            "Install ganache via npm:\n\
            npm install -g ganache\n\n",
        );
    }

    // Check solc
    if !check_command("solc", &["--version"]) {
        missing_deps.push("solc".to_string());
        install_instructions.push_str(
            "Install solc:\n\
            - On Ubuntu/Debian: sudo apt-get install solc\n\
            - On macOS: brew install solidity\n\
            - Using pip: pip install solc-select && solc-select install 0.8.19 && solc-select use 0.8.19\n\n"
        );
    }

    // Check python3
    if !check_command("python3", &["--version"]) {
        missing_deps.push("python3".to_string());
        install_instructions.push_str(
            "Install Python 3:\n\
            - On Ubuntu/Debian: sudo apt-get install python3\n\
            - On macOS: brew install python3\n\
            - Windows: Download from https://www.python.org/downloads/\n\n",
        );
    }

    // Check forge
    if !check_command("forge", &["--version"]) {
        missing_deps.push("forge".to_string());
        install_instructions.push_str(
            "Install Foundry (includes forge):\n\
            curl -L https://foundry.paradigm.xyz | bash\n\
            Then run: foundryup\n\n",
        );
    }

    println!("Checking dependencies...");
    println!("{}", "-".repeat(50));

    check_and_print_version("Ganache", "ganache", &["--version"]);
    check_and_print_version("Solc", "solc", &["--version"]);
    check_and_print_version("Python3", "python3", &["--version"]);
    check_and_print_version("Forge", "forge", &["--version"]);

    println!("{}", "-".repeat(50));

    if !missing_deps.is_empty() {
        return Err(Box::new(DependencyError {
            missing_deps,
            install_instructions,
        }));
    }

    println!("All dependencies are installed! ✨");
    Ok(())
}

pub fn check_command(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn check_and_print_version(name: &str, cmd: &str, args: &[&str]) {
    let status = Command::new(cmd)
        .args(args)
        .output()
        .map(|output| {
            let version = String::from_utf8_lossy(&output.stdout);
            (output.status.success(), version.trim().to_string())
        })
        .unwrap_or((false, String::new()));

    let status_str = if status.0 {
        format!("{} ({})", "✓", status.1)
    } else {
        "✗".to_string()
    };

    println!("{:<12} {}", name, status_str);
}
