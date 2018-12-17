use std::sync::mpsc;  // multi-producer single-consumer
use std::thread;
use std::time::Duration;

fn main() {
        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);
        // tx.send(()).unwrap();

        thread::spawn(move || {
                let val = String::from("hi");
                tx.send(val).unwrap();  // thread send
                // println!("{}", val);  // !! error for val had moved

                let vals = vec![
                        String::from("hi"),
                        String::from("from"),
                        String::from("the"),
                        String::from("thread"),
                ];
                for val in vals {
                        tx.send(val).unwrap();
                        thread::sleep(Duration::from_secs(1));
                }
        });

        // clone the transmiter
        thread::spawn(move || {
                let vals = vec![
                        String::from("more"),
                        String::from("messages"),
                        String::from("for"),
                        String::from("you"),
                ];
                for val in vals {
                        tx1.send(val).unwrap();
                        thread::sleep(Duration::from_secs(1));
                }
        });

        let v = rx.recv().unwrap();  // main thread receive
        println!("Got {}!", v);
        // disorder receiving when multi-senders
        for val in rx {  // wait the message -- blocking
                println!("Got {}!", val);
        }
}

// channel to transmite data between thread
// channel is said to be closed if either the transmitter or reciver half is dropped.

