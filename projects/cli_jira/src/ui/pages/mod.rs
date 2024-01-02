use std::any::Any;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::db::JiraDatabase;
use crate::models::Action;

pub mod page_helpers;
use page_helpers::*;

pub trait Page {
    fn draw_page(&self) -> Result<()>;
    fn handle_input(&self, input: &str) -> Result<Option<Action>>;
    fn as_any(&self) -> &dyn Any;
}

pub struct HomePage {
    pub db: Rc<JiraDatabase>,
}
impl Page for HomePage {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epics = db_state.epics;

        println!("----------------------------- EPICS -----------------------------");
        println!("     id     |               name               |      status      ");

        for epic_id in epics.keys().sorted() {
            let epic = &epics[epic_id];

            let id_col = get_column_string(&epic_id.to_string(), 11);
            let name_col = get_column_string(&epic.name, 32);
            let status_col = get_column_string(&epic.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[q] quit | [c] create epic | [:id:] navigate to epic");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let action = match input {
            "q" => Some(Action::Exit),
            "c" => Some(Action::CreateEpic),
            _ => None,
        };

        if action.is_some() {
            return Ok(action);
        }

        let epic_id = match input.parse::<u32>() {
            Ok(v) => v,
            _ => return Ok(None),
        };

        let db_state = self.db.read_db()?;

        if !db_state.epics.contains_key(&epic_id) {
            return Ok(None);
        }

        Ok(Some(Action::NavigateToEpicDetail { epic_id }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EpicDetail {
    pub epic_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for EpicDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let epic = db_state
            .epics
            .get(&self.epic_id)
            .ok_or_else(|| anyhow!("could not find epic!"))?;

        println!("------------------------------ EPIC ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        let id_col = get_column_string(&self.epic_id.to_string(), 5);
        let name_col = get_column_string(&epic.name, 12);
        let desc_col = get_column_string(&epic.description, 27);
        let status_col = get_column_string(&epic.status.to_string(), 13);
        println!("{} | {} | {} | {}", id_col, name_col, desc_col, status_col);

        println!();

        println!("---------------------------- STORIES ----------------------------");
        println!("     id     |               name               |      status      ");

        let stories = &db_state.stories;

        for story_id in epic.stories.iter().sorted() {
            let story = &stories[story_id];

            let id_col = get_column_string(&story_id.to_string(), 11);
            let name_col = get_column_string(&story.name, 32);
            let status_col = get_column_string(&story.status.to_string(), 17);
            println!("{} | {} | {}", id_col, name_col, status_col);
        }

        println!();
        println!();

        println!("[p] previous | [u] update epic | [d] delete epic | [c] create story | [:id:] navigate to story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epic_id = self.epic_id;

        let action = match input {
            "p" => Some(Action::NavigateToPreviousPage),
            "u" => Some(Action::UpdateEpicStatus { epic_id }),
            "d" => Some(Action::DeleteEpic { epic_id }),
            "c" => Some(Action::CreateStory { epic_id }),
            _ => None,
        };

        if action.is_some() {
            return Ok(action);
        }

        let story_id = match input.parse::<u32>() {
            Ok(v) => v,
            _ => return Ok(None),
        };

        let db_state = self.db.read_db()?;

        if !db_state.stories.contains_key(&story_id) {
            return Ok(None);
        }

        Ok(Some(Action::NavigateToStoryDetail { epic_id, story_id }))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct StoryDetail {
    pub epic_id: u32,
    pub story_id: u32,
    pub db: Rc<JiraDatabase>,
}

impl Page for StoryDetail {
    fn draw_page(&self) -> Result<()> {
        let db_state = self.db.read_db()?;
        let story = db_state
            .stories
            .get(&self.story_id)
            .ok_or_else(|| anyhow!("could not find story!"))?;

        println!("------------------------------ STORY ------------------------------");
        println!("  id  |     name     |         description         |    status    ");

        print!("{}|", get_column_string(&self.story_id.to_string(), 6));
        print!("{}|", get_column_string(&story.name, 14));
        print!("{}|", get_column_string(&story.description, 29));
        print!("{}", get_column_string(&story.status.to_string(), 14));

        println!();
        println!();

        println!("[p] previous | [u] update story | [d] delete story");

        Ok(())
    }

    fn handle_input(&self, input: &str) -> Result<Option<Action>> {
        let epic_id = self.epic_id;
        let story_id = self.story_id;

        match input {
            "p" => Ok(Some(Action::NavigateToPreviousPage)),
            "u" => Ok(Some(Action::UpdateStoryStatus { story_id })),
            "d" => Ok(Some(Action::DeleteStory { epic_id, story_id })),
            _ => Ok(None),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
