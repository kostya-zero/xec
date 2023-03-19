use std::{process::{Command, exit, Stdio}};

use crate::term;

pub struct Shell;

impl Shell {
    pub fn run(cmd: String) {
        let output = Command::new("sh")
            .stdout(Stdio::inherit())
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Command in scriptlet failed to run.");

        if !output.status.success()  {
            term::Term::fatal("Command exited with bad exit code.".to_string());
            exit(1);
        }
    }
}
