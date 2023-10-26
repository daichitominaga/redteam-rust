use std::env;
use std::process::Command;

fn exeCmd(cmd: &str) -> String {
    let temp = "/c ".to_owned();
    let fullcmd = temp + cmd;

    let cmds: Vec<&str> = fullcmd.split(" ").collect();
    let res = Command::new("cmd.exe").args(&cmds).output().unwrap();

    let stdout = String::from_utf8_lossy(res.stdout.as_slice());
    let stderr = String::from_utf8_lossy(res.stderr.as_slice());

    if stdout.len() > 0 {
        return stdout.to_string();
    } else {
        return stderr.to_string();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let res = exeCmd(&args[1]);
        println!("{}", res);
    } else {
        println!("[+] Usage: {} command", args[0])
    }
}
