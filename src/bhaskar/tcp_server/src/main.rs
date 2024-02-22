#![feature(type_name_of_val)]
use std::{net::{SocketAddr, ToSocketAddrs}, io::{Read, Write}, net::TcpListener, thread};

struct Cli {
    url: String,
}


fn handle_client(mut stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request from {} : {}", stream.peer_addr().unwrap(), String::from_utf8_lossy(&buffer[..]));

    let response = "Hello from the server!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn main() {
    // parsing args
    // let pattern = std::env::args().nth(1).expect("no pattern given");

    // see type of pattern
    // println!("{:?}", std::any::type_name_of_val(&pattern));
    // println!("{:?}", pattern);


    let input = std::env::args().nth(1);

    let url = match input {
        Some(x) => {
            println!("url: {:?}", x);
            x
        },
        None => {
            println!("Usage: cargo run <url>");
            panic!("No URL provided in the arguments")
        }
    };

    let args = Cli { url };

    // // assuming 'localhost' resolves to 127.0.0.1
    // let mut addrs_iter = "localhost:443".to_socket_addrs().unwrap();
    // assert_eq!(addrs_iter.next(), Some(SocketAddr::from(([127, 0, 0, 1], 443))));
    // assert!(addrs_iter.next().is_none());
    //
    // // assuming 'foo' does not resolve
    // assert!("foo:443".to_socket_addrs().is_err());

    let addrs_iter = args.url.to_socket_addrs();
    match addrs_iter {
        Ok(x) => {
            for i in x {
                println!("addr: {:?}", i);
            }
        },
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    }

    // move all addresses into a vector and then into ipv4 and ipv6 addresses

    let addrs = args.url.to_socket_addrs().unwrap().collect::<Vec<_>>();
    let (ipv4, ipv6): (Vec<_>, Vec<_>) = addrs.into_iter().partition(|x| x.is_ipv4());

    println!("ipv4: {:?}", ipv4);
    println!("ipv6: {:?}", ipv6);

    let socket = SocketAddr::new(ipv4[0].ip(), ipv4[0].port());
    println!("socket: {:?}", socket);

    let listener = TcpListener::bind(socket).expect("Could not bind to address");
    println!("listener: {:?}", listener);

    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }

    println!("Listening on {:?}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
