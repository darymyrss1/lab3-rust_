use serde::{Serialize, Deserialize};
use std::fs;
use std::env;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    text: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks = load_tasks();

    if args.len() < 2 {
        println!("Команды: add <текст>, list, done <id>");
        return;
    }
        
        match args[1].as_str() {
        "add" => {
            let id = (tasks.len() as u32) + 1;
            tasks.push(Task { id, text: args[2..].join(" "), completed: false });
            save_tasks(&tasks);
            println!("Добавлено!");
        }
        "list" => {
            for t in &tasks {
                let check = if t.completed { "[x]" } else { "[ ]" };
                println!("{} {}: {}", check, t.id, t.text);
            }
        }
        "done" => {
            if let Some(id_arg) = args.get(2) {
                if let Ok(id) = id_arg.parse::<u32>() {
                    if let Some(t) = tasks.iter_mut().find(|task| task.id == id) {
                        t.completed = true;
                        save_tasks(&tasks);
                        println!("Задача {} выполнена", id);
                    }
                }
            }
        }
        _ => println!("Ошибка: неизвестная команда"),
    }
}

fn load_tasks() -> Vec<Task> {
    fs::read_to_string("tasks.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_default())
        .unwrap_or_default()
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    fs::write("tasks.json", json).ok();
}
