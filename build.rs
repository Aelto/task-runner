
fn main() {
  // defines what language will be used by the task runner.
  // By default the task runner uses node for node.js and so
  // it will run javascript files that end with .js
  //
  // change it to whatever interpret you wish to use:
  let lang = "node";

  // defines the name of the folder the task runner will look for
  //
  // change it to whatever folder name you wish to use:
  let tasks_folder_name = "tasks";


  // this should not be changed.
  println!("cargo:rustc-env=lang={}", lang);
  println!("cargo:rustc-env=tasks_folder_name={}", tasks_folder_name);
}