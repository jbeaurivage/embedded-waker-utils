#[cfg(feature = "embassy")]
#[path = "waitqueue/embassy.rs"]
mod waker;

#[cfg(feature = "rtic")]
#[path = "waitqueue/rtic.rs"]
mod waker;

#[cfg(not(any(feature = "embassy", feature = "rtic")))]
#[path = "waitqueue/executor_agnostic.rs"]
mod waker;

pub use waker::*;
