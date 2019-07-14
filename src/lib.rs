use std::{
    cmp::min,
    error::Error,
    fs::File,
    io::{prelude::*, SeekFrom},
};

#[inline]
fn color(b: u8) -> u8 {
    match b {
        0x00 => 37,                     // null
        0x20 => 92,                     // space
        0x21 ... 0x7e => 96,            // printable char
        0x01 ... 0x1f | 0x7f => 95,     // control char
        _ => 93,                        // nonascii
    }
}

fn line(
    offset: u64,
    offset_width: usize,
    buf: [u8; 16],
    n: usize,
    plain: bool,
) -> String {
    let hexes = buf.iter().enumerate().take(n).map(|(i, &b)| {
        let s = match i {
            8 => format!(" {:02x}", b),
            _ => format!("{:02x}", b),
        };

        if plain {
            s
        } else {
            format!("\x1b[{}m{}", color(b), s)
        }
    }).collect::<Vec<String>>().join(" ");

    let chars = buf.iter().take(n).map(|&b| {
        let s = match b {
            0x21 ..= 0x7e => format!("{}", b as char),
            _ => ".".to_string(),
        };

        if plain {
            s
        } else {
            format!("\x1b[{}m{}", color(b), s)
        }
    }).collect::<String>();

    if plain {
        format!("{:0width$x}  {:48}  {}", offset, hexes, chars, width=offset_width)
    } else {
        format!("{:0width0$x}  {:width1$}  {}\x1b[0m",
            offset, hexes, chars, width0=offset_width, width1=48+n*5)
    }
}

fn offset_bits_count(mut offset: u64) -> usize {
    let mut count = 8;
    offset >>= 32;

    while offset != 0 {
        count += 1;
        offset >>= 4;
    }

    count
}

pub fn run(
    filename: &str,
    filesize: u64,
    offset: u64,
    length: Option<u64>,
    plain: bool,
) {
    let mut file = match File::open(filename) {
        Err(err) => panic!("couldn't open {}: {}", filename, err.description()),
        Ok(file) => file,
    };

    let max_offset = match length {
        Some(len) => offset + len,
        None => filesize,
    };

    let offset_width = offset_bits_count(max_offset);

    let mut buf = [0u8; 16];
    let mut offset = if offset > 0 {
        file.seek(SeekFrom::Start(offset)).unwrap()
    } else {
        0
    };

    loop {
        let n = min(
            file.read(&mut buf).expect("error reading file") as u64,
            max_offset - offset,
        );

        println!("{}", line(offset, offset_width, buf, n as usize, plain));

        offset += n;
        if n < 16 || offset == max_offset {break;}
    }
}
