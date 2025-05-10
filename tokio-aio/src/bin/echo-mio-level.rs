use std::{collections::VecDeque, net::{IpAddr, Ipv4Addr, SocketAddr}};

use mio::{net::UdpSocket, Events, Poll, PollOpt, Ready, Token};

/// A simple UDP echo server using the cross-platform mio crate to
/// multiplex reads and writes. This program uses level-triggered events.


const MAX_MESSAGE_SIZE: usize = 1500;
const MAX_OUTGOING_MESSAGES: usize = 8;
const MAX_EVENTS: usize = 16;
const ECHO_PORT: u16 = 2000;

struct Message {
    buffer: Vec<u8>,
    addr: SocketAddr,
}


fn main() {
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0,1));

    let socket = UdpSocket::bind(&SocketAddr::new(localhost, ECHO_PORT)).unwrap();

    // set up mio polling

    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(MAX_EVENTS);
    poll.register(&socket, Token(0), Ready::readable(), PollOpt::level()).unwrap();

    let mut outgoing_queue: VecDeque<Message> = VecDeque::new();
    loop {
        if outgoing_queue.is_empty() {
            poll.reregister(&socket, Token(0), Ready::readable(), PollOpt::level()).unwrap();
        } else {
            poll.reregister(&socket, Token(0), Ready::readable() | Ready::writable(), PollOpt::level()).unwrap();
        }

        poll.poll(&mut events, None).unwrap();
        for event in &events {
            assert!(event.token() == Token(0));
            if event.readiness().is_readable() {
                let mut inbuf = [0u8; MAX_MESSAGE_SIZE];
                let (nbytes, addr) = socket.recv_from(&mut inbuf).unwrap();
                println!("recv {} bytes from {}.", nbytes, addr);
                if outgoing_queue.len() > MAX_OUTGOING_MESSAGES {
                    println!("Outgoing buffers exhausted; dropping packets");
                } else {
                    outgoing_queue.push_back(Message {buffer: inbuf[0..nbytes].to_vec(), addr});
                    println!("total pending writes: {}", outgoing_queue.len());
                }
            }
            if event.readiness().is_writable() {
                let message  = outgoing_queue.pop_front().unwrap();
                let nbytes = socket.send_to(&message.buffer, &message.addr).unwrap();
                println!("Sent {} bytes to {}.", nbytes, message.addr);
            }
        }
    }
}