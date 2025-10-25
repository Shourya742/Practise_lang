use std::{
    collections::{HashMap, hash_map::Entry},
    io::ErrorKind,
    net::{SocketAddr, ToSocketAddrs},
    pin::Pin,
    sync::{Arc, Mutex, OnceLock, mpsc},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use mio::{Events, Interest, Registry, Token};

pub(crate) struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    spawner: Spawner,
}

pub struct Executor {
    ready_queue: mpsc::Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future = task.future.lock().unwrap();

            let waker = Arc::clone(&task).waker();
            let mut context = Context::from_waker(&waker);

            let _ = future.as_mut().poll(&mut context);
        }
    }
}

#[derive(Clone)]
pub struct Spawner {
    task_sender: mpsc::SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            spawner: self.clone(),
        });
        self.spawn_task(task)
    }

    pub(crate) fn spawn_task(&self, task: Arc<Task>) {
        self.task_sender.send(task).expect("Too many tasks queued");
    }
}

pub fn new_executor_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASK: usize = 10_000;

    let (task_sender, ready_queue) = mpsc::sync_channel(MAX_QUEUED_TASK);

    (Executor { ready_queue }, Spawner { task_sender })
}

fn clone(ptr: *const ()) -> RawWaker {
    let original: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };

    let cloned = original.clone();

    std::mem::forget(original);
    std::mem::forget(cloned);

    RawWaker::new(ptr, &Task::WAKER_VTABLE)
}

fn drop(ptr: *const ()) {
    let _: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
}

fn wake(ptr: *const ()) {
    let arc: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
    let spawner = arc.spawner.clone();
    spawner.spawn_task(arc);
}

fn wake_by_ref(ptr: *const ()) {
    let arc: Arc<Task> = unsafe { Arc::from_raw(ptr as _) };
    arc.spawner.spawn_task(arc.clone());

    std::mem::forget(arc);
}

impl Task {
    const WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    pub fn waker(self: Arc<Self>) -> Waker {
        let opaque_ptr = Arc::into_raw(self) as *const ();
        let vtable = &Self::WAKER_VTABLE;
        unsafe { Waker::from_raw(RawWaker::new(opaque_ptr, vtable)) }
    }
}

pub enum Status {
    Awaited(Waker),
    Happened,
}

pub struct Reactor {
    registry: Registry,
    statues: Mutex<HashMap<Token, Status>>,
}

impl Reactor {
    pub fn get() -> &'static Self {
        static REACTOR: OnceLock<Reactor> = OnceLock::new();

        REACTOR.get_or_init(|| {
            let poll = mio::Poll::new().unwrap();
            let reactor = Reactor {
                registry: poll.registry().try_clone().unwrap(),
                statues: Mutex::new(HashMap::new()),
            };

            std::thread::Builder::new()
                .name("reactor".to_owned())
                .spawn(|| run(poll))
                .unwrap();

            reactor
        })
    }

    pub fn poll(&self, token: Token, cx: &mut Context) -> Poll<std::io::Result<()>> {
        let mut guard = self.statues.lock().unwrap();
        match guard.entry(token) {
            Entry::Vacant(vacant) => {
                vacant.insert(Status::Awaited(cx.waker().clone()));
                Poll::Pending
            }
            Entry::Occupied(mut occupied) => match occupied.get() {
                Status::Awaited(waker) => {
                    if !waker.will_wake(cx.waker()) {
                        occupied.insert(Status::Awaited(cx.waker().clone()));
                    }
                    Poll::Pending
                }
                Status::Happened => {
                    occupied.remove();
                    Poll::Ready(Ok(()))
                }
            },
        }
    }

    fn unique_token(&self) -> Token {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static CURRENT_TOKEN: AtomicUsize = AtomicUsize::new(0);

        Token(CURRENT_TOKEN.fetch_add(1, Ordering::Relaxed))
    }
}

fn run(mut poll: mio::Poll) -> ! {
    let reactor = Reactor::get();
    let mut events: Events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();
        for event in &events {
            let mut guard = reactor.statues.lock().unwrap();
            let previous = guard.insert(event.token(), Status::Happened);

            if let Some(Status::Awaited(waker)) = previous {
                waker.wake();
            }
        }
    }
}

pub struct UdpSocket {
    socket: mio::net::UdpSocket,
    token: Token,
}

impl UdpSocket {
    pub fn bind(addr: impl ToSocketAddrs) -> std::io::Result<Self> {
        let std_socket = std::net::UdpSocket::bind(addr)?;
        std_socket.set_nonblocking(true)?;

        let mut socket = mio::net::UdpSocket::from_std(std_socket);

        let reactor = Reactor::get();
        let token = reactor.unique_token();
        Reactor::get().registry.register(
            &mut socket,
            token,
            Interest::READABLE | Interest::WRITABLE,
        )?;
        Ok(UdpSocket { socket, token })
    }

    pub async fn send_to(&self, buf: &[u8], dest: SocketAddr) -> std::io::Result<usize> {
        loop {
            match self.socket.send_to(buf, dest) {
                Ok(value) => return Ok(value),
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    std::future::poll_fn(|cx| Reactor::get().poll(self.token, cx)).await?;
                }
                Err(error) => return Err(error),
            }
        }
    }

    pub async fn recv_from(&self, buf: &mut [u8]) -> std::io::Result<(usize, SocketAddr)> {
        loop {
            match self.socket.recv_from(buf) {
                Ok(value) => return Ok(value),
                Err(error) if error.kind() == ErrorKind::WouldBlock => {
                    std::future::poll_fn(|cx| Reactor::get().poll(self.token, cx)).await?
                }
                Err(error) => return Err(error),
            }
        }
    }
}

impl Drop for UdpSocket {
    fn drop(&mut self) {
        let _ = Reactor::get().registry.deregister(&mut self.socket);
    }
}