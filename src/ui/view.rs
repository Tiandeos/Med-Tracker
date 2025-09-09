
use iced::{self as ice, Border, Element, Length, Length::Fill, Theme};
use ice::widget::{button,text,row,column,container};
use iced::widget::container::Style;
use crate::states::{message::Message, panel::Panel};

pub fn view(a : &String) -> Element<Message> {
    let sidebar = container( // Side navigation Bar
    column![
        button("panel 1").on_press(Message::OpenPanel(Panel::Time)).
        height(Length::FillPortion(1)).width(Fill),
        button("panel 2").on_press(Message::OpenPanel(Panel::Record))
        .height(Length::FillPortion(1)).width(Fill),
        button("panel 3").on_press(Message::OpenPanel(Panel::ManageMeds))
        .height(Length::FillPortion(1)).width(Fill),
        button("panel 4").on_press(Message::OpenPanel(Panel::Settings))
        .height(Length::FillPortion(1)).width(Fill)
    ])
    .width(Length::Fixed(150.0)).height(Fill);


    let main_content = container(
        column![
            
        ]
    ).width(Fill) 
    .height(Fill);
    row![
        sidebar,
        main_content
    ].width(Fill)
    .height(Fill)
    .into()
}