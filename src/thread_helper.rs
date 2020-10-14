use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::JoinHandle;

pub type StopFn = Box<dyn Fn() -> ()>;

pub fn run_in_thread<F: Fn(Receiver<bool>) -> T + Send + 'static, T: Send + 'static>(
    fun: F,
    thead_name: String,
) -> (StopFn, JoinHandle<T>) {
    let (sender, receiver) = channel::<bool>();

    let handle = thread::spawn(move || {
        println!("Starting {}", &thead_name);

        let res = fun(receiver);

        println!("Stopping {}", &thead_name);

        res
    });

    (
        Box::new(move || sender.send(true).unwrap_or_default()),
        handle,
    )
}
