use std::{
    env,
    process::{Command, Stdio},
};
// use syscalls::{Sysno, syscall};

fn main() {
   match env::args().collect::<Vec<String>>()[2].as_str() {
    "run" => run(),
    "child" => child(),
    _ => panic!("damn son, where'd you find this?"),
   } 
}

fn run() {
    println!("Running {}", env::args().collect::<Vec<String>>()[2..].join(" "));
    
    let args: Vec<String> = env::args().collect();

    let mut cmd = Command::new("/proc/self/exe");
    cmd.args(["child", &args[3..].join(",")])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("big boy grilling on the roof");
}

fn child() {
    println!("Running {}", env::args().collect::<Vec<String>>()[2..].join(" "));
    
    let args: Vec<String> = env::args().collect();

    let mut cmd = Command::new("/proc/self/exe");
    cmd.args(["child", &args[3..].join(",")])
        .stdout(Stdio::inherit())
        .spawn()
        .expect("big boy grilling on the roof");
}

