#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut headers = [httparser::EMPTY_HEADER; 16];
    let mut resp = httparser::Response::new(&mut headers);
    let _ = httparser::ParserConfig::default()
        .allow_multiple_spaces_in_response_status_delimiters(true)
        .parse_response(&mut resp, data);
});
