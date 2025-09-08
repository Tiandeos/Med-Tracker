
use iced::{self as ice, Element, Length, Length::Fill};
use ice::widget::{button,text,row,column,container};

use crate::states::{message::Message, panel::Panel};

pub fn view(a : &String) -> Element<Message> {
    let sidebar = container( // Side navigation Bar
    column![
        button("panel 1").on_press(Message::OpenPanel(Panel::Time)).
        height(Length::FillPortion(1)).width(Fill),
        button("panel 2").on_press(Message::OpenPanel(Panel::Record))
        .height(Length::FillPortion(1)).width(Fill),
        button("panel 3").on_press(Message::OpenPanel(Panel::Stock))
        .height(Length::FillPortion(1)).width(Fill),
        button("panel 4").on_press(Message::OpenPanel(Panel::Settings))
        .height(Length::FillPortion(1)).width(Fill)
    ])
    .width(Length::FillPortion(1)) // Sidebar takes 1/4 of the width
    .height(Fill);

    let main_content = container(
        column![
            
        ]
    ).width(Length::FillPortion(3)) // Main content takes 3/4 of the width
    .height(Fill);
    row![
        sidebar,
        main_content
    ].width(Fill)
    .height(Fill)
    .into()
}
