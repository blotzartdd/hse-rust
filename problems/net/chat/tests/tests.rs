#![allow(unused)]

use rand::Rng;
use std::io::{self, prelude::*};
use std::mem::take;
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::process::{Child, Command};
use std::str::{self, FromStr};
use std::thread;
use std::time::{self, Duration};

const BINARY_PATH: &str = env!("CARGO_BIN_EXE_chat");

const SOH: u8 = 1;

fn take_a_nap() {
    thread::sleep(Duration::from_millis(100));
}

enum IpVersion {
    V4,
    V6,
}

struct ServerWrapper {
    proc: Option<Child>,
    addr: SocketAddr,
}

impl ServerWrapper {
    fn start(ip_version: IpVersion) -> Self {
        let mut rng = rand::thread_rng();
        let port = rng.gen_range(40000..49151);
        let ip = match ip_version {
            IpVersion::V4 => IpAddr::from_str("127.0.0.1").unwrap(),
            IpVersion::V6 => IpAddr::from_str("::1").unwrap(),
        };

        eprintln!("binary path {}", BINARY_PATH);
        let proc = Command::new(BINARY_PATH)
            .arg("--ip")
            .arg(ip.to_string())
            .arg("--port")
            .arg(port.to_string())
            .spawn()
            .unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        Self {
            proc: Some(proc),
            addr: SocketAddr::new(ip, port),
        }
    }

    fn is_alive(&mut self) -> bool {
        self.proc
            .as_mut()
            .map_or(false, |proc| proc.try_wait().unwrap().is_none())
    }

    fn stop(&mut self) -> std::io::Result<()> {
        self.proc.take().map_or(Ok(()), |mut proc| proc.kill())
    }
}

impl Drop for ServerWrapper {
    fn drop(&mut self) {
        let _ = self.stop().unwrap();
    }
}

#[derive(Debug)]
struct Client {
    name: Option<Vec<u8>>,
    conn: TcpStream,
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            conn: self.conn.try_clone().unwrap(),
        }
    }
}

impl Client {
    fn start(server_addr: SocketAddr) -> std::io::Result<Self> {
        let conn = TcpStream::connect(server_addr)?;
        Ok(Self { name: None, conn })
    }

    fn name(&self) -> &[u8] {
        self.name.as_ref().unwrap()
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.conn.write_all(data)
    }

    fn login(&mut self, name: &[u8]) -> std::io::Result<()> {
        self.name = Some(name.to_vec());
        self.write(name)?;
        self.write(&[SOH])
    }

    fn message_to(&mut self, to: &[u8], message: &[u8]) -> std::io::Result<()> {
        self.write(to)?;
        self.write(&[SOH])?;
        self.write(message)?;
        self.write(&[SOH])?;
        Ok(())
    }

    fn message(&mut self, to: &Client, message: &[u8]) -> std::io::Result<()> {
        self.message_to(to.name(), message)
    }

    fn read_expect(&mut self, expected: &[u8]) {
        let mut buf = vec![0; expected.len() + 1];
        self.conn.read_exact(&mut buf).unwrap();
        assert_eq!(expected, &buf[..expected.len()]);
        assert_eq!(SOH, buf[expected.len()]);
    }

    fn expect_message_from(&mut self, from: &[u8], expected: &[u8]) {
        self.read_expect(from);
        self.read_expect(expected);
    }

    fn expect_message(&mut self, from: &Client, expected: &[u8]) {
        self.expect_message_from(from.name(), expected)
    }

    fn expect_nothing(&mut self) -> io::Result<()> {
        self.conn.set_nonblocking(true).unwrap();
        let mut buf = [0; 1];
        let is_empty = match self.conn.peek(&mut buf) {
            Ok(n) => n == 0,
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => true,
            Err(err) => {
                self.conn.set_nonblocking(false).unwrap();
                return Err(err);
            }
        };
        self.conn.set_nonblocking(false).unwrap();
        assert!(is_empty);
        Ok(())
    }

    fn shutdown(&mut self, how: Shutdown) {
        let _ = self.conn.shutdown(how);
    }
}

#[test]
fn test_login() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(b"Bob").unwrap();

    let mut carol = Client::start(server.addr).unwrap();
    carol.login(b"Carol").unwrap();
}

#[test]
fn test_non_utf8() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(&[2, 3, 4, 5, 228]).unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(&[255, 12, 22, 33, 44]).unwrap();

    take_a_nap();

    alice.message(&bob, &[0, 2, 3, 4, 5]).unwrap();
    bob.expect_message(&alice, &[0, 2, 3, 4, 5]);
}

#[test]
fn test_message_to_each_other() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(b"Bob").unwrap();

    let mut carol = Client::start(server.addr).unwrap();
    carol.login(b"Carol").unwrap();

    take_a_nap();

    alice.message(&bob, b"Hello, Bob!").unwrap();
    bob.message(&carol, b"Hello, Carol!").unwrap();
    carol.message(&alice, b"Hello, Alice!").unwrap();
    alice.expect_message(&carol, b"Hello, Alice!");
    bob.expect_message(&alice, b"Hello, Bob!");
    carol.expect_message(&bob, b"Hello, Carol!");

    alice.message(&carol, b"Hi from Alice!").unwrap();
    bob.message(&alice, b"Hi from Bob!").unwrap();
    carol.message(&bob, b"Hi from Carol!").unwrap();
    alice.expect_message(&bob, b"Hi from Bob!");
    bob.expect_message(&carol, b"Hi from Carol!");
    carol.expect_message(&alice, b"Hi from Alice!");
}

#[test]
fn test_message_to_nowhere() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(b"Bob").unwrap();

    take_a_nap();

    alice.message_to(b"Carol", b"Hi carol").unwrap();
    bob.message_to(b"Dick", b"Hi dick").unwrap();
}

#[test]
fn test_cant_message_myself() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();
    alice.write(&alice.name().to_owned()).unwrap();
    alice.write(&[SOH]).unwrap();
    alice.write(b"Hi?").unwrap();
    alice.write(&[SOH]).unwrap();

    for _ in 0..5 {
        take_a_nap();
        alice.expect_nothing().unwrap();
    }
}

#[test]
fn test_relogin() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    let bob_name = b"Bob";
    bob.login(bob_name).unwrap();

    take_a_nap();

    alice.message(&bob, b"hi bob").unwrap();
    bob.expect_message(&alice, b"hi bob");
    std::mem::drop(bob);

    alice.message_to(bob_name, b"hi bob again").unwrap();

    take_a_nap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(bob_name).unwrap();

    for _ in 0..5 {
        take_a_nap();
        bob.expect_nothing().unwrap();
    }

    alice.message_to(bob_name, b"again hi bob again").unwrap();
    bob.expect_message(&alice, b"again hi bob again");
}

#[test]
fn test_two_talkers() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(b"Bob").unwrap();

    take_a_nap();

    let iterations = 1000;

    let mut alice_write = alice.clone();
    let alice_write_thread = thread::spawn(move || {
        for it in 0..iterations {
            alice_write
                .message_to(b"Bob", format!("to bob {}", it).as_bytes())
                .unwrap();
        }
    });

    let alice_read_thread = thread::spawn(move || {
        for it in 0..iterations {
            alice.expect_message_from(b"Bob", format!("to alice {}", it).as_bytes())
        }
        take_a_nap();
        for _ in 0..5 {
            alice.expect_nothing().unwrap();
        }
    });

    let mut bob_write = bob.clone();
    let bob_write_thread = thread::spawn(move || {
        for it in 0..iterations {
            bob_write
                .message_to(b"Alice", format!("to alice {}", it).as_bytes())
                .unwrap();
        }
    });

    let bob_read_thread = thread::spawn(move || {
        for it in 0..iterations {
            bob.expect_message_from(b"Alice", format!("to bob {}", it).as_bytes())
        }
        take_a_nap();
        for _ in 0..5 {
            bob.expect_nothing().unwrap();
        }
    });

    alice_write_thread.join().unwrap();
    alice_read_thread.join().unwrap();
    bob_write_thread.join().unwrap();
    bob_read_thread.join().unwrap();
}

#[test]
fn test_cut_in_mid_sente() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    let mut bob = Client::start(server.addr).unwrap();
    bob.login(b"Bob").unwrap();

    take_a_nap();

    alice.write(b"Bob");
    alice.write(&[SOH]);
    alice.write(b"Hi ther");
    alice.shutdown(Shutdown::Both);

    bob.expect_nothing();

    let mut alice = Client::start(server.addr).unwrap();
    alice.login(b"Alice").unwrap();

    alice.write(b"Bob");
    alice.write(&[SOH]);
    alice.write(b"Hi again");
    alice.write(&[SOH]);

    bob.expect_message(&alice, b"Hi again");
}
