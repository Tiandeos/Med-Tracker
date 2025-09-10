use iced::{self as ice, alignment, Background, Border, Color, Element, Length, Length::Fill};
use ice::ContentFit;
use ice::widget::{Image,button,text,row,column,container};
use crate::states::{message::Message, panel::Panel};

macro_rules! button_with_icon {
($label:expr, $icon_path:expr) => {
        container(
            row![
                Image::new($icon_path)
                    .content_fit(ContentFit::Cover)
                    .width(40)
                    .height(40),
                text($label).size(11)
            ].spacing(10)
        ).align_y(alignment::Vertical::Center)
    };
}
pub fn view(a : &String) -> Element<Message> {
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
        ]
    )
    .width(Length::Fixed(150.0)).height(Fill);

    let border = container("").
        width(3).height(Fill).style(|_| container::Style{
        background: Some(Background::Color(Color::from_rgb8(20,20,122))),
        ..Default::default()
    });
    let main_content = container(
        column![

        ]
    ).width(Fill)
    .height(Fill);
    row![
        sidebar,
        border,
        main_content
    ].width(Fill)
    .height(Fill)
    .into()
}
