use std;
use std::string::String;
use std::time::SystemTime;
use std::vec::Vec;

use base64_bench::*;

/*
method                                input  output  avg time
base64::encode_config                     3       4        50
base64::encode_config                    12      16        69
base64::encode_config                    51      68        98
base64::encode_config                   102     136       195
base64::encode_config                   501     668       587
base64::encode_config                  1002    1336      1060
base64::encode_config                 10002   13336      9704
crypto2::encode_with_config               3       4        44
crypto2::encode_with_config              12      16        68
crypto2::encode_with_config              51      68       137
crypto2::encode_with_config             102     136       298
crypto2::encode_with_config             501     668      1024
crypto2::encode_with_config            1002    1336      1957
crypto2::encode_with_config           10002   13336     18638
jdk::encode                               3       4        44
jdk::encode                              12      16        65
jdk::encode                              51      68       140
jdk::encode                             102     136       275
jdk::encode                             501     668       988
jdk::encode                            1002    1336      1888
jdk::encode                           10002   13336     17954
jdk::encode_measter                       3       4        44
jdk::encode_measter                      12      16        62
jdk::encode_measter                      51      68       107
jdk::encode_measter                     102     136       243
jdk::encode_measter                     501     668       793
jdk::encode_measter                    1002    1336      1488
jdk::encode_measter                   10002   13336     14037
data_encoding::encode                     3       4        59
data_encoding::encode                    12      16        69
data_encoding::encode                    51      68       103
data_encoding::encode                   102     136       196
data_encoding::encode                   501     668       539
data_encoding::encode                  1002    1336       973
data_encoding::encode                 10002   13336      8832
base64_simd::Base64::encode_type          3       4        27
base64_simd::Base64::encode_type         12      16        39
base64_simd::Base64::encode_type         51      68        47
base64_simd::Base64::encode_type        102     136        56
base64_simd::Base64::encode_type        501     668       110
base64_simd::Base64::encode_type       1002    1336       235
base64_simd::Base64::encode_type      10002   13336      1425
base64::decode_config (excessive)         4       3        82
base64::decode_config (excessive)        16      12        84
base64::decode_config (excessive)        68      51       121
base64::decode_config (excessive)       136     102       173
base64::decode_config (excessive)       668     501       571
base64::decode_config (excessive)      1336    1002      1117
base64::decode_config (excessive)     13336   10002     10164
base64::decode_config_buf (no alloc)      4       3        63
base64::decode_config_buf (no alloc)     16      12        93
base64::decode_config_buf (no alloc)     68      51       129
base64::decode_config_buf (no alloc)    136     102       179
base64::decode_config_buf (no alloc)    668     501       580
base64::decode_config_buf (no alloc)   1336    1002      1080
base64::decode_config_buf (no alloc)  13336   10002     10189
base64::decode_config_slice (unsafe)      4       3        46
base64::decode_config_slice (unsafe)     16      12        72
base64::decode_config_slice (unsafe)     68      51       107
base64::decode_config_slice (unsafe)    136     102       163
base64::decode_config_slice (unsafe)    668     501       562
base64::decode_config_slice (unsafe)   1336    1002      1074
base64::decode_config_slice (unsafe)  13336   10002     10060
base64::decode_config_slice (safe)        4       3        60
base64::decode_config_slice (safe)       16      12        88
base64::decode_config_slice (safe)       68      51       122
base64::decode_config_slice (safe)      136     102       175
base64::decode_config_slice (safe)      668     501       585
base64::decode_config_slice (safe)     1336    1002      1172
base64::decode_config_slice (safe)    13336   10002     10208
crypto2::decode_with_config               4       3        49
crypto2::decode_with_config              16      12        88
crypto2::decode_with_config              68      51       249
crypto2::decode_with_config             136     102       477
crypto2::decode_with_config             668     501      2149
crypto2::decode_with_config            1336    1002      4257
crypto2::decode_with_config           13336   10002     41615
jdk::decode                               4       3        52
jdk::decode                              16      12        65
jdk::decode                              68      51       137
jdk::decode                             136     102       249
jdk::decode                             668     501      1007
jdk::decode                            1336    1002      1939
jdk::decode                           13336   10002     18520
data_encoding::decode                     4       3        65
data_encoding::decode                    16      12        77
data_encoding::decode                    68      51       149
data_encoding::decode                   136     102       221
data_encoding::decode                   668     501       785
data_encoding::decode                  1336    1002      1480
data_encoding::decode                 13336   10002     14023
base64_simd::Base64::decode_type          4       3        30
base64_simd::Base64::decode_type         16      12        40
base64_simd::Base64::decode_type         68      51        59
base64_simd::Base64::decode_type        136     102        45
base64_simd::Base64::decode_type        668     501       116
base64_simd::Base64::decode_type       1336    1002       179
base64_simd::Base64::decode_type      13336   10002      1349
 */
fn main() {
    let encoded: Vec<String> = get_all_test_data().iter().map(|td| td.encoded.clone()).collect();
    let payloads: Vec<Vec<u8>> = encoded.iter().map(|s| base64_decode_config(s)).collect();

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
        run_benchmark::<Vec<u8>, Vec<u8>>("jdk::encode_measter                 ", &input, jdk_encode_measter);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, String>("data_encoding::encode               ", &input, data_encoding_encode);
    }
    for input in &payloads {
        run_benchmark::<Vec<u8>, Vec<u8>>("base64_simd::Base64::encode_type    ", &input, base64simd_encode_to_vec);
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
    for input in &encoded {
        run_benchmark::<String, Vec<u8>>("base64_simd::Base64::decode_type    ", &input, base64simd_decode);
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
