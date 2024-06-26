use uniffi_bindgen::generate_bindings;
use std::process::Command;

fn main() {
    let udl_file = "./src/qsharp-bridge.udl";
    let out_dir = "./bindings/";
    uniffi_build::generate_scaffolding(udl_file).unwrap();
    generate_bindings(udl_file.into(), 
        None, 
        vec!["kotlin", "swift", "python"], 
        Some(out_dir.into()), 
        None,
        false).unwrap(); 

    Command::new("uniffi-bindgen-cs").arg("--out-dir").arg(out_dir).arg(udl_file).output().expect("Failed when generating C# bindings");
}