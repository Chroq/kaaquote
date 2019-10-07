#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod crawler {
    extern crate hyper;

    use std::io::{self, Write};
    use hyper::Client;
    use hyper::rt::{self, Future, Stream, run};

    pub struct Quote {
        text: String,
        actor: String,
    }

    pub fn generate(url: hyper::Uri) -> Vec<Quote> {
        let content = self::get_src(url);

        let v: Vec<Quote> = Vec::new();

        v
    }

    fn get_src(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
        let client = Client::new();

        client
            .get(url)
            .and_then(|res| {
                res.into_body().for_each(|chunk| {
                    io::stdout().write_all(&chunk)
                        .map_err(|e| panic!("example expects stdout is open, error={}", e))
                })
            })
            .map(|_| {
                format!("\n\nDone.");
            })
            .map_err(|err| {
                format!("Error {}", err);
            })
    }
}
