use std;
use std::time::SystemTime;

use libc;

/*
$ cargo run -r --bin getloadavg_bench && uptime
   Compiling getloadavg_bench v0.1.0 (/home/dkomanov/src/dkomanov/rust-stuff/getloadavg_bench)
    Finished release [optimized] target(s) in 6.02s
     Running `target/release/getloadavg_bench`
LA: 1.08 0.97 0.93
Warmup is 6420000 iterations for ~3 seconds
getloadavg      465
LA: 1.07 0.97 0.93
 21:20:33 up 18 days, 12:01,  1 user,  load average: 1.07, 0.97, 0.93
 */
fn main() {
    let mut output = vec![0.0; 3];
    get_and_print();
    run_benchmark(output.as_mut_ptr());
    get_and_print();
}

#[inline]
fn getloadavg(output: *mut libc::c_double) {
    unsafe {
        let r = libc::getloadavg(output, 3);
        assert_eq!(r, 3);
    }
}

fn get_and_print() {
    let mut output = vec![0.0; 3];
    getloadavg(output.as_mut_ptr());
    std::println!("LA: {:.2} {:.2} {:.2}", output[0], output[1], output[2]);
}

fn run_benchmark(output: *mut libc::c_double) {
    static N: usize = 5_000_000;
    static WARMUP_ITERATIONS: usize = 10_000;

    let warmup_start = SystemTime::now();
    let mut warmup_iterations = 0;
    while SystemTime::now().duration_since(warmup_start).unwrap().as_secs() < 3 {
        for _ in 0..WARMUP_ITERATIONS {
            getloadavg(output);
        }
        warmup_iterations += 1;
    }
    std::println!("Warmup is {} iterations for ~3 seconds", warmup_iterations * WARMUP_ITERATIONS);

    let start = SystemTime::now();
    for _ in 0..N {
        getloadavg(output);
    }
    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    let nanos = duration.as_nanos();

    std::println!("{} {:>8}", "getloadavg", nanos / (N as u128));
}
