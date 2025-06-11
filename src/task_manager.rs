use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
//use std::io::{self,Write};

const FILE_PATH: &str = "tasks.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id:u32,
    pub description:String,
    pub done: bool,
}

impl Task {
 pub fn new(id: u32, description: String) -> Self {
    Task {
        id,
        description,
        done: false,
    }
 }

}



pub fn run(action:String,task:String){
    println!("Processing {}: {}", task,action);
    
    let mut tasks = load_tasks();
    

    match action.as_str() {
        "add" => {
            let id = next_id(&tasks);
            tasks.push(addtask(&task,id));
        },
        "done" => done(&mut tasks,1),
        "delete" =>delete(&mut tasks, 1),
        "list" => listall(&tasks),
        _ => eprintln!("Unknown action: {} actions are add, delete, list, done", action),
    }

    save_tasks(&tasks);

}

fn addtask(desc:&String,id:u32) -> Task {
    let task = Task::new(id,desc.to_string());
    println!("Task added.");
    return task;
}

fn done(tasks:&mut Vec<Task>,id:u32){
    match tasks.iter_mut().find(|t| t.id == id) {
        Some(task) => {
            task.done = true;
             println!("Marked task {} as done.",id);
        }

        None => { eprintln!("Task with ID {} not found.",id);}
    }
}

fn delete(tasks:&mut Vec<Task>,id:u32){
    let original_len = tasks.len();
    tasks.retain(|t| t.id != id);
    if tasks.len() < original_len {
        println!("Deleted task {}",id);
    }
    else{
        eprintln!(" Task with ID {} not found.", id);
    }
}

fn listall(tasks: &Vec<Task>){
 if tasks.is_empty(){
    println!("No tasks found.");
 }else{
    for task in tasks {
        let status = if task.done { "True" } else { "False" };
         println!("{} [{}] {}", status, task.id, task.description);
    }
 }

}

fn load_tasks() -> Vec<Task> {
   if !Path::new(FILE_PATH).exists(){
        return vec![];
   }

   let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".into());
   serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

fn save_tasks(tasks: &Vec<Task>){
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks.");
    fs::write(FILE_PATH, json).expect("Failed to write tasks file.");
}

fn next_id(tasks: &Vec<Task>) -> u32 {
    tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

