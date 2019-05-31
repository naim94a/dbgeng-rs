fn main() {
    println!("cargo:rustc-link-search=\"C:\\Program Files (x86)\\Windows Kits\\10\\Debuggers\\lib\\x64\"");
    println!("cargo:rustc-link-lib=dbgeng");
    println!("cargo:rustc-link-lib=dbghelp");
}