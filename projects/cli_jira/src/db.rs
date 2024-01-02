use anyhow::{anyhow, Ok, Result};
use std::fs;

use crate::models::{DBState, Epic, RecordStatus, Story};

pub struct JiraDatabase {
    pub database: Box<dyn Database>,
}

impl JiraDatabase {
    pub fn new(file_path: String) -> Self {
        JiraDatabase {
            database: Box::new(JSONFileDatabase { file_path }),
        }
    }

    pub fn read_db(&self) -> Result<DBState> {
        self.database.read_db()
    }

    pub fn create_epic(&self, epic: Epic) -> Result<u32> {
        let mut db_state = self.database.read_db()?;

        let new_id = db_state.last_item_id + 1;

        db_state.epics.insert(new_id, epic);
        db_state.last_item_id = new_id;

        self.database.write_db(&db_state)?;
        Ok(new_id)
    }

    pub fn create_story(&self, story: Story, epic_id: u32) -> Result<u32> {
        let mut db_state = self.database.read_db()?;

        let new_id = db_state.last_item_id + 1;

        db_state.stories.insert(new_id, story);
        db_state.last_item_id = new_id;
        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Epic of id {} not found", epic_id))?
            .stories
            .push(new_id);

        self.database.write_db(&db_state)?;
        Ok(new_id)
    }

    pub fn delete_epic(&self, epic_id: u32) -> Result<()> {
        let mut db_state = self.database.read_db()?;

        let epic_stories = &db_state
            .epics
            .get(&epic_id)
            .ok_or_else(|| anyhow!("Epic of id {} not found", epic_id))?
            .stories;

        for story_id in epic_stories {
            db_state.stories.remove(story_id);
        }

        db_state.epics.remove(&epic_id);

        self.database.write_db(&db_state)?;
        Ok(())
    }

    pub fn delete_story(&self, epic_id: u32, story_id: u32) -> Result<()> {
        let mut db_state = self.database.read_db()?;

        db_state
            .stories
            .remove(&story_id)
            .ok_or_else(|| anyhow!("Story of id {} not found", story_id))?;

        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Epic of id {} not found", epic_id))?
            .stories
            .retain(|&x| x != story_id);

        self.database.write_db(&db_state)?;
        Ok(())
    }

    pub fn update_epic_status(&self, epic_id: u32, status: RecordStatus) -> Result<()> {
        let mut db_state = self.database.read_db()?;

        db_state
            .epics
            .get_mut(&epic_id)
            .ok_or_else(|| anyhow!("Epic of id {} not found", epic_id))?
            .status = status;

        self.database.write_db(&db_state)?;
        Ok(())
    }

    pub fn update_story_status(&self, story_id: u32, status: RecordStatus) -> Result<()> {
        let mut db_state = self.database.read_db()?;

        db_state
            .stories
            .get_mut(&story_id)
            .ok_or_else(|| anyhow!("Story of id {} not found", story_id))?
            .status = status;

        self.database.write_db(&db_state)?;
        Ok(())
    }
}

pub trait Database {
    fn read_db(&self) -> Result<DBState>;
    fn write_db(&self, db_state: &DBState) -> Result<()>;
}

pub struct JSONFileDatabase {
    pub file_path: String,
}

impl Database for JSONFileDatabase {
    fn read_db(&self) -> Result<DBState> {
        let db_content = fs::read_to_string(&self.file_path)?;
        let db_state: DBState = serde_json::from_str(&db_content)?;
        Ok(db_state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        fs::write(&self.file_path, serde_json::to_string(db_state)?)?;

        Ok(())
    }
}
