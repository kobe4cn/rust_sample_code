use anyhow::Result;
use std::{sync::mpsc, thread};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = vec![1, 2, 3, 4, 5];
        for i in val {
            match tx.send(i) {
                Ok(_) => println!("Sent: {} successfully", i),
                Err(e) => println!("Error sending: {}", e),
            }
        }
    });
    while let Ok(received) = rx.recv() {
        println!("Received: {}", received);
    }
    Ok(())
}
