use std::{fs::ReadDir, path::Path, path::PathBuf};
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::env::args;

use colored::*;

fn main() {
  let some_task_name = args().nth(1);
  let tasks_folder_name = std::env!("tasks_folder_name");

  let global_tasks_folder = std::env::current_exe()
    .expect("could not find the current exe path")
    .parent()
    .expect("could not get the binary parent directory")
    .join(Path::new(tasks_folder_name));

  let local_tasks_folder = Path::new(tasks_folder_name);

  if some_task_name.is_none() {
    list_tasks(&local_tasks_folder, &global_tasks_folder);
    
    return;
  }

  let task_name = &some_task_name.unwrap();
  let task_path = get_task_path(&local_tasks_folder, &global_tasks_folder, &task_name);
  let task_args = args().skip(1);
  let lang = std::env!("lang");
  
  if !task_path.exists() {
    println!("no local nor global task was found with the name {}", &task_name);

    return;
  }

  let output = Command::new(lang)
    .stdout(Stdio::inherit())
    .arg(task_path)
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

fn list_tasks(local_task_folder: &Path, global_task_folder: &Path) {
  println!("❕ {}\n   {}", "no task was provided".magenta(), "here is a list of all available tasks:");

  // 1.0
  // first we show the local tasks
  let local_tasks = std::fs::read_dir(local_task_folder);

  if local_tasks.is_ok() {
    display_tasks(local_tasks.unwrap(), true);
  }

  // 2.0
  // then we show the global tasks
  let global_tasks = std::fs::read_dir(global_task_folder);

  if global_tasks.is_ok() {
    display_tasks(global_tasks.unwrap(), false);
  }
}

fn display_tasks(tasks: ReadDir, is_colored: bool) {
  let valid_tasks = tasks
    .filter(Result::is_ok)
    .map(Result::unwrap);

  let extension = format!(".{}", std::env!("extension"));

  for task in valid_tasks {
    if let Some(s) = task.file_name().to_str() {
      let dot_index = s.find(&extension).unwrap_or(s.len());

      // we do not show files or folder that start with a dot.
      // this allows users to create file for util functions and such.
      if !s.starts_with('.') {
        let dash = if is_colored {
          "-".blue()
        } else {
          "-".dimmed()
        };

        println!("   {} {}", dash, &s[..dot_index]);
      }
    }
  }
}

fn get_task_path(local_task_folder: &Path, global_task_folder: &Path, task_name: &str) -> PathBuf {
  let extension = std::env!("extension");

  let mut local_path = local_task_folder.join(task_name);

  if local_path.exists() {
    return local_path;
  }

  local_path.set_extension(extension);

  if local_path.exists() {
    return local_path;
  }

  let mut global_path = global_task_folder.join(task_name);

  if global_path.exists() {
    return global_path;
  }

  global_path.set_extension(extension);

  global_path
}