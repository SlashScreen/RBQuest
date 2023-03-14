//For launching the window.
use iced::alignment;
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space, Image,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};

pub struct RBQuest;

impl Sandbox for RBQuest {
    type Message = Message;

    fn title(&self) -> String {
        //TODO: Config file
        "RBQuest".to_owned()
    }

    fn new() -> Self {
        RBQuest
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        container(row![text("Hello world"), Image::new("res/ruby.png")])
            .height(Length::Fill)
            .center_y()
            .into()
    }

    fn update(&mut self, message: Self::Message) {}
}

#[derive(Debug, Clone)]
pub enum Message {}

pub fn launch() -> iced::Result {
    RBQuest::run(Settings::default())
}
