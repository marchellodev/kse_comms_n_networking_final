use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use rand::Rng;
use std::{
    env,
    io::{self, prelude::*, BufReader, BufWriter, Cursor},
    net::{TcpListener, TcpStream},
    thread,
};

const PING: u32 = 50000;
const PONG: u32 = 50001;

const TOO_HIGH: u32 = 50002;
const TOO_LOW: u32 = 50003;
const CORRECT: u32 = 50004;

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
        > <number>: guess the number from 0 to 255
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
            TOO_LOW => {
                println!("> Server says: the number is too low");
            }
            TOO_HIGH => {
                println!("> Server says: the number is too high");
            }
            CORRECT => {
                println!("> Server says: You guessed the number! Restart the program to try again");
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

    let mut rng = rand::thread_rng();
    let secret: u8 = rng.gen();
    println!("the number generated: {}", secret);

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
                // parse a number
                println!("> Received number: {}", decoded);

                if decoded > secret as u32 {
                    buf_writer.write_u32::<BigEndian>(TOO_HIGH).unwrap();
                    buf_writer.flush().unwrap();
                }
                if decoded < secret as u32 {
                    buf_writer.write_u32::<BigEndian>(TOO_LOW).unwrap();
                    buf_writer.flush().unwrap();
                }
                if decoded == secret as u32 {
                    buf_writer.write_u32::<BigEndian>(CORRECT).unwrap();
                    buf_writer.flush().unwrap();
                }
            }
        }
    }
}
