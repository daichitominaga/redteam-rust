use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream, Shutdown};
use std::process::{Command, exit};
use std::io::{self, Write, BufReader};

fn main() {
    let bind_ip = "128.0.0.1"; 
    let bind_port: u16 = 1234;

    let ip = bind_ip.parse::<Ipv4Addr>();
    let ip_addr = match ip {
        Ok(i) => i,
        Err(e) => {println!("{}", e); exit(0)}
    };

    if bind_port<0 || bind_port > 65535 {
        println!("Invalid Port number");
        exit(0);
    }

    let cs = SocketAddrV4::new(ip_addr, bind_port);

    let listener = TcpListener::bind(cs);
    match listener {
        Ok(i) => i,
        Err(e) => {println!("{}", e); exit(0)}
    };

    println!("Binded to: {}:{}", cs.ip(), cs.port());

    let (mut socket_client, client_address) = listener.accept().unwrap();
    println!("Client Connected from: {}", socket_client);

    loop {
        println!("Enter Command to send: ");
        let mut  input = String::new();
        io::stdin().read_line(&mut input).expect("String expected");
        input.push('\0');

        if input.as_str() == "quit" {
            break;
        }

        socket_client.write(&mut input.as_bytes());
        
        let mut buffer:Vec<u8> = Vec::new();
        let reader = BufReader::new(&socket_client);
        reader.read_until(b'\0', &mut buffer);
    
        println!("Received: {}", String::from_utf8_lossy((&buffer)));
    };

    println!("shutting down the client: {}", client_address);
    socket_client.shutdown(Shutdown::Both);
    drop(listener);
}
