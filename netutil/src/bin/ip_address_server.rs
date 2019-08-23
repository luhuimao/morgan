use clap::{crate_version, App, Arg};

fn main() {
    morgan_logger::setup();
    let matches = App::new("morgan-ip-address-server")
        .version(crate_version!())
        .arg(
            Arg::with_name("port")
                .index(1)
                .required(true)
                .help("TCP port to bind to"),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let port = port
        .parse()
        .unwrap_or_else(|_| panic!("Unable to parse {}", port));
    let _runtime = morgan_netutil::ip_echo_server(port);
    loop {
        std::thread::park();
    }
}
