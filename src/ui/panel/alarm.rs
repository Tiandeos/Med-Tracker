use crate::application::medication::record::Record;
use crate::application::states::medicationtracker::MedicationTracker;
use crate::ui::style::alarm::button::{alarm_action_button, alarm_take_button};
use crate::ui::style::alarm::container::alarm_panel_container;
use ice::widget::{button, column, container, row, scrollable, text};
use ice::{Element, Length};
use iced as ice;

pub struct AlarmUI {
    pub alarming_records: Vec<String>,
}
impl AlarmUI {
    pub fn new() -> Self {
        Self {
            alarming_records: Vec::new(),
        }
    }

    pub fn view<'a>(&self, tracker: &'a MedicationTracker) -> Element<'a, Message> {
        let records: Vec<&Record> = self
            .alarming_records
            .iter()
            .filter_map(|id| tracker.records.iter().find(|r| &r.id == id))
            .collect();

        let inner = if records.is_empty() {
            column![text("No alarms")].into()
        } else if records.len() == 1 {
            self.single_record_content(tracker, records[0])
        } else {
            self.multiple_records_content(tracker, &records)
        };
        container(
            container(inner)
                .max_width(1000)
                .max_height(640)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(alarm_panel_container)
                .padding(30),
        )
        .center(Length::Fill)
        .into()
    }

    fn single_record_content<'a>(
        &self,
        tracker: &'a MedicationTracker,
        record: &'a Record,
    ) -> Element<'a, Message> {
        let medication = tracker
            .medications
            .iter()
            .find(|m| m.id == record.medication_id);
        let med_name = medication.map(|m| m.name.as_str()).unwrap_or("Unknown");
        let schedule =
            medication.and_then(|med| med.schedules.iter().find(|s| s.id == record.schedule_id));
        let dose = schedule.map(|s| s.dose).unwrap_or(0.0);
        let time = record.time.format("%H:%M").to_string();
        let schedule_time_text = format!("{} - Medication", time);
        column![
            container(
                text(schedule_time_text)
                    .size(24)
                    .style(|theme: &ice::Theme| {
                        let palette = theme.extended_palette();
                        ice::widget::text::Style {
                            color: Some(palette.background.strong.text),
                        }
                    })
            )
            .padding(ice::Padding {
                top: 35.0,
                right: 0.0,
                bottom: 25.0,
                left: 0.0
            })
            .center_x(Length::Fill),
            column![
                text(med_name).size(32),
                text(format!("{} mg", dose)).size(16),
            ]
            .spacing(20)
            .align_x(ice::alignment::Horizontal::Center),
            container("").height(Length::Fill),
            column![
                button(
                    container(text("Take Medication"))
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                )
                .style(alarm_take_button)
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .on_press(Message::MarkTaken(record.id.clone())),
                button(
                    container(text("Skipped"))
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                )
                .style(alarm_action_button)
                .width(Length::Fill)
                .height(Length::FillPortion(1))
                .on_press(Message::MarkSkipped(record.id.clone())),
                container(
                    button(
                        container(text("Reschedule"))
                            .center_x(Length::Fill)
                            .center_y(Length::Fill)
                    )
                    .style(alarm_action_button)
                    .width(Length::Shrink)
                    .height(Length::FillPortion(1))
                    .on_press(Message::MarkRescheduled(record.id.clone()))
                )
                .center_x(Length::Fill),
            ]
            .spacing(25)
            .width(Length::Fill),
        ]
        .align_x(ice::alignment::Horizontal::Center)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn multiple_records_content<'a>(
        &self,
        tracker: &'a MedicationTracker,
        records: &[&'a Record],
    ) -> Element<'a, Message> {
        let mut records_list = column![].spacing(10);
        for record in records {
            let med_name = tracker
                .medications
                .iter()
                .find(|m| m.id == record.medication_id)
                .map(|m| m.name.as_str())
                .unwrap_or("Unknown");

            let record_row = row![
                text(med_name).width(Length::Fill),
                button(text("Take")).on_press(Message::MarkTaken(record.id.clone())),
                button(text("Skip")).on_press(Message::MarkSkipped(record.id.clone())),
                button(text("Reschedule")).on_press(Message::MarkRescheduled(record.id.clone())),
            ]
            .spacing(10)
            .align_y(ice::alignment::Vertical::Center);

            records_list = records_list.push(record_row);
        }

        scrollable(records_list).height(Length::Fill).into()
    }

    pub fn update(&mut self, tracker: &mut MedicationTracker, message: Message) {
        match message {
            // TODO: implement functions
            Message::MarkTaken(record_id) => {
                self.remove_record(&record_id);
            }
            Message::MarkSkipped(record_id) => {
                self.remove_record(&record_id);
            }
            Message::MarkRescheduled(record_id) => {
                self.remove_record(&record_id);
            }
        }
    }

    fn remove_record(&mut self, record_id: &str) {
        self.alarming_records.retain(|id| id != record_id);
    }

    pub fn is_active(&self) -> bool {
        !self.alarming_records.is_empty()
    }

    pub fn add_alarming_record(&mut self, record_id: String) {
        if !self.alarming_records.contains(&record_id) {
            self.alarming_records.push(record_id);
        }
    }

    pub fn set_section_to_main(&mut self) {}
}

#[derive(Debug, Clone)]
pub enum Message {
    MarkTaken(String),
    MarkSkipped(String),
    MarkRescheduled(String),
}
