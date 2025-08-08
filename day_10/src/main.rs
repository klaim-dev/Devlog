fn main() {

    let mut manager = TaskManager::new();

    let add_user = manager.add_user("Alice".to_string());
    match add_user {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    }; 

    let add = manager.add_task("Rust".to_string(), "Alice");
    match add {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let tasks_complete = manager.complete_task(1);
    match tasks_complete {
        Ok(value) => println!("{}", value),
        Err(e) => println!("{}", e),
    };

    let print = manager.print_status("Alice", 1);
    match print {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    };
}

struct Task {
    id: u32,
    title: String,
    completed: bool,
}

struct User {
    name: String,
    tasks: Vec<Task>,
}

struct TaskManager {
    users: Vec<User>,
    next_id: u32,
}

impl TaskManager {

    fn new() -> Self {
        TaskManager { users: Vec::new(), next_id: 1, }
    }

    fn add_user(&mut self, user_name: String) -> Result<String, String> {
        let add = if self.users
        .iter()
        .any(|u| u.name == user_name) {
            Err("User allready add".to_string())
        } else {

        let user = User {
        name: user_name,
        tasks: Vec::new(),
        };
        self.users.push(user);

            Ok("User add".to_string())
        };
        add
    }

    fn add_task(&mut self, title: String, name: &str) -> Result<String, String> {
        let user = self.users
        .iter_mut()
        .find(|u| u.name == name)
        .ok_or_else(|| "User not found".to_string())?;

    let task = Task{
        id: self.next_id,
        title,
        completed: false,
    };

    self.next_id += 1;

    user.tasks.push(task);

    Ok("Task added".to_string())

    }

    fn complete_task(&mut self, task_id: u32) -> Result<String, String> {
        let task = self.users
        .iter_mut()
        .flat_map(|u| u.tasks.iter_mut())
        .find(|t| t.id == task_id)
        .ok_or_else(||"Task not found".to_string())?;

    let complite = if task.completed {
        Err("Task allready completed".to_string())
    } else {
        task.completed = true;
        Ok("Task completed".to_string())
    };
    complite
    } 

    fn print_status(&self, user_name: &str, task_id: u32) -> Result<(), String> {
        let user = self.users
        .iter()
        .find(|u| u.name == user_name)
        .ok_or_else(|| "User mot found".to_string())?;
         
          println!("Tasks for {}", user_name);
          let task = user.tasks
          .iter()
          .find(|t| t.id == task_id)
          .ok_or_else(|| "Task id not found".to_string())?;
            let status = if task.completed { "✓" } else { "✗" };
            println!("[{}] {}", status, task.title);
            Ok(())
        }
}

