use std::io::{self, Write, Read};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use std::env;

fn main() -> io::Result<()> {

    let args: Vec<_> = env::args().collect();
    let ip = args[1].as_str();

    for port in 1..9999 {
        if checkport(ip, &port.to_string()) {
            grab_banner(ip, &port.to_string());
        }
    }

    Ok(())
}

fn checkport(ip: &str, port: &str) -> bool
{
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(ip).unwrap()), u16::from_str(port).unwrap());

    match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(5))
    {
        Ok(_stream) => { 
            
            println!("Port {} is OPEN", port);

            return true; 
        }

        _ => { return false; }        
    }
}

fn grab_banner(ip: &str, port: &str) 
{
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(ip).unwrap()), u16::from_str(port).unwrap());

    let helo_vals  = ["", "HELO example.com\r\n", "GET / HTTP/1.0\r\n\r\n"];
    for val in helo_vals {

        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(5))
        {
            Ok(mut stream) => {

                if val != "" {
                    stream.set_write_timeout(Some(Duration::new(5, 0)));
                    match stream.write_all(val.as_bytes())
                    {
                        Ok(_) => {}
                        _ => {}
                    }
                }

                stream.set_read_timeout(Some(Duration::new(5, 0)));
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer)
                {
                    Ok(bytes_read) => {
                        let banner = String::from_utf8_lossy(&buffer[..bytes_read]);
                        println!("Received banner: \n{}", banner);
                        break;
                    }
                    _ => {}
                }
            }

            Err(stream) => {
                println!("Error: \n{}", stream);
            }
        }


    }

}
