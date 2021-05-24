
fn main() {
  // defines what language will be used by the task runner.
  // By default the task runner uses node for node.js and so
  // it will run javascript files that end with .js
  //
  // change it to whatever interpreter you wish to use:
  let lang = "node";

  // defines what extension the task runner will look for.
  // By default the task runner uses node.js and javascript
  // so it looks for `.js` files.
  //
  // change it to whatever extension you wish to use:
  let extension = "js";

  // defines the name of the folder the task runner will look for
  //
  // change it to whatever folder name you wish to use:
  let tasks_folder_name = "tasks";


  // this should not be changed.
  println!("cargo:rustc-env=lang={}", lang);
  println!("cargo:rustc-env=extension={}", extension);
  println!("cargo:rustc-env=tasks_folder_name={}", tasks_folder_name);
}