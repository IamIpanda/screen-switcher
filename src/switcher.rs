use std::fmt::Display;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

pub struct Connection<A: ToSocketAddrs + Send + Sync> {
    socket: UdpSocket,
    target: A
}

impl<A: ToSocketAddrs + Copy + Display + Send + Sync> Connection<A> {
    pub fn new(target: A) -> Self {
        Self {
            socket: UdpSocket::bind("0.0.0.0:0").expect("Failed to get a port."),
            target
        }
    }

    pub fn send(&self, message: &[u8]) {
        println!("Send: {} to {}", String::from_utf8_lossy(message), self.target);
        match self.socket.send_to(message, self.target) {
            Ok(_) => (),
            Err(err) => println!("{}", err)
        };
    }

    pub fn set(&self, from: u8, to: u8) {
        if to == 127 {
            self.send(format!("{}All.", from).as_bytes())
        } else {
            self.send(format!("{}V{}.", from, to).as_bytes())
        }
    }

    pub fn reset(&self) {
        self.send("All#.".as_bytes())
    }

    pub fn save(&self, pos: u8) {
        self.send(format!("Save{:0>2}", pos).as_bytes())
    }

    pub fn load(&self, pos: u8) {
        self.send(format!("Recall{:0>2}", pos).as_bytes())
    }
}
