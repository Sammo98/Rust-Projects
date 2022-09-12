use crate::{db::NotReallyADatabase, menu::start_menu};
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Action {
    Add,
    Remove,
    Oink,
}

/// CLI Tool to add remove, and copy items from a menu.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the action to perform.
    /// If action is removed, select which values you would like to remove with the 's' key and enter to confirm.
    #[clap(arg_enum, value_parser)]
    pub action: Option<Action>,
    /// Key (Only required when adding a new value)
    #[clap(value_parser)]
    pub key: Option<String>,
    /// Value (Only required when adding a new value)
    #[clap(value_parser)]
    pub value: Option<String>,
}

impl Args {
    pub fn handle_action(self, db: &mut NotReallyADatabase) {
        match self.action.unwrap() {
            Action::Add => db.add(self.key, self.value),
            Action::Remove => start_menu(db, true),
            Action::Oink => {
                println!("🐷")
            }
        }
    }
}