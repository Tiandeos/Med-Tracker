use crate::application::medication::medication::Medication;
use crate::application::medication::occurrencestatus::OccurrenceStatus;
use crate::application::medication::periodtype::PeriodType;
use crate::application::medication::record::Record;
use crate::application::medication::schedule::Schedule;
use crate::application::states::medicationtracker::MedicationTracker;
use crate::ui::macros::{self, button_with_icon};
use crate::ui::panel::time::Section::Main;
use crate::ui::style;
use crate::ui::style::button::close_button;
use crate::ui::style::container::container_panel;
use crate::ui::style::time::container::{record_status_container, schedule_container};
use crate::update::generate_records::generate_records_for_medication;
use chrono::{Datelike, Duration, Local, NaiveDate, Timelike, Utc};
use ice::Length::Fill;
use ice::widget::{Image, button, column, container, row, scrollable, text, text_input};
use ice::{ContentFit, Element, alignment};
use iced::Length::{FillPortion, Shrink};
use iced::{self as ice, Padding};
use std::collections::BTreeMap;

pub struct TimeUI {
    section: Section,
    pub selected_date: NaiveDate,
    medication_name: String,
    medication_time_hour: String,
    medication_time_minute: String,
}
impl TimeUI {
    pub fn new() -> TimeUI {
        Self {
            section: Main,
            selected_date: Local::now().date_naive(),
            medication_name: String::from(""),
            medication_time_hour: String::from(""),
            medication_time_minute: String::from(""),
        }
    }
    pub fn view<'a>(&self, tracker: &'a MedicationTracker) -> Element<'a, Message> {
        match self.section {
            Section::Main => column![
                self.calendar_part(),
                container(self.main_part(tracker)).height(Fill),
                self.add_panel()
            ]
            .into(),
            Section::AddMedication => self.medication_add_panel(),
        }
    }
    pub fn update(&mut self, state: &mut MedicationTracker, message: Message) {
        match message {
            Message::OpenSection(Main) => self.section = Section::Main,
            Message::OpenSection(Section::AddMedication) => self.section = Section::AddMedication,
            Message::SelectDay(date) => {
                self.selected_date = date;
                println!("Current TimeUI Selected Date: {}", self.selected_date)
            }
            Message::MedicationNameChange(content) => self.medication_name = content,
            Message::MedicationTimeHourChange(content) => self.medication_time_hour = content,
            Message::MedicationTimeMinuteChange(content) => self.medication_time_minute = content,
            Message::AddMedication => self.add_medication(state),
            Message::MarkTaken(_id) => {}
            Message::MarkSkipped(_id) => {}
            Message::MarkPostponed(_id) => {}
            Message::ToggleSound(_hour, _minute) => {}
        }
    }
    fn main_part<'a>(&self, tracker: &'a MedicationTracker) -> Element<'a, Message> {
        let mut grouped: BTreeMap<(u32, u32), Vec<&Record>> = BTreeMap::new();
        for record in &tracker.records {
            if record.time.with_timezone(&Local).date_naive() != self.selected_date {
                continue;
            }
            grouped
                .entry((
                    record.time.with_timezone(&Local).hour(),
                    record.time.with_timezone(&Local).minute(),
                ))
                .or_default()
                .push(record);
        }
        let mut medications_container_list = column![].spacing(20);
        for ((hour, minute), records) in &grouped {
            let mut schedule_container_column = column![].padding([20, 40]).spacing(20);
            let hour_minute = format!("{:02}:{:02}", hour, minute);
            let schedule_label = text(hour_minute).size(32).width(Fill);
            let sound_button = button(button_with_icon!("icons/icons8-sound-50.png", 32, 10))
                .style(style::time::button::record_action_button)
                .padding(10)
                .on_press(Message::ToggleSound(*hour, *minute));
            let schedule_header =
                row![schedule_label, sound_button].align_y(alignment::Vertical::Center);
            schedule_container_column = schedule_container_column.push(schedule_header);
            let mut medications_list = column![].spacing(10).padding([0, 20]);
            for record in records {
                if let Some(med) = tracker
                    .medications
                    .iter()
                    .find(|med| med.id == record.medication_id)
                {
                    let status_icon: Element<'a, Message> = match &record.occurrence_status {
                        OccurrenceStatus::Taken { .. } => {
                            Image::new("icons/icons8-complete-50.png")
                                .content_fit(ContentFit::Cover)
                                .width(42)
                                .height(42)
                                .into()
                        }
                        OccurrenceStatus::Skipped { .. } | OccurrenceStatus::Missed => {
                            Image::new("icons/icons8-cross-50.png")
                                .content_fit(ContentFit::Cover)
                                .width(42)
                                .height(42)
                                .into()
                        }
                        OccurrenceStatus::Pending => column![].width(42).height(42).into(),
                    };
                    let status_container = container(status_icon).style(record_status_container);
                    let medication_info = column![
                        text(&med.name).size(22),
                        text(&med.stock).size(16), // TODO: PLACEHOLDER CHANGE IT
                    ]
                    .spacing(5)
                    .width(Fill);
                    let action_buttons = row![
                        button(button_with_icon!("icons/icons8-complete-50.png", 32, 10))
                            .style(style::time::button::record_action_button)
                            .padding(10)
                            .on_press(Message::MarkTaken(record.id.clone())),
                        button(button_with_icon!("icons/icons8-cross-50.png", 32, 10))
                            .style(style::time::button::record_action_button)
                            .padding(10)
                            .on_press(Message::MarkSkipped(record.id.clone())),
                        button(button_with_icon!("icons/icons8-clock-50.png", 32, 10))
                            .style(style::time::button::record_action_button)
                            .padding(10)
                            .on_press(Message::MarkPostponed(record.id.clone())),
                    ]
                    .spacing(30)
                    .align_y(alignment::Vertical::Center);
                    let medication_row = row![status_container, medication_info, action_buttons]
                        .align_y(alignment::Vertical::Center)
                        .spacing(20);

                    medications_list = medications_list.push(medication_row);
                }
            }
            schedule_container_column = schedule_container_column
                .push(medications_list)
                .width(FillPortion(1));
            let schedule_container = container(schedule_container_column).style(schedule_container);
            medications_container_list = medications_container_list.push(schedule_container);
        }
        scrollable(container(medications_container_list.max_width(750)).center_x(Fill)).into()
    }
    fn add_panel<'a>(&self) -> Element<'a, Message> {
        container(
            button(macros::button_with_icon_text!("Add Med", "icons/plus.png"))
                .style(style::time::button::add_button)
                .padding([20, 100])
                .on_press(Message::OpenSection(Section::AddMedication)),
        )
        .center_x(Fill)
        .height(Shrink)
        .padding(Padding::new(0.0).bottom(20))
        .into()
    }
    fn calendar_part<'a>(&self) -> Element<'a, Message> {
        let today = Local::now().date_naive();
        let mut days = row![].spacing(35);
        for i in 0..8 {
            let date = today + Duration::days(i);
            let weekday = match date.weekday() {
                chrono::Weekday::Mon => "Monday",
                chrono::Weekday::Tue => "Tuesday",
                chrono::Weekday::Wed => "Wednesday",
                chrono::Weekday::Thu => "Thursday",
                chrono::Weekday::Fri => "Friday",
                chrono::Weekday::Sat => "Saturday",
                chrono::Weekday::Sun => "Sunday",
            };
            let day_month = format!("{}/{}", date.day(), date.month());
            let label = column![text(weekday).center(), text(day_month).center(),].spacing(50);
            let is_selected = date == self.selected_date;
            days = days.push(
                button(label)
                    .style(style::time::button::calendar_button(is_selected))
                    .padding([20, 30])
                    .width(FillPortion(1))
                    .on_press(Message::SelectDay(date)),
            );
        }
        container(container(days).max_width(1358).width(Fill))
            .center_x(Fill)
            .height(Shrink)
            .padding(Padding::new(0.0).bottom(20))
            .into()
    }
    fn medication_add_panel<'a>(&self) -> Element<'a, Message> {
        container(column![
            row![
                button(button_with_icon!("icons/plus.png", 30, 10))
                    .on_press(Message::OpenSection(Section::Main))
                    .style(close_button)
            ]
            .height(100),
            row![
                text("Medication Name: ").align_y(alignment::Vertical::Center),
                text_input("...", &self.medication_name).on_input(Message::MedicationNameChange),
            ]
            .spacing(20)
            .height(Fill),
            row![
                text("Hour: ").align_y(alignment::Vertical::Center),
                text_input("...", &self.medication_time_hour)
                    .on_input(Message::MedicationTimeHourChange),
                text("Minute: ").align_y(alignment::Vertical::Center),
                text_input("...", &self.medication_time_minute)
                    .on_input(Message::MedicationTimeMinuteChange),
            ]
            .spacing(20)
            .height(Fill),
            button("Add Medication")
                .on_press(Message::AddMedication)
                .height(100),
        ])
        .style(container_panel)
        .padding(20)
        .height(Fill)
        .width(Fill)
        .into()
    }
    fn add_medication(&self, state: &mut MedicationTracker) {
        let hour: u8 = self.medication_time_hour.parse().expect("Not a number");
        let minute: u8 = self.medication_time_minute.parse().expect("Not a number");
        let time: [u8; 2] = [hour, minute];
        let mut medication: Medication = Medication::new(self.medication_name.clone(), 0.0);
        let schedule: Schedule = Schedule::new(time, Some(PeriodType::Daily), 3, 1.0);
        medication.schedules.push(schedule);
        let medication_id = medication.id.clone();
        state.medications.push(medication);
        generate_records_for_medication(state, &medication_id);
    }
    pub fn set_section_to_main(&mut self) {
        self.section = Section::Main;
    }
}
#[derive(Debug, Clone)]
pub enum Section {
    Main,
    AddMedication,
}
#[derive(Debug, Clone)]
pub enum Message {
    OpenSection(Section),
    SelectDay(NaiveDate),
    MedicationNameChange(String),
    MedicationTimeHourChange(String),
    MedicationTimeMinuteChange(String),
    AddMedication,
    MarkTaken(String),
    MarkSkipped(String),
    MarkPostponed(String),
    ToggleSound(u32, u32),
}
