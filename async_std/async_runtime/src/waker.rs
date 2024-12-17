use std::task::{RawWaker, RawWakerVTable};

static VTABLE: RawWakerVTable = RawWakerVTable::new(my_clone, my_wake, my_wake_by_ref, my_drop);


/// This usually gets called when we poll a function, 
/// as we need to clone an atomic reference of our waker in
/// our executor to wrap in a context and pass into the future
/// being polled.
unsafe fn my_clone(raw_waker: *const()) -> RawWaker {
    RawWaker::new(raw_waker, &VTABLE)
}

/// The wake and wake_by_red functions are called when the future should be polled again
/// because the future that is being waited on is ready. We are just going to be polling
/// our futures without being prompted by the waker to see if they are ready.
/// 
/// This my_wake function converts the raw pointer back to a box and drops it.
/// This makes sense as the my_wake function is supposed to consume the Waker.
unsafe fn my_wake(raw_waker: *const()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}

/// The my_wake_by_ref function is the same as the my_wake function but does not consume the waker. If we wanted
/// to experiment with notifying the executor, we could have an AtomicBool that we set to true in the functions
/// , and come up with some form of mechanism of the executor to check the AtomicBool before bothering to poll
/// the future, as checking an AtomicBool is lighter than polling a future. We could also have another queue where
/// notification could be sent for task readiness, but for our server implementation we are going to stick with polling
/// without checking before the poll is performed.
unsafe fn my_wake_by_ref(_raw_waker: *const()) {}


/// When our task is finished or has been cancelled, we no longer need to poll our task
/// and our waker is dropped.
unsafe fn my_drop(raw_waker: *const()) {
    drop(Box::from_raw(raw_waker as *mut u32));
}


/// We could have multiple different `RawWakerTable` definitions, we could
/// construct different function tables to the RawWaker depending on what we pass into
/// create_raw_waker function. In our executor, we could vary the input depending on the type of 
/// future we are processing. We could also pass in a reference to a data structure that our
/// executor is holding instead of the u32 number 42. That data structure can be referenced in our functions 
/// bound to the table, as this is the data that is passed into our functions is bound to the table.
pub fn create_raw_waker() -> RawWaker {
    let data = Box::into_raw(Box::new(42u32));
    RawWaker::new(data as *const (), &VTABLE)
}