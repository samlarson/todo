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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

fn get_list_path(list_name: &str) -> String {
    let mut file_path = String::new();
    match list_name {
        "todo" => {
            file_path = "/home/eidolon/git/todo/src/todo.json".to_owned();
        },
        "active" => {
            file_path = "/home/eidolon/git/todo/src/active.json".to_owned();
        },
        "done" => {
            file_path = "/home/eidolon/git/todo/src/done.json".to_owned();
        },
        _ => unreachable!(),
    }
    file_path
}

fn read_list(list_name: &str) -> Result<Vec<Item>, Error> {
    let mut file_path = String::new();
    match list_name {
        "todo" => {
            // println!("Reading contents of Todo List...");
            file_path = "/home/eidolon/git/todo/src/todo.json".to_owned();
        },
        "active" => {
            // println!("Reading contents of Active List...");
            file_path = "/home/eidolon/git/todo/src/active.json".to_owned();
        },
        "done" => {
            // println!("Reading contents of Done List...");
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
    // println!("Displaying contents of list...");
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

fn validate_entry_id(list: &Vec<Item>, item_id: i32) -> bool {
    for elem in list.iter() {
        if elem.id == item_id {
            // let entry: Item = elem.cloned();
            // let mut entry = Item::from(elem.clone());
            // entry
            return true
        }
    }
    false
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

fn move_entry(src_name: &str, dst_name: &str, mut src_list: Vec<Item>, mut dst_list: Vec<Item>, item_id: i32) -> Result<(), Error> {
    // let item: Item = validate_entry_id(&src_list, item_id);
    // match validate_entry_id(&src_list, item_id) {
    //     true => println!(""),
    //     false => panic!("Specified Item ID was not found in list."),
    // }
    if validate_entry_id(&src_list, item_id) {
        let index_opt: Option<usize> = src_list.iter().position(|x|x.id == item_id);
        let index: usize = index_opt.unwrap();


        let item: Item = src_list[index].to_owned();
        // let item: Item = src_list.iter().position(|&x| x.id == item_id);

        // Pop
        // src_list.remove(src_list.iter().position(|x| *x.id == item_id).expect("Entry not found"));
        src_list.retain(|x|x.id != item_id);
        let src_serial = serde_json::to_string_pretty(&src_list).unwrap();
        let src_path = get_list_path(src_name);
        write(src_path, src_serial).expect("Unable to write source file");

        // Push
        dst_list.push(item);
        let dst_serial = serde_json::to_string_pretty(&dst_list).unwrap();
        let dst_path = get_list_path(dst_name);
        write(dst_path, dst_serial).expect("Unable to write destination file");

        Ok(())
    } else {
        panic!("Specified Item ID was not found in list.");
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
        // println!("Detected *add* subcommand");
        let todo_list: Vec<Item> = read_list("todo").unwrap();
        let field_array = get_item_fields();
        write_entry(todo_list, field_array);
    }

    match m.subcommand() {
        Some(("list", list_matches)) => {
            // println!("Detected *list* subcommand");
            // println!("{}", list_matches.value_of("listname").unwrap());
            let read_list_name = list_matches.value_of("listname").unwrap();
            let list: Vec<Item> = read_list(read_list_name).unwrap();
            display_list(list);
        }
        Some(("assign", assign_matches)) => {
            println!("Detected *assign* subcommand");
            match assign_matches.subcommand() {
                Some(("todo", assign_todo_matches)) => {
                    println!("{}", assign_todo_matches.value_of("itemid").unwrap());
                }
                Some(("done", assign_done_matches)) => {
                    println!("{}", assign_done_matches.value_of("itemid").unwrap());
                }
                Some(("active", assign_active_matches)) => {
                    // println!("{}", assign_active_matches.value_of("itemid").unwrap());
                    match assign_active_matches.subcommand() {
                        Some(("item", assign_active_item_matches)) => {
                            // println!("{}", assign_active_item_matches.value_of("itemid").unwrap());
                            let str_id = assign_active_item_matches.value_of("itemid").unwrap();
                            let item_id = str_id.parse::<i32>().unwrap();
                            let todo_list: Vec<Item> = read_list("todo").unwrap();
                            let active_list: Vec<Item> = read_list("active").unwrap();
                            move_entry("todo", "active", todo_list, active_list, item_id);
                        }
                        Some(("rand", _)) => {
                            println!("Assign random entry to active");
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => {},
    }

}

// TODO: clean up write_entry()
// TODO: handle categorical variables in struct?
// TODO: move on to the next subcommand, start to thing about handling next level down

// TODO: check if a given list exists
// TODO: move add subcommand to match structure
// TODO: finish list functionality (todo features, active and done lists)
// TODO: remove commented out code

// TODO: add second parameter to move_entry, you need src and dst lists to keep fxn abstract
// TODO: figure out most efficient way to pop struct off vector
// TODO: change subcommand reference to get_list_path, pass read_list by getting that path
// TODO: dynamic filepaths for actual executable folder