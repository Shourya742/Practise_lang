/// This program demonstrates how a single mio instance can be used to
/// receive both system events (e.g. file descriptor events) and 
/// non-system events (e.g. events sourced on user-space threads other
/// than the thread running the mio poll). We listen for incoming UDP
/// datagrams on port 2000, and also listen for events created by our
/// timer thread every three seconds.
/// 
/// Running this program on Linux via strace shows how mio notifies the
/// polling thread of the non-system events by writing to a pipe:
/// 
// 28365 write(6, "\1", 1)                 = 1
// 28365 nanosleep({3, 0},  <unfinished ...>
// 28364 <... epoll_wait resumed> [{EPOLLIN, {u32=4294967295, u64=18446744073709551615}}], 16, -1) = 1
// 28364 read(5, "\1", 128)                = 1
// 28364 read(5, 0x7ffc96a72cf8, 128)      = -1 EAGAIN (Resource temporarily unavailable)
// 28364 write(1, "after poll\n", 11)      = 11
// 28364 write(1, "3-second timer\n", 15)  = 15

use std::{thread, time::Duration};

use mio::{event::Evented, Events, PollOpt, Ready, Registration, Token};


const MAX_MESSAGE_SIZE: usize = 1500;
const MAX_EVENTS: usize = 16;
const ECHO_PORT: u16 = 2000;
const TIMER_INTERVAL_SECONDS: u64 = 3;

struct PeriodicTimer {
    registration: Registration,
    set_readiness: mio::SetReadiness
}

impl PeriodicTimer {
    /// Create a PeriodicTime and begin signalling readiness at the specified interval.
    fn new(interval: u64) -> PeriodicTimer {
        let (registration, set_readiness) = Registration::new2();
        let set_readiness_clone = set_readiness.clone();

        thread::spawn(move || loop {
            let now  = Instant::now();
            let when = now + Duration::from_secs(interval);
            if now < when {
                thread::sleep(when - now);
            }
            set_readiness_clone.set_readiness(Ready::readable()).unwrap()
        });

        PeriodicTimer { registration, set_readiness }
    }

    /// Clear the read readiness of this timer.
    fn reset(&self) {
        self.set_readiness.set_readiness(Ready::empty()).unwrap();
    }
}

/// Proxy Evented functions to the Registration.
impl Evented for PeriodicTimer {
    fn register(&self, poll: &mio::Poll, token: mio::Token, interest: Ready, opts: mio::PollOpt) -> std::io::Result<()> {
        self.registration.register(poll, token, interest, opts)
    }

    fn reregister(&self, poll: &mio::Poll, token: mio::Token, interest: Ready, opts: mio::PollOpt) -> std::io::Result<()> {
        self.registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &mio::Poll) -> std::io::Result<()> {
        <Registration as Evented>::deregister(&self.registration, poll)
    }
}

fn main() {
    let localhost = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    // Create and bind the socket
    let socket = UdpSocket::bind(&SocketAddr::new(localhost, ECHO_PORT)).unwrap();

    let poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(MAX_EVENTS);
    poll.register(&socket, Token(0), Ready::readable(), PollOpt::level()).unwrap();
    let timer = PeriodicTimer::new(TIMER_INTERVAL_SECONDS);
    poll.register(&timer, Token(1), Ready::readable(), PollOpt::level()).unwrap();

    loop {
        println!("Before poll()");
        poll.poll(&mut events, None).unwrap();
        println!("after poll(");

        for event in &events {
            assert!(event.token() == Token(0) || event.token() == Toke(1));
            assert!(event.readiness().is_readable());
            match event.token() {
                Token(0) => {
                    let mut inbuf = [0u8; MAX_MESSAGE_SIZE];
                    let (nbytes, addr) = socket.recv_from(&mut inbuf).unwrap();
                    println!("recv {} bytes from {}.", nbytes, addr);
                }
                Token(1) => {
                    println!("{}-second timer", TIMER_INTERVAL_SECONDS);
                    timer.reset();;
                }
                Token(_) => {
                    panic!("Unknown token in poll.");
                }
            }
        }
    }

}