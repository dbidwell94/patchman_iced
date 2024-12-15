use std::sync::LazyLock;

use crate::{Message, ViewUpdate};
use iced::widget::{text_input, Text};
use iced::Task;
use iced::{
    widget::{
        button, pick_list,
        pick_list::{default as pl_default, Style as PLStyle},
        row,
        text_input::{default as ti_default, Style},
    },
    Border,
};
use reqwest::header::HeaderMap;
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
    SendRequest {
        method: Method,
        url: Url,
        data: Option<String>,
        headers: HeaderMap,
    },
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
            row![
                // Http method picker
                pick_list(
                    METHOD_OPTIONS.clone(),
                    Some(Method::from(self.method.clone())),
                    |selected_method| { UrlMessage::MethodChanged(selected_method).into() }
                )
                .style(|t, s| PLStyle {
                    border: Border {
                        radius: pl_default(t, s)
                            .border
                            .radius
                            .top_right(0.)
                            .bottom_right(0.),
                        ..pl_default(t, s).border
                    },
                    ..pl_default(t, s)
                }),
                // Url input
                text_input("URL", &self.raw_url)
                    .on_input(|new_input| UrlMessage::UrlChanged(new_input).into())
                    .style(|t, s| Style {
                        border: Border {
                            color: if self.url.is_some() {
                                ti_default(t, s).border.color
                            } else {
                                [1.0, 0.0, 0.0, 1.0].into()
                            },
                            radius: ti_default(t, s).border.radius.top_left(0.).bottom_left(0.),
                            ..ti_default(t, s).border
                        },
                        ..ti_default(t, s)
                    }),
            ],
            // Send button
            button(Text::new(t!("send"))).on_press_maybe(self.url.as_ref().map(|url| {
                UrlMessage::SendRequest {
                    method: self.method.clone(),
                    url: url.clone(),
                    data: None,
                    headers: HeaderMap::new(),
                }
                .into()
            }))
        ]
        .spacing(5.)
        .padding(5.)
        .into()
    }

    fn update(&mut self, message: UrlMessage) -> iced::Task<Message> {
        match message {
            UrlMessage::UrlChanged(new_url) => {
                self.raw_url = new_url;
                self.url = Url::parse(&self.raw_url).ok();
                Task::none()
            }
            UrlMessage::MethodChanged(new_method) => {
                self.method = new_method;
                Task::none()
            }
            UrlMessage::SendRequest {
                method,
                url,
                data,
                headers,
            } => Task::perform::<Result<crate::global_messages::Response, reqwest::Error>>(
                async move {
                    let now = std::time::Instant::now();
                    let response = reqwest::Client::default()
                        .request(method, url)
                        .body(data.map(|d| d.into_bytes()).unwrap_or_default())
                        .headers(headers)
                        .send()
                        .await?;
                    let elapsed = now.elapsed();
                    Ok(crate::global_messages::Response {
                        response: response.into(),
                        elapsed,
                    })
                },
                |res| match res {
                    Ok(response) => Message::GlobalMessage(
                        crate::global_messages::GlobalMessage::ResponseReceived(response),
                    ),
                    Err(err) => Message::GlobalMessage(
                        crate::global_messages::GlobalMessage::ResponseError(err.into()),
                    ),
                },
            ),
        }
    }
}
