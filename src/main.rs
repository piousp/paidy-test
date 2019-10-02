use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
mod client;
mod database;
mod item;
mod server;

fn main() {
    match std::env::var("PORT") {
        Ok(port) => server::init(port),
        Err(_) => panic!("The environment variable `PORT` is not defined")
    }
}

pub fn original_simulation() {
    let restaurant = Arc::new(Mutex::new(database::Restaurant::new()));

    let mut handles = vec![];

    for i in 0..25 {
        let rest = Arc::clone(&restaurant);
        let handle = thread::spawn(move || {
            let client = client::Client::new(i);
            for i in 0..=rand::thread_rng().gen_range(1, 4) {
                match Arc::clone(&rest).lock() {
                    Ok(val) => {
                        println!("Job #{:?} for waitress #{}", i, client.id);
                        client.do_job(val);
                        thread::sleep(client.wait_some_time());
                    }
                    Err(_) => println!(
                        "Kitchen was busy and didn't listen to waitress #{}",
                        client.id
                    ),
                };
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
