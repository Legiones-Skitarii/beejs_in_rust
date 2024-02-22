use std::{net::{SocketAddr, ToSocketAddrs}, io::{Read, Write}};

struct Cli {
    url: String,
}

fn main() -> std::io::Result<()> {
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

    let addrs = args.url.to_socket_addrs().unwrap().collect::<Vec<_>>();
    let (ipv4, ipv6): (Vec<_>, Vec<_>) = addrs.into_iter().partition(|x| x.is_ipv4());

    println!("ipv4: {:?}", ipv4);
    println!("ipv6: {:?}", ipv6);

    let socket = SocketAddr::new(ipv4[0].ip(), ipv4[0].port());
    println!("socket: {:?}", socket);

    let mut stream = std::net::TcpStream::connect(socket)?;
    println!("Connected: {:?}", stream); 
    stream.write(b"Hello from the client!")?;
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;
    println!("Response from {}: {}", socket ,String::from_utf8_lossy(&buffer[..]));

    // Keep program running
    loop {}

    Ok(())
}
