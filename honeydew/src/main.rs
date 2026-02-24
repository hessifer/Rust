/*
This tool will be a command-line tool to track daily tasks and goals.
The tool shoudl implement the following features:
- present a menu to the user with the following options:
  - View today's tasks
  - View all tasks
  - Add a new task
  - Delete a task
  - Exit
- The tool should be able to store the tasks in a file
- The tool should be able to load the tasks from a file
- The tool should be able to save the tasks to a file
- The tool should be able to display the tasks in a list format
*/
fn main() {
}

fn display_menu() {
  println!("Welcome to the Honeydew Task Tracker");
  println!("Please select an option:");
  println!("1. View today's tasks");
  println!("2. View all tasks");
  println!("3. Add a new task");
  println!("4. Delete a task");
  println!("5. Exit");
}

fn view_today_tasks() {
  println!("Today's tasks:");
  println!("1. Task 1");
  println!("2. Task 2");
  println!("3. Task 3");
}

fn view_all_tasks() {
  println!("All tasks:");
  println!("1. Task 1");
  println!("2. Task 2");
  println!("3. Task 3");
}

fn add_new_task() {
  println!("Enter the task:");
  let task = String::new();
  println!("Task added: {}", task);
}

fn delete_task() {
  println!("Enter the task to delete:");
  let task = String::new(); 
  println!("Task deleted: {}", task);
}

fn exit() {
  println!("Exiting the tool...");
}
