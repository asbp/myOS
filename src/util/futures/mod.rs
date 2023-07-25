use futures_util::task::AtomicWaker;

pub static WAKER: AtomicWaker = AtomicWaker::new();