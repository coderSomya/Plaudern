use std::net::{TcpListener, TcpStream}; 
use std::result;
use std::io::Write;
use std::fmt;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;

type Result<T> = result::Result<T,()>;

const SAFE_MODE: bool = false;

enum Message{
    ClientConnected,
    ClientDisconnected,
    NewMessage
}

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

fn client(mut stream: TcpStream){
    let _  = writeln!(stream, "hallo meine freunde! kkrat").map_err(|err|{
        eprintln!("ERROR: could not write message to user: {err}")
    });
    todo!()
}

fn server(_message: Receiver<Message>) -> Result<()> {
    Ok(())
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

    let (message_sender, message_receiver) = channel();

    thread::spawn(|| server(message_receiver));

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                thread::spawn(|| {client(stream) });
            },
            Err(err) => {
                eprintln!("Error: could not accept connection: {:?}", err)
            }
        }
    }
    Ok(())
}
