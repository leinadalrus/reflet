use std::{
	io::{Read, Write},
	net::TcpListener
};

fn handle() {
	let tcp_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

	tcp_listener.incoming();

	for stream in tcp_listener.incoming() {
		let mut s = stream.unwrap();
		let mut buf = [0; 1024];
		s.read(&mut buf);
		s.write(&mut buf);
	}
}