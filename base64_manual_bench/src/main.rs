use std;
use std::string::String;
use std::time::SystemTime;
use std::vec::Vec;

use base64;

use base64_bench::{jdk_decode, jdk_encode};
use crypto2_base64 as crypto2;

/*
method                       input  output  avg time
baseline                         5       1        24
base64::encode_config           12      16        76
base64::encode_config           51      68       120
base64::encode_config          102     136       184
base64::encode_config          501     668       584
base64::encode_config         1002    1336      1078
crypto2::encode_with_config     12      16        67
crypto2::encode_with_config     51      68       145
crypto2::encode_with_config    102     136       274
crypto2::encode_with_config    501     668      1101
crypto2::encode_with_config   1002    1336      1977
jdk::encode                     12      16        70
jdk::encode                     51      68       138
jdk::encode                    102     136       262
jdk::encode                    501     668      1004
jdk::encode                   1002    1336      1913
base64::decode_config           16      12        85
base64::decode_config           68      51       119
base64::decode_config          136     102       174
base64::decode_config          668     501       584
base64::decode_config         1336    1002      1163
base64::decode_config_slice     16      12        76
base64::decode_config_slice     68      51       152
base64::decode_config_slice    136     102       165
base64::decode_config_slice    668     501       586
base64::decode_config_slice   1336    1002      1075
crypto2::decode_with_config     16      12        96
crypto2::decode_with_config     68      51       239
crypto2::decode_with_config    136     102       442
crypto2::decode_with_config    668     501      1934
crypto2::decode_with_config   1336    1002      4014
jdk::decode                     16      12        75
jdk::decode                     68      51       150
jdk::decode                    136     102       254
jdk::decode                    668     501      1007
jdk::decode                   1336    1002      1985
 */
fn main() {
    let encoded = vec![
        "cmdWcTRWSGtmYUhx".to_string(),
        "ZXBueFd1alQwRUZkaGk3bVNXOHhReThoZGRWd1ZQa2llSUxzVG9HakYxYnptbUFmUjlp".to_string(),
        "Y2hqMHk1OVd3MXBiZlVFeFpVa1M3dXhVcmlVZVpUWWpBYVZEYmE0dndwbWxtTkRVY2RraHNNeWtVSDZjNmRNUjB0SjNVZDdKcVdhTlhkM2pxZGU3TEw4R3lObWU2T2VUbWxsaWNV".to_string(),
        "S0R3R3AwYnAxNGc2UnpueXhzMUVuNkc1MEFaMThPMTg5TWlHaWduMUVrcUFKa05Fem1kTjRFSlAyVUVOOTY2MVpTSDZpZHM3VTdZRFJEZ2JXckZSTHRLa05iRnQ0cHJHclliMTlOZkU5bGlKcnZiR0IxVG5tR0dHckNEVFFqSFo5UVk2VW16UVpkbEJieWNvUEl6YmtuYkF0OVg3c2FKWGZzYlhvMmNSSGdSR0FQQVVxeGU2Tk03RUpBSXN2VnZXRnBlN2hDekFWa0dVS0ZxdmM4cVlIWTNwbEI1RXJoSVdyVXpZekJyU3JOU2UyOEkzbU1GS2dTNVF0Y2Y2WDRCMmpWNFNYVmg1V3V1ZkR3ZFpiZnFnMlFZU2xUZEE1bG1IRXdVMURTMDR2U0FsTmNHRXpWckxPYmNlV0tucUxjRTNZV2x0b1pHbk9JcXRoNXZEaEhjQm00Zm5xN2huclh3NDFXVFB2QjJ1aXR0UmtOaHdocTBlSnhlMHJtb2RyZ2c3SnIzakFwZWNUYTlQRnV6NE9PZDR3TmNMeHBqaTBmNUtDc24yOHJsRUxwN1Z2M0F4Nk45dkVvSnhrY0FlWW9RUjlrdkhZMW9kN3loa2pWaUM0c2xwb3NjWUJjMEVndEdacW5VUWIyRkVobEJDeUhrdXp2MWNl".to_string(),
        "VVBEQ3VGNW0yRlBQUGdaeXh6YVdGWXYxMG5UVWYyM1M1OFU1a0RDeWRYRjRyUDdBN1dFUGdROUt3akc1WDA2VGo2MXQ3Uk5HNFRDWEpYR0JrVm1pSGhack91RmNnVDR4TDRBc0R4d3dtd2N2bHlKU1RqUXZkem43MHd1SXhYZVVBbnI5bUw4TjhyalBnWG5CakRNZWNhZUUxOElWbXcyZ3F1MjFHaDR1N0ZZeHcwSG1SQUtRcFFQWFNGNzVka2Jjb3NVT1AzT1VNVjd6ZFNxYTVhckRDM2pUT3F4N0llaVc1TzNIa2hqblo0dVdJeElVUnZOaVppbDZZNXNCUTJUSkpldldQN1dIaVdOWEprYU92WU9qMjg0Z0NHeXptYlZJN3ZvSnM0eVdLZUdNdWhSYk9Sczg5aEk2dmlVY25ZNmoyVHE5Q2UyYlNLMlZtRkxQaVd0bzdFQ0IzYzBDeFZsbUtaVGtyOHRtMUpUT2hadVltOXlVbkQxdHRzc3ZSemcwZVhFMVdjREZjUmZ1dEZINENHeEdqYU9sUTJxaktkTElaQ0ZBejcxNnJMa096SkdHSDRrRXVza2UyVDhPT3R4Y2xWZ3JHcktNMTh5YXJiS0xBalBYYkJudWlHVTRUUFMxUDkzZHJyRExIbklJTlpDeTExdXJyNHFEMWNhODkxZUlzZ1FFQ3F6d0piY1NiTFlFMTVZNzhqS1VtdmYwQ2RpNW1ZTTM0MlVKU3QyVjZQSHRybkFmZXVvT1o2ZGtKNjlrV014ck10ZldES0VzZ01jNHlLU1hudEpIZHhpbFpYVFNwZk5Lb0hLWk9IbWRKSEFuRVlVWUNnTk9waExiQ2NRNHNKazQ3aWpGTWZRWklPV1Q2Y1NDdHI1VzlHS0dBb1BPb2dPQnczMmFDektJbjR4dmhxREdPZ3ZMcTUwR3Y4QzdKYk1ZODdEMHdtTGNhS2FON1JCbFU1bGRUWDFqdmlGNGZmV21IQ2Y1VkhMRzJXbjZWS0xyMXBqVlFwdExiZlZITGxoTjd1WTR1WkdjSmRuNm1ZTmtmTWhPSWFRWHJ2d0tTaFU1RkZucTlzZHI5bnBCYUNaSU4xNEM0bFpId09KUXoySzl6OHJtSkRZSlQ4R0RLVFNLeDBVNmJaR0ZBM3VzN0dxelBoc0VKUXhpWktIbVRlekpQVTRPUlAwYm5TUzgwd2FaSlR4V3Zic0hUeXEyTjQ0RjZYM0lhcTMwaHdpVGtBTDdoUWRoWVBYUGEybXl2Z0lvNE16UWtwUFBPTms2bzlOM3lCUjA1TVk0VXZPbmdOZ0R2RUZXZ2xaMlpwZmpoQ1lhZnFxdVRv".to_string(),
    ];
    let payloads: Vec<Vec<u8>> = encoded.iter().map(|s| base64::decode(s).unwrap()).collect();

    std::println!("method                       input  output  avg time");

    run_benchmark("baseline                   ", &"empty".to_string(), baseline);

    for input in &payloads {
        run_benchmark::<Vec<u8>, String>("base64::encode_config      ", &input, base64_encode_config);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, String>("crypto2::encode_with_config", &input, crypto2_encode_config);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, Vec<u8>>("jdk::encode                ", &input, jdk_encode);
    }

    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64::decode_config      ", &input, base64_decode_config);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64::decode_config_slice", &input, base64_decode_config_slice);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("crypto2::decode_with_config", &input, crypto2_decode_config);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("jdk::decode                ", &input, jdk_decode);
    }
}

fn baseline(_s: &String) -> Vec<u8> {
    vec![0]
}

fn base64_encode_config(s: &Vec<u8>) -> String {
    base64::encode_config(s, base64::STANDARD_NO_PAD)
}

fn crypto2_encode_config(s: &Vec<u8>) -> String {
    crypto2::encode_with_config(s, crypto2::DEFAULT_CONFIG)
}

fn base64_decode_config_slice(s: &String) -> Vec<u8> {
    let mut buffer = Vec::<u8>::with_capacity(s.len() * 3 / 4);
    unsafe {
        let mut sl = std::slice::from_raw_parts_mut(buffer.as_mut_ptr(), buffer.capacity());
        let size = base64::decode_config_slice(s, base64::STANDARD_NO_PAD, &mut sl).unwrap();
        assert_eq!(size, buffer.capacity());
        buffer.set_len(size);
    }
    buffer
}

fn base64_decode_config(s: &String) -> Vec<u8> {
    base64::decode_config(s, base64::STANDARD_NO_PAD).unwrap()
}

fn crypto2_decode_config(s: &String) -> Vec<u8> {
    crypto2::decode_with_config(s, crypto2::DEFAULT_CONFIG).unwrap()
}

fn run_benchmark<I: AsRef<[u8]>, O: AsRef<[u8]>>(name: &str, input: &I, f: fn(&I) -> O) {
    static N: usize = 500_000;

    let warmup_start = SystemTime::now();
    while SystemTime::now().duration_since(warmup_start).unwrap().as_secs() < 3 {
        for _ in 0..10_000 {
            let r = f(input);
            assert!(r.as_ref().len() > 0, "name = {}, len = {}", name, r.as_ref().len());
        }
    }

    let start = SystemTime::now();
    for _ in 0..N {
        let r = f(input);
        assert!(r.as_ref().len() > 0, "name = {}, len = {}", name, r.as_ref().len());
    }
    let end = SystemTime::now();

    let duration = end.duration_since(start).unwrap();
    let nanos = duration.as_nanos();

    std::println!("{}  {:>5}  {:>6}  {:>8}", name, input.as_ref().len(), f(input).as_ref().len(), nanos / (N as u128));
}
