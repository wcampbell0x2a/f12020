use deku::DekuContainerRead;
use f12022::{Packet, PacketType};
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:20777").expect("couldn't bind to address");
    println!("connected");

    loop {
        let mut buf = [0; 5000];
        let (amt, addr) = socket.recv_from(&mut buf).unwrap();

        let buf = &mut buf[..amt];
        //println!("{:02x?}", buf);
        let (left, packet) = Packet::from_bytes((buf, 0)).unwrap();
        println!("{:#?} {:02x?}", packet, left);
    }
}
