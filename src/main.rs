extern crate chrono;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell};

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
        .expect("Error reading user input - description");
    if descr == "\n" {
        panic!("You must enter a description for new entries");
    } else {
        descr.pop();
    }

    println!("Priority [low | medium | high] (l/m/h):");
    io::stdin()
        .read_line(&mut priority)
        .expect("Error reading user input - priority");
    if priority == "l\n" {
        priority = "low".parse().unwrap();
    } else if priority == "m\n" {
        priority = "medium".parse().unwrap();
    } else if priority == "h\n" {
        priority = "high".parse().unwrap();
    } else {
        priority.pop();
    }

    println!("Size [small | medium | large] (s/m/l):");
    io::stdin()
        .read_line(&mut size)
        .expect("Error reading user input - size");
    if size == "s\n" {
        size = "small".parse().unwrap();
    } else if size == "m\n" {
        size = "medium".parse().unwrap();
    } else if size == "l\n" {
        size = "large".parse().unwrap();
    }
    else {
        size.pop();
    }

    println!("(Optional) Tags [<TAG1>, <TAG2>, ...]:");
    io::stdin()
        .read_line(&mut tags)
        .expect("Error reading user input - tags");
    tags.pop();

    // TODO: assert input is a valid date
    // https://docs.rs/time/0.2.22/time/struct.Date.html
    println!("(Optional) Due Date [MM-DD-YYYY]:");
    io::stdin()
        .read_line(&mut due_date)
        .expect("Error reading user input - due date");
    due_date.pop();

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
    // for elem in list.iter() {
    //     println!("{:?}", elem);
    // }

    let mut table = Table::new();
    table.add_row(row!["ID", "Description", "Priority", "Size", "Tags", "Create Date", "Due Date"]);

    for elem in list.iter() {
        // println!("{:?}", elem);
        table.add_row(row![elem.id, elem.descr, elem.priority, elem.size,
        check_empty_elem(&elem.tags), elem.create_date, check_empty_elem(&elem.due_date)]);
    }
    table.printstd();
}

fn check_empty_elem(elem: &str) -> String {
    if elem.is_empty() == true {
        let fmt_val = String::from("None");
        fmt_val
    }
    else {
        elem.to_string()
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
    let date = local.format("%m-%d-%Y").to_string();
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

    // println!("{}", field_array[4]);

    // let naive_due = NaiveDate::parse_from_str(&entry_struct.due_date, "%m-%d-%Y").unwrap();
    let naive_due = NaiveDate::parse_from_str(&field_array[4], "%m-%d-%Y").unwrap();
    let x: Date<Local> = Local.from_local_date(&naive_due).unwrap();
    let naive_curr: Date<Local> = Local::today();

    if naive_curr > x {
        panic!("The date specified in the Due Date field has already passed.");
    }
    else {
        // serde_json::to_writer(&file, &entry);
        // let entry_array: Item = serde_json::from_str(&entry)?;
        // let x: String = serde_json::to_string_pretty(&entry).unwrap();

        todo_list.push(entry_struct);
        let serialized = serde_json::to_string_pretty(&todo_list).unwrap();

        write("/home/eidolon/git/todo/src/todo.json", serialized).expect("Unable to write file");
        Ok(())
    }
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

// TODO: clean up write_entry()
// TODO: handle categorical variables in struct?
// TODO: move on to the next subcommand, start to thing about handling next level down