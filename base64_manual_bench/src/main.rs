use std;
use std::string::String;
use std::time::SystemTime;
use std::vec::Vec;

use base64_bench::*;

/*
method                                input  output  avg time
base64::encode_config                    12      16        74
base64::encode_config                    51      68       107
base64::encode_config                   102     136       171
base64::encode_config                   501     668       568
base64::encode_config                  1002    1336      1053
crypto2::encode_with_config              12      16        66
crypto2::encode_with_config              51      68       154
crypto2::encode_with_config             102     136       266
crypto2::encode_with_config             501     668      1004
crypto2::encode_with_config            1002    1336      1934
jdk::encode                              12      16        66
jdk::encode                              51      68       139
jdk::encode                             102     136       255
jdk::encode                             501     668       965
jdk::encode                            1002    1336      1861
jdk::encode                              12      16        65
jdk::encode                              51      68       144
jdk::encode                             102     136       255
jdk::encode                             501     668       963
jdk::encode                            1002    1336      1859
data_encoding::encode                    12      16        69
data_encoding::encode                    51      68       104
data_encoding::encode                   102     136       162
data_encoding::encode                   501     668       506
data_encoding::encode                  1002    1336       947
base64::decode_config (excessive)        16      12        86
base64::decode_config (excessive)        68      51       122
base64::decode_config (excessive)       136     102       170
base64::decode_config (excessive)       668     501       577
base64::decode_config (excessive)      1336    1002      1142
base64::decode_config_buf (no alloc)     16      12        91
base64::decode_config_buf (no alloc)     68      51       140
base64::decode_config_buf (no alloc)    136     102       177
base64::decode_config_buf (no alloc)    668     501       576
base64::decode_config_buf (no alloc)   1336    1002      1092
base64::decode_config_slice (unsafe)     16      12        78
base64::decode_config_slice (unsafe)     68      51       114
base64::decode_config_slice (unsafe)    136     102       164
base64::decode_config_slice (unsafe)    668     501       571
base64::decode_config_slice (unsafe)   1336    1002      1073
base64::decode_config_slice (safe)       16      12        91
base64::decode_config_slice (safe)       68      51       125
base64::decode_config_slice (safe)      136     102       176
base64::decode_config_slice (safe)      668     501       844
base64::decode_config_slice (safe)     1336    1002      1104
crypto2::decode_with_config              16      12        94
crypto2::decode_with_config              68      51       243
crypto2::decode_with_config             136     102       437
crypto2::decode_with_config             668     501      2101
crypto2::decode_with_config            1336    1002      4111
jdk::decode                              16      12        77
jdk::decode                              68      51       150
jdk::decode                             136     102       279
jdk::decode                             668     501      1045
jdk::decode                            1336    1002      1975
data_encoding::decode                    16      12        92
data_encoding::decode                    68      51       157
data_encoding::decode                   136     102       243
data_encoding::decode                   668     501       930
data_encoding::decode                  1336    1002      1734
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

    std::println!("method                                input  output  avg time");

    for input in &payloads {
        run_benchmark::<Vec<u8>, String>("base64::encode_config               ", &input, base64_encode_config);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, String>("crypto2::encode_with_config         ", &input, crypto2_encode_config);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, Vec<u8>>("jdk::encode                         ", &input, jdk_encode);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, String>("data_encoding::encode               ", &input, data_encoding_encode);
    }

    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64::decode_config (excessive)   ", &input, base64_decode_config_buf_excessive_alloc);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64::decode_config_buf (no alloc)", &input, base64_decode_config_buf_no_prealloc);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64::decode_config_slice (unsafe)", &input, base64_decode_config_slice);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64::decode_config_slice (safe)  ", &input, base64_decode_config_slice_memset);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("crypto2::decode_with_config         ", &input, crypto2_decode_config);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("jdk::decode                         ", &input, jdk_decode);
    }
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("data_encoding::decode               ", &input, |s| data_encoding_decode(s.as_bytes()));
    }
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
