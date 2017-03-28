extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

fn main() {
    let mut core = ::tokio_core::reactor::Core::new().unwrap();

    let client = ::hyper::Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()))
        .build(&core.handle());

    let res = core.run(client.get("https://www.rust-lang.org".parse().unwrap())).unwrap();
    // assert_eq!(*res.status(), ::hyper::Ok);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
