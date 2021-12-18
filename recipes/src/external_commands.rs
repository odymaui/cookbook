use std::process::{Command, Stdio};
use std::str;

pub fn run_os_command() {
    //https://doc.rust-lang.org/std/process/index.html#structs
    /*
        output => Result<Output>
        spawn => Result<Child>
        status => Result<ExitStatus>
    */


    //https://doc.rust-lang.org/std/process/struct.Command.html
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello foo"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello foo")
                .output()
                .expect("failed to execute process")
    };

    let hello = output.stdout;

    println!("command stdio output: {:?}", str::from_utf8(&hello).unwrap());

    //https://stackoverflow.com/questions/21011330/how-do-i-invoke-a-system-command-and-capture-its-output

    //spawn()=>Result<Child>=>.wait()=> Result<ExitStatus>
    //https://doc.rust-lang.org/std/process/struct.Child.html#method.wait
    //to get output, pipe output and use wait_with_output
    let output_child = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello bar"])
                .spawn()
                .unwrap()
                .wait()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello bar")
                .spawn()
                .unwrap()
                .wait()
                .expect("failed to execute process")
    };

    //wait returns exit status...
    println!("command spawn.wait() output status: {:?}", output_child);
    

    let output_child = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello baz"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello baz")
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .expect("failed to execute process")
    };

    //wait returns exit status...
    println!("command spawn.wait() raw output result: {:?}", output_child);

    println!("command spawn.wait() output stdout: {:?}", str::from_utf8(&output_child.stdout).unwrap());
    
    

}