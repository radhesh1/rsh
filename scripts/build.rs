#[cfg(windows)]
fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set("ProductName", "rsh");
    res.set("FileDescription", "rsh");
    res.set("LegalCopyright", "Copyright (C) 2022");
    res.set_icon("assets/rsh_logo.ico");
    res.compile()
        .expect("Failed to run the Windows resource compiler (rc.exe)");
}

#[cfg(not(windows))]
fn main() {}
