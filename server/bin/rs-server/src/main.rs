use rs_network;

fn main() {
    const MAXIMUM_CONCURRENT_USERS: usize = 100;

    rs_network::initialise_connection(MAXIMUM_CONCURRENT_USERS);
}
