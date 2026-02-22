use crate::application::medication::record::Record;
use crate::application::states::medicationtracker::MedicationTracker;
use crate::ui::style::container::container_panel;
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
        if records.is_empty() {
            return container(text("No alarms")).style(container_panel).into();
        }
        if records.len() == 1 {
            self.single_record_view(tracker, records[0])
        } else {
            self.multiple_records_view(tracker, &records)
        }
    }

    fn single_record_view<'a>(
        &self,
        tracker: &'a MedicationTracker,
        record: &'a Record,
    ) -> Element<'a, Message> {
        let med_name = tracker
            .medications
            .iter()
            .find(|m| m.id == record.medication_id)
            .map(|m| m.name.as_str())
            .unwrap_or("Unknown");
        container(
            column![
                text(med_name).size(48),
                row![
                    button(text("Take")).on_press(Message::MarkTaken(record.id.clone())),
                    button(text("Skip")).on_press(Message::MarkSkipped(record.id.clone())),
                    button(text("Reschedule"))
                        .on_press(Message::MarkRescheduled(record.id.clone())),
                ]
                .spacing(20)
            ]
            .spacing(40)
            .align_x(ice::alignment::Horizontal::Center),
        )
        .center(Length::Fill)
        .style(container_panel)
        .into()
    }

    fn multiple_records_view<'a>(
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
        container(scrollable(records_list).height(Length::Fill))
            .padding(20)
            .center_x(Length::Fill)
            .style(container_panel)
            .into()
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
