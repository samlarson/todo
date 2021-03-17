extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io;
use std::fs::File;
use std::fs::write;
use std::io::Read;
use std::path::Path;
use serde_json::{Error, json};
use std::cmp::Ordering;
use clap::ArgMatches;
use chrono::prelude::*;
// use std::ffi::OsStr;
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
// use std::error::Error;

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

// TODO: calculate create_date, calculate id, fix data types (where to handle tags vec?)
// TODO: order compare if due date before create date
// TODO: return the info in a tuple/other data structure?
fn get_item_fields() -> [String; 5] {
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

    // descr.trim();

    let array = [descr, priority, size, tags, due_date];
    array
}

fn read_list(list_name: &str) -> Result<Vec<Item>, Error> {
    let mut file_path = String::new();
    match list_name {
        "todo" => {
            println!("Reading contents of Todo List...");
            file_path = "/home/eidolon/git/todo/src/todo.json".to_owned();
        },
        "active" => {
            println!("Reading contents of Active List...");
            file_path = "/home/eidolon/git/todo/src/active.json".to_owned();
        },
        "done" => {
            println!("Reading contents of Done List...");
            file_path = "/home/eidolon/git/todo/src/done.json".to_owned();
        },
        _ => unreachable!(),
    }

    let mut s = String::new();
    let json_file_path = Path::new(&file_path);
    File::open(json_file_path).unwrap().read_to_string(&mut s).unwrap();
    let array: Vec<Item> = serde_json::from_str(&s)?;

    Ok(array)
}

fn display_list(list: Vec<Item>) {
    println!("Displaying contents of list...");
    for elem in list.iter() {
        println!("{:?}", elem);
    }
}

fn max_list_id(list: &Vec<Item>) -> i32 {
    let mut max_id: i32 = 0;
    for elem in list.iter() {
        match max_id.cmp(&elem.id) {
            Ordering::Less => max_id = elem.id,
            Ordering::Equal => {}
            Ordering::Greater => {}
        }
    }
    max_id
}

fn get_curr_date() -> String {
    let local: Date<Local> = Local::today();
    let date = local.format("%m-%d-Y").to_string();
    date
}

fn write_entry(mut todo_list: Vec<Item>, field_array: [String; 5]) -> Result<(), Error>{
    let max_id: i32 = max_list_id(&todo_list);
    let curr_id: i32 = max_id + 1;
    let curr_date: String = get_curr_date();
    let mut file = File::open("/home/eidolon/git/todo/src/todo.json").unwrap();

    let entry = json!({
        "id": curr_id,
        "descr": field_array[0],
        "priority": field_array[1],
        "size": field_array[2],
        "tags": field_array[3],
        "create_date": curr_date,
        "due_date": field_array[4]
    });

    let entry_struct = Item {
        id: curr_id,
        descr: field_array[0].clone(),
        priority: field_array[1].clone(),
        size: field_array[2].clone(),
        tags: field_array[3].clone(),
        create_date: curr_date,
        due_date: field_array[4].clone()
    };

    // serde_json::to_writer(&file, &entry);
    // let entry_array: Item = serde_json::from_str(&entry)?;

    let x: String = serde_json::to_string_pretty(&entry).unwrap();
    todo_list.push(entry_struct);
    let serialized = serde_json::to_string_pretty(&todo_list).unwrap();

    write("/home/eidolon/git/todo/src/todo.json", serialized).expect("Unable to write file");

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
        let todo_list: Vec<Item> = read_list("todo").unwrap();
        let field_array = get_item_fields();
        write_entry(todo_list, field_array);
    }

    match m.subcommand() {
        Some(("list", list_matches)) => {
            println!("Detected *list* subcommand");
            println!("{}", list_matches.value_of("listname").unwrap());
            let read_list_name = list_matches.value_of("listname").unwrap();
            let list: Vec<Item> = read_list(read_list_name).unwrap();
            display_list(list);
        }
        _ => {},
    }


}

// TODO: create_date year is 'Y', truncate new entry newlines, catch empty due_date entry
// TODO: clean up write_entry()
// TODO: handle categorical variables in struct, catch for them via clap
// TODO: move on to the next subcommand, start to thing about handling next level down