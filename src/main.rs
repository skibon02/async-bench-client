
mod v_async;
use v_async::run as run_async;

mod v_std;
use v_std::run as run_std;

fn main() {
    run_std(1);
}

//both are std:
// 1: 8.8, 10, 15           110 k/s
// 8: 15.9, 15.9, 17.5      490 k/s
// 16: 31

// std lib client: 
// 1: 14, 20, 21            61k/s 
// 4: 19, 30, 42
// 8: 30, 52, 82            260k/s
// 9: 33, 59, 106
// 18: 56, 110, 387
// 40: 114, 221, 648        307k/s

//async std client:
// 1: 23, 28, 35            42k/s (-42%)
// 8: 47, 70, 100           165k/s (-36%)
// 16: 77, 125, 161         
// 40: 183, 294, 375        206k/s (-33%)