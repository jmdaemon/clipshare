//use std::sync::mpsc::{Sender, Receiver, SendError, RecvError};
//use std::sync::mpsc;
//extern crate bus;
use std::{thread, vec};
use async_channel::{Sender, Receiver, SendError, RecvError};

/*

pub enum ClipboardEvent {
    Nothing,
    UpdateSent(String),
    UpdateReceived(String),
}

pub struct ClipboardChannel {
    //tx: Sender<ClipboardEvent>,
    //rx: Receiver<ClipboardEvent>,
    s: Sender<ClipboardEvent>,
    r: Receiver<ClipboardEvent>,
}

impl ClipboardChannel {
    pub fn new() -> ClipboardChannel {
        //let (tx, rx) = mpsc::channel();
    let (s, r) = async_channel::unbounded();
        //ClipboardChannel { tx, rx }
        ClipboardChannel { s, r }
    }
}

//static NTHREADS: i32 = 3;

/// Callback to notify listeners for a clipboard update
//pub fn cb_send_update(tx: Sender<ClipboardEvent>, last_copied: String) -> Result<(), SendError<ClipboardEvent>> {
//pub async fn cb_send_update(tx: Sender<ClipboardEvent>, last_copied: String) -> async_channel::Send<ClipboardEvent>{
//pub async fn cb_send_update(tx: Sender<ClipboardEvent>, last_copied: String) -> async_channel::Send<'static, ClipboardEvent> {
pub async fn cb_send_update(tx: Sender<ClipboardEvent>, last_copied: String) -> Result<(), SendError<ClipboardEvent>> {
    let event = ClipboardEvent::UpdateSent(last_copied);
    tx.send(event).await
}

/// Callback to parse notification from senders for a clipboard update
//pub fn cb_receive_update(rx: Receiver<ClipboardEvent>) -> Result<ClipboardEvent, RecvError>{
pub async fn cb_receive_update(rx: Receiver<ClipboardEvent>) -> Result<ClipboardEvent, RecvError>{
    //rx.recv()
    rx.recv().await
}

fn main() {
    //let (tx, rx) = mpsc::channel();
    //let ClipboardChannel = { };
    let cb_chan = ClipboardChannel::new();

    //let bus = bus::Bus::new(len)
    //let (tx, rx) = mpsc::channel();

    //let mut channels = vec![];
    //let mut channels = Vec::new();
    //let (s, r) = async_channel::unbounded();

    let mut messages = vec![];
    let mut threads = vec![];
    let mut thread_count = 2;

    //let (tx, rx): (Sender<ClipboardEvent>, Receiver<ClipboardEvent>) = mpsc::channel();
    //let mut children = Vec::new();


    // Setup listeners
    for thread_id in 0..thread_count {
        // Create multiple listeners
        //let rx = cb_chan.rx;
        let rx = cb_chan.r.clone();
        let thread = thread::spawn(move || {
            //threads.push(thread_id);
            //let res = rx.recv().unwrap_or(ClipboardEvent::Nothing);
            let res = rx.recv().await.unwrap_or(ClipboardEvent::Nothing);

            // Parse request
            match res {
                ClipboardEvent::UpdateSent(last_copied) => {
                    let msg = format!("Thread {} sent: {}", thread_count, last_copied);
                    println!("{}", msg);
                    messages.push(msg);
                }
                ClipboardEvent::UpdateReceived(last_copied) => {
                    let msg = format!("Thread {} received: {}", thread_count, last_copied);
                    println!("{}", msg);
                    messages.push(msg);
                }
                _ => {}
            }
            //println!("Thread {} received {}", thread_id, res.unwrap_or("Nothing"));
        });
        threads.push(thread);
    }

    // Setup senders
    thread_count += 1;
    let thread_sender = thread::spawn(move || {
        // Send update
        //cb_send_update(cb_chan.tx, String::from("string-to-be-copied"));
        cb_send_update(tx, String::from("string-to-be-copied"));
    });
    //threads.push(thread_count);
    threads.push(thread_sender);
    
    // Send an update
    //cb_send_update(cb_chan.tx, String::from("string-to-be-copied"));
    //cb_send_update(cb_chan.tx, String::from("string-to-be-copied"));

    // Perform the execution
    for thread in threads {
        thread.join().expect("oops! the child thread panicked");
    }

    //for id in 0..NTHREADS {
        //// The sender endpoint can be copied
        //let thread_tx = tx.clone();

        //// Each thread will send its id via the channel
        //let child = thread::spawn(move || {
            //// The thread takes ownership over `thread_tx`
            //// Each thread queues a message in the channel
            //thread_tx.send(id).unwrap();

            //// Sending is a non-blocking operation, the thread will continue
            //// immediately after sending its message
            //println!("thread {} finished", id);
        //});

        //children.push(child);
    //}

    //// Here, all the messages are collected
    //let mut ids = Vec::with_capacity(NTHREADS as usize);
    //for _ in 0..NTHREADS {
        //// The `recv` method picks a message from the channel
        //// `recv` will block the current thread if there are no messages available
        //ids.push(rx.recv());
    //}
    
    //// Wait for the threads to complete any remaining work
    //for child in children {
        //child.join().expect("oops! the child thread panicked");
    //}

    // Show the order in which the messages were sent

    //println!("{:?}", messages);
}

*/
