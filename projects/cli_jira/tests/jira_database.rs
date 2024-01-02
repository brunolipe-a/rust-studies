mod common;

use cli_jira::db::JiraDatabase;
use cli_jira::models::{Epic, RecordStatus, Story};
use common::MockDB;

#[test]
fn create_epic_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic.clone());

    assert_eq!(result.is_ok(), true);

    let id = result.unwrap();
    let db_state = db.read_db().unwrap();

    let expected_id = 1;

    assert_eq!(id, expected_id);
    assert_eq!(db_state.last_item_id, expected_id);
    assert_eq!(db_state.epics.get(&id), Some(&epic));
}

#[test]
fn create_story_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let story = Story::new("".to_owned(), "".to_owned());

    let non_existent_epic_id = 999;

    let result = db.create_story(story, non_existent_epic_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn create_story_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story.clone(), epic_id);
    assert_eq!(result.is_ok(), true);

    let id = result.unwrap();
    let db_state = db.read_db().unwrap();

    let expected_id = 2;

    assert_eq!(id, expected_id);
    assert_eq!(db_state.last_item_id, expected_id);
    assert_eq!(
        db_state.epics.get(&epic_id).unwrap().stories.contains(&id),
        true
    );
    assert_eq!(db_state.stories.get(&id), Some(&story));
}

#[test]
fn delete_epic_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };

    let non_existent_epic_id = 999;

    let result = db.delete_epic(non_existent_epic_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn delete_epic_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let story_id = result.unwrap();

    let result = db.delete_epic(epic_id);
    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    let expected_last_id = 2;

    assert_eq!(db_state.last_item_id, expected_last_id);
    assert_eq!(db_state.epics.get(&epic_id), None);
    assert_eq!(db_state.stories.get(&story_id), None);
}

#[test]
fn delete_story_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let story_id = result.unwrap();

    let non_existent_epic_id = 999;

    let result = db.delete_story(non_existent_epic_id, story_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn delete_story_should_error_if_story_not_found_in_epic() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let non_existent_story_id = 999;

    let result = db.delete_story(epic_id, non_existent_story_id);
    assert_eq!(result.is_err(), true);
}

#[test]
fn delete_story_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);
    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);
    assert_eq!(result.is_ok(), true);

    let story_id = result.unwrap();

    let result = db.delete_story(epic_id, story_id);
    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    let expected_last_id = 2;

    assert_eq!(db_state.last_item_id, expected_last_id);
    assert_eq!(
        db_state
            .epics
            .get(&epic_id)
            .unwrap()
            .stories
            .contains(&story_id),
        false
    );
    assert_eq!(db_state.stories.get(&story_id), None);
}

#[test]
fn update_epic_status_should_error_if_invalid_epic_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };

    let non_existent_epic_id = 999;

    let result = db.update_epic_status(non_existent_epic_id, RecordStatus::Closed);
    assert_eq!(result.is_err(), true);
}

#[test]
fn update_epic_status_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);

    assert_eq!(result.is_ok(), true);

    let epic_id = result.unwrap();

    let result = db.update_epic_status(epic_id, RecordStatus::Closed);

    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    assert_eq!(
        db_state.epics.get(&epic_id).unwrap().status,
        RecordStatus::Closed
    );
}

#[test]
fn update_story_status_should_error_if_invalid_story_id() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };

    let non_existent_story_id = 999;

    let result = db.update_story_status(non_existent_story_id, RecordStatus::Closed);
    assert_eq!(result.is_err(), true);
}

#[test]
fn update_story_status_should_work() {
    let db = JiraDatabase {
        database: Box::new(MockDB::new()),
    };
    let epic = Epic::new("".to_owned(), "".to_owned());
    let story = Story::new("".to_owned(), "".to_owned());

    let result = db.create_epic(epic);

    let epic_id = result.unwrap();

    let result = db.create_story(story, epic_id);

    let story_id = result.unwrap();

    let result = db.update_story_status(story_id, RecordStatus::Closed);

    assert_eq!(result.is_ok(), true);

    let db_state = db.read_db().unwrap();

    assert_eq!(
        db_state.stories.get(&story_id).unwrap().status,
        RecordStatus::Closed
    );
}
