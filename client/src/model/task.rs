use chrono::{DateTime, FixedOffset, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, PartialEq, Eq, Deserialize, Debug)]
pub struct Task {
    pub id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    completed_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_title(&mut self, value: String) {
        self.title = Some(value);
    }

    pub fn set_priority(&mut self, value: String) {
        self.priority = Some(value);
    }

    pub fn set_description(&mut self, value: String) {
        self.description = Some(value);
    }

    pub fn set_completed(&mut self, value: bool) {
        if value {
            self.completed_at = Some(Utc::now().fixed_offset());
        } else {
            self.completed_at = Some(DateTime::<FixedOffset>::MIN_UTC.fixed_offset());
        }
    }

    pub fn completed_at(&self) -> Option<DateTime<FixedOffset>> {
        let min = DateTime::<FixedOffset>::MIN_UTC.fixed_offset();
        self.completed_at.filter(|&value| min != value)
    }

    pub fn priority_name(&self) -> String {
        match &self.priority {
            Some(priority) => Self::priority_by_name(priority),
            None => "".to_owned(),
        }
    }

    pub fn priority_by_name(value: &str) -> String {
        let res = match value {
            "C" => "Критический",
            "H" => "Высокий",
            "N" => "Нормальный",
            "L" => "Низкий",
            _ => "",
        };

        res.to_owned()
    }

    pub fn priorities() -> Vec<(String, String)> {
        vec![
            ("C".to_owned(), Self::priority_by_name("C")),
            ("H".to_owned(), Self::priority_by_name("H")),
            ("N".to_owned(), Self::priority_by_name("N")),
            ("L".to_owned(), Self::priority_by_name("L")),
        ]
    }

    pub fn set_from_task(&mut self, task: &Task) {
        self.id = task.id;

        self.title = task.title.to_owned();
        self.description = task.to_owned().description;
        self.priority = task.to_owned().priority;
        self.completed_at = task.completed_at;
    }

    pub fn create_patch(old_task: &Task, changed_task: &Task) -> Self {
        let mut patch_for_task = Task::new();

        patch_for_task.id = old_task.id;

        if changed_task.title != old_task.title && changed_task.title.is_some() {
            patch_for_task.title = changed_task.title.to_owned();
        }
        if changed_task.description != old_task.description && changed_task.description.is_some() {
            patch_for_task.description = changed_task.to_owned().description;
        }
        if changed_task.priority != old_task.priority && changed_task.priority.is_some() {
            patch_for_task.priority = changed_task.to_owned().priority;
        }
        if changed_task.completed_at != old_task.completed_at && changed_task.completed_at.is_some()
        {
            patch_for_task.completed_at = changed_task.completed_at;
        }

        patch_for_task
    }
}
