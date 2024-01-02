use anyhow::Result;
use cli_jira::db::Database;
use cli_jira::models::DBState;
use std::{cell::RefCell, collections::HashMap};

pub struct MockDB {
    last_written_state: RefCell<DBState>,
}

#[allow(dead_code)]
impl MockDB {
    pub fn new() -> Self {
        Self {
            last_written_state: RefCell::new(DBState {
                last_item_id: 0,
                epics: HashMap::new(),
                stories: HashMap::new(),
            }),
        }
    }
}

impl Database for MockDB {
    fn read_db(&self) -> Result<DBState> {
        let state = self.last_written_state.borrow().clone();
        Ok(state)
    }

    fn write_db(&self, db_state: &DBState) -> Result<()> {
        let latest_state = &self.last_written_state;
        *latest_state.borrow_mut() = db_state.clone();
        Ok(())
    }
}
