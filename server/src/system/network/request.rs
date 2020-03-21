pub fn handle_request(buffer: &[u8; 1024]) -> (&str, &str) {
    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    }
}
