#![windows_subsystem = "windows"]

use web_view::*;

fn main() {
    let base = "https://passport.yandex.com/auth?".to_string();
    let p: &str = "origin=music_button-header&retpath=https%3A%2F%2Fmusic.yandex.com%2Fhome";
    let url: String = base + p;

    web_view::builder()
        .title("Yandex Music Desktop")
        .content(Content::Url(url))
        .size(800, 940)
        .resizable(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
