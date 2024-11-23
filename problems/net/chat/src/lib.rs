#![forbid(unsafe_code)]

use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, ErrorKind, Write},
    net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

use log::{debug, error, info, warn};

pub fn run(ip: IpAddr, port: u16) {
    // TODO: your code goes here.
    unimplemented!()
}
