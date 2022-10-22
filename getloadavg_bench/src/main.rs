use std;
use std::time::SystemTime;

use libc;

/*
$ cargo run -r --bin getloadavg_bench && uptime
    Finished release [optimized] target(s) in 0.05s
     Running `target/release/getloadavg_bench`
LA: 1.42 1.18 0.92
getloadavg      469
LA: 1.38 1.17 0.92
 21:07:56 up 18 days, 11:49,  1 user,  load average: 1.38, 1.17, 0.92
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
    static N: usize = 1_000_000;

    let warmup_start = SystemTime::now();
    while SystemTime::now().duration_since(warmup_start).unwrap().as_secs() < 3 {
        for _ in 0..10_000 {
            getloadavg(output);
        }
    }

    let start = SystemTime::now();
    for _ in 0..N {
        getloadavg(output);
    }
    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    let nanos = duration.as_nanos();

    std::println!("{} {:>8}", "getloadavg", nanos / (N as u128));
}
