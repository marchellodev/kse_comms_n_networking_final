use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    env,
    io::{prelude::*, BufReader, BufWriter, Cursor},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

const PING: u32 = 5;
const PONG: u32 = 6;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).unwrap();
    match command.as_str() {
        "server" => run_server(),
        "client" => run_client(),
        _ => println!("Unknown command"),
    };
}

fn run_client() {
    println!("RUNNIG CLIENT")
}
fn run_server() {
    println!("RUNNING SERVER");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");

        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(stream: TcpStream) {
    let stream_clone = stream.try_clone().expect("Failed to clone TcpStream.");

    let mut buf_reader = BufReader::new(stream);
    let mut buf_writer = BufWriter::new(stream_clone);

    loop {
        let mut buffer = [0; 4];
        if buf_reader.read_exact(&mut buffer).is_err() {
            println!("Connection closed");
            break;
        }

        let mut rdr = Cursor::new(buffer);
        let decoded = rdr.read_u32::<BigEndian>().unwrap();

        println!("> Received request: {:#?}", decoded);

        match decoded {
            PING => {
                println!("> Received ping from the server, sendig pong");
                buf_writer.write_u32::<BigEndian>(PONG).unwrap();
                buf_writer.flush().unwrap();
            }
            _ => {
                println!("Unknown request")
            }
        }
    }
}
