use std::net::TcpListener; 
use std::result;
use std::io::Write;
use std::fmt;

type Result<T> = result::Result<T,()>;

const SAFE_MODE: bool = false;

struct Sensitive <T> (T);


impl <T: fmt::Display> fmt::Display for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        let Self(inner) = self;
        if SAFE_MODE {
            writeln!(f, "[REDACTED]")
        }
        else{
            writeln!(f, "{inner}")
        }
    }
}

#[allow(unused_variables)]
fn main()-> Result<()> {

    let address = "127.0.0.1:6969";
    let listener = TcpListener::bind(address).map_err(|err|{
        eprintln!("Error binding to {}: {}",
        Sensitive(address),
        Sensitive(err));
    })?;

    println!("INFO: listening on {}", Sensitive(address));

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
