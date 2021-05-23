# task-runner

a minimalist task runner in Rust for Node/Python/...

# Usage
## Folders hierarchy
the task-runner looks for two specific folders whenever it's run:
 - a folder placed besides the task-runner's binary which is named `/tasks`
 - a folder in your current working directory when you ran the task-runner which
 also named `/tasks`.

The folder placed near the task-runner binary can be considered the global tasks
library. wherever you are on your disk, running the task-runner will work on any
of the tasks that are in this folder.

The other `/tasks` folder in your current working directories are used for
workspace tasks. You can place in them tasks that are unique to your projects.

---

If you wish to change the name of the task folder, before building from the sources head into the [`build.rs`](build.rs) file and edit the `tasks_folder_name` variable accordingly. For example if you want your tasks folder to be named `mes-taches` (French for "my tasks") then change the variable to the following:
```rust
tasks_folder_name = "mes-taches";
```

## The tasks
the task-runner runs javascript files with node.js by default. If you wish to change that, before building the tool from the sources head into the [`build.rs`](build.rs) file and edit the `lang` variable accordingly. For example if you want your task-runner to use python then change the variable like so:
```rust
let lang = "python";
```

there is also an `extension` variable. This extension defines what extensions the task runner looks for, it will ignore
anything that has not this extension. Following the previous example of python, here is what the new value would be:
```rust
let extension = ".py";
```

## Running it
Running the task runner is pretty simple because it has no subcommand and only does two things.

Get all available tasks by running the task-runner without any task name:
```
$ rr
```

Run a task named `example`:
```
$ rr example
```

Run the example task with a custom parameter:
```
$ rr example fail
```
the `fail` parameter will be passed to the example task.