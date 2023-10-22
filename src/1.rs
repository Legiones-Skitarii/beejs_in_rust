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

    match addrs_iter {
	    Ok(x) => {
            for i in x {
                println!("Address is {}", i);
            }
        },
	    Err(_) => println!("Error in reading hostname")
    }
    
}