use std::net::TcpListener; 
use std::result;
use std::io::Write;
use std::fmt;

type Result<T> = result::Result<T,()>;

const SAFE_MODE: bool = true;

struct Sensitive <T> {
    inner: T
}

impl <T> Sensitive <T>{
    fn new(inner: T) -> Self{
        Self {inner}
    }
}

impl <T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        }
        else{
            writeln!(f, "{inner}", inner=self.inner)
        }
    }
}

#[allow(unused_variables)]
fn main()-> Result<()> {

    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address).map_err(|err|{
        eprintln!("Error binding to {address}: {}",
        Sensitive::new(err));
    })?;

    println!("INFO: listening on {address}");

    for stream in listener.incoming(){
        match stream {
            Ok(mut stream) => {
                let _ = writeln!(stream, "Hallo meine Freunde! KKraut").map_err(|err|{
                    eprintln!("ERROR: could not write message : {err}");
                });
            },
            Err(err) => {
                eprintln!("Error: could not accept connection: {:?}", err)
            }
        }
    }
    Ok(())
}
