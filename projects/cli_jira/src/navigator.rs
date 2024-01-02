use anyhow::{anyhow, Context, Ok, Result};
use std::rc::Rc;

use crate::{
    db::JiraDatabase,
    models::Action,
    ui::{EpicDetail, HomePage, Page, Prompts, StoryDetail},
};

pub struct Navigator {
    pages: Vec<Box<dyn Page>>,
    prompts: Prompts,
    db: Rc<JiraDatabase>,
}

impl Navigator {
    pub fn new(db: Rc<JiraDatabase>) -> Self {
        let home_page = Box::new(HomePage { db: db.clone() });

        Navigator {
            pages: vec![home_page],
            prompts: Prompts::new(),
            db,
        }
    }

    pub fn get_current_page(&self) -> Option<&Box<dyn Page>> {
        self.pages.last()
    }

    pub fn handle_action(&mut self, action: Action) -> Result<()> {
        match action {
            Action::NavigateToEpicDetail { epic_id } => {
                let epic_detail_page = Box::new(EpicDetail {
                    db: self.db.clone(),
                    epic_id,
                });

                self.pages.push(epic_detail_page);

                Ok(())
            }
            Action::NavigateToStoryDetail { epic_id, story_id } => {
                let story_detail_page = Box::new(StoryDetail {
                    db: self.db.clone(),
                    story_id,
                    epic_id,
                });

                self.pages.push(story_detail_page);

                Ok(())
            }
            Action::NavigateToPreviousPage => {
                self.pages.pop();

                Ok(())
            }
            Action::CreateEpic => {
                let epic = (self.prompts.create_epic)();

                self.db
                    .create_epic(epic)
                    .with_context(|| anyhow!("failed to create epic"))?;

                Ok(())
            }
            Action::UpdateEpicStatus { epic_id } => {
                let status = (self.prompts.update_status)()
                    .ok_or_else(|| anyhow!("Error to get valid status"))?;

                self.db
                    .update_epic_status(epic_id, status)
                    .with_context(|| anyhow!("failed to update epic ({epic_id}) status"))?;

                Ok(())
            }
            Action::DeleteEpic { epic_id } => {
                let should_delete = (self.prompts.delete_epic)();

                if should_delete {
                    self.db
                        .delete_epic(epic_id)
                        .with_context(|| anyhow!("failed to delete epic ({epic_id})"))?;

                    self.pages.pop();
                }

                Ok(())
            }
            Action::CreateStory { epic_id } => {
                let story = (self.prompts.create_story)();

                self.db
                    .create_story(story, epic_id)
                    .with_context(|| anyhow!("failed to create Story"))?;

                Ok(())
            }
            Action::UpdateStoryStatus { story_id } => {
                let status = (self.prompts.update_status)()
                    .ok_or_else(|| anyhow!("Error to get valid status"))?;

                self.db
                    .update_story_status(story_id, status)
                    .with_context(|| anyhow!("failed to update Story ({story_id}) status"))?;

                Ok(())
            }
            Action::DeleteStory { epic_id, story_id } => {
                let should_delete = (self.prompts.delete_story)();

                if should_delete {
                    self.db
                        .delete_story(epic_id, story_id)
                        .with_context(|| anyhow!("failed to delete Story ({story_id})"))?;

                    self.pages.pop();
                }

                Ok(())
            }
            Action::Exit => {
                self.pages.clear();

                Ok(())
            }
        }
    }

    // Private functions used for testing

    pub fn _get_page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn _set_prompts(&mut self, prompts: Prompts) {
        self.prompts = prompts;
    }
}
