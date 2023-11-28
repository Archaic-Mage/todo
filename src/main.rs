use clap::{Parser, Subcommand};
use serde_json::{json,Result, Value};
use std::env;
use console::style;

const DEFAULT: &str = "/home/mage/Documents/todo.json";
const JSON_FILE: &str = "todo.json";

#[derive(Parser)]
#[clap(name = "todo", version = "1.0", author = "Your Name")]
struct Opt {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Create,
    Add {
        val: Vec<String>,
        #[arg(short='g', long="global")]
        global: bool,
    },
    Done {
        val: Vec<i32>,
        #[arg(short='g', long="global")]
        global: bool,
    },
    List {
        #[arg(short='g', long="global")]
        global: bool,
        #[arg(short='a', long="all")]
        all: bool,
    },
    Clear {
        #[arg(short='a', long="all")]
        verbose: bool,
        #[arg(short='g', long="global")]
        global: bool,
    }
}

fn which_file(global: bool) -> String {
    if global {
        DEFAULT.to_string()
    } else if std::path::Path::new(JSON_FILE).exists() {
        JSON_FILE.to_string()
    } else {
        DEFAULT.to_string()
    }
}

fn create_json(file: &str) -> String {
    let json = json!({
        "files": [],
        "default": []
    });
    let json = serde_json::to_string(&json).unwrap();
    std::fs::write(file, &json).unwrap();
    //add file to files in DEFAULT JSON file
    let mut djson = open_json(DEFAULT).unwrap();
    let file_loc = env::current_dir().unwrap().to_str().unwrap().to_string() + "/" + file;
    djson["files"].as_array_mut().unwrap().push(json!(file_loc));
    dump_json(&djson, DEFAULT);
    json
}

fn open_json(file: &str) -> Result<Value> {
    let json_file = std::fs::read_to_string(file);
    let json = match json_file {
        Ok(json) => json,
        Err(_) =>  create_json(file) 
    };
    let json: Value = serde_json::from_str(&json)?;
    Ok(json)
}

fn dump_json(json: &Value, file: &str) {
    let json = serde_json::to_string(json).unwrap();
    std::fs::write(file, json).unwrap();
}

fn add(val: Vec<String>, global: bool) {
    let file = &which_file(global);
    let mut json = open_json(file).unwrap();
    for v in val {
        let item = json![{"name": v, "done": false}];
        json["default"].as_array_mut().unwrap().push(item);
    }
    println!("{}", file);
    dump_json(&json, file);
}

fn done(val: Vec<i32>, global: bool) {
    let file = &which_file(global);
    let mut json = open_json(file).unwrap();
    let default = json["default"].as_array_mut().unwrap(); 
    for v in val {
        default[v as usize]["done"] = json!(true);
    }
    dump_json(&json, file);
}

fn list_todo(file: &str) {
    let json = open_json(file).unwrap();
    let default = json["default"].as_array().unwrap();
    for (i, v) in default.iter().enumerate() {
        let done = v["done"].as_bool().unwrap();
        let name = v["name"].as_str().unwrap();
        println!("{}: {}", i, if done {style(name).strikethrough()} else {style(name)})
    }
}

fn list(global: bool, all: bool) {
    let file = &which_file(global); 
    if all {
        let json = open_json(DEFAULT).unwrap();
        let files = json["files"].as_array().unwrap();
        println!("File: Global");
        list_todo(DEFAULT);
        for f in files {
            let file = f.as_str().unwrap();
            println!("File: {}", file);
            list_todo(file);
        }
    } else {
        list_todo(file);
    } 
}

fn clear(all: bool, global: bool) {
    let file = &which_file(global);
    let mut json = open_json(file).unwrap();
    if all {
        json["default"] = json!([]);
    } else {
        let default = json["default"].as_array_mut().unwrap();
        let mut i = 0;
        while i < default.len() {
            if default[i]["done"].as_bool().unwrap() {
                default.remove(i);
            } else {
                i += 1;
            }
        }
    }
    dump_json(&json, file);
}

fn main() {
    let cli = Opt::parse();

    match cli.subcmd {
        SubCommand::Create => {
            create_json(JSON_FILE);
        }
        SubCommand::Add { val , global } => {
            println!("Add: {:?}", val);
            add(val, global);    
        }
        SubCommand::Done { val, global } => {
            done(val, global);
        }
        SubCommand::List { global, all } => {
            list(global, all);
        }
        SubCommand::Clear { verbose, global } => {
            clear(verbose, global);
        }
    }
}
