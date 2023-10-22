use std::net::{ToSocketAddrs};


struct Cli {
    url: String,
}

fn main() {
    // Overflows and gives 127
    // let localhost = Ipv4Addr::new(9999999, 0, 0, 1);
    //println!("{:?}" , localhost);

    let input = std::env::args().nth(1);
    
    let url = match input {
        Some(x) => {
            println!("URL is {}", x);
            x
        },
        None => {
            println!("No URL provided");
            panic!("No URL provided")
        },
    };

    let args = Cli {
	    url: url
    };

    let addrs_iter = args.url.to_socket_addrs();

    let ips = match addrs_iter {
	    Ok(x) => x,
	    Err(_) => {
            panic!("Error in reading hostname");
        }
    };

    // String -> IPv4Addr -> SockAddrv4
    for i in ips {
        println!("{:?}", i);
    }
    
}