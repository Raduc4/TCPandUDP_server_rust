use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to bind");
    socket.connect("127.0.0.1:3000").expect("unable to connect");
    println!("Socket peer addr is {:?}", socket.peer_addr());
    socket
        .send("hello sent using send() call".as_bytes())
        .expect("Unable to send bytes");
}
