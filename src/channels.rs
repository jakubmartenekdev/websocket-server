use std::{io::{self, BufWriter, Read, Write}, sync::mpsc::{channel, sync_channel}, time::Duration};
use std::sync::mpsc;
// use std::cmp;

fn main() {

    let handles = Vec::<std::thread::JoinHandle<()>>::with_capacity(4);
    let (tx, rx) = sync_channel::<i32>(4);


    for i in 0..4 {
        let tx = tx.clone();
        std::thread::spawn(move || {
            tx.send(i).expect("receiver disconnected.");
        });
    }

    for _ in 0..4 {
        let rx = rx.recv();
        if let Ok(msg) = rx {
            println!("received message {msg}");
        }
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
