extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io;
use std::fs::File;
use std::io::{Read};
use std::path::Path;
use serde_json::Error;
// use std::ffi::OsStr;
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use std::error::Error;



// TODO: calculate create_date, calculate id, fix data types (where to handle tags vec?)
// TODO: order compare if due date before create date
// TODO: return the info in a tuple/other data structure?
fn get_item_fields() {
    let mut descr = String::new();
    let mut priority = String::new();
    let mut size = String::new();
    let mut tags = String::new();
    let mut due_date = String::new();

    println!("Description:");
    io::stdin()
        .read_line(&mut descr)
        .expect("Error reading user input");
    println!("Priority [low | medium | high]:");
    io::stdin()
        .read_line(&mut priority)
        .expect("Error reading user input");
    println!("Size [small | medium | large]:");
    io::stdin()
        .read_line(&mut size)
        .expect("Error reading user input");
    println!("Tags [<TAG1>, <TAG2>, ...]:");
    io::stdin()
        .read_line(&mut tags)
        .expect("Error reading user input");
    println!("(Optional) Due Date [MM/DD/YYYY]:");
    io::stdin()
        .read_line(&mut due_date)
        .expect("Error reading user input");

}

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: i32,
    descr: String,
    priority: String,
    size: String,
    tags: String,
    create_date: String,
    due_date: String
}



fn read_list(list: &str) -> Result<(), Error> {
    let mut file_path = String::new();
    match list {
        "todo" => {
            println!("Displaying contents of Todo List...");
            file_path = "/home/eidolon/git/todo/src/todo.json".to_owned();
        },
        "active" => {
            println!("Displaying contents of Active List...");
            file_path = "/home/eidolon/git/todo/src/active.json".to_owned();
        },
        "done" => {
            println!("Displaying contents of Done List...");
            file_path = "/home/eidolon/git/todo/src/done.json".to_owned();
        },
        _ => unreachable!(),
    }

    let mut s = String::new();
    let json_file_path = Path::new(&file_path);
    File::open(json_file_path).unwrap().read_to_string(&mut s).unwrap();
    let array: Vec<Item> = serde_json::from_str(&s)?;

    for elem in array.iter() {
        println!("{:?}", elem);
    }
    Ok(())
}

fn main() {
    use clap::{load_yaml, App};

    let yaml = load_yaml!("cli.yaml");
    let m = App::from(yaml).get_matches();

    if let Some(mode) = m.value_of("mode") {
        match mode {
            "vi" => println!("You are using vi"),
            "emacs" => println!("You are using emacs..."),
            _ => unreachable!(),
        }
    }

    if m.is_present("add") {
        println!("Detected *add* subcommand");
        // get_item_fields();
        // read_list();
        let read_list_name = "todo";
        read_list(read_list_name).unwrap();
    }

}
