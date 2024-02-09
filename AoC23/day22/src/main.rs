use std::{
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{Receiver, SyncSender},
        Arc, Mutex, MutexGuard,
    },
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use futures::{future::BoxFuture, task::ArcWake};

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // Implement `wake` by sending this task back onto the task channel
        // so that it will be polled again by the executor.
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

fn main() {
    let mut five = Arc::new(Mutex::new(5));

    for (i, _) in (0..10).enumerate() {
        let f = Arc::clone(&five);
        thread::spawn(move || {
            let mut data = f.lock().unwrap();
            *data += i;
        });
    }

    thread::sleep(Duration::from_secs(1));
    let ff = five.lock().unwrap();
    println!("{}", ff);
}
