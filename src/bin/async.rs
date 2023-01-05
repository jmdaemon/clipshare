//extern crate bus;
use std::{thread, vec};
//use async_channel::{Sender, Receiver, SendError, RecvError};
use tokio::sync::oneshot;
use tokio::sync::oneshot::{Sender, Receiver};

// Execution model
// We send clipboard events over this channel
// we also listen for clipboard events over this channel

pub enum ClipboardEvent {
    Nothing,
    UpdateSent(String),
    UpdateReceived(String),
}

pub struct ClipboardChannel {
    s: Sender<ClipboardEvent>,
    r: Receiver<ClipboardEvent>,
}

impl ClipboardChannel {
    pub fn new() -> ClipboardChannel {
    //let (s, r) = async_channel::unbounded();
    let (s, r): (Sender<ClipboardEvent>, Receiver<ClipboardEvent>) = oneshot::channel();
        ClipboardChannel { s, r }
    }
}

/// Callback to notify listeners for a clipboard update
//pub async fn cb_send_update(tx: Sender<ClipboardEvent>, last_copied: String) -> Result<(), SendError<ClipboardEvent>> {
    //let event = ClipboardEvent::UpdateSent(last_copied);
    //tx.send(event).await
//}

///// Callback to parse notification from senders for a clipboard update
//pub async fn cb_receive_update(rx: Receiver<ClipboardEvent>) -> Result<ClipboardEvent, RecvError>{
    //rx.recv().await
//}

#[tokio::main]
async fn main() {
    let cb_chan = ClipboardChannel::new();
    let (s, r) = (cb_chan.s, cb_chan.r);

    s.send(ClipboardEvent::UpdateSent(String::from("asdf")));

    let res = r.await.unwrap();

    match res {
        ClipboardEvent::UpdateSent(last_copied) => {
            //let msg = format!("Thread {} sent: {}", thread_count, last_copied);
            //println!("{}", msg);
            //messages.push(msg);
            //println!("Sent {}", last_copied);
            println!("Received {}", last_copied);
        }
        ClipboardEvent::UpdateReceived(last_copied) => {
            //let msg = format!("Thread {} received: {}", thread_count, last_copied);
            //println!("{}", msg);
            //messages.push(msg);
            //println!("Received {}", last_copied);
        }
        _ => {}
    }


    

    /*
    let mut messages = vec![];
    let mut threads = vec![];
    let mut thread_count = 2;

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
        cb_send_update(cb_chan., String::from("string-to-be-copied"));
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
    */
}
