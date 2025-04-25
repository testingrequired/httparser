fn main() {
    env_logger::init();

    let mut headers = [httparser::EMPTY_HEADER; 1];
    let mut req = httparser::Request::new(&mut headers);

    let buf = b"GET /index.html HTTP/1.1\r\n{{key}}: example.com\r\n\r\n";
    assert!(req.parse(buf).unwrap().is_complete());
}
