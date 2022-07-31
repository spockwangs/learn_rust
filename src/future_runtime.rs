use std::sync::mpsc::{
    sync_channel,
    SyncSender,
    Receiver
};
use std::sync::{
    Arc,
    Mutex
};
use std::task::{
    Wake,
    Context,
    Waker,
};
use std::future::Future;
use std::pin::Pin;
use std::thread;
use std::boxed::Box;

pub struct Executor {
    task_sender: Option<SyncSender<Arc<Task>>>,
    thread_handle: thread::JoinHandle<()>,
}

impl Executor {
    pub fn new() -> Self {
        const MAX_QUEUED_TASKS: usize = 10_000;
        let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
        Executor {
            task_sender: Some(task_sender),
            thread_handle: thread::spawn(move || {
                while let Ok(task) = ready_queue.recv() {
                    // Take the future, and if it has not yet completed (is still Some),
                    // poll it in an attempt to complete it.
                    let mut future_slot = task.future.lock().unwrap();
                    if let Some(mut future) = future_slot.take() {
                        // Create a `LocalWaker` from the task itself
                        let waker = Waker::from(task.clone());
                        let context = &mut Context::from_waker(&waker);
                        // `BoxFuture<T>` is a type alias for
                        // `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
                        // We can get a `Pin<&mut dyn Future + Send + 'static>`
                        // from it by calling the `Pin::as_mut` method.
                        if future.as_mut().poll(context).is_pending() {
                            // We're not done processing the future, so put it
                            // back in its task to be run again in the future.
                            *future_slot = Some(future);
                        }
                    }
                }
            }),
        }
    }

    pub fn spawn(&self, future: impl Future<Output=()> + Send + 'static) {
        let future = Box::pin(future);
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone().unwrap(),
        });
        self.task_sender.as_ref().unwrap().send(task).expect("task queue is full");
    }

    pub fn wait(mut self) {
        self.task_sender = None;
        self.thread_handle.join().unwrap();
    }
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl Wake for Task {
    fn wake(self: Arc<Self>) {
        self.task_sender.send(self.clone()).expect("task queue is full");
    }
}
