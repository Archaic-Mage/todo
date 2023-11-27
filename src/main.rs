use clap::{Parser, Subcommand};
use serde_json::{json,Result, Value};

const JSON_FILE: &str = "/tmp/todo.json";

#[derive(Parser)]
#[clap(name = "todo", version = "1.0", author = "Your Name")]
struct Opt {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Add {
        val: Vec<String>,
    },
    Done {
        val: Vec<i32>,
    },
    List,
    Clear,
    Clearall,
}

fn create_json() -> String {
    let json = json!({
        "default": []
    });
    let json = serde_json::to_string(&json).unwrap();
    std::fs::write(JSON_FILE, &json).unwrap();
    json
}

fn open_json() -> Result<Value> {
    let json_file = std::fs::read_to_string(JSON_FILE);
    let json = match json_file {
        Ok(json) => json,
        Err(_) => create_json(),
    };
    let json: Value = serde_json::from_str(&json)?;
    Ok(json)
}

fn dump_json(json: &Value) {
    let json = serde_json::to_string(json).unwrap();
    std::fs::write(JSON_FILE, json).unwrap();
}

fn add(val: Vec<String>) {
    let mut json = open_json().unwrap();
    for v in val {
        let item = json![{"name": v, "done": false}];
        json["default"].as_array_mut().unwrap().push(item);
    }
    dump_json(&json);
}

fn done(val: Vec<i32>) {
    let mut json = open_json().unwrap();
    let default = json["default"].as_array_mut().unwrap(); 
    for v in val {
        default[v as usize]["done"] = json!(true);
    }
    dump_json(&json);
}

fn list() {
    let json = open_json().unwrap();
    let default = json["default"].as_array().unwrap();
    for (i, v) in default.iter().enumerate() {
        let done = v["done"].as_bool().unwrap();
        let name = v["name"].as_str().unwrap();
        println!("{}: {} {}", i, if done { "☑" } else { "☐" }, name);
    }
}

fn clear(all: bool) {
    let mut json = open_json().unwrap();
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
    dump_json(&json);
}

fn main() {
    let cli = Opt::parse();

    match cli.subcmd {
        SubCommand::Add { val } => {
            println!("Add: {:?}", val);
            add(val);    
        }
        SubCommand::Done { val } => {
            done(val);
        }
        SubCommand::List => {
            list();
        }
        SubCommand::Clear => {
            clear(false);
        }
        SubCommand::Clearall => {
            clear(true);
        }
    }
}
