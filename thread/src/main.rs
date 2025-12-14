use anyhow::Result;
use std::{sync::mpsc, thread};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        match tx.send(val.clone()) {
            Ok(_) => println!("Sent: {} successfully", val),
            Err(e) => println!("Error sending: {}", e),
        }
    });
    let received = rx.recv()?;
    println!("Received: {}", received);
    Ok(())
}
