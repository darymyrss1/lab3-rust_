use serde::{Serialize, Deserialize};
use std::fs;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    text: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && args[1] == "add" {
        let new_task = Task {
            id: 1, 
            text: args[2].clone(),
            completed: false,
        };
        
        let json = serde_json::to_string(&vec![new_task]).unwrap();
        fs::write("tasks.json", json).expect("Ошибка записи");
        println!("Задача сохранена в tasks.json");
    }
}
