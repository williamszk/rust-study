// https://youtu.be/ygL_xcavzQ4?t=8105

use std::thread;
use std::time::Duration;

fn main() {
    let thread1 =  thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..20 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1))
    }

    // without this this the program will not wait for the threads to finish
    thread1.join().unwrap();


}
