use crate::db::NotReallyADatabase;
use anyhow::Result;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use youchoose::Menu;

pub fn start_menu(db: &mut NotReallyADatabase, remove: bool) {
    // clone db and create menu. If remove allow multiselect.
    // Clone used as menu takes ownership.
    let menu_values = db.db.clone();
    let mut menu = match remove {
        true => Menu::new(menu_values.iter())
            .multiselect()
            .add_multiselect_key('s' as i32),
        false => Menu::new(menu_values.iter()),
    };

    // For each selected index, pop value (sorted then reversed to preserve order).
    let mut selected_indices = menu.show();
    selected_indices.sort();
    for i in selected_indices.iter().rev() {
        let selected_value = db.db.remove(*i);
        // If remove then continue. Else copy to clipboard and push front LRU type simulation.
        match remove {
            true => continue,
            false => match selected_value {
                Some(value) => {
                    _ = format_and_copy_to_clipboard(value.clone());
                    db.db.push_front(value)
                }
                None => println!("Unable to Yoink your selected choice."),
            },
        }
    }
}

fn format_and_copy_to_clipboard(text: String) -> Result<()> {
    let mut ctx = ClipboardContext::new()?;
    let format_regex = regex::Regex::new(r".* - ")?;
    let formatted_value = format_regex.replace(&text, "").as_ref().to_owned();
    ctx.set_contents(formatted_value.clone())?;
    println!("Succesfully Yoinked {formatted_value} to clipboard.");
    Ok(())
}