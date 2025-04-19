#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut headers = [httparser::EMPTY_HEADER; 16];
    let mut resp = httparser::Response::new(&mut headers);
    let _ = resp.parse(data);
});
