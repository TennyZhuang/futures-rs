//! Asynchronous channels.
//!
//! This crate provides channels that can be used to communicate between
//! asynchronous tasks.

#![feature(pin, arbitrary_self_types, futures_api)]

#![deny(missing_docs, missing_debug_implementations)]
#![doc(html_root_url = "https://docs.rs/futures-channel/0.2.0")]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate futures_core;

macro_rules! if_std {
    ($($i:item)*) => ($(
        #[cfg(feature = "std")]
        $i
    )*)
}

if_std! {
    mod lock;
    pub mod mpsc;
    pub mod oneshot;
}