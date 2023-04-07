// tcp sender to echo server

use async_std::prelude::*;
use async_std::task;
use async_std::net::{TcpStream};

use std::time::Instant;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[allow(unreachable_code)]
async fn run_sender() -> Result<()> {

    let mut stream = TcpStream::connect(crate::addr).await?;
    
    let mut time_points = Box::new([Instant::now(); 100000]); // max is i32_max
    let mut time_elapsed = vec![];
    time_elapsed.reserve(time_points.len());
    let mut buf = [0; 4];

    loop {
        for i in 0..time_points.len() as i32 {
            time_points[i as usize] = Instant::now();
            stream.write(&i.to_le_bytes()).await?;
            stream.read(&mut buf).await?;
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
    Ok(())
}


async fn worker(num: u32) {

    let mut handles = vec![];
    for _ in 0..num {
        let handle = task::spawn(async move {
            run_sender().await;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await;
    }

}

pub fn run(num: u32) {
    task::block_on(async move {
        worker(num).await;
    });
}