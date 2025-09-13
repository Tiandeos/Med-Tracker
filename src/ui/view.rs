use iced::{self as ice, alignment, Background, Border, Color, Element, Length, Length::Fill};
use ice::ContentFit;
use ice::widget::{Image,button,text,row,column,container};
use iced::widget::Container;
use crate::states::{message::Message, panel::Panel};
use crate::states::state::State;
use crate::ui::style::button::navbar_button;

macro_rules! button_with_icon {
($label:expr, $icon_path:expr) => {
        container(
            row![
                Image::new($icon_path)
                    .content_fit(ContentFit::Cover)
                    .width(40)
                    .height(40),
                text($label).size(12)
            ].spacing(10)
        ).align_y(alignment::Vertical::Center)
    };
}
pub fn view(state : &State) -> Element<Message> {
    let sidebar = container( // Side navigation Bar
    column![
        button(button_with_icon!("Home","icons/home.png"))
        .style(navbar_button).padding(0)
        .on_press(Message::OpenPanel(Panel::Time)).
        height(Length::FillPortion(1)).width(Fill),
        button(button_with_icon!("Calendar","icons/calendar.png"))
        .style(navbar_button).padding(0)
        .on_press(Message::OpenPanel(Panel::Record))
        .height(Length::FillPortion(1)).width(Fill),
        button(button_with_icon!("Medications","icons/pill.png"))
        .style(navbar_button).padding(0)
        .on_press(Message::OpenPanel(Panel::ManageMeds))
        .height(Length::FillPortion(1)).width(Fill),
        button(button_with_icon!("Settings","icons/settings.png"))
        .style(navbar_button).padding(0)
        .on_press(Message::OpenPanel(Panel::Settings))
        .height(Length::FillPortion(1)).width(Fill)
        ].spacing(4)
    ).style(|_| container::Style
    {
        background: Some(Background::Color(Color::from_rgb8(0,85,175))),
        ..Default::default()
    })
    .width(Length::Fixed(150.0)).height(Fill);

    let border = container("").
        width(4).height(Fill).style(|_| container::Style{
        background: Some(Background::Color(Color::from_rgb8(0,85,175))),
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
