use std::io;

#[derive(Debug)]
#[derive(Clone)] // Add the Clone trait to the Task struct
struct Task {
    name: String,
    description: String,
    priority: u8,
}

impl Task {
    fn new(name: String, description: String, priority: u8) -> Task {
        Task {
            name,
            description,
            priority,
        }
    }

    fn print(&self) {
        println!("Name: {}", self.name);
        println!("Description: {}", self.description);
        println!("Priority: {}", self.priority);
        println!("----------------");
    }
}

struct Tasks {
    tasks: Vec<Task>,
}

impl Tasks {
    fn new() -> Tasks {
        Tasks { tasks: Vec::new() }
    }

    fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn view(&self) {
        println!("Here are your tasks:");

        if self.tasks.len() == 0 {
            println!("No tasks found");
            return;
        }

        let mut tasks = self.tasks.clone();
        //tasks.cloneの中身を出力する
        println!("{:?}", tasks);

        tasks.sort_by(|a, b| a.priority.cmp(&b.priority));

        for task in tasks {
            task.print();
        }
        println!("End of tasks");
    }
}

fn main() {
    println!("Welcome to your Todo App!");

    let mut tasks: Tasks = Tasks::new();

    loop {
        println!("What would you like to do?");
        println!("1. Add a new task");
        println!("2. View all tasks");
        println!("3. Exit");

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match input {
            1 => add_task(&mut tasks),
            2 => tasks.view(),
            3 => break,
            _ => println!("Please enter a valid number"),
        }
    }
}

fn add_task(tasks: &mut Tasks) {
    println!("Enter the task you would like to add:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: Vec<&str> = input.split(",").collect();

    if input.len() != 3 {
        println!("Please enter a valid task");
        return;
    }

    let priority: u8 = match input[2].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid priority");
            return;
        }
    };

    if priority < 1 || priority > 3 {
        println!("Please enter a valid priority");
        return;
    }

    tasks.add(Task::new(
        input[0].trim().to_string(),
        input[1].trim().to_string(),
        priority,
    ));
}
