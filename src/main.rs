#![windows_subsystem = "windows"]

mod app;
mod url_bar;

#[macro_use]
extern crate rust_i18n;

use iced::{window::Settings, Theme};
use std::fmt::Debug;

i18n!("locales", fallback = "en");

pub(crate) trait ViewUpdate<T>
where
    T: Debug,
{
    fn view(&self) -> iced::Element<Message>;
    fn update(&mut self, message: T);
}

#[derive(Debug, Clone)]
pub enum Message {
    UrlBar(url_bar::UrlMessage),
}

#[derive(Default)]
pub struct ViewState {
    url_bar: url_bar::UrlBar,
}

fn main() -> iced::Result {
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
