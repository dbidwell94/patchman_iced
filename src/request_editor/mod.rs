use iced::widget::Text;

use crate::{Message, ViewUpdate};

#[derive(Debug, Clone)]
pub enum RequestEditorMessage {}

#[derive(Debug, Default)]
pub struct RequestEditor {}

impl ViewUpdate<RequestEditorMessage> for RequestEditor {
    fn view(&self) -> iced::Element<Message> {
        iced::Element::new(Text::new("Request Editor"))
    }

    fn update(&mut self, message: RequestEditorMessage) -> iced::Task<Message> {
        iced::Task::none()
    }
}
