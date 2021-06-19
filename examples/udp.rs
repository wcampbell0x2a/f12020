use std::net::UdpSocket;
use deku::DekuContainerRead;

fn main() {

    let socket = UdpSocket::bind("0.0.0.0:20777").expect("couldn't bind to address");
    println!("connected");

    loop {
    let mut buf = [0; 5000];
    let (amt, addr) = socket.recv_from(&mut buf).unwrap();

    let buf = &mut buf[..amt];
    let packet = f12020::Packet::from_bytes((buf, 0));
    println!("{:#?}", packet);
    }
}
