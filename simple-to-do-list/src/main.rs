use std::io;

struct Task {
    name: String,
    completed: bool,
}

fn main() {
    let mut tasks = Vec::new();

    loop {
        println!("Enter command (add / complete / show / exit):");
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match command.trim() {
            "add" => {
                println!("Enter task name:");
                let mut task_name = String::new();
                io::stdin()
                    .read_line(&mut task_name)
                    .expect("Failed to read line");
                add_task(&mut tasks, &task_name);
            }
            "complete" => {
                println!("Enter task index:");
                let mut index_str = String::new();
                io::stdin()
                    .read_line(&mut index_str)
                    .expect("Failed to read line");
                let index: usize = index_str.trim().parse().expect("Please type a number!");
                complete_task(&mut tasks, index);
            }
            "show" => display_tasks(&tasks),
            "exit" => break,
            _ => println!("Invalid command"),
        }
    }
}

fn add_task(tasks: &mut Vec<Task>, name: &str) {
    tasks.push(Task {
        name: name.to_string(),
        completed: false,
    });
}

fn complete_task(tasks: &mut Vec<Task>, index: usize) {
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
    }
}

fn display_tasks(tasks: &[Task]) {
    for (index, task) in tasks.iter().enumerate() {
        println!(
            "{}: {} - {}",
            index,
            task.name,
            if task.completed {
                "Completed"
            } else {
                "Not Completed"
            }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut tasks = Vec::new();
        add_task(&mut tasks, "Learn Rust");
        assert_eq!(tasks.len(), 1);
    }

    #[test]
    fn test_complete_task() {
        let mut tasks = vec![Task {
            name: String::from("Learn Rust"),
            completed: false,
        }];
        complete_task(&mut tasks, 0);
        assert!(tasks[0].completed);
    }
}
