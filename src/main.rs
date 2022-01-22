use std::fs::metadata;

use anyhow::Result;
use clap::{app_from_crate, AppSettings, Arg};

mod types;
mod viewer;

fn main() -> Result<()> {
    let app = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .args(&[
            Arg::new("length")
                .short('n')
                .takes_value(true)
                .help("the input file to use"),
            Arg::new("offset")
                .short('s')
                .takes_value(true)
                .help("Skip `offset` bytes from the beginning of the input"),
            Arg::new("plain").short('p').help("Output in plain text"),
            Arg::new("file")
                .index(1)
                .takes_value(true)
                .help("File to display"),
        ]);
    let m = app.get_matches();

    let filename = m.value_of("file").unwrap();
    let filesize = metadata(filename).unwrap().len();

    let offset: u64 = m
        .value_of("offset")
        .unwrap_or("0")
        .parse()
        .expect("invalid offset");

    if offset > filesize {
        panic!("`offset` is greater than the file size");
    }

    let length: Option<u64> = match m.value_of("length") {
        Some(len) => {
            let len = len.parse().expect("invalid `length` value");
            Some(if len > filesize { filesize } else { len })
        }
        None => None,
    };

    let plain = m.occurrences_of("plain") > 0;

    viewer::run(filename, filesize, offset, length, plain)
}
