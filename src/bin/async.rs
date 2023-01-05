//extern crate bus;


use std::{thread, vec};
use futures::TryFutureExt;
use log::warn;
//use async_channel::{Sender, Receiver, SendError, RecvError};
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::{SendError, RecvError};
use tokio::sync::broadcast::{Sender, Receiver};
use tokio::time::{Duration};

// Execution model
// We send clipboard events over this channel
// we also listen for clipboard events over this channel

#[derive(Debug, Clone)]
pub enum ClipboardEvent {
    ReceiveCopied(String),
}

pub struct ClipboardChannel {
    s: Sender<ClipboardEvent>,
    r: Receiver<ClipboardEvent>,
}

impl ClipboardChannel {
    pub fn new() -> ClipboardChannel {
        let (s, r): (Sender<ClipboardEvent>, Receiver<ClipboardEvent>) = broadcast::channel(32);
        ClipboardChannel { s, r }
    }
}

/// Callback to notify listeners for a clipboard update
pub fn cb_send_update(r: Sender<ClipboardEvent>, last_copied: &str) -> Result<usize, SendError<ClipboardEvent>> {
    let event = ClipboardEvent::ReceiveCopied(String::from(last_copied));
    r.send(event)
}

/// Callback to parse notification from senders for a clipboard update
pub async fn cb_receive_update(r: &mut Receiver<ClipboardEvent>) {
    let res = r.recv().await.unwrap();
    match res {
        ClipboardEvent::ReceiveCopied(last_copied) => {
            println!("Received {}", last_copied);
        }
    }
}

#[tokio::main]
async fn main() {
    // Setup the broadcast channel
    let cb_chan = ClipboardChannel::new();
    let (s, mut r) = (cb_chan.s, cb_chan.r);

    // Setup our subscribers
    let mut dev_r2 = s.subscribe();
    let mut dev_r3 = s.subscribe();

    let last_copied = "string-to-be-copied";


    // Our subscribers will poll for data, the data will be moved in here
    let handle_dev1 = tokio::spawn(async move {
            cb_receive_update(&mut r).await;
        }).unwrap_or_else(|_| {});
    let handle_dev2 = tokio::spawn(async move {
            cb_receive_update(&mut dev_r2).await;
        }).unwrap_or_else(|_| {});
    let handle_dev3 = tokio::spawn(async move {
            cb_receive_update(&mut dev_r3).await;
        }).unwrap_or_else(|_| {});
    let handle_wait = async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
        };

    let handle_sender = async move {
            match cb_send_update(s, last_copied) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Could not send data.");
                    eprintln!("Error: {}", e);
                }
            }
    };


    tokio::join!(
        // Spawn three separate subscribers
        handle_wait,    // Wait a bit
        handle_dev1,
        handle_dev2,
        handle_dev3,
        handle_sender,  // Send one update
    );
    //tokio::spawn(async move {
        //cb_receive_update(&mut r).await;
    //});
    //tokio::spawn(async move {
        //cb_receive_update(&mut dev_r2).await;
    //});
    //tokio::spawn(async move {
        //cb_receive_update(&mut dev_r3).await;
    //});

    // Send the data if possible
    //cb_receive_update(&mut r).await;
}
