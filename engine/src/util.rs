use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub fn block_on<F: std::future::Future>(mut future: F) -> F::Output {
    fn noop(_: *const ()) {}
    fn clone(pointer: *const ()) -> RawWaker {
        RawWaker::new(pointer, &VTABLE)
    }
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
    let mut context = Context::from_waker(&waker);
    let mut future = unsafe { std::pin::Pin::new_unchecked(&mut future) };
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
}
