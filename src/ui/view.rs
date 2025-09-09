use iced::{self as ice, alignment, Alignment, Border, Element, Length, Length::Fill, Renderer, Theme};
use ice::ContentFit;
use ice::widget::{Image,button,text,row,column,container,};
use crate::states::{message::Message, panel::Panel};

macro_rules! button_with_icon {
($label:expr, $icon_path:expr) => {
        container(
            row![
                Image::new($icon_path)
                    .content_fit(ContentFit::Cover)
                    .width(40)
                    .height(40),
                text($label)
            ].spacing(10)
        ).align_y(alignment::Vertical::Center)
    };
}
pub fn view(a : &String) -> Element<Message> {
    /*let button1 = container( row![
        Image::new("icons/home.png")
        .content_fit(ContentFit::Cover)
        .width(40)
        .height(40),
        text("Today")
    ].spacing(10));*/
    let sidebar = container( // Side navigation Bar
    column![
        button(button_with_icon!("Home","icons/home.png"))
        .on_press(Message::OpenPanel(Panel::Time)).
        height(Length::FillPortion(1)).width(Fill),
        button(button_with_icon!("Calendar","icons/calendar.png"))
        .on_press(Message::OpenPanel(Panel::Record))
        .height(Length::FillPortion(1)).width(Fill),
        button(button_with_icon!("Medications","icons/pill.png"))
        .on_press(Message::OpenPanel(Panel::ManageMeds))
        .height(Length::FillPortion(1)).width(Fill),
        button(button_with_icon!("Settings","icons/settings.png"))
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
