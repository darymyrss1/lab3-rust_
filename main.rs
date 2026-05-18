use std::env;

struct Task {
    id: u32,
    text: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 && args[1] == "add" {
        let task_text = &args[2];
        println!("План: Добавить задачу '{}' в список", task_text);
    } else {
        println!("Использование: add <текст_задачи>");
    }
}
