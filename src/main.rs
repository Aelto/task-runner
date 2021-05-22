use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::env::args;

use colored::*;

fn main() {
  let some_task_name = args().nth(1);

  #[cfg(not(debug_assertions))]
  let tasks_folder = std::env::current_exe()
    .expect("could not find the current exe path")
    .parent()
    .expect("could not get the binary parent directory")
    .join(Path::new("tasks"));

  #[cfg(debug_assertions)]
  let tasks_folder = Path::new("tasks");

  if some_task_name.is_none() {
    list_tasks(&tasks_folder);
    
    return;
  }

  let task_name = some_task_name.unwrap();
  let task_args = args().skip(1);

  let output = Command::new("node")
    .stdout(Stdio::piped())
    .arg(tasks_folder.join(&task_name))
    .args(task_args)
    .output()
    .expect("failed to spawn child process");

  match output.status.success() {
    true => println!("🚀 {} - {}", task_name.green(), output.status),
    false => println!("❌ {} - {}", task_name.red(), output.status),
  }

  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
}

fn list_tasks(task_folder: &Path) {
  let tasks = std::fs::read_dir(task_folder)
    .expect("could not read the tasks directory");

  println!("❕ {}\n   {}", "no task was provided".magenta(), "here is a list of all available tasks:");
  for task in tasks {
    if let Ok(task) = task {
      if let Some(s) = task.file_name().to_str() {
        let dot_index = s.find('.').unwrap_or(s.len());

        println!("   - {}", &s[..dot_index]);
      }
    }
  }
}