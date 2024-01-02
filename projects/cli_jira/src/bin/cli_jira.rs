use anyhow::Error;
use cli_jira::{db::*, io_utils::*, navigator::*};

use std::rc::Rc;

fn main() -> Result<(), Error> {
    let db = JiraDatabase::new("data/db.json".to_owned());

    let mut navigator = Navigator::new(Rc::new(db));

    loop {
        clearscreen::clear().unwrap();

        let current_page = match navigator.get_current_page() {
            Some(v) => v,
            _ => break,
        };

        current_page.draw_page()?;

        let action = current_page.handle_input(get_user_input().as_str())?;

        if let Some(action) = action {
            navigator.handle_action(action)?;
        }
    }

    Ok(())
}
