#![cfg_attr(not(feature = "console"), windows_subsystem = "windows")]

mod app;
mod global_messages;
mod url_bar;

#[macro_use]
extern crate rust_i18n;

use iced::{window::Settings, Theme};
use rust_i18n::set_locale;
use std::fmt::Debug;

i18n!("locales", fallback = "en-US");

pub(crate) trait ViewUpdate<T>
where
    T: Debug,
{
    fn view(&self) -> iced::Element<Message>;
    fn update(&mut self, message: T) -> iced::Task<Message>;
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlBar(url_bar::UrlMessage),
    GlobalMessage(global_messages::GlobalMessage),
}

#[derive(Default)]
pub struct ViewState {
    url_bar: url_bar::UrlBar,
}

fn main() -> iced::Result {
    let locale = sys_locale::get_locale().unwrap_or("en-US".to_string());
    set_locale(&locale);

    iced::application("Patchman", app::update, app::view)
        .executor::<iced::executor::Default>()
        .theme(|_| match dark_light::detect() {
            dark_light::Mode::Dark => Theme::Dark,
            dark_light::Mode::Light => Theme::Light,
            dark_light::Mode::Default => Theme::Dark,
        })
        .centered()
        .antialiasing(true)
        .window(Settings {
            min_size: Some((800., 600.).into()),
            icon: Some(iced::window::icon::from_file("assets/icon.png").unwrap()),
            ..Settings::default()
        })
        .run()
}
