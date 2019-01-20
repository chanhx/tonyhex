use std::fs::{File, metadata};

extern crate clap;
use clap::{App, AppSettings, Arg, crate_name};

mod lib;

fn main() {
    let matches = App::new(crate_name!())
                          .setting(AppSettings::ColoredHelp)
                          .setting(AppSettings::DeriveDisplayOrder)
                          .setting(AppSettings::UnifiedHelpMessage)
                          .arg(Arg::with_name("length")
                               .short("n")
                               .help("Interpret only `length` bytes of input")
                               .takes_value(true)
                               .value_name("length"))
                          .arg(Arg::with_name("offset")
                               .short("s")
                               .help("Skip `offset` bytes from the beginning of the input")
                               .takes_value(true)
                               .value_name("offset"))
                          .arg(Arg::with_name("plain")
                               .short("p")
                               .help("Output in plain text"))
                          .arg(Arg::with_name("file")
                               .help("File to display")
                               .required(true)
                               .index(1))
                          .get_matches();

    let filename = matches.value_of("file").unwrap();
    let mut file = File::open(filename).expect("file not found");

    let metadata = metadata(filename).unwrap();
    let filesize = metadata.len();

    let initial_offset: u64 = matches.value_of("offset")
                                     .unwrap_or("0")
                                     .parse()
                                     .expect("invalid offset");

    if initial_offset > filesize {
        panic!("`offset` is greater than the file size");
    }

    let length: Option<u64> = match matches.value_of("length") {
        Some(len) => {
             let len = len.parse().expect("invalid `length` value");
             Some(if len > filesize {filesize} else {len} )
        },
        None      => None,
    };

    let plain = matches.occurrences_of("plain") > 0;

    lib::run(&mut file, filesize, initial_offset, length, plain);
}
