use std::{
    process::Command,
    env,
    path::Path,
    fs,
    io::ErrorKind,
};
use regex::Regex;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=external/junk/index.js");
    println!("cargo:rerun-if-changed=external/junk/package.json");

    check_node_command();

    let js_code = r#"
        import { junkRegex } from './external/junk/index.js';
        console.log(junkRegex.toString().replace(/^\/|\/$/g, ''));
    "#;

    let output = Command::new("node")
        .arg("--input-type=module")
        .arg("-e")
        .arg(js_code)
        .output()
        .expect("Failed to execute node.js");

    if !output.status.success() {
        eprintln!("Node.js execution failed with error: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Node.js command execution failed.");
    }

    let regex_string = String::from_utf8(output.stdout).expect("Failed to convert Node.js output to string");
    let regex_string = regex_string.trim_end();

    if let Err(err) = Regex::new(regex_string) {
        eprintln!("Regex parsing from the Node.js output failed with error: {}", err);
        panic!("Regex parsing failed.");
    }

    let out_dir = env::var("OUT_DIR").expect("Failed to get the OUT_DIR environment variable");
    let dest_path = Path::new(&out_dir).join("generated.rs");

    fs::write(&dest_path, format!("pub static JUNK_REGEX_STR: &str = \"{}\";", escape_string_for_rust_string(regex_string)))
        .expect("Could not write to the generated.rs file");
}

fn check_node_command() {
    match Command::new("node").arg("--version").output() {
        Ok(output) => {
            if !output.status.success() {
                panic!("Failed to retrieve Node.js version. Please ensure Node.js is properly installed and accessible.");
            }
        },
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                panic!("Node.js is not found on the system. Please install Node.js to proceed.");
            } else {
                panic!("Failed to run the 'node' command due to error: {}", e);
            }
        }
    }
}

fn escape_string_for_rust_string(target: impl AsRef<str>) -> String {
    target.as_ref()
        .replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
}

