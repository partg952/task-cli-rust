use std::env;
use serde::{Serialize,Deserialize};
use serde_json;
use std::fs;
#[derive(Debug,Serialize,Deserialize)]
struct Task {
    name:String,
    completed:bool,
    startTime:DateAndTime,
    endTime:DateAndTime
}
#[derive(Debug,Serialize,Deserialize)]
struct DateAndTime {
    date:String,
    time:String
}
trait Commands {
    fn create_task(&mut self,task:Task);
    fn read_tasks(self);
}

impl Commands for Vec<Task> {
    fn create_task(&mut self,task:Task) {
        self.push(task);
        let encode = serde_json::to_string(&self).expect("failed to encode");
        fs::write("../tasks.txt", encode).expect("failed to write the file");
    }
    fn read_tasks(self) {
        let content = fs::read_to_string("../tasks.txt").expect("failed to read the file");
        let decoded:Vec<Task> = serde_json::from_str(&content).expect("failed to decode the json");

        println!("the content of the file are : {:?}",decoded);
    }
}
fn main() {
    let args:Vec<String> = env::args().collect(); 
    let command = args[1].clone();
    let mut tasks:Vec<Task> = vec![];      
    match command.as_str() {
        "create" => {
           let task = Task {
               name:args[2].clone(),
               completed:false,
               startTime:DateAndTime {
                   date:"22-04-2024".to_string(),
                   time:"9:00".to_string()
               },
               endTime : DateAndTime {
                   date:"25-04-2024".to_string(),
                   time:"9:00".to_string()
               }
           };
           tasks.create_task(task);
           println!("{:?}",tasks);
        },
        "read" => {
            tasks.read_tasks();
        }
        &_=>{
            println!("not correct command");
        }
    }


}
