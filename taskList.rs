use std::fs;
use std::io;
use std::path::Path;
use serde::{Serialize, Deserialize};
use clap::{App, Arg, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskList {
    tasks: Vec<Task>,
}

impl TaskList {
    fn new() -> Self {
        TaskList { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        self.tasks.push(Task {
            description,
            completed: false,
        });
    }

    fn mark_completed(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            task.completed = true;
        }
    }

    fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
        }
    }

    fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            let status = if task.completed { "Completed" } else { "Pending" };
            println!("{}: {} [{}]", i, task.description, status);
        }
    }

    fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string(&self)?;
        fs::write("tasks.json", json)
    }

    fn load() -> io::Result<Self> {
        if Path::new("tasks.json").exists() {
            let json = fs::read_to_string("tasks.json")?;
            let tasks = serde_json::from_str(&json)?;
            Ok(tasks)
        } else {
            Ok(TaskList::new())
        }
    }
}

fn main() {
    let matches = App::new("Todo List CLI")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("Manage your todo list from the command line")
        .subcommand(SubCommand::with_name("add")
            .about("Add a new task")
            .arg(Arg::with_name("description")
                .required(true)
                .help("The description of the task")))
        .subcommand(SubCommand::with_name("complete")
            .about("Mark a task as completed")
            .arg(Arg::with_name("index")
                .required(true)
                .help("The index of the task to mark as completed")))
        .subcommand(SubCommand::with_name("remove")
            .about("Remove a task")
            .arg(Arg::with_name("index")
                .required(true)
                .help("The index of the task to remove")))
        .subcommand(SubCommand::with_name("list")
            .about("List all tasks"))
        .get_matches();

    let mut task_list = TaskList::load().unwrap_or_else(|err| {
        eprintln!("Error loading tasks: {}", err);
        TaskList::new()
    });

    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let description = sub_m.value_of("description").unwrap().to_string();
            task_list.add_task(description);
            task_list.save().unwrap();
        }
        ("complete", Some(sub_m)) => {
            let index: usize = sub_m.value_of("index").unwrap().parse().unwrap();
            task_list.mark_completed(index);
            task_list.save().unwrap();
        }
        ("remove", Some(sub_m)) => {
            let index: usize = sub_m.value_of("index").unwrap().parse().unwrap();
            task_list.remove_task(index);
            task_list.save().unwrap();
        }
        ("list", _) => {
            task_list.list_tasks();
        }
        _ => {
            println!("No valid subcommand provided.");
        }
    }
}
