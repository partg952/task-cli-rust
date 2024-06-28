use std::env;
use chrono::prelude::*;
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
    fn get_prev_tasks(&mut self);
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
        for i in self {
            println!("Task name : {0} \n completed : {1} \n startTime : {2} \n endTime : {3}",i.name,i.completed,i.startTime.time,i.endTime.time);
        }
        

    }
    fn get_prev_tasks(&mut self) {
        let content = fs::read_to_string("../tasks.txt").expect("failed to fetch the previous tasks");
        let decoded:Vec<Task> = serde_json::from_str(&content).expect("failed to decode");
        for i in decoded {
            self.push(i);
        }
        println!("{:?}",self);
    }
}
fn main() {
    let args:Vec<String> = env::args().collect(); 
    let current_time = Utc::now().time();
    let current_date = Utc::now().date();
    let command = args[1].clone();
    let mut tasks:Vec<Task> = vec![];      
    tasks.get_prev_tasks(); 
    match command.as_str() {
        "create" => {
           let task = Task {
               name:args[2].clone(),
               completed:false,
               startTime:DateAndTime {
                   date:current_date.to_string(),
                   time:current_time.to_string()
               },
               endTime : DateAndTime {
                   date:args[2].clone(),
                   time:args[3].clone()
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
