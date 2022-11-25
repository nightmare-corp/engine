// diagnostics for mult-threaded situations
use std::{thread};


pub fn println_current_thread_id() {
    println!("{:?}", thread::current().id());
}