use gloo_storage::{LocalStorage, Storage};
use web_sys::{Element, Window};

pub fn init(window: &Window, doc: &Element) {
    let is_dark = LocalStorage::get("theme").unwrap_or_else(|_| {
        let media = window
            .match_media("(prefers-color-scheme: dark)")
            .unwrap()
            .unwrap();
        media.matches()
    });
    update(doc, is_dark)
}

pub fn toggle(doc: &Element) {
    let is_dark = !get();
    update(doc, is_dark);
}

const fn as_str(is_dark: bool) -> &'static str {
    if is_dark {
        "dark"
    } else {
        "light"
    }
}

fn update(doc: &Element, is_dark: bool) {
    let theme = as_str(is_dark);
    doc.set_attribute("data-theme", theme).unwrap();
    set(is_dark);
}

fn get() -> bool {
    LocalStorage::get("theme").unwrap_or_default()
}

fn set(is_dark: bool) {
    LocalStorage::set("theme", is_dark).unwrap()
}
