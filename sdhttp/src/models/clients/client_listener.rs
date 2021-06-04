use std::{
	io::{Read, Write},
	net::TcpStream,
	str
};

fn listen() {
	let mut tcp_stream = TcpStream::bind("localhost:8080").unwrap();

	for stream in tcp_stream.incoming() {
		let mut s = stream.unwrap();
		let mut buf = [0; 8];
		s.read(&mut buf);
		s.write(&mut buf);
	}
}