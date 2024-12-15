use crate::{Message, ViewState, ViewUpdate};
use iced::{widget::column, Element};

pub fn view(views: &ViewState) -> Element<Message> {
    column![views.url_bar.view(), views.request_editor.view()].into()
}

pub fn update(views: &mut ViewState, message: Message) -> iced::Task<Message> {
    match message {
        Message::UrlBar(msg) => views.url_bar.update(msg),
        Message::GlobalMessage(msg) => {
            println!("Global message: {:?}", msg);
            // Handle global messages here
            iced::Task::none()
        }
    }
}
