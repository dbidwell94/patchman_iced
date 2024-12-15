use crate::{Message, ViewState, ViewUpdate};
use iced::{widget::column, Element};

pub fn view(views: &ViewState) -> Element<Message> {
    column![views.url_bar.view()].into()
}

pub fn update(views: &mut ViewState, message: Message) {
    match message {
        Message::UrlBar(msg) => views.url_bar.update(msg),
    }
}
