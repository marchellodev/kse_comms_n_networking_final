use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::{
    env,
    io::{self, prelude::*, BufReader, BufWriter, Cursor},
    net::{TcpListener, TcpStream},
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
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let desc = r#"
        Available commands:
        > ping
        > <number>
    "#;

    println!("Connected to the server!");
    println!("{}", desc);

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = stream.try_clone().unwrap();

    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        match input {
            "ping" => {
                writer.write_u32::<BigEndian>(PING).unwrap();
            }
            _ => {
                println!("sending a numebr");
                let num = input.parse::<u32>().unwrap();
                writer.write_u32::<BigEndian>(num).unwrap();
            }
        }

        println!("Sending user input: {}", input);

        let mut buffer = [0; 4];

        if reader.read_exact(&mut buffer).is_err() {
            println!("Connection closed");
            break;
        }
        let mut rdr = Cursor::new(buffer);
        let decoded = rdr.read_u32::<BigEndian>().unwrap();
        match decoded {
            PONG => {
                println!("> Received pong from the server");
            }
            _ => {
                println!("Unknown response")
            }
        }
    }
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
