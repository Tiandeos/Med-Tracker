use crate::application::medication::medication::Medication;
use crate::application::medication::schedule::Schedule;
use crate::application::states::state::State;
use crate::ui::macros::{self, button_with_icon};
use crate::ui::panel::time::Section::Main;
use crate::ui::style::button::{bordered_button, close_button};
use crate::ui::style::container::container_panel;
use ice::Length::Fill;
use ice::widget::{Image, button, column, container, row, scrollable, text, text_input};
use ice::{ContentFit, Element, Length, alignment};
use iced::{self as ice};

pub struct TimeUI {
    section: Section,
    medication_name: String,
    medication_time_hour: String,
    medication_time_minute: String,
}
impl TimeUI {
    pub fn new() -> TimeUI {
        Self {
            section: Main,
            medication_name: String::from(""),
            medication_time_hour: String::from(""),
            medication_time_minute: String::from(""),
        }
    }
    pub fn view<'a>(&self) -> Element<'a, Message> {
        match self.section {
            Section::Main => column![self.schedule_panel(), self.add_panel()].into(),
            Section::AddMedication => self.medication_add_panel(),
        }
    }
    pub fn update(&mut self, state: &mut State, message: Message) {
        match message {
            Message::OpenSection(Main) => self.section = Section::Main,
            Message::OpenSection(Section::AddMedication) => self.section = Section::AddMedication,
            Message::MedicationNameChange(content) => self.medication_name = content,
            Message::MedicationTimeHourChange(content) => self.medication_time_hour = content,
            Message::MedicationTimeMinuteChange(content) => self.medication_time_minute = content,
            Message::AddMedication => self.add_medication(state),
        }
    }
    fn add_panel<'a>(&self) -> Element<'a, Message> {
        container(
            button(macros::button_with_icon_text!("Add Med", "icons/plus.png"))
                .style(bordered_button)
                .on_press(Message::OpenSection(Section::AddMedication)),
        )
        .width(Fill)
        .height(Length::FillPortion(1))
        .into()
    }
    fn schedule_panel<'a>(&self) -> Element<'a, Message> {
        container(
            button(macros::button_with_icon_text!("Add Med", "icons/plus.png"))
                .style(bordered_button)
                .on_press(Message::OpenSection(Section::AddMedication)),
        )
        .width(Fill)
        .height(Length::FillPortion(6))
        .into()
    }
    fn medication_add_panel<'a>(&self) -> Element<'a, Message> {
        container(column![
            row![
                button(button_with_icon!("icons/plus.png"))
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
    fn add_medication(&self, state: &mut State) {
        let medications_list: &mut Vec<Medication> = &mut state.medications;
        let hour: u8 = self.medication_time_hour.parse().expect("Not a number");
        let minute: u8 = self.medication_time_minute.parse().expect("Not a number");
        let time: [u8; 2] = [hour, minute];
        let mut medication: Medication = Medication::new(self.medication_name.clone(), 0);
        let schedule: Schedule = Schedule::new(time, None, 0);
        medication.schedule.push(schedule);
        medications_list.push(medication);
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
    MedicationNameChange(String),
    MedicationTimeHourChange(String),
    MedicationTimeMinuteChange(String),
    AddMedication,
}
