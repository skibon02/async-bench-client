
use std::net::TcpStream;
use std::io::prelude::*;

use std::thread;
use std::time::Instant;

fn run_sender() {

    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    
    let mut time_points = Box::new([Instant::now(); 100000]); // max is i32_max
    let mut time_elapsed = vec![];
    time_elapsed.reserve(time_points.len());
    let mut buf = [0; 4];

    loop {
        for i in 0..time_points.len() as i32 {
            time_points[i as usize] = Instant::now();
            stream.write(&i.to_le_bytes()).unwrap();
            stream.read(&mut buf).unwrap();
            let elapsed = time_points[i as usize].elapsed();
            time_elapsed.push(elapsed);
    
            let val = u32::from_le_bytes(buf);
            if val != i as u32 {
                println!("Error: {} != {}", val, i);
            }
        }
        //sort
        time_elapsed.sort();
        let median = time_elapsed[time_elapsed.len() / 2];
        println!("Median time: {:?}", median);
        //99th percentile
        let percentile = time_elapsed[(time_elapsed.len() as f64 * 0.99) as usize];
        println!("99th percentile: {:?}", percentile);

        //95th percentile
        let percentile = time_elapsed[(time_elapsed.len() as f64 * 0.95) as usize];
        println!("95th percentile: {:?}", percentile);

        time_elapsed.clear();
        time_elapsed.reserve(time_points.len());
    }
}

pub fn run(num: u32) {
    let mut handles = vec![];
    for _ in 0..num {
        let handle = thread::spawn(|| {
            run_sender();
        });
        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }
}