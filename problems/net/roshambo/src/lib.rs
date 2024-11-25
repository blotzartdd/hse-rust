#![forbid(unsafe_code)]

use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream},
};

use std::sync::{Arc, Mutex};

use std::thread;

fn determine_winner(c1: u8, c2: u8) -> i32 {
    if (c1 == b'R' && c2 == b'S') || (c1 == b'P' && c2 == b'R') || (c1 == b'S' && c2 == b'P') {
        return 1;
    }

    if (c2 == b'R' && c1 == b'S') || (c2 == b'P' && c1 == b'R') || (c2 == b'S' && c1 == b'P') {
        return 2;
    }

    3
}

fn handle_game(stream1: TcpStream, stream2: TcpStream) {
    let mut writer1 = stream1.try_clone().expect("Failed to clone stream 1");
    let mut reader1 = BufReader::new(stream1.try_clone().unwrap());
    let mut buffer1 = [0; 1];
    let mut c1: u8;

    let mut writer2 = stream2.try_clone().expect("Failed to clone stream 2");
    let mut reader2 = BufReader::new(stream2.try_clone().unwrap());
    let mut buffer2 = [0; 1];
    let mut c2: u8;

    loop {
        match reader1.read_exact(&mut buffer1)  {
            Ok(_) => {
                if buffer1.is_empty() {
                    break;
                }

                c1 = buffer1[0]; 
                if c1 != b'R' && c1 != b'P' && c1 != b'S' {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }

        match reader2.read_exact(&mut buffer2) {
            Ok(_) => {
                if buffer2.is_empty() {
                    break;
                }

                c2 = buffer2[0];
                if c2 != b'R' && c2 != b'P' && c2 != b'S' {
                    break;
                }
            }
            Err(_) => {
                break;
            }
        }

        if !buffer1.is_empty() && !buffer2.is_empty() {
            let res = determine_winner(c1, c2);

            if res == 1 {
                let winner_slice: &[u8] = &[c2, b'W'];
                let loser_slice: &[u8] = &[c1, b'L'];
                let _ = writer1.write_all(winner_slice);
                let _ = writer2.write_all(loser_slice);
            } else if res == 2 {
                let winner_slice: &[u8] = &[c1, b'W'];
                let loser_slice: &[u8] = &[c2, b'L'];
                let _ = writer1.write_all(loser_slice);
                let _ = writer2.write_all(winner_slice);
            } else {
                let first_slice: &[u8] = &[c2, b'D'];
                let second_slice: &[u8] = &[c1, b'D'];
                let _ = writer1.write_all(first_slice);
                let _ = writer2.write_all(second_slice);
            }
        }
    }
}

pub fn run(ip: IpAddr, port: u16) {
    let socket = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(socket).unwrap();
    let mut stream_counter = Arc::new(Mutex::new(0));
    let mut read_stream_counter = Arc::clone(&stream_counter);
    let mut game_stream_counter = Arc::clone(&stream_counter);

    let mut stream_vec = Arc::new(Mutex::new(Vec::new()));
    let mut read_stream_vec = Arc::clone(&stream_vec);
    let mut game_stream_vec = Arc::clone(&stream_vec);

    let read_thread = thread::spawn(move || {
        let stream_counter = Arc::clone(&read_stream_counter);
        let mut counter = stream_counter.lock().unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if *counter >= 2 {
                        let _ = stream.shutdown(Shutdown::Read);
                        break;
                    }

                    *counter += 1;
                    let vec = Arc::clone(&read_stream_vec);
                    let mut stream_vec = vec.lock().unwrap();
                    stream_vec.push(stream.try_clone().unwrap());
                    if *counter == 2 {
                        break;
                    }
                }
                Err(_) => {}
            }
        }
    });


    let game_thread = thread::spawn(move || {
        loop {
            let stream_counter = Arc::clone(&game_stream_counter);
            let counter = stream_counter.lock().unwrap();

            let vec = Arc::clone(&game_stream_vec);
            let stream_vec = vec.lock().unwrap();
            if *counter == 2 {
                let stream1 = stream_vec[0].try_clone().unwrap();
                let stream2 = stream_vec[1].try_clone().unwrap();
                handle_game(stream1, stream2);
                break;
            }
        }
    });

    thread::sleep_ms(1500);
    if game_thread.is_finished() {
        drop(read_thread);
    }

    // let _ = stream_vec[0].try_clone().unwrap().shutdown(Shutdown::Both);
    // let _ = stream_vec[1].try_clone().unwrap().shutdown(Shutdown::Both);
}
