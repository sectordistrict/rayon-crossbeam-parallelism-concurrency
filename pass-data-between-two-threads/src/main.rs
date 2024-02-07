use std::{thread, time};
extern crate crossbeam;
use crossbeam::channel::bounded;
use crossbeam::channel::unbounded;

fn main() {
    // let (snd, rcv) = bounded(1);
    let (snd, rcv) = unbounded();
    let n_msgs = 5;
    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100));
            }
            // drop(snd)
        });
    })
    .unwrap();
    // for msg in rcv.iter() {
    //     println!("Worker {:?} received {}.", thread::current().id(), msg);
    // }
    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}
