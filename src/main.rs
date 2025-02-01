use std::io;


enum MyTask {
    Pending,
    Completed
}
impl Default for MyTask {
  fn default() -> Self {
      MyTask::Pending
  }
}
 struct Task {
  name: String,
  status: MyTask
 }

 impl Task {
  fn new(name: &str) -> Self {
      Task {
          name: name.to_string(),
          status: MyTask::default(),
      }
  }
}



fn main() {
  let mut tasks: Vec<Task> = Vec::new();
  loop{
    println!("Choose an Option: \n1.\tAdd Task\n2.\tView Tasks\n3.\tDelete Tasks\n4.\tExit");
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input");

    let answer: i32 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number between 1 and 4.");
            continue;
        }
    };
    if answer == 1 {
      println!("Enter name of the task: ");
      let mut task = String::new();
      io::stdin()
      .read_line(&mut task )
      .expect("Failed to read task");
     tasks.push(Task::new(task.trim()));
     println!("Added succesfully");
    }
    else if answer == 2{
      if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        println!("Tasks:");
        for (index, task) in tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task.name);
        }
        println!("\n");
    }
  }
    else if answer == 4 {
      break;
    }
    continue;
  }
}
