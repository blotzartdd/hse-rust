#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

const SOH: u8 = 1;

type MutexClientsHashmap = Arc<Mutex<HashMap<Vec<u8>, Arc<Mutex<Client>>>>>;

struct Client {
    client_reader: BufReader<TcpStream>,
    client_writer: TcpStream,
}

#[derive(PartialEq)]
enum CurState {
    RecieverNameWaiting,
    MessageWaiting,
}

fn handle_client(stream: TcpStream, clients_info: MutexClientsHashmap) {
    let writer = stream.try_clone().unwrap();
    let reader = BufReader::new(stream.try_clone().unwrap());

    let mut buf: Vec<u8> = Vec::new();

    let client_name: Vec<u8>;
    let mut client = Client {
        client_reader: reader,
        client_writer: writer,
    };

    match client.client_reader.read_until(SOH, &mut buf) {
        Ok(0) => {
            // println!("Client disconnected");
            return;
        }
        Ok(_) => {
            buf.pop();
            let name = buf.clone();
            client_name = name.clone();
            let mutex_client = Arc::new(Mutex::new(Client {
                client_reader: BufReader::new(stream.try_clone().unwrap()),
                client_writer: stream.try_clone().unwrap(),
            }));

            let mut info = clients_info.lock().unwrap();
            info.insert(name.clone(), mutex_client);

            // println!("Client {} logined!", name.clone());

            buf.clear();
        }
        Err(_) => {
            return;
        }
    }

    let mut cur_state = CurState::RecieverNameWaiting;
    let mut receiver_name: Vec<u8> = Vec::new();

    loop {
        match client.client_reader.read_until(SOH, &mut buf) {
            Ok(0) => {
                let mut info = clients_info.lock().unwrap();
                info.remove(&client_name);
                buf.clear();
                return;
            }
            Ok(_) => {
                if buf.last().is_none() {
                    return;
                }
                if buf.last().unwrap() != &SOH {
                    return;
                }

                buf.pop();

                let client_message = buf.clone();
                let info = clients_info.lock().unwrap();

                if cur_state == CurState::RecieverNameWaiting
                    && info.get(&client_message).is_some()
                    && client_message != client_name
                {
                    receiver_name = client_message;
                    cur_state = CurState::MessageWaiting;
                } else if cur_state == CurState::MessageWaiting {
                    let mut receiver = info.get(&receiver_name).unwrap().lock().unwrap();

                    let mut client_name_ = client_name.clone();
                    client_name_.push(SOH);
                    let _ = receiver.client_writer.write_all(&client_name_);

                    let mut client_message_ = client_message;
                    client_message_.push(SOH);
                    let _ = receiver.client_writer.write_all(&client_message_);

                    cur_state = CurState::RecieverNameWaiting;
                }

                buf.clear();
            }
            Err(_) => {
                let mut info = clients_info.lock().unwrap();
                info.remove(&client_name);
                buf.clear();
                return;
            }
        }
    }
}

pub fn run(ip: IpAddr, port: u16) {
    let socket = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket).unwrap();

    let clients_info: MutexClientsHashmap = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = {
                    let clients_info = Arc::clone(&clients_info);
                    thread::spawn(move || {
                        handle_client(stream, clients_info);
                    })
                };
                // println!("Clinet {:?} connected!", stream);
            }
            Err(e) => {
                println!("Error {} occured!", e);
            }
        }
    }
}
