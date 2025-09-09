use iced::{self as ice, alignment, Border, Element, Length, Length::Fill, Theme};
use ice::widget::{button,text,row,column,container};
use crate::states::{message::Message, panel::Panel};

pub fn view(a : &String) -> Element<Message> {
    let sidebar = container( // Side navigation Bar
    column![
        button(text("panel 1").align_y(alignment::Vertical::Center))
        .on_press(Message::OpenPanel(Panel::Time)).
        height(Length::FillPortion(1)).width(Fill),
        button(text("panel 2").align_y(alignment::Vertical::Center))
        .on_press(Message::OpenPanel(Panel::Record))
        .height(Length::FillPortion(1)).width(Fill),
        button(text("panel 3").align_y(alignment::Vertical::Center))
        .on_press(Message::OpenPanel(Panel::ManageMeds))
        .height(Length::FillPortion(1)).width(Fill),
        button(text("panel 4").align_y(alignment::Vertical::Center))
        .on_press(Message::OpenPanel(Panel::Settings))
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