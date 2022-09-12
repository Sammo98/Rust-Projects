mod args;
mod db;
mod menu;

use args::Args;
use clap::Parser;
use db::NotReallyADatabase;
use menu::start_menu;

fn main() {
    // Initiate db and get args through CLAP. Panic if db initiation fails.
    let mut db = NotReallyADatabase::init().expect("Failed to initiate db.");
    let args = Args::parse();

    match args.action.is_some() {
        // If action supplied then handle the requested action,
        true => args.handle_action(&mut db),

        // or get values and start menu with multiselect disabled with check for empty db
        false => {
            if db.db.is_empty() {
                println!("No items in db. Please run 'yoink --help' for more information or add an item with the command 'yoink add <key> <value>'.")
            } else {
                start_menu(&mut db, false);
            }
        }
    }
    _ = db.save();
}