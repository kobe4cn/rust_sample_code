use ::thread::{Button, Page, SelectBox};
use anyhow::Result;
// use std::sync::Mutex;

fn main() -> Result<()> {
    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // let handle1 = thread::spawn(move || {
    //     let val = vec![6, 7, 8, 9, 10];
    //     for i in val {
    //         match tx1.send(i) {
    //             Ok(_) => println!("Sent: {} successfully", i),
    //             Err(e) => println!("Error sending: {}", e),
    //         }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // let handle2 = thread::spawn(move || {
    //     let val = vec![1, 2, 3, 4, 5];
    //     for i in val {
    //         match tx.send(i) {
    //             Ok(_) => println!("Sent: {} successfully", i),
    //             Err(e) => println!("Error sending: {}", e),
    //         }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });
    // handle2.join().unwrap_or_else(|e| {
    //     println!("Error joining thread: {:?}", e);
    // });
    // handle1.join().unwrap_or_else(|e| {
    //     println!("Error joining thread: {:?}", e);
    // });

    // while let Ok(received) = rx.recv() {
    //     println!("Received: {}", received);
    // }

    // let m = Mutex::new(5);
    // {
    //     let mut num = m.lock().unwrap();
    //     *num += 1;
    // }
    // println!("num: {:?}", m);

    // let mut average_collection = AverageCollection::new();
    // average_collection.add(1);
    // average_collection.add(2);
    // average_collection.add(3);
    // average_collection.add(4);
    // average_collection.add(5);
    // println!("average: {}", average_collection.average());

    // let mut page = Page::new();
    let page = Page {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 50,
                label: "Click me".to_string(),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 50,
                options: vec![
                    "Option 1".to_string(),
                    "Option 2".to_string(),
                    "Option 3".to_string(),
                ],
            }),
        ],
    };
    page.render();

    Ok(())
}
