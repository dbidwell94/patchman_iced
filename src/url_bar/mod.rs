use std::sync::LazyLock;

use crate::{Message, ViewUpdate};
use iced::widget::{text_input, Text};
use iced::{
    widget::{
        button, pick_list, row,
        text_input::{default, Style},
    },
    Border,
};
use reqwest::{Method, Url};

static METHOD_OPTIONS: LazyLock<Vec<Method>> = LazyLock::new(|| {
    vec![
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
    ]
});

#[derive(Debug, Clone)]
pub enum UrlMessage {
    UrlChanged(String),
    MethodChanged(Method),
    SendRequest { method: Method, url: Url },
}

impl Into<Message> for UrlMessage {
    fn into(self) -> Message {
        Message::UrlBar(self)
    }
}

#[derive(Default)]
pub struct UrlBar {
    raw_url: String,
    pub method: Method,
    pub url: Option<Url>,
}

impl ViewUpdate<UrlMessage> for UrlBar {
    fn view(&self) -> iced::Element<Message> {
        row![
            // Http method picker
            pick_list(
                METHOD_OPTIONS.clone(),
                Some(Method::from(self.method.clone())),
                |selected_method| { UrlMessage::MethodChanged(selected_method).into() }
            ),
            // Url input
            text_input("URL", &self.raw_url)
                .on_input(|new_input| UrlMessage::UrlChanged(new_input).into())
                .style(|t, s| Style {
                    border: Border {
                        color: if self.url.is_some() {
                            default(t, s).border.color
                        } else {
                            [1.0, 0.0, 0.0, 1.0].into()
                        },
                        ..default(t, s).border
                    },
                    ..default(t, s)
                }),
            // Send button
            button(Text::new(t!("send"))).on_press_maybe(
                self.url.as_ref().map(|url| {
                    UrlMessage::SendRequest {
                        method: self.method.clone().into(),
                        url: url.clone(),
                    }
                    .into()
                })
            )
        ]
        .spacing(5.)
        .padding(5.)
        .into()
    }

    fn update(&mut self, message: UrlMessage) {
        match message {
            UrlMessage::UrlChanged(new_url) => {
                self.raw_url = new_url;
                self.url = Url::parse(&self.raw_url).ok();
            }
            UrlMessage::MethodChanged(new_method) => {
                self.method = new_method;
            }
            UrlMessage::SendRequest { method, url } => {
                todo!()
            }
        }
    }
}
