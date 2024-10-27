use names::Generator;
use std::process::{exit, Command};

fn update_commit_push() {
    // git add -A
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command");
    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo.");
        exit(1)
    };
    // git commit -m ""
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command");
    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes.");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("Failed to push code to origin");
    if !push_command.status.success() {
        eprintln!("Error: failed to push changes.");
        exit(1);
    }

    println!("Successfuly added, commited, and pushed all changes!");
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap()
}

fn main() {
    update_commit_push();
}
