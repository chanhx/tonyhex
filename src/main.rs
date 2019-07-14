use std::fs::metadata;

#[macro_use]
extern crate clap;
use clap::App;

mod lib;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();

    let filename = m.value_of("file").unwrap();
    let filesize = metadata(filename).unwrap().len();

    let offset: u64 = m.value_of("offset")
                       .unwrap_or("0")
                       .parse()
                       .expect("invalid offset");

    if offset > filesize {
        panic!("`offset` is greater than the file size");
    }

    let length: Option<u64> = match m.value_of("length") {
        Some(len) => {
            let len = len.parse().expect("invalid `length` value");
            Some(if len > filesize {filesize} else {len} )
        },
        None      => None,
    };

    let plain = m.occurrences_of("plain") > 0;

    lib::run(filename, filesize, offset, length, plain);
}
