#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut headers = [httparser::EMPTY_HEADER; 16];
    let mut req = httparser::Request::new(&mut headers);
    req.parse(data);
});
