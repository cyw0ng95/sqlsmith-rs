use sqlsmith_rs_common::profile::read_profile;

mod fork_server;

fn main() {
    sqlsmith_rs_common::logger::init(); // Configure logging

    let profile = read_profile();
    profile.print();

    fork_server::fork_server_main(&profile);
}
