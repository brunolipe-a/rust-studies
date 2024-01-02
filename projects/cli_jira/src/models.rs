use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum RecordStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl fmt::Display for RecordStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordStatus::Open => write!(f, "Open"),
            RecordStatus::InProgress => write!(f, "In progress"),
            RecordStatus::Resolved => write!(f, "Resolved"),
            RecordStatus::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: RecordStatus,
    pub stories: Vec<u32>,
}

#[allow(dead_code)]
impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: RecordStatus::Open,
            stories: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: RecordStatus,
}

#[allow(dead_code)]
impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: RecordStatus::Open,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
