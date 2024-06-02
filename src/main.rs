use std::net::{TcpListener, TcpStream}; 
use std::ops::Deref;
use std::result;
use std::io::{Read,Write};
use std::fmt;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::sync::Arc;
use std::collections::HashMap;

type Result<T> = result::Result<T,()>;

const SAFE_MODE: bool = false;

struct Client {
    conn: Arc<TcpStream>
}

enum Message{
    ClientConnected{
        author: Arc<TcpStream>
    },
    ClientDisconnected{
        author: Arc<TcpStream>
    },
    NewMessage{
        author: Arc<TcpStream>,
        bytes:Vec<u8>
    }
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

fn client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<()>{
    messages.send(Message::ClientConnected{author: stream.clone()}).map_err(|err| {
        eprintln!("ERROR: could not send message to the server thread: {err}");
    })?;
    let mut buffer = Vec::new();
    buffer.resize(64,0);
    loop{
        let n = stream.deref().read(&mut buffer).map_err(|err|{
           let _ = messages.send(Message::ClientDisconnected{author:stream.clone()});
        })?;
        let _ = messages.send(Message::NewMessage{author: stream.clone(), bytes:buffer[0..n].to_vec()}).map_err(|err|{
            eprintln!("ERROR: could not read message from client: {err}");
        })?;
    }
}

fn server(messages: Receiver<Message>) -> Result<()> {
    let mut clients = HashMap::new();

    loop {
        let msg = messages.recv().expect("The server receiver is not hung up");

        match msg {
            Message::ClientConnected{author} =>{
                let addr = author.peer_addr().expect("Todo: cache it");
                clients.insert(addr.clone(), Client{
                    conn: author.clone()
                });
            },
            Message::ClientDisconnected{author}=>{
                let addr = author.peer_addr().expect("Todo: cache it");
                clients.remove(&addr);
            },
            Message::NewMessage{author, bytes} =>{
                let author_addr = author.peer_addr().expect("Todo: cache it");                
                for (addr, client) in clients.iter(){
                    if *addr != author_addr {
                        let _ = client.conn.as_ref().write(&bytes);
                    }
                }
            }
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

    let (message_sender, message_receiver) = channel();

    thread::spawn(|| server(message_receiver));

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let message_sender = message_sender.clone();
                thread::spawn(|| client(stream.into(), message_sender));
            },
            Err(err) => {
                eprintln!("Error: could not accept connection: {:?}", err)
            }
        }
    }
    Ok(())
}
