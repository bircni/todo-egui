use std::env;

#[expect(clippy::expect_used, reason = "This is a build script")]
fn main() {
    let target = env::var("CARGO_CFG_TARGET_OS").expect("Failed to get target OS");
    if target == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("./res/icon.ico");
        res.compile().expect("Failed to compile Windows resources");
    }
}
