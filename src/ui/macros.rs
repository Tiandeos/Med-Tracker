macro_rules! button_with_icon {
($label:expr, $icon_path:expr) => {
        container(
            row![
                Image::new($icon_path)
                    .content_fit(ContentFit::Cover)
                    .width(30)
                    .height(30),
                text($label).size(14).align_y(alignment::Vertical::Bottom)
            ].spacing(10).align_y(alignment::Vertical::Center)
        ).align_y(alignment::Vertical::Center)
    };
}
pub(crate) use button_with_icon;
