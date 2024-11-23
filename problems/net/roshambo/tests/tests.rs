use rand::Rng;
use std::io::{self, prelude::*};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpStream};
use std::process::{Child, Command};
use std::str::{self, FromStr};
use std::thread;
use std::time;

const BINARY_PATH: &str = env!("CARGO_BIN_EXE_roshambo");

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

    fn expected_to_be_dead(&mut self) {
        let _ = self.stop();
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
struct Player {
    conn: TcpStream,
}

impl Player {
    fn start(server_addr: SocketAddr) -> std::io::Result<Self> {
        let conn = TcpStream::connect(server_addr)?;
        Ok(Self { conn })
    }

    fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.conn.write_all(data)
    }

    fn read(&mut self, bytes: usize) -> std::io::Result<Vec<u8>> {
        let mut buf = vec![0; bytes];
        self.conn.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn shutdown(&mut self, how: Shutdown) {
        let _ = self.conn.shutdown(how);
    }
}

#[test]
fn test_rounds() {
    let server = ServerWrapper::start(IpVersion::V4);

    let mut player_a = Player::start(server.addr).unwrap();
    let mut player_b = Player::start(server.addr).unwrap();

    player_a.write(b"P").unwrap();
    player_b.write(b"S").unwrap();
    assert_eq!(player_a.read(2).unwrap(), b"SL");
    assert_eq!(player_b.read(2).unwrap(), b"PW");

    player_b.write(b"P").unwrap();
    player_a.write(b"S").unwrap();
    assert_eq!(player_b.read(2).unwrap(), b"SL");
    assert_eq!(player_a.read(2).unwrap(), b"PW");

    player_a.write(b"R").unwrap();
    player_b.write(b"S").unwrap();
    assert_eq!(player_b.read(2).unwrap(), b"RL");
    assert_eq!(player_a.read(2).unwrap(), b"SW");

    // even rounds
    player_b.write(b"R").unwrap();
    player_a.write(b"R").unwrap();
    assert_eq!(player_b.read(2).unwrap(), b"RD");
    assert_eq!(player_a.read(2).unwrap(), b"RD");

    player_a.write(b"S").unwrap();
    player_b.write(b"S").unwrap();
    assert_eq!(player_b.read(2).unwrap(), b"SD");
    assert_eq!(player_a.read(2).unwrap(), b"SD");

    player_b.write(b"P").unwrap();
    player_a.write(b"P").unwrap();
    assert_eq!(player_b.read(2).unwrap(), b"PD");
    assert_eq!(player_a.read(2).unwrap(), b"PD");
}

#[test]
fn test_client_closing_write_connection() {
    let mut server = ServerWrapper::start(IpVersion::V4);

    let mut player_a = Player::start(server.addr).unwrap();
    let mut player_b = Player::start(server.addr).unwrap();

    player_a.write(b"R").unwrap();
    player_b.write(b"S").unwrap();
    assert_eq!(player_a.read(2).unwrap(), b"SW");
    assert_eq!(player_b.read(2).unwrap(), b"RL");

    player_a.shutdown(Shutdown::Write);
    player_b.shutdown(Shutdown::Write);

    // server unable to read messages from clients
    thread::sleep(time::Duration::from_millis(1000));
    assert!(!server.is_alive());
    server.expected_to_be_dead();
}

#[test]
fn test_client_closing_both_connection() {
    let mut server = ServerWrapper::start(IpVersion::V4);

    let mut player_a = Player::start(server.addr).unwrap();
    let mut player_b = Player::start(server.addr).unwrap();

    player_a.write(b"R").unwrap();
    player_b.write(b"S").unwrap();
    assert_eq!(player_a.read(2).unwrap(), b"SW");
    assert_eq!(player_b.read(2).unwrap(), b"RL");

    player_a.shutdown(Shutdown::Both);
    player_b.shutdown(Shutdown::Both);

    // server unable to send messages to clients
    thread::sleep(time::Duration::from_millis(1000));
    assert!(!server.is_alive());
    server.expected_to_be_dead();
}

#[test]
fn test_invalid_messages() {
    let mut server = ServerWrapper::start(IpVersion::V4);

    let mut player_a = Player::start(server.addr).unwrap();
    let mut player_b = Player::start(server.addr).unwrap();

    player_a.write(b"A").unwrap();
    player_b.write(b"S").unwrap();

    thread::sleep(time::Duration::from_millis(1000));
    assert!(!server.is_alive());
    server.expected_to_be_dead();
}

#[test]
fn test_max_two_connections_accepted() {
    let server = ServerWrapper::start(IpVersion::V4);

    let _player_a = Player::start(server.addr).unwrap();
    let _player_b = Player::start(server.addr).unwrap();
    match Player::start(server.addr) {
        Ok(mut player) => {
            let mut buf = [0];
            assert_eq!(
                player.conn.read(&mut buf).unwrap_err().kind(),
                io::ErrorKind::ConnectionReset
            );
        }
        Err(err) => {
            assert!(matches!(
                err.kind(),
                io::ErrorKind::ConnectionReset | io::ErrorKind::ConnectionRefused
            ));
        }
    }
}
