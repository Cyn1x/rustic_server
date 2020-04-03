pub fn handle_response<'a>(request: &str, handler: &'a rs_handler::Handler) -> &'a str {
    handler.handle_request(request)
}

fn process_response() {

}
