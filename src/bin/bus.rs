extern crate bus;

use std::thread;

fn main() {
    let mut bus = bus::Bus::new(10);
    let mut receiver1 = bus.add_rx();
    let mut receiver2 = bus.add_rx();
    let mut receiver3 = bus.add_rx();

    let a = thread::spawn(move || {
        println!("receiver1 - {}", receiver1.recv().expect("1"));
    });
    let b = thread::spawn(move || {
        println!("receiver2 - {}", receiver2.recv().expect("2"));
    });
    let c = thread::spawn(move || {
        println!("receiver3 - {}", receiver3.recv().expect("3"));
    });

    bus.broadcast(42);
    a.join().expect("a");
    b.join().expect("b");
    c.join().expect("c");
}
